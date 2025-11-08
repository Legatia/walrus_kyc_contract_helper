# PDF Contract Signing Flow

## Overview

This document describes how PDF contracts are handled in the KYC & Contract Compliance service.

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                 PDF Contract Flow                    │
└─────────────────────────────────────────────────────┘

1. Contract Creation
   │
   ├─→ Client uploads PDF (base64 or multipart)
   │
   ├─→ Service receives PDF bytes
   │
   ├─→ Calculate hash: SHA-256(PDF)
   │
   ├─→ Store in Walrus
   │   └─→ Returns: blob_id
   │
   └─→ Register on Sui blockchain
       └─→ Creates Contract object with:
           - original_blob_id
           - document_hash
           - required_signers[]
           - signatures (empty map)
           - status: "pending_signatures"

2. Signing Process
   │
   ├─→ Signer downloads PDF via API
   │   GET /api/v1/contract/{id}/document
   │   └─→ Retrieves from Walrus using blob_id
   │
   ├─→ Signer reviews PDF in browser/app
   │
   ├─→ Signer clicks "Sign" → Sui wallet prompt
   │
   ├─→ Wallet signs: signature = sign(document_hash)
   │
   ├─→ POST /api/v1/contract/sign
   │   {
   │     contract_id,
   │     signer_address,
   │     signature
   │   }
   │
   └─→ Service records on Sui blockchain
       └─→ Updates Contract.signatures[address] = {
           signature_hash,
           signed_at,
           tx_digest
       }

3. Completion
   │
   ├─→ When all signatures collected
   │
   ├─→ Status updated to "fully_signed"
   │
   ├─→ Generate Certificate PDF
   │   - Contract details
   │   - All signers + timestamps
   │   - Blockchain transaction links
   │   - QR code for verification
   │
   └─→ Store certificate in Walrus
       └─→ blob_id_certificate

4. Verification
   │
   ├─→ Download original PDF from Walrus
   │
   ├─→ Calculate hash locally
   │
   ├─→ Query Sui blockchain for Contract object
   │
   ├─→ Verify: hash(PDF) == on-chain document_hash
   │
   └─→ Verify all signatures on-chain
       └─→ Each signature verifiable with public key
```

## API Enhancements Needed

### 1. Download Contract PDF

```rust
GET /api/v1/contract/:contract_id/document

Response: Binary PDF stream
Headers:
  Content-Type: application/pdf
  Content-Disposition: attachment; filename="contract.pdf"
```

### 2. Enhanced Sign Endpoint

```rust
POST /api/v1/contract/sign

Request:
{
  "contract_id": "uuid",
  "signer_address": "0x123...",
  "signature": "base64_signature_data",
  "signature_type": "ed25519" // or "secp256k1"
}

Response:
{
  "success": true,
  "data": {
    "tx_digest": "0xabc...",
    "signed_at": "2024-01-15T10:30:00Z",
    "remaining_signers": ["0x456..."],
    "fully_signed": false
  }
}
```

### 3. Generate Certificate

```rust
POST /api/v1/contract/:contract_id/certificate

Response:
{
  "success": true,
  "data": {
    "certificate_blob_id": "XYZ789...",
    "certificate_url": "https://aggregator.../XYZ789",
    "download_url": "/api/v1/contract/{id}/certificate"
  }
}
```

## Signature Verification

### On-Chain Verification

```move
// In contract_registry.move
public fun verify_signature(
    contract: &Contract,
    signer: address
): bool {
    vec_map::contains(&contract.signatures, &signer)
}

public fun verify_all_signatures(
    contract: &Contract
): bool {
    contract.status == 2 && // fully_signed
    vec_map::size(&contract.signatures) ==
        vector::length(&contract.required_signers)
}
```

### Off-Chain Verification (Rust)

```rust
use domain::{Contract, DocumentHash};

pub async fn verify_contract_integrity(
    walrus: &WalrusClient,
    sui: &SuiClient,
    contract_id: &str,
) -> Result<bool> {
    // 1. Get contract from blockchain
    let contract = sui.get_contract(contract_id).await?;

    // 2. Download PDF from Walrus
    let pdf_bytes = walrus.read(&contract.blob_id).await?;

    // 3. Calculate hash
    let actual_hash = DocumentHash::from_bytes(&pdf_bytes);

    // 4. Compare
    Ok(actual_hash == contract.document_hash)
}
```

## Legal Compliance

### For Telecom/MVNO Regulations

**Electronic Signatures Act (ESIGN):**
- ✅ Intent to sign (user clicks "Sign")
- ✅ Consent to electronic records (terms acceptance)
- ✅ Association with record (signature linked to hash)
- ✅ Retention (immutable on blockchain)

**eIDAS (EU):**
- ✅ Uniqueness (Sui address)
- ✅ Control (private key)
- ✅ Link to document (hash-based)
- ✅ Detect tampering (hash verification)

**Additional Evidence:**
- Timestamp (block timestamp)
- IP address (log in service)
- Device fingerprint (client-side)
- Audit trail (compliance_log.move)

## Advanced: PDF Digital Signatures (Optional)

For traditional PDF signatures, we can add:

```toml
[dependencies]
# Add to service/Cargo.toml
pdf = "0.8"
# Or use external service like DocuSign API
```

Then generate PDFs with embedded signatures:

```rust
use pdf::file::File as PdfFile;

pub async fn embed_signatures_in_pdf(
    original_pdf: Vec<u8>,
    signatures: Vec<SignatureData>,
) -> Result<Vec<u8>> {
    // 1. Parse PDF
    let pdf = PdfFile::from_data(&original_pdf)?;

    // 2. Add signature fields
    // (This is complex - requires PDF manipulation)

    // 3. Return signed PDF
    Ok(signed_pdf_bytes)
}
```

## Example: Complete Flow

```bash
# 1. Create contract
curl -X POST http://localhost:8080/api/v1/contract/create \
  -H "Content-Type: application/json" \
  -d '{
    "title": "MVNO Service Agreement",
    "document_content": "'$(base64 -w0 contract.pdf)'",
    "signers": ["0xAAA...", "0xBBB..."],
    "created_by": "operator_001"
  }'
# Returns: { "contract_id": "...", "blob_id": "..." }

# 2. Download PDF for review
curl http://localhost:8080/api/v1/contract/{id}/document \
  -o contract_to_review.pdf

# 3. Sign with Sui wallet (in your app)
# User reviews PDF → clicks Sign → wallet prompt
signature = sui_wallet.sign(document_hash)

# 4. Submit signature
curl -X POST http://localhost:8080/api/v1/contract/sign \
  -H "Content-Type: application/json" \
  -d '{
    "contract_id": "...",
    "signer_address": "0xAAA...",
    "signature": "'$signature'"
  }'

# 5. Check status
curl http://localhost:8080/api/v1/contract/{id}
# Returns: { "status": "fully_signed", ... }

# 6. Generate certificate
curl -X POST http://localhost:8080/api/v1/contract/{id}/certificate
# Returns: { "certificate_blob_id": "..." }

# 7. Download certificate
curl http://localhost:8080/api/v1/contract/{id}/certificate \
  -o completion_certificate.pdf
```

## Security Considerations

1. **Document Integrity**
   - Hash verification ensures PDF hasn't been modified
   - Immutable storage in Walrus

2. **Non-Repudiation**
   - Signatures are cryptographically tied to Sui addresses
   - Private key required to sign
   - Blockchain provides timestamping

3. **Multi-Party Signing**
   - All required signers must sign
   - Order doesn't matter (parallel signing)
   - Status only changes when complete

4. **Audit Trail**
   - Every action logged on-chain
   - Compliance events in compliance_log.move
   - Full history retrievable

## Next Steps

To fully implement this flow:

1. Add PDF download endpoint
2. Enhance signature verification
3. Add certificate generation (using PDF library or external service)
4. Implement signature validation (Ed25519/Secp256k1)
5. Add QR codes to certificates for easy verification
6. Create verification webpage (scan QR → verify on blockchain)

Would you like me to implement any of these enhancements?
