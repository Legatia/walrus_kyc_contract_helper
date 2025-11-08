# Usage Examples

## Prerequisites

1. Start the service:
```bash
cargo run --bin service
```

2. Set environment variables in `.env`:
```bash
cp .env.example .env
# Edit .env with your configuration
```

## Example 1: Upload KYC Document

```bash
# Upload a passport document
curl -X POST http://localhost:8080/api/v1/kyc/upload \
  -F "user_id=user123" \
  -F "document_type=passport" \
  -F "file=@/path/to/passport.pdf"
```

Response:
```json
{
  "success": true,
  "data": {
    "document_id": "550e8400-e29b-41d4-a716-446655440000",
    "blob_id": "ABC123xyz...",
    "document_hash": "3a7bd3e2360a3d29eea436fcfb7e44c735d117c42d1c1835420b6b9942dd4f1b",
    "sui_object_id": "0x123..."
  }
}
```

## Example 2: Check KYC Status

```bash
# Get KYC verification status
curl http://localhost:8080/api/v1/kyc/user123
```

## Example 3: Verify KYC Document

```bash
# Approve a KYC document
curl -X POST http://localhost:8080/api/v1/kyc/verify \
  -H "Content-Type: application/json" \
  -d '{
    "document_id": "550e8400-e29b-41d4-a716-446655440000",
    "status": "approved"
  }'
```

## Example 4: Create Contract

```bash
# Create a service agreement contract
curl -X POST http://localhost:8080/api/v1/contract/create \
  -H "Content-Type: application/json" \
  -d '{
    "title": "MVNO Service Agreement",
    "description": "12-month telecom service contract",
    "document_content": "JVBERi0xLjQKJeLjz9MKMSAwIG9iag...",
    "signers": [
      "0xabc123...",
      "0xdef456..."
    ],
    "created_by": "operator_001"
  }'
```

## Example 5: Sign Contract

```bash
# Sign a contract
curl -X POST http://localhost:8080/api/v1/contract/sign \
  -H "Content-Type: application/json" \
  -d '{
    "contract_id": "550e8400-e29b-41d4-a716-446655440000",
    "signer_address": "0xabc123...",
    "signature": "signature_data_base64_encoded"
  }'
```

## Example 6: Get Contract Details

```bash
# Retrieve contract information
curl http://localhost:8080/api/v1/contract/550e8400-e29b-41d4-a716-446655440000
```

## Example 7: Verify Signature On-Chain

```bash
# Verify contract signature on Sui blockchain
curl http://localhost:8080/api/v1/contract/550e8400-e29b-41d4-a716-446655440000/verify
```

## Example 8: Get Audit Trail

```bash
# Get compliance audit trail for a user
curl http://localhost:8080/api/v1/compliance/audit/user123
```

## Example 9: GDPR Deletion Request

```bash
# Submit GDPR deletion request
curl -X POST http://localhost:8080/api/v1/compliance/gdpr/delete \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "user123",
    "reason": "User requested complete data deletion"
  }'
```

## Example 10: Health Check

```bash
# Check service health
curl http://localhost:8080/health
```

## Integration Example: Complete KYC Flow

```bash
# 1. Upload KYC document
RESPONSE=$(curl -X POST http://localhost:8080/api/v1/kyc/upload \
  -F "user_id=user123" \
  -F "document_type=passport" \
  -F "file=@passport.pdf")

DOCUMENT_ID=$(echo $RESPONSE | jq -r '.data.document_id')

# 2. Verify document
curl -X POST http://localhost:8080/api/v1/kyc/verify \
  -H "Content-Type: application/json" \
  -d "{\"document_id\": \"$DOCUMENT_ID\", \"status\": \"approved\"}"

# 3. Check verification status
curl http://localhost:8080/api/v1/kyc/user123

# 4. Get audit trail
curl http://localhost:8080/api/v1/compliance/audit/user123
```

## Testing with Walrus Testnet

1. Install Walrus CLI:
```bash
# Follow Walrus documentation for installation
```

2. Configure endpoints in `.env`:
```bash
WALRUS_PUBLISHER_URL=https://publisher.walrus-testnet.walrus.space
WALRUS_AGGREGATOR_URL=https://aggregator.walrus-testnet.walrus.space
```

3. Test upload:
```bash
# The service will automatically use Walrus testnet
curl -X POST http://localhost:8080/api/v1/kyc/upload \
  -F "user_id=testuser" \
  -F "document_type=passport" \
  -F "file=@test.pdf"
```

## Testing with Sui Testnet

1. Get Sui testnet tokens:
```bash
sui client faucet
```

2. Deploy Move contracts:
```bash
cd move
sui move build
sui client publish --gas-budget 100000000
```

3. Update `.env` with package ID:
```bash
SUI_PACKAGE_ID=0x... # From publish output
```

4. Test contract creation:
```bash
curl -X POST http://localhost:8080/api/v1/contract/create \
  -H "Content-Type: application/json" \
  -d '{...}'
```
