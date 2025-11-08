# Architecture Documentation

## System Overview

The Walrus KYC & Contract Compliance Service is a decentralized microservice built for Web3 MVNO (Mobile Virtual Network Operator) platforms. It provides KYC document management, electronic contract signing, and compliance audit trails using Walrus for storage and Sui blockchain for verification.

## Core Components

```
┌─────────────────────────────────────────────────────────┐
│                   Client Applications                    │
│         (Web App, Mobile App, Partner Systems)          │
└────────────────────────┬────────────────────────────────┘
                         │ REST API
                         │
┌────────────────────────▼────────────────────────────────┐
│                  Service Layer (Rust)                    │
│  ┌──────────────┬─────────────────┬─────────────────┐  │
│  │  KYC Module  │  Contract Module │ Compliance Log  │  │
│  └──────┬───────┴─────────┬───────┴────────┬────────┘  │
│         │                 │                 │            │
│  ┌──────▼─────────────────▼─────────────────▼────────┐  │
│  │          Walrus Client  │  Sui Client             │  │
│  └──────┬──────────────────┴─────────┬────────────────┘  │
└─────────┼────────────────────────────┼───────────────────┘
          │                            │
┌─────────▼─────────┐        ┌─────────▼──────────────────┐
│  Walrus Storage   │        │   Sui Blockchain           │
│  ┌─────────────┐  │        │  ┌──────────────────────┐  │
│  │ Blob Store  │  │        │  │  Move Contracts      │  │
│  │ - KYC Docs  │  │        │  │  - kyc_registry      │  │
│  │ - Contracts │  │        │  │  - contract_registry │  │
│  │ - CDRs      │  │        │  │  - compliance_log    │  │
│  └─────────────┘  │        │  └──────────────────────┘  │
└───────────────────┘        └────────────────────────────┘
```

## Layer Architecture

### Layer 1: Storage & Compliance

#### Walrus Storage Layer
- **Purpose**: Decentralized blob storage for large documents
- **Stores**:
  - KYC documents (passports, licenses, etc.)
  - Signed contracts (PDFs)
  - Call Detail Records (CDRs) - future
  - Audit logs
- **Features**:
  - Content-addressable (immutable)
  - Erasure-coded for redundancy
  - Cost-effective for large files

#### Sui Blockchain Layer
- **Purpose**: Metadata registry and verification
- **Stores**:
  - Document metadata (blob_id → user mapping)
  - Verification records
  - Signature proofs
  - Compliance events
- **Features**:
  - Queryable via RPC/GraphQL
  - Smart contract logic (Move)
  - Cryptographic verification

### Layer 2: Service Layer (Rust)

#### Core Module
- Shared types and domain models
- Error handling
- Common utilities

#### Walrus Client
- HTTP client for Walrus API
- Store/retrieve operations
- Blob metadata queries

#### Sui Client
- Sui SDK integration
- Move contract interaction
- Transaction building
- Event listening

#### Service API
- REST endpoints (Axum framework)
- Request validation
- Business logic orchestration
- Response formatting

## Data Flow

### KYC Document Upload Flow

```
1. Client uploads document
   ↓
2. Service validates & hashes document
   ↓
3. Store blob in Walrus → Get blob_id
   ↓
4. Register metadata on Sui
   - user_id → blob_id
   - document_hash
   - timestamp
   ↓
5. Return references to client
   ↓
6. Log compliance event
```

### Contract Signing Flow

```
1. Create contract
   ↓
2. Store contract PDF in Walrus
   ↓
3. Create Contract object on Sui
   - Required signers
   - Document hash
   - Blob ID
   ↓
4. Signers sign document
   ↓
5. Record each signature on-chain
   ↓
6. Verify: hash(Walrus blob) == on-chain hash
   ↓
7. Mark contract as fully_signed
   ↓
8. Log compliance events
```

## Move Contract Design

### kyc_registry.move

**Purpose**: Manage KYC document records

**Structs**:
- `KycDocument`: On-chain record linking user to blob
  - user_id
  - blob_id (Walrus)
  - document_hash (SHA-256)
  - verification_status
  - timestamps

**Functions**:
- `register_document()`: Create new KYC record
- `update_verification_status()`: Approve/reject
- `get_document_hash()`: For verification

### contract_registry.move

**Purpose**: Manage contracts and signatures

**Structs**:
- `Contract`: Contract metadata
  - blob_id
  - document_hash
  - required_signers
  - signatures map
  - status
- `Signature`: Signature record
  - signer address
  - signature_hash
  - timestamp

**Functions**:
- `create_contract()`: Initialize contract
- `sign_contract()`: Record signature
- `verify_signature()`: Check signature validity

### compliance_log.move

**Purpose**: Immutable audit trail

**Structs**:
- `ComplianceEvent`: Audit event
  - event_type
  - user_id
  - resource_id
  - action
  - timestamp
- `GdprDeletionRequest`: GDPR compliance

**Functions**:
- `log_event()`: Create audit record
- `create_gdpr_deletion_request()`

## Security Considerations

### Data Privacy
- **Seal Integration (Planned)**: Encrypt sensitive PII
  - Client-side encryption before Walrus upload
  - Key management via Sui objects
  - Zero-knowledge proofs for verification

### Access Control
- **Authentication**: JWT tokens (to be implemented)
- **Authorization**: Role-based access
  - Users can upload their own KYC
  - Admins can verify documents
  - Contract signers must be authorized

### Cryptographic Verification
- **Document Integrity**: SHA-256 hashing
- **Signature Verification**: On-chain signature checks
- **Replay Protection**: Nonce-based transactions

## Scalability

### Horizontal Scaling
- Stateless API servers
- Load balancing ready
- No session state

### Storage Scaling
- Walrus handles large blobs efficiently
- Sui handles metadata queries
- Optional caching layer (PostgreSQL)

### Performance Optimizations
- Parallel operations (upload + register)
- Async I/O throughout
- Connection pooling

## Compliance Features

### GDPR Compliance
- Right to be forgotten
- Data portability
- Audit trails
- Consent management

### Telecom Regulations
- KYC/AML compliance
- Call record retention
- Data sovereignty
- Audit requirements

## Future Enhancements

### Phase 2: Privacy Layer
- Seal encryption integration
- Zero-knowledge KYC proofs
- Selective disclosure

### Phase 3: CDR Storage
- Call Detail Record management
- Billing integration
- Analytics support

### Phase 4: Carrier Integration
- API gateways for carriers
- Webhook notifications
- Real-time event streaming

### Phase 5: Advanced Features
- Multi-signature workflows
- Automated compliance checks
- AI-powered document verification
- Decentralized identity (DID) integration

## Monitoring & Observability

### Metrics (To Implement)
- API request latency
- Walrus upload success rate
- Sui transaction confirmation time
- Document verification throughput

### Logging
- Structured logging (tracing crate)
- Request/response tracking
- Error tracking
- Audit event logging

### Alerting
- Failed transactions
- Verification failures
- GDPR request monitoring
- System health checks
