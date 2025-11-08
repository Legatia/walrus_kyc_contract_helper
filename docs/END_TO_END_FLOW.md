# End-to-End Contract Flow

## Complete User Journey: Template → Instance → Signatures

This document shows how all components work together in the KYC & Contract Compliance system.

## Actors

1. **Template Creator** - Legal firm, creates reusable templates
2. **Service Provider** - MVNO operator using templates
3. **Customer** - End user signing service agreement
4. **System** - Our microservice orchestrating everything

## Flow Diagram

```
┌──────────────────────────────────────────────────────────────┐
│                   COMPLETE CONTRACT FLOW                      │
└──────────────────────────────────────────────────────────────┘

PHASE 1: Template Creation (One-time)
═══════════════════════════════════════

Template Creator
    │
    ├─→ Creates PDF: "5G_Service_Agreement.pdf"
    │   Content: "This agreement between {{provider}} and
    │            {{customer_name}} for {{service_plan}}
    │            at {{monthly_fee}}/month..."
    │
    ↓
Service (API)
    │
    ├─→ POST /api/v1/templates/create
    │   {
    │     "name": "5G Service Agreement",
    │     "template_pdf": "base64...",
    │     "variables": ["provider", "customer_name",
    │                   "service_plan", "monthly_fee"],
    │     "price_per_use": 10000000000, // 10 SUI
    │     "is_public": true
    │   }
    │
    ↓
PDF Generator
    │
    ├─→ Extract variables from PDF
    │   Found: {{provider}}, {{customer_name}},
    │          {{service_plan}}, {{monthly_fee}}
    │
    ↓
Walrus Storage
    │
    ├─→ Store template PDF → blob_id: "TMPL_ABC123"
    │
    ↓
Sui Blockchain (Move)
    │
    ├─→ create_template()
    │   ContractTemplate {
    │     id: 0xTMPL001,
    │     template_blob_id: "TMPL_ABC123",
    │     price_per_use: 10 SUI,
    │     variables: [...],
    │     creator: 0xLegalFirm,
    │     is_public: true
    │   }
    │
    ↓
Marketplace
    │
    └─→ Template listed in public marketplace


PHASE 2: Instance Creation (Per Customer)
═══════════════════════════════════════════

Service Provider browses marketplace
    │
    ↓
Service Provider
    │
    ├─→ GET /api/v1/marketplace/templates?category=mvno
    │   Response: [
    │     {
    │       "id": "0xTMPL001",
    │       "name": "5G Service Agreement",
    │       "price": 10 SUI,
    │       "usage_count": 147
    │     }
    │   ]
    │
    ├─→ Selects template "0xTMPL001"
    │
    ├─→ POST /api/v1/templates/0xTMPL001/instances
    │   {
    │     "variable_data": {
    │       "provider": "ACME MVNO Inc",
    │       "customer_name": "John Doe",
    │       "service_plan": "5G Premium 10GB",
    │       "monthly_fee": "$49.99"
    │     },
    │     "required_signers": [
    │       { "sui_address": "0xCustomer", "role": "customer" },
    │       { "sui_address": "0xOperator", "role": "provider" }
    │     ],
    │     "payment_coin_id": "0xCoin123"
    │   }
    │
    ↓
Service (Orchestrator)
    │
    ├─→ 1. Validate payment (10 SUI)
    │
    ├─→ 2. Fetch template from Walrus
    │      GET blob_id "TMPL_ABC123"
    │      ↓
    │      Template PDF downloaded
    │
    ├─→ 3. PDF Generator: generate_from_template()
    │      ↓
    │      VariableSubstitutor.substitute_simple()
    │      Replace {{provider}} → "ACME MVNO Inc"
    │      Replace {{customer_name}} → "John Doe"
    │      Replace {{service_plan}} → "5G Premium 10GB"
    │      Replace {{monthly_fee}} → "$49.99"
    │      ↓
    │      Generated unsigned PDF (filled contract)
    │
    ├─→ 4. Store unsigned PDF in Walrus
    │      → blob_id: "INST_XYZ789"
    │
    ├─→ 5. Pay template creator
    │      Transfer 10 SUI → 0xLegalFirm
    │      ↓
    │      Payment TX: 0xPAY456
    │
    ├─→ 6. Create instance on Sui blockchain
    │      create_instance_from_template()
    │      ContractInstance {
    │        id: 0xINST001,
    │        template_id: 0xTMPL001,
    │        instance_blob_id: "INST_XYZ789",
    │        variable_data: {...},
    │        required_signers: [0xCustomer, 0xOperator],
    │        signatures: {}, // empty
    │        status: PendingSignatures,
    │        payment_tx: 0xPAY456
    │      }
    │
    └─→ Response: {
          "instance_id": "0xINST001",
          "generated_blob_id": "INST_XYZ789",
          "document_url": "/instances/0xINST001/document",
          "status": "pending_signatures"
        }


PHASE 3: Signature Collection
═══════════════════════════════

Customer receives notification
    │
    ├─→ GET /api/v1/instances/0xINST001/document
    │   ↓
    │   Downloads filled PDF from Walrus (INST_XYZ789)
    │   ↓
    │   Reviews contract:
    │   "This agreement between ACME MVNO Inc and John Doe
    │    for 5G Premium 10GB at $49.99/month..."
    │
    ├─→ Decides to sign
    │
    ├─→ POST /api/v1/instances/0xINST001/sign
    │   {
    │     "signer_address": "0xCustomer",
    │     "signature": "ed25519_signature_data"
    │   }
    │
    ↓
Service
    │
    ├─→ 1. Get instance from Sui: 0xINST001
    │
    ├─→ 2. Verify signer is required
    │      0xCustomer in required_signers? ✓
    │
    ├─→ 3. Calculate signature hash
    │      hash(signature_data) → "0xSIG_HASH_1"
    │
    ├─→ 4. Record on blockchain
    │      sign_instance(0xINST001, "0xSIG_HASH_1")
    │      ↓
    │      TX Digest: 0xTX_SIGN_1
    │      ↓
    │      Instance.signatures[0xCustomer] = {
    │        signature_hash: "0xSIG_HASH_1",
    │        signed_at: 1706184000,
    │        tx_digest: "0xTX_SIGN_1"
    │      }
    │      ↓
    │      Instance.status = PendingSignatures (1 of 2)
    │
    └─→ Response: {
          "signed_at": "2024-01-25T10:00:00Z",
          "remaining_signers": ["0xOperator"],
          "fully_signed": false
        }

Service Provider signs
    │
    ├─→ POST /api/v1/instances/0xINST001/sign
    │   {
    │     "signer_address": "0xOperator",
    │     "signature": "ed25519_signature_data"
    │   }
    │
    ↓
Service
    │
    ├─→ Record signature on blockchain
    │   ↓
    │   Instance.signatures[0xOperator] = {...}
    │   ↓
    │   All signatures collected! (2 of 2)
    │   ↓
    │   Instance.status = FullySigned ✓
    │   ↓
    │   Event emitted: InstanceSigned { fully_signed: true }
    │
    └─→ Response: {
          "signed_at": "2024-01-25T10:05:00Z",
          "remaining_signers": [],
          "fully_signed": true ✓
        }


PHASE 4: Final Document Generation
════════════════════════════════════

System (Background Job or API trigger)
    │
    ├─→ Detects fully_signed event
    │
    ├─→ POST /api/v1/instances/0xINST001/finalize
    │   (or automatic trigger)
    │
    ↓
Service
    │
    ├─→ 1. Get instance: 0xINST001
    │      Status: FullySigned ✓
    │
    ├─→ 2. Collect all signatures from blockchain
    │      [
    │        {
    │          signer_name: "John Doe",
    │          sui_address: "0xCustomer",
    │          signed_at: 2024-01-25 10:00:00,
    │          signature_hash: "0xSIG_HASH_1",
    │          tx_digest: "0xTX_SIGN_1"
    │        },
    │        {
    │          signer_name: "ACME MVNO",
    │          sui_address: "0xOperator",
    │          signed_at: 2024-01-25 10:05:00,
    │          signature_hash: "0xSIG_HASH_2",
    │          tx_digest: "0xTX_SIGN_2"
    │        }
    │      ]
    │
    ├─→ 3. PDF Generator: generate_signed_pdf()
    │      ↓
    │      Fetch unsigned PDF (INST_XYZ789)
    │      ↓
    │      SignatureBlockGenerator.generate_all_blocks()
    │      ↓
    │      Append signature section:
    │      "
    │      ═══════════════════════════════
    │       DIGITAL SIGNATURES
    │      ═══════════════════════════════
    │
    │      Signature 1 of 2
    │      Signed by: John Doe
    │      Address: 0xCustomer
    │      Date: 2024-01-25 10:00:00 UTC
    │      Hash: 0xSIG_HASH_1
    │      TX: 0xTX_SIGN_1
    │      Verify: https://suiscan.xyz/.../0xTX_SIGN_1
    │      [QR CODE]
    │
    │      Signature 2 of 2
    │      Signed by: ACME MVNO
    │      ...
    │      "
    │      ↓
    │      Final signed PDF generated
    │
    ├─→ 4. Store in Walrus
    │      → blob_id: "FINAL_456"
    │
    ├─→ 5. Update instance on Sui
    │      Instance.signed_blob_id = "FINAL_456"
    │
    └─→ Response: {
          "signed_blob_id": "FINAL_456",
          "download_url": "/instances/0xINST001/document/signed"
        }


PHASE 5: Verification & Distribution
══════════════════════════════════════

Anyone can verify
    │
    ├─→ GET /instances/0xINST001/document/signed
    │   ↓
    │   Downloads final PDF (FINAL_456)
    │
    ├─→ Scans QR code on document
    │   ↓
    │   https://suiscan.xyz/mainnet/tx/0xTX_SIGN_1
    │   ↓
    │   Sees on-chain signature proof
    │
    ├─→ GET /api/v1/instances/0xINST001/verify
    │   ↓
    │   Returns:
    │   {
    │     "template": "5G Service Agreement",
    │     "instance_hash": "0x...",
    │     "signatures": [
    │       {
    │         "signer": "0xCustomer",
    │         "verified": true,
    │         "timestamp": "2024-01-25T10:00:00Z",
    │         "tx": "0xTX_SIGN_1"
    │       },
    │       {
    │         "signer": "0xOperator",
    │         "verified": true,
    │         "timestamp": "2024-01-25T10:05:00Z",
    │         "tx": "0xTX_SIGN_2"
    │       }
    │     ],
    │     "fully_signed": true,
    │     "blockchain_verified": true ✓
    │   }
    │
    └─→ Verification complete!
```

## Storage Breakdown

### What's Stored Where

**Walrus (Blob Storage)**
```
TMPL_ABC123 (immutable)
├─ Original template PDF with {{variables}}
├─ Size: ~100KB
├─ Reused: 1000+ times
└─ Cost: One-time storage fee

INST_XYZ789 (temporary/reference)
├─ Unsigned filled contract
├─ Size: ~100KB
├─ Used for: Review before signing
└─ Optional: Can be deleted after signing

FINAL_456 (immutable/permanent)
├─ Fully signed contract
├─ Size: ~120KB (with signature blocks)
├─ Legal record
└─ Permanent storage
```

**Sui Blockchain (Metadata + Verification)**
```
ContractTemplate (0xTMPL001)
├─ template_blob_id: "TMPL_ABC123"
├─ variables: [...]
├─ price: 10 SUI
├─ creator: 0xLegalFirm
└─ usage_count: 147

ContractInstance (0xINST001)
├─ template_id: 0xTMPL001 (reference)
├─ instance_blob_id: "INST_XYZ789"
├─ signed_blob_id: "FINAL_456"
├─ variable_data: {...}  // For verification
├─ signatures: {
│    0xCustomer: { hash, timestamp, tx },
│    0xOperator: { hash, timestamp, tx }
│  }
└─ status: FullySigned
```

## Cost Analysis

### Per Contract (Traditional)
```
Manual process:
- Legal review: $50/hour × 2 hours = $100
- Document preparation: $25
- Physical signing: $10
- Storage/filing: $5
Total: $140 per contract

For 1000 customers: $140,000
```

### Per Contract (This System)
```
Template marketplace:
- Template fee: 10 SUI × $0.50 = $5
- Walrus storage: $0.10 (unsigned + signed)
- Sui gas fees: $0.05
Total: $5.15 per contract

For 1000 customers: $5,150
Savings: 96.3% 🎉
```

### Template Creator Revenue
```
Create template once:
- Initial work: 2-4 hours
- List in marketplace: 10 SUI/use

After 1000 uses:
- Revenue: 10,000 SUI × $0.50 = $5,000
- Passive income from single template!
```

## Key Innovations

1. **Separation of Concerns**
   - Template (reusable) ≠ Instance (case-specific)
   - Storage efficiency: 1 template → ∞ instances

2. **Three-Phase PDFs**
   - Template: Variables placeholder
   - Unsigned: Filled data
   - Signed: Complete with signatures

3. **Blockchain Verification**
   - All signatures on-chain
   - Anyone can verify
   - Immutable audit trail

4. **Marketplace Economics**
   - Template creators monetize expertise
   - Users save 95%+ costs
   - Platform enables standardization

## Implementation Status

✅ **Completed**
- Template registry (Move contract)
- Instance creation (Move contract)
- Signature recording (Move contract)
- PDF generator service (MVP)
- Variable substitution engine
- Signature block generator
- API endpoints (stubs)

⏳ **In Progress**
- PDF library integration (lopdf/printpdf)
- Payment handling with SUI coins
- QR code generation

🔜 **Next Steps**
- Marketplace UI
- Template preview
- Analytics dashboard
- Advanced PDF features (tables, conditionals)

This complete flow shows how template reusability + blockchain verification creates a revolutionary contract management system!
