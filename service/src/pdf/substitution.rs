use domain::{Error, Result};
use lopdf::{Document, Object, ObjectId};
use std::collections::HashMap;
use tracing::{debug, warn};

/// PDF variable substitution engine using lopdf
pub struct VariableSubstitutor;

impl VariableSubstitutor {
    /// Replace {{variable}} placeholders in PDF with actual values
    /// Uses lopdf for proper PDF manipulation
    pub fn substitute_in_pdf(
        pdf_bytes: &[u8],
        variables: &HashMap<String, String>,
    ) -> Result<Vec<u8>> {
        debug!("Loading PDF document ({} bytes)", pdf_bytes.len());

        // Load PDF document
        let mut doc = Document::load_mem(pdf_bytes)
            .map_err(|e| Error::InvalidDocument(format!("Failed to load PDF: {}", e)))?;

        debug!("PDF loaded successfully, {} pages", doc.get_pages().len());

        let mut replacements_made = 0;

        // Iterate through all pages
        let pages: Vec<(u32, Object)> = doc
            .get_pages()
            .into_iter()
            .map(|(page_num, page_id)| (page_num, doc.get_object(page_id).unwrap().clone()))
            .collect();

        for (page_num, _page_obj) in pages {
            debug!("Processing page {}", page_num);

            // Get page content streams
            let page_id = doc.page_iter().nth((page_num - 1) as usize).unwrap();
            let content_streams = Self::get_page_content_streams(&doc, page_id)?;

            for (stream_id, content) in content_streams {
                // Convert content to string
                let content_str = String::from_utf8_lossy(&content);
                let mut modified = content_str.to_string();
                let mut changed = false;

                // Replace variables
                for (key, value) in variables {
                    let placeholder = format!("{{{{{}}}}}", key);
                    if modified.contains(&placeholder) {
                        debug!(
                            "Replacing '{}' with '{}' on page {}",
                            placeholder, value, page_num
                        );
                        modified = modified.replace(&placeholder, value);
                        changed = true;
                        replacements_made += 1;
                    }
                }

                // Update content if changed
                if changed {
                    let new_content = modified.into_bytes();
                    // Replace the stream content
                    if let Ok(stream_obj) = doc.get_object_mut(stream_id) {
                        *stream_obj = Object::Stream(lopdf::Stream::new(
                            lopdf::Dictionary::new(),
                            new_content,
                        ));
                    }
                }
            }
        }

        debug!(
            "Variable substitution complete: {} replacements made",
            replacements_made
        );

        // Save modified PDF
        let mut output = Vec::new();
        doc.save_to(&mut output)
            .map_err(|e| Error::Internal(format!("Failed to save PDF: {}", e)))?;

        Ok(output)
    }

    /// Get content streams for a page
    fn get_page_content_streams(
        doc: &Document,
        page_id: ObjectId,
    ) -> Result<Vec<(ObjectId, Vec<u8>)>> {
        let mut streams = Vec::new();

        if let Ok(page_obj) = doc.get_object(page_id) {
            if let Ok(page_dict) = page_obj.as_dict() {
                // Get Contents
                if let Ok(contents_ref) = page_dict.get(b"Contents") {
                    match contents_ref {
                        Object::Reference(ref_id) => {
                            if let Ok(content_obj) = doc.get_object(*ref_id) {
                                if let Ok(stream) = content_obj.as_stream() {
                                    if let Ok(decoded) = stream.decompressed_content() {
                                        streams.push((*ref_id, decoded));
                                    }
                                }
                            }
                        }
                        Object::Array(ref_array) => {
                            for item in ref_array {
                                if let Object::Reference(ref_id) = item {
                                    if let Ok(content_obj) = doc.get_object(*ref_id) {
                                        if let Ok(stream) = content_obj.as_stream() {
                                            if let Ok(decoded) = stream.decompressed_content() {
                                                streams.push((*ref_id, decoded));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(streams)
    }

    /// Simple text-based substitution (fallback for non-PDF files)
    /// Replaces {{variable}} with actual values
    pub fn substitute_simple(
        content: &[u8],
        variables: &HashMap<String, String>,
    ) -> Result<Vec<u8>> {
        // Try PDF substitution first
        match Self::substitute_in_pdf(content, variables) {
            Ok(result) => Ok(result),
            Err(e) => {
                warn!("PDF substitution failed, falling back to text mode: {}", e);
                // Fallback to simple text replacement
                let mut text = String::from_utf8(content.to_vec())
                    .map_err(|e| Error::InvalidDocument(format!("Not UTF-8: {}", e)))?;

                debug!("Original content length: {} bytes", content.len());

                for (key, value) in variables {
                    let placeholder = format!("{{{{{}}}}}", key);
                    let occurrences = text.matches(&placeholder).count();

                    if occurrences > 0 {
                        debug!(
                            "Replacing '{}' with '{}' ({} occurrences)",
                            placeholder, value, occurrences
                        );
                        text = text.replace(&placeholder, value);
                    }
                }

                let result = text.into_bytes();
                debug!("Substituted content length: {} bytes", result.len());

                Ok(result)
            }
        }
    }

    /// Validate that all required variables are provided
    pub fn validate_variables(
        required: &[String],
        provided: &HashMap<String, String>,
    ) -> Result<()> {
        let mut missing = Vec::new();

        for var in required {
            if !provided.contains_key(var) {
                missing.push(var.clone());
            }
        }

        if !missing.is_empty() {
            return Err(Error::InvalidDocument(format!(
                "Missing required variables: {}",
                missing.join(", ")
            )));
        }

        Ok(())
    }

    /// Extract variable names from template content
    pub fn extract_variables(content: &[u8]) -> Result<Vec<String>> {
        // Try to load as PDF first
        if let Ok(doc) = Document::load_mem(content) {
            return Self::extract_variables_from_pdf(&doc);
        }

        // Fallback to text extraction
        let text = String::from_utf8(content.to_vec())
            .map_err(|e| Error::InvalidDocument(format!("Not UTF-8: {}", e)))?;

        let mut variables = Vec::new();
        let mut chars = text.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '{' && chars.peek() == Some(&'{') {
                chars.next(); // consume second '{'

                let mut var_name = String::new();
                let mut found_closing = false;

                while let Some(c) = chars.next() {
                    if c == '}' && chars.peek() == Some(&'}') {
                        chars.next(); // consume second '}'
                        found_closing = true;
                        break;
                    }
                    var_name.push(c);
                }

                if found_closing && !var_name.is_empty() {
                    let trimmed = var_name.trim().to_string();
                    if !variables.contains(&trimmed) {
                        variables.push(trimmed);
                    }
                }
            }
        }

        Ok(variables)
    }

    /// Extract variables from PDF document
    fn extract_variables_from_pdf(doc: &Document) -> Result<Vec<String>> {
        let mut variables = Vec::new();

        for (_page_num, page_id) in doc.get_pages() {
            if let Ok(content_streams) = Self::get_page_content_streams(doc, page_id) {
                for (_stream_id, content) in content_streams {
                    let content_str = String::from_utf8_lossy(&content);

                    // Extract {{variable}} patterns
                    let mut chars = content_str.chars().peekable();
                    while let Some(c) = chars.next() {
                        if c == '{' && chars.peek() == Some(&'{') {
                            chars.next();
                            let mut var_name = String::new();
                            let mut found_closing = false;

                            while let Some(c) = chars.next() {
                                if c == '}' && chars.peek() == Some(&'}') {
                                    chars.next();
                                    found_closing = true;
                                    break;
                                }
                                var_name.push(c);
                            }

                            if found_closing && !var_name.is_empty() {
                                let trimmed = var_name.trim().to_string();
                                if !variables.contains(&trimmed) {
                                    variables.push(trimmed);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(variables)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_substitution() {
        let template = b"Hello {{name}}, your fee is {{fee}}.";
        let mut vars = HashMap::new();
        vars.insert("name".to_string(), "John".to_string());
        vars.insert("fee".to_string(), "$49.99".to_string());

        let result = VariableSubstitutor::substitute_simple(template, &vars).unwrap();
        let text = String::from_utf8(result).unwrap();

        assert_eq!(text, "Hello John, your fee is $49.99.");
    }

    #[test]
    fn test_extract_variables() {
        let template = b"Hello {{name}}, your plan is {{plan}} at {{price}}/month.";
        let vars = VariableSubstitutor::extract_variables(template).unwrap();

        assert_eq!(vars.len(), 3);
        assert!(vars.contains(&"name".to_string()));
        assert!(vars.contains(&"plan".to_string()));
        assert!(vars.contains(&"price".to_string()));
    }

    #[test]
    fn test_validate_variables() {
        let required = vec!["name".to_string(), "fee".to_string()];
        let mut provided = HashMap::new();
        provided.insert("name".to_string(), "John".to_string());

        let result = VariableSubstitutor::validate_variables(&required, &provided);
        assert!(result.is_err());

        provided.insert("fee".to_string(), "$49.99".to_string());
        let result = VariableSubstitutor::validate_variables(&required, &provided);
        assert!(result.is_ok());
    }
}
