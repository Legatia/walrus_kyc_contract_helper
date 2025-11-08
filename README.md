# Walrus KYC & Contract Compliance Service

A decentralized microservice for managing KYC documents, electronic contract signing, and compliance audit trails using Walrus storage and Sui blockchain.

## Architecture Overview

```
┌─────────────────────────────────────────────────┐
│         REST API (Rust/Axum)                    │
├─────────────────────────────────────────────────┤
│  KYC Module  │  Contract Module  │  Compliance  │
├──────────────┴───────────────────┴──────────────┤
│         Walrus Client    │    Sui Client        │
└──────────────┬───────────┴──────────┬───────────┘
               │                      │
        ┌──────▼──────┐        ┌─────▼──────────┐
        │   Walrus    │        │  Sui Blockchain │
        │  (Blobs)    │        │  (Move/Metadata)│
        └─────────────┘        └────────────────┘
```

## Features

### Layer 1: Storage & Compliance
- **KYC Document Management**: Upload, verify, and manage identity documents
- **Contract Storage & E-Signature**: Digitally sign contracts with on-chain verification
- **Compliance Audit Trails**: Immutable logs for regulatory compliance
- **GDPR-Compliant Operations**: Privacy-preserving document handling with Seal
- **API for Integration**: RESTful API for carrier/service integration

## Components

### 1. Rust Service (`/service`)
- REST API server (Axum)
- Business logic and validation
- Authentication & authorization
- Rate limiting and security

### 2. Walrus Client (`/walrus-client`)
- Blob storage operations
- Upload/download/delete documents
- Seal encryption integration (planned)

### 3. Sui Client (`/sui-client`)
- Move contract interaction
- Transaction building and signing
- Event listening and indexing

### 4. Core (`/core`)
- Shared types and traits
- Domain models
- Error types

### 5. Move Contracts (`/move`)
- KYC registry
- Contract verification
- Compliance event log

## Getting Started

### Prerequisites
- Rust 1.75+ (install from https://rustup.rs)
- Sui CLI (for Move contracts)
- Walrus CLI (for storage operations)

### Build

```bash
cargo build --release
```

### Run

```bash
# Development
cargo run --bin service

# Production
./target/release/service
```

### Configuration

Copy `.env.example` to `.env` and configure:

```env
SUI_NETWORK=testnet
SUI_PRIVATE_KEY=your_private_key
WALRUS_ENDPOINT=https://walrus-testnet.example.com
API_PORT=8080
```

## API Endpoints

### KYC Management
- `POST /api/v1/kyc/upload` - Upload KYC document
- `GET /api/v1/kyc/:user_id` - Get KYC status
- `POST /api/v1/kyc/verify` - Verify KYC document

### Contract Management
- `POST /api/v1/contract/create` - Create new contract
- `POST /api/v1/contract/sign` - Sign contract
- `GET /api/v1/contract/:contract_id` - Get contract details
- `GET /api/v1/contract/:contract_id/verify` - Verify signature on-chain

### Compliance
- `GET /api/v1/compliance/audit/:user_id` - Get audit trail
- `POST /api/v1/compliance/gdpr/delete` - GDPR deletion request

## Development Roadmap

- [x] Project structure and architecture
- [ ] Walrus integration for document storage
- [ ] Sui Move contracts for metadata
- [ ] KYC document upload and verification
- [ ] E-signature implementation
- [ ] On-chain signature verification
- [ ] Compliance audit trails
- [ ] Seal privacy layer integration
- [ ] CDR storage module
- [ ] Carrier API integration

## License

MIT OR Apache-2.0

## Contributing

This is a preliminary project for a Web3 MVNO SaaS platform.
