use domain::{Error, Result};
use std::collections::HashMap;
use tracing::debug;

/// Variable substitution engine
pub struct VariableSubstitutor;

impl VariableSubstitutor {
    /// Simple text-based substitution (MVP approach)
    /// Replaces {{variable}} with actual values
    ///
    /// NOTE: This is a simple implementation for text-based files.
    /// For real PDF manipulation, use a PDF library like lopdf or printpdf.
    pub fn substitute_simple(
        content: &[u8],
        variables: &HashMap<String, String>,
    ) -> Result<Vec<u8>> {
        // Convert to string (assuming UTF-8 text file for MVP)
        let mut text = String::from_utf8(content.to_vec())
            .map_err(|e| Error::InvalidDocument(format!("Not UTF-8: {}", e)))?;

        debug!("Original content length: {} bytes", content.len());

        // Replace each variable
        for (key, value) in variables {
            let placeholder = format!("{{{{{}}}}}", key);
            let occurrences = text.matches(&placeholder).count();

            if occurrences > 0 {
                debug!("Replacing '{}' with '{}' ({} occurrences)", placeholder, value, occurrences);
                text = text.replace(&placeholder, value);
            }
        }

        let result = text.into_bytes();
        debug!("Substituted content length: {} bytes", result.len());

        Ok(result)
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
