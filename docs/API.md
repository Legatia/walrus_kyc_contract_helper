# API Documentation

## Base URL

```
http://localhost:8080/api/v1
```

## Health Check

### GET /health

Check service health status.

**Response:**
```json
{
  "status": "ok",
  "version": "0.1.0"
}
```

---

## KYC Management

### POST /api/v1/kyc/upload

Upload a KYC document for verification.

**Request:**
- Content-Type: `multipart/form-data`
- Fields:
  - `user_id` (string): Unique user identifier
  - `document_type` (string): Type of document (passport, driver_license, national_id, proof_of_address)
  - `file` (file): Document file (PDF, JPG, PNG)

**Response:**
```json
{
  "success": true,
  "data": {
    "document_id": "550e8400-e29b-41d4-a716-446655440000",
    "blob_id": "ABC123...",
    "document_hash": "3a7bd3e2360a3d29eea436fcfb7e44c735d117c42d1c1835420b6b9942dd4f1b",
    "sui_object_id": "0x123..."
  }
}
```

### GET /api/v1/kyc/:user_id

Get KYC verification status for a user.

**Response:**
```json
{
  "success": true,
  "data": {
    "user_id": "user123",
    "documents": [
      {
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "document_type": "passport",
        "verification_status": "approved",
        "uploaded_at": "2024-01-15T10:30:00Z"
      }
    ]
  }
}
```

### POST /api/v1/kyc/verify

Verify or reject a KYC document (admin only).

**Request:**
```json
{
  "document_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "approved"
}
```

**Response:**
```json
{
  "success": true,
  "data": "KYC document verified: approved"
}
```

---

## Contract Management

### POST /api/v1/contract/create

Create a new contract for e-signature.

**Request:**
```json
{
  "title": "Service Agreement",
  "description": "Monthly service contract",
  "document_content": "base64_encoded_pdf_content",
  "signers": [
    "0x123...",
    "0x456..."
  ],
  "created_by": "user123"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "contract_id": "550e8400-e29b-41d4-a716-446655440000",
    "blob_id": "XYZ789...",
    "document_hash": "7c211433f02071597741e6ff5a8ea34789abbf43fc0cb....",
    "sui_object_id": "0x789..."
  }
}
```

### POST /api/v1/contract/sign

Sign a contract.

**Request:**
```json
{
  "contract_id": "550e8400-e29b-41d4-a716-446655440000",
  "signer_address": "0x123...",
  "signature": "signature_data_here"
}
```

**Response:**
```json
{
  "success": true,
  "data": "Contract signed successfully"
}
```

### GET /api/v1/contract/:contract_id

Get contract details.

**Response:**
```json
{
  "success": true,
  "data": {
    "contract_id": "550e8400-e29b-41d4-a716-446655440000",
    "title": "Service Agreement",
    "blob_id": "XYZ789...",
    "document_hash": "7c211433f02071597741e6ff5a8ea34789abbf43fc0cb....",
    "status": "pending_signatures",
    "signers": [
      {
        "address": "0x123...",
        "signed": true,
        "signed_at": "2024-01-15T10:30:00Z"
      },
      {
        "address": "0x456...",
        "signed": false,
        "signed_at": null
      }
    ]
  }
}
```

### GET /api/v1/contract/:contract_id/verify

Verify contract signatures on-chain.

**Response:**
```json
{
  "success": true,
  "data": true
}
```

---

## Compliance

### GET /api/v1/compliance/audit/:user_id

Get audit trail for a user.

**Response:**
```json
{
  "success": true,
  "data": {
    "user_id": "user123",
    "events": [
      {
        "event_id": "evt_123",
        "event_type": "kyc_upload",
        "action": "KYC document uploaded",
        "timestamp": "2024-01-15T10:30:00Z",
        "resource_id": "doc_456"
      }
    ]
  }
}
```

### POST /api/v1/compliance/gdpr/delete

Submit a GDPR deletion request.

**Request:**
```json
{
  "user_id": "user123",
  "reason": "User requested account deletion"
}
```

**Response:**
```json
{
  "success": true,
  "data": "GDPR deletion request submitted and logged: 0x123..."
}
```

---

## Error Responses

All errors follow this format:

```json
{
  "success": false,
  "data": null,
  "error": "Error message here"
}
```

Common HTTP status codes:
- 200: Success
- 400: Bad Request
- 401: Unauthorized
- 404: Not Found
- 500: Internal Server Error
