# Contract Template Marketplace

## Overview

The Template Marketplace is a revolutionary approach to contract management that separates **reusable templates** from **case-specific data**. This enables:

- **Template creators** to monetize their work
- **Template users** to save time with pre-built contracts
- **Storage efficiency** through reuse
- **Standardization** across industries

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                Template Marketplace Flow                 │
└─────────────────────────────────────────────────────────┘

Step 1: Template Creation
┌────────────────────────────────────┐
│ Creator uploads PDF template       │
│ with variables: {{customer_name}}  │
│                 {{service_plan}}   │
│                 {{monthly_fee}}    │
└─────────────────┬──────────────────┘
                  │
                  ↓
         Store in Walrus → blob_id
                  ↓
         Mint Template NFT on Sui
         - Price: 10 SUI per use
         - Royalty: 5%
         - Public marketplace
                  ↓
         ┌─────────────────────┐
         │  Template NFT       │
         │  Owner: Creator     │
         │  ID: 0xAAA...       │
         └─────────────────────┘

Step 2: Browse Marketplace
┌────────────────────────────────────┐
│ User searches marketplace          │
│ Category: "MVNO Service Agreement" │
└─────────────────┬──────────────────┘
                  │
                  ↓
         GET /api/v1/templates?category=mvno
                  ↓
         Returns list of templates
         - Preview URL
         - Price
         - Usage count
         - Rating

Step 3: Create Instance from Template
┌────────────────────────────────────┐
│ User selects template              │
│ Fills in variables:                │
│   customer_name: "John Doe"        │
│   service_plan: "5GB/month"        │
│   monthly_fee: "29.99"             │
└─────────────────┬──────────────────┘
                  │
                  ↓
         Pay template fee (10 SUI)
                  ↓
         Generate PDF with filled data
                  ↓
         Store in Walrus → new_blob_id
                  ↓
         Create Contract Instance on Sui
         - References template
         - Stores variable data
         - Payment recorded
                  ↓
         Template creator receives 10 SUI
         Usage count incremented

Step 4: Sign Instance
┌────────────────────────────────────┐
│ Signers review generated contract  │
│ Sign with Sui wallet               │
└─────────────────┬──────────────────┘
                  │
                  ↓
         Record signatures on-chain
                  ↓
         When fully signed → Status = Complete
```

## Data Model

### Template Structure

```rust
ContractTemplate {
    id: "template_123",
    name: "MVNO Service Agreement",
    description: "Standard service agreement for MVNO customers",
    template_blob_id: "ABC...",  // Walrus blob
    creator: "0xCreator...",
    category: "mvno",
    variables: [
        "customer_name",
        "customer_email",
        "service_plan",
        "data_limit",
        "monthly_fee",
        "billing_date",
        "contract_duration"
    ],
    price_per_use: 10_000_000_000,  // 10 SUI in MIST
    royalty_percentage: 5,
    usage_count: 147,
    is_public: true,
    created_at: "2024-01-01T00:00:00Z",
    version: 1
}
```

### Contract Instance Structure

```rust
ContractInstance {
    id: "instance_456",
    template_id: "template_123",
    instance_blob_id: "XYZ...",  // Generated PDF in Walrus
    variable_data: {
        "customer_name": "John Doe",
        "customer_email": "john@example.com",
        "service_plan": "Premium 5G",
        "data_limit": "10GB",
        "monthly_fee": "$49.99",
        "billing_date": "1st of each month",
        "contract_duration": "12 months"
    },
    created_by: "0xUser...",
    required_signers: [
        {
            user_id: "user_789",
            sui_address: "0xUser...",
            role: "customer"
        },
        {
            user_id: "operator_001",
            sui_address: "0xOperator...",
            role: "service_provider"
        }
    ],
    status: "pending_signatures",
    payment_tx: "0xTx123...",
    created_at: "2024-01-15T10:00:00Z"
}
```

## PDF Template Format

### Variable Syntax

Templates use double curly braces for variables:

```
SERVICE AGREEMENT

This agreement is made between {{service_provider}} and {{customer_name}}.

Service Plan: {{service_plan}}
Monthly Fee: {{monthly_fee}}
Data Limit: {{data_limit}}

Signed on: {{contract_date}}

Customer Signature: ____________________
Provider Signature: ____________________
```

### Variable Types

Support different types for validation:

- **Text**: `{{customer_name}}` - any string
- **Email**: `{{customer_email}}` - validated email format
- **Currency**: `{{monthly_fee}}` - formatted as currency
- **Date**: `{{contract_date}}` - date format
- **Number**: `{{data_limit}}` - numeric values

## API Endpoints

### Template Management

#### Create Template

```bash
POST /api/v1/templates/create

Request:
{
  "name": "MVNO Service Agreement",
  "description": "Standard service contract",
  "template_pdf": "base64_encoded_pdf_with_variables",
  "category": "mvno",
  "variables": ["customer_name", "service_plan", "monthly_fee"],
  "price_per_use": 10000000000,  // 10 SUI
  "royalty_percentage": 5,
  "is_public": true
}

Response:
{
  "success": true,
  "data": {
    "template_id": "template_123",
    "blob_id": "ABC...",
    "sui_object_id": "0x...",
    "marketplace_url": "/marketplace/template_123"
  }
}
```

#### Browse Marketplace

```bash
GET /api/v1/marketplace/templates?category=mvno&sort=popular

Response:
{
  "success": true,
  "data": {
    "templates": [
      {
        "id": "template_123",
        "name": "MVNO Service Agreement",
        "description": "...",
        "creator": "0x...",
        "price": 10000000000,
        "usage_count": 147,
        "rating": 4.8,
        "preview_url": "/templates/template_123/preview"
      }
    ],
    "total": 42,
    "page": 1
  }
}
```

#### Get Template Details

```bash
GET /api/v1/templates/:template_id

Response:
{
  "success": true,
  "data": {
    "template": {...},
    "variables": [
      {
        "name": "customer_name",
        "type": "text",
        "required": true,
        "description": "Full legal name of customer"
      },
      {
        "name": "monthly_fee",
        "type": "currency",
        "required": true,
        "description": "Monthly service fee in USD"
      }
    ],
    "preview_url": "..."
  }
}
```

### Instance Management

#### Create Instance from Template

```bash
POST /api/v1/templates/:template_id/instances

Request:
{
  "variable_data": {
    "customer_name": "John Doe",
    "customer_email": "john@example.com",
    "service_plan": "Premium 5G",
    "data_limit": "10GB",
    "monthly_fee": "$49.99",
    "billing_date": "2024-01-20",
    "contract_duration": "12 months"
  },
  "required_signers": [
    {
      "sui_address": "0xUser...",
      "role": "customer"
    },
    {
      "sui_address": "0xOperator...",
      "role": "service_provider"
    }
  ],
  "payment_coin_id": "0xCoin..."  // SUI coin object for payment
}

Response:
{
  "success": true,
  "data": {
    "instance_id": "instance_456",
    "generated_blob_id": "XYZ...",
    "document_url": "/instances/instance_456/document",
    "payment_tx": "0xTx...",
    "status": "pending_signatures"
  }
}
```

#### Download Instance PDF

```bash
GET /api/v1/instances/:instance_id/document

Response: Binary PDF stream with filled variables
```

#### Sign Instance

```bash
POST /api/v1/instances/:instance_id/sign

Request:
{
  "signer_address": "0xUser...",
  "signature": "base64_signature"
}

Response:
{
  "success": true,
  "data": {
    "signed_at": "2024-01-15T10:30:00Z",
    "remaining_signers": ["0xOperator..."],
    "fully_signed": false
  }
}
```

## Business Model

### Template Creators Earn

```
Template: "MVNO Service Agreement"
Price per use: 10 SUI
Usage: 147 times
Revenue: 1,470 SUI

Usage breakdown:
- Creator revenue: 1,470 SUI
- Platform fee (optional): 0 SUI (could add 2-5%)
```

### Revenue Sharing (Future)

For collaborative templates:

```
Template created by:
- Legal expert: 60% of revenue
- Industry expert: 30%
- Designer: 10%

Split automatically on-chain via Move contract
```

## Use Cases

### 1. MVNO Operator

**Problem**: Need to onboard 1,000 customers with service agreements

**Solution**:
1. Create template once with standard terms
2. Each customer fills in their details
3. Both parties sign electronically
4. Template creator earns from each use

**Savings**:
- Time: 95% reduction (instant generation vs. manual)
- Legal costs: $50 → $1 per contract
- Storage: 1 PDF template vs. 1,000 individual PDFs

### 2. Template Marketplace

**Categories**:
- MVNO contracts
- SaaS agreements
- NDAs
- Employment contracts
- Rental agreements
- Partnership agreements

**Featured templates**:
- Industry-standard contracts
- Legally reviewed
- Highly rated by users
- Multi-language support

### 3. Enterprise Customization

**Private templates**:
- Not listed in marketplace
- Only accessible to creator
- Used for internal standardization
- Free to use (or internal pricing)

## Smart Contract Flow

### Template Creation (Move)

```move
public entry fun create_template(
    name: vector<u8>,
    template_blob_id: vector<u8>,
    variables: vector<vector<u8>>,
    price_per_use: u64,
    ctx: &mut TxContext
)
```

### Instance Creation (Move)

```move
public entry fun create_instance_from_template(
    template: &mut ContractTemplate,
    instance_blob_id: vector<u8>,
    variable_data: VecMap<String, String>,
    required_signers: vector<address>,
    payment: Coin<SUI>,
    ctx: &mut TxContext
)
```

This automatically:
1. Validates payment amount
2. Transfers payment to template creator
3. Increments usage count
4. Creates instance object
5. Emits events

### Signature Recording (Move)

```move
public entry fun sign_instance(
    instance: &mut ContractInstance,
    signature_hash: vector<u8>,
    ctx: &mut TxContext
)
```

## PDF Generation

### Server-Side (Rust)

```rust
use pdf_lib; // Library for PDF manipulation

pub async fn generate_pdf_from_template(
    template_blob_id: &BlobId,
    variable_data: HashMap<String, String>,
) -> Result<Vec<u8>> {
    // 1. Download template PDF from Walrus
    let template_pdf = walrus.read(template_blob_id).await?;

    // 2. Parse PDF
    let mut pdf = PdfDocument::load(&template_pdf)?;

    // 3. Replace variables
    for (key, value) in variable_data {
        let placeholder = format!("{{{{{}}}}}", key);
        pdf.replace_text(&placeholder, &value);
    }

    // 4. Generate new PDF
    let generated_pdf = pdf.to_bytes()?;

    Ok(generated_pdf)
}
```

### Client-Side (JavaScript)

```javascript
// Using PDF.js or similar
async function generateContract(templateId, variables) {
  // 1. Get template
  const template = await fetch(`/api/v1/templates/${templateId}`);

  // 2. Submit variables
  const instance = await fetch(`/api/v1/templates/${templateId}/instances`, {
    method: 'POST',
    body: JSON.stringify({
      variable_data: variables,
      required_signers: [...]
    })
  });

  // 3. Download generated PDF
  const pdf = await fetch(instance.document_url);

  return pdf;
}
```

## Security Considerations

### Template Validation

- **Malware scanning**: Scan uploaded PDFs
- **Content filtering**: Block inappropriate content
- **Variable validation**: Ensure no script injection
- **Size limits**: Max 10MB per template

### Payment Security

- **Escrow**: Payment held until instance created
- **Refunds**: If generation fails, refund payment
- **Rate limiting**: Prevent spam template creation

### Data Privacy

- **Variable encryption**: Sensitive data encrypted
- **Access control**: Only signers can view instance
- **GDPR compliance**: Support deletion requests

## Future Enhancements

### 1. Template Versioning

```
Template v1.0 → v1.1 (bug fixes)
            → v2.0 (major changes)

Instances reference specific version
Backward compatible
```

### 2. Multi-language Templates

```
Template:
- English: template_en_123
- Spanish: template_es_123
- French: template_fr_123

Same variables, different language
```

### 3. Conditional Logic

```
IF {{customer_type}} == "enterprise" THEN
  Include enterprise terms
ELSE
  Include standard terms
END
```

### 4. Template Bundles

```
Bundle: "Complete MVNO Onboarding"
- Service Agreement
- Privacy Policy
- Acceptable Use Policy

Price: 25 SUI (vs. 30 SUI individually)
```

### 5. Template Analytics

```
Dashboard for creators:
- Usage over time
- Revenue tracking
- User ratings
- Popular variables
- Geographic distribution
```

## Conclusion

The template marketplace transforms contract management from a repetitive, manual process into an automated, monetizable platform. By separating templates from instances:

✅ **Creators** earn passive income
✅ **Users** save time and money
✅ **Platform** enables new business models
✅ **Storage** is efficient and scalable
✅ **Compliance** is maintained via blockchain

This is Web3-native contract management for the modern enterprise.
