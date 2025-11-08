# PDF Generation Pipeline

## Overview

The PDF generation pipeline transforms templates into final signed documents through multiple stages:

```
┌──────────────────────────────────────────────────────┐
│           PDF Generation Pipeline                     │
└──────────────────────────────────────────────────────┘

Stage 1: Template Storage
┌─────────────────────────────┐
│ Original Template PDF       │
│ - Contains {{variables}}    │
│ - Signature placeholders    │
│ - Stored in Walrus         │
└──────────┬──────────────────┘
           │ blob_id: ABC123
           │
Stage 2: Instance Creation
           │
           ↓
┌─────────────────────────────┐
│ PDF Generator Service       │
│                             │
│ 1. Fetch template (ABC123)  │
│ 2. Load instance data       │
│    {                        │
│      customer_name: "John", │
│      monthly_fee: "$49.99"  │
│    }                        │
│ 3. Replace {{variables}}    │
│ 4. Generate unsigned PDF    │
└──────────┬──────────────────┘
           │
           ↓
┌─────────────────────────────┐
│ Unsigned Contract PDF       │
│ - All variables filled      │
│ - Empty signature boxes     │
│ - Stored in Walrus         │
└──────────┬──────────────────┘
           │ blob_id: XYZ789
           │
Stage 3: Signature Collection
           │
           ↓
┌─────────────────────────────┐
│ Signature Aggregator        │
│                             │
│ Collects from blockchain:   │
│ - Signer 1: 0xAAA (signed)  │
│ - Signer 2: 0xBBB (signed)  │
│ - Timestamps                │
│ - Transaction digests       │
└──────────┬──────────────────┘
           │
           ↓
┌─────────────────────────────┐
│ PDF Signer Service          │
│                             │
│ 1. Fetch unsigned PDF       │
│ 2. Add signature blocks:    │
│    - Signer name/address    │
│    - Signature hash         │
│    - Timestamp              │
│    - QR code (verification) │
│ 3. Generate final PDF       │
└──────────┬──────────────────┘
           │
           ↓
┌─────────────────────────────┐
│ Fully Signed Contract PDF   │
│ - All data filled           │
│ - All signatures present    │
│ - Blockchain verifiable     │
│ - Stored in Walrus         │
└──────────┬──────────────────┘
           │ blob_id: FINAL456
           │
           ↓
        Immutable
```

## Component Design

### 1. PDF Generator Service (Rust)

Located in: `service/src/pdf/`

```rust
pub struct PdfGenerator {
    walrus: Arc<WalrusClient>,
}

impl PdfGenerator {
    /// Generate unsigned PDF from template + instance data
    pub async fn generate_from_template(
        &self,
        template_blob_id: &BlobId,
        variable_data: &HashMap<String, String>,
    ) -> Result<Vec<u8>> {
        // 1. Fetch template from Walrus
        let template_pdf = self.walrus.read(template_blob_id).await?;

        // 2. Parse PDF
        let mut document = Self::parse_pdf(&template_pdf)?;

        // 3. Replace variables
        Self::substitute_variables(&mut document, variable_data)?;

        // 4. Prepare signature fields
        Self::add_signature_placeholders(&mut document)?;

        // 5. Render to bytes
        let unsigned_pdf = document.to_bytes()?;

        Ok(unsigned_pdf)
    }

    /// Generate signed PDF with signature blocks
    pub async fn generate_signed_pdf(
        &self,
        unsigned_blob_id: &BlobId,
        signatures: &[SignatureData],
    ) -> Result<Vec<u8>> {
        // 1. Fetch unsigned PDF
        let unsigned_pdf = self.walrus.read(unsigned_blob_id).await?;

        // 2. Parse PDF
        let mut document = Self::parse_pdf(&unsigned_pdf)?;

        // 3. Add signature information
        for sig in signatures {
            Self::add_signature_block(&mut document, sig)?;
        }

        // 4. Add verification QR code
        Self::add_verification_qr(&mut document)?;

        // 5. Render to bytes
        let signed_pdf = document.to_bytes()?;

        Ok(signed_pdf)
    }
}
```

### 2. Variable Substitution Engine

```rust
pub struct VariableSubstitutor;

impl VariableSubstitutor {
    /// Replace {{variable}} placeholders with actual values
    pub fn substitute(
        pdf_content: &mut PdfDocument,
        variables: &HashMap<String, String>,
    ) -> Result<()> {
        // Iterate through all text objects in PDF
        for page in pdf_content.pages_mut() {
            for text_object in page.text_objects_mut() {
                let content = text_object.text();

                // Find {{variable}} patterns
                let mut replaced = content.to_string();
                for (key, value) in variables {
                    let placeholder = format!("{{{{{}}}}}", key);
                    replaced = replaced.replace(&placeholder, value);
                }

                // Update text
                text_object.set_text(&replaced);
            }
        }

        Ok(())
    }

    /// Validate all required variables are provided
    pub fn validate_variables(
        template: &ContractTemplate,
        provided: &HashMap<String, String>,
    ) -> Result<()> {
        for var in &template.variables {
            if !provided.contains_key(var) {
                return Err(Error::InvalidDocument(
                    format!("Missing required variable: {}", var)
                ));
            }
        }
        Ok(())
    }
}
```

### 3. Signature Block Generator

```rust
pub struct SignatureBlockGenerator;

impl SignatureBlockGenerator {
    /// Add signature block to PDF
    pub fn add_signature(
        pdf: &mut PdfDocument,
        signature: &SignatureData,
        position: SignaturePosition,
    ) -> Result<()> {
        let signature_block = format!(
            "───────────────────────────────\n\
             Signed by: {}\n\
             Address: {}\n\
             Date: {}\n\
             Signature Hash: {}\n\
             Tx Digest: {}\n\
             ───────────────────────────────",
            signature.signer_name,
            signature.sui_address,
            signature.signed_at.format("%Y-%m-%d %H:%M:%S UTC"),
            signature.signature_hash.as_str(),
            signature.transaction_digest,
        );

        // Add text at specified position
        pdf.add_text_at_position(
            &signature_block,
            position.x,
            position.y,
            FontSize(10),
        )?;

        // Add QR code for verification
        let verification_url = format!(
            "https://suiscan.xyz/mainnet/tx/{}",
            signature.transaction_digest
        );
        let qr_code = Self::generate_qr_code(&verification_url)?;
        pdf.add_image_at_position(&qr_code, position.qr_x, position.qr_y)?;

        Ok(())
    }

    fn generate_qr_code(data: &str) -> Result<Vec<u8>> {
        // Use qrcode crate to generate QR code
        // Returns PNG bytes
        todo!("Implement QR code generation")
    }
}
```

## Implementation Options

### Option A: Rust PDF Libraries (Recommended)

```toml
# Add to service/Cargo.toml
[dependencies]
lopdf = "0.32"          # PDF manipulation
printpdf = "0.7"        # PDF generation
qrcode = "0.14"         # QR code generation
image = "0.25"          # Image handling
```

**Pros:**
- Full control, no external dependencies
- Fast, runs in-process
- No API rate limits

**Cons:**
- Complex PDF manipulation
- Need to handle PDF quirks

### Option B: External PDF Service

```rust
pub struct ExternalPdfService {
    api_key: String,
    endpoint: String,
}

impl ExternalPdfService {
    pub async fn fill_template(
        &self,
        template_url: &str,
        variables: &HashMap<String, String>,
    ) -> Result<Vec<u8>> {
        // Call external API (e.g., DocuSpring, PDFMonkey)
        let response = reqwest::Client::new()
            .post(&format!("{}/templates/fill", self.endpoint))
            .json(&json!({
                "template_url": template_url,
                "data": variables
            }))
            .send()
            .await?;

        Ok(response.bytes().await?.to_vec())
    }
}
```

**Pros:**
- Easier implementation
- Professional PDF handling
- Signature support built-in

**Cons:**
- External dependency
- API costs
- Latency

### Option C: Hybrid Approach (BEST)

Use Rust for simple operations, external service for complex:

```rust
pub enum PdfGenerationStrategy {
    Native,      // Use Rust libraries
    External,    // Use external service
    Auto,        // Choose based on complexity
}

pub struct PdfGeneratorOrchestrator {
    native_generator: NativePdfGenerator,
    external_service: Option<ExternalPdfService>,
}

impl PdfGeneratorOrchestrator {
    pub async fn generate(
        &self,
        template: &Template,
        data: &InstanceData,
        strategy: PdfGenerationStrategy,
    ) -> Result<Vec<u8>> {
        match strategy {
            PdfGenerationStrategy::Native => {
                self.native_generator.generate(template, data).await
            }
            PdfGenerationStrategy::External => {
                self.external_service
                    .as_ref()
                    .ok_or(Error::Internal("External service not configured".into()))?
                    .generate(template, data)
                    .await
            }
            PdfGenerationStrategy::Auto => {
                // Use native for simple templates, external for complex
                if template.complexity_score() > 7 {
                    self.generate(template, data, PdfGenerationStrategy::External).await
                } else {
                    self.generate(template, data, PdfGenerationStrategy::Native).await
                }
            }
        }
    }
}
```

## Data Flow

### Instance Creation Flow

```rust
// API Handler
pub async fn create_instance(
    State(state): State<AppState>,
    Path(template_id): Path<String>,
    Json(request): Json<CreateInstanceRequest>,
) -> ApiResponse<CreateInstanceResponse> {
    // 1. Get template from blockchain
    let template = state.sui.get_template(&template_id).await?;

    // 2. Validate payment
    // ... payment validation ...

    // 3. Validate variables
    VariableSubstitutor::validate_variables(&template, &request.variable_data)?;

    // 4. Generate unsigned PDF
    let unsigned_pdf = state.pdf_generator
        .generate_from_template(
            &template.template_blob_id,
            &request.variable_data,
        )
        .await?;

    // 5. Store unsigned PDF in Walrus
    let unsigned_blob_id = state.walrus.store(unsigned_pdf).await?;

    // 6. Create instance on Sui
    let instance_id = state.sui
        .create_instance(
            &template_id,
            &unsigned_blob_id,
            &request.variable_data,
            &request.required_signers,
        )
        .await?;

    // 7. Return response
    ApiResponse::success(CreateInstanceResponse {
        instance_id: instance_id.to_string(),
        generated_blob_id: unsigned_blob_id.as_str().to_string(),
        document_url: format!("/instances/{}/document", instance_id),
        payment_tx: "0x...".to_string(),
        status: "pending_signatures".to_string(),
    })
}
```

### Signature Completion Flow

```rust
// Background job or triggered by final signature
pub async fn finalize_instance(
    instance_id: &InstanceId,
    state: &AppState,
) -> Result<BlobId> {
    // 1. Get instance from blockchain
    let instance = state.sui.get_instance(instance_id).await?;

    // 2. Verify all signatures present
    if instance.status != InstanceStatus::FullySigned {
        return Err(Error::InvalidDocument("Not fully signed".into()));
    }

    // 3. Collect signature data
    let signatures = instance.required_signers
        .iter()
        .map(|signer| {
            // Get signature from blockchain
            SignatureData {
                signer_name: signer.user_id.0.clone(),
                sui_address: signer.sui_address.clone(),
                signed_at: signer.signed_at.unwrap(),
                signature_hash: signer.signature_hash.clone().unwrap(),
                transaction_digest: "0x...".to_string(), // Get from event
            }
        })
        .collect::<Vec<_>>();

    // 4. Generate signed PDF
    let signed_pdf = state.pdf_generator
        .generate_signed_pdf(
            &instance.instance_blob_id,
            &signatures,
        )
        .await?;

    // 5. Store signed PDF in Walrus
    let signed_blob_id = state.walrus.store(signed_pdf).await?;

    // 6. Update instance on blockchain
    state.sui.update_instance_signed_document(
        instance_id,
        &signed_blob_id,
    ).await?;

    Ok(signed_blob_id)
}
```

## Storage Strategy

### Three-Blob Approach

```
Template:
  blob_id: ABC123 (template.pdf with {{vars}})
  Status: Immutable, reusable

Instance (Unsigned):
  blob_id: XYZ789 (filled.pdf, no signatures)
  Status: Readable by signers, temporary

Instance (Signed):
  blob_id: FINAL456 (complete.pdf with signatures)
  Status: Immutable, permanent, legal record
```

### On-Chain Metadata

```rust
ContractInstance {
    template_blob_id: "ABC123",      // Reference
    unsigned_blob_id: "XYZ789",      // For review
    signed_blob_id: Some("FINAL456"), // Final version
    variable_data: {...},            // For verification
    signatures: {...},               // Blockchain proof
}
```

## API Endpoints

```
# Get unsigned PDF for review
GET /api/v1/instances/:id/document/unsigned
→ Returns PDF with filled data, empty signature boxes

# Get signed PDF (final)
GET /api/v1/instances/:id/document/signed
→ Returns PDF with all signatures
→ Only available when fully signed

# Trigger signed PDF generation (if not auto)
POST /api/v1/instances/:id/finalize
→ Generates and stores signed PDF
→ Returns signed_blob_id
```

## Verification Flow

```
User receives signed PDF
  ↓
Scans QR code on document
  ↓
Links to verification page
  ↓
Page shows:
  1. Template used
  2. Variable data hash
  3. All signers + signatures
  4. Blockchain transactions
  5. Document hash verification
```

## Implementation Priority

**Phase 1: MVP**
1. Simple text substitution (find/replace {{vars}})
2. Store 2 versions: unsigned + metadata
3. Generate "completion certificate" instead of embedded signatures

**Phase 2: Professional**
1. Add PDF manipulation library
2. Embed signature blocks in PDF
3. Add QR codes for verification

**Phase 3: Advanced**
1. Support complex templates (conditionals, tables)
2. Multiple signature flows
3. Template versioning

Would you like me to implement Phase 1 (MVP) with simple text substitution, or jump to Phase 2 with full PDF manipulation?
