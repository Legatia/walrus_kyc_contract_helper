# Deployment Guide

## Prerequisites

### System Requirements
- Rust 1.75 or higher
- Sui CLI (latest version)
- Walrus CLI (for testnet interaction)
- Docker (optional, for containerized deployment)
- PostgreSQL 14+ (optional, for caching)

### Network Access
- Access to Sui testnet/mainnet RPC
- Access to Walrus testnet/mainnet endpoints
- Outbound HTTPS connections

## Local Development Setup

### 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version
```

### 2. Install Sui CLI

```bash
cargo install --locked --git https://github.com/MystenLabs/sui.git --branch mainnet sui
sui --version
```

### 3. Clone and Build

```bash
git clone https://github.com/Legatia/walrus_kyc_contract_helper.git
cd walrus_kyc_contract_helper
cargo build --release
```

### 4. Configure Environment

```bash
cp .env.example .env
```

Edit `.env` with your configuration:

```env
# Sui Configuration
SUI_NETWORK=testnet
SUI_RPC_URL=https://fullnode.testnet.sui.io:443
SUI_PRIVATE_KEY=suiprivkey...
SUI_PACKAGE_ID=  # Will be set after deploying Move contracts

# Walrus Configuration
WALRUS_PUBLISHER_URL=https://publisher.walrus-testnet.walrus.space
WALRUS_AGGREGATOR_URL=https://aggregator.walrus-testnet.walrus.space

# API Configuration
API_HOST=0.0.0.0
API_PORT=8080

# Security
JWT_SECRET=your_random_secret_key_change_this
CORS_ALLOWED_ORIGINS=http://localhost:3000

# Logging
RUST_LOG=info,service=debug
```

### 5. Deploy Move Contracts

```bash
cd move

# Build contracts
sui move build

# Test contracts
sui move test

# Deploy to testnet
sui client publish --gas-budget 100000000

# Copy the package ID from output and update .env
# Look for "Published Objects" → "PackageID"
```

Update `.env`:
```env
SUI_PACKAGE_ID=0xYOUR_PACKAGE_ID_HERE
```

### 6. Run Service

```bash
# Development mode
cargo run --bin service

# Production mode
cargo run --release --bin service
```

### 7. Test Deployment

```bash
# Health check
curl http://localhost:8080/health

# Should return: {"status":"ok","version":"0.1.0"}
```

## Testnet Deployment

### Using Docker

1. Build Docker image:

```bash
docker build -t walrus-kyc-service .
```

2. Create `.env` file for production

3. Run container:

```bash
docker run -d \
  --name walrus-kyc-service \
  -p 8080:8080 \
  --env-file .env \
  walrus-kyc-service
```

### Using Docker Compose

```yaml
version: '3.8'

services:
  api:
    build: .
    ports:
      - "8080:8080"
    env_file:
      - .env
    restart: unless-stopped
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"

  # Optional: Add PostgreSQL for caching
  postgres:
    image: postgres:14
    environment:
      POSTGRES_DB: walrus_kyc
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: changeme
    volumes:
      - postgres_data:/var/lib/postgresql/data
    restart: unless-stopped

volumes:
  postgres_data:
```

Run with:
```bash
docker-compose up -d
```

## Production Deployment

### AWS Deployment

#### Using ECS (Elastic Container Service)

1. **Build and push image to ECR**:

```bash
# Authenticate to ECR
aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin YOUR_AWS_ACCOUNT.dkr.ecr.us-east-1.amazonaws.com

# Build and tag
docker build -t walrus-kyc-service .
docker tag walrus-kyc-service:latest YOUR_AWS_ACCOUNT.dkr.ecr.us-east-1.amazonaws.com/walrus-kyc-service:latest

# Push
docker push YOUR_AWS_ACCOUNT.dkr.ecr.us-east-1.amazonaws.com/walrus-kyc-service:latest
```

2. **Create ECS Task Definition** (task-definition.json):

```json
{
  "family": "walrus-kyc-service",
  "networkMode": "awsvpc",
  "requiresCompatibilities": ["FARGATE"],
  "cpu": "512",
  "memory": "1024",
  "containerDefinitions": [
    {
      "name": "walrus-kyc-service",
      "image": "YOUR_AWS_ACCOUNT.dkr.ecr.us-east-1.amazonaws.com/walrus-kyc-service:latest",
      "portMappings": [
        {
          "containerPort": 8080,
          "protocol": "tcp"
        }
      ],
      "environment": [
        {"name": "SUI_NETWORK", "value": "testnet"},
        {"name": "API_PORT", "value": "8080"}
      ],
      "secrets": [
        {
          "name": "SUI_PRIVATE_KEY",
          "valueFrom": "arn:aws:secretsmanager:us-east-1:ACCOUNT:secret:sui-private-key"
        }
      ],
      "logConfiguration": {
        "logDriver": "awslogs",
        "options": {
          "awslogs-group": "/ecs/walrus-kyc-service",
          "awslogs-region": "us-east-1",
          "awslogs-stream-prefix": "ecs"
        }
      }
    }
  ]
}
```

3. **Create ECS Service with Application Load Balancer**

### GCP Deployment (Cloud Run)

```bash
# Build and push to GCR
gcloud builds submit --tag gcr.io/PROJECT_ID/walrus-kyc-service

# Deploy to Cloud Run
gcloud run deploy walrus-kyc-service \
  --image gcr.io/PROJECT_ID/walrus-kyc-service \
  --platform managed \
  --region us-central1 \
  --allow-unauthenticated \
  --set-env-vars SUI_NETWORK=testnet,API_PORT=8080 \
  --set-secrets SUI_PRIVATE_KEY=sui-private-key:latest
```

### Kubernetes Deployment

**deployment.yaml**:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: walrus-kyc-service
spec:
  replicas: 3
  selector:
    matchLabels:
      app: walrus-kyc-service
  template:
    metadata:
      labels:
        app: walrus-kyc-service
    spec:
      containers:
      - name: service
        image: walrus-kyc-service:latest
        ports:
        - containerPort: 8080
        env:
        - name: SUI_NETWORK
          value: "mainnet"
        - name: API_PORT
          value: "8080"
        - name: SUI_PRIVATE_KEY
          valueFrom:
            secretKeyRef:
              name: sui-credentials
              key: private-key
        resources:
          requests:
            memory: "512Mi"
            cpu: "500m"
          limits:
            memory: "1Gi"
            cpu: "1000m"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
---
apiVersion: v1
kind: Service
metadata:
  name: walrus-kyc-service
spec:
  type: LoadBalancer
  ports:
  - port: 80
    targetPort: 8080
  selector:
    app: walrus-kyc-service
```

Apply:
```bash
kubectl apply -f deployment.yaml
```

## Mainnet Deployment Checklist

- [ ] Test all functionality on testnet
- [ ] Security audit of Move contracts
- [ ] Load testing completed
- [ ] Monitoring and alerting configured
- [ ] Backup strategy implemented
- [ ] Disaster recovery plan documented
- [ ] Update environment variables for mainnet:
  - [ ] SUI_NETWORK=mainnet
  - [ ] SUI_RPC_URL=https://fullnode.mainnet.sui.io:443
  - [ ] WALRUS_PUBLISHER_URL (mainnet endpoint)
  - [ ] WALRUS_AGGREGATOR_URL (mainnet endpoint)
- [ ] Deploy Move contracts to mainnet
- [ ] Update SUI_PACKAGE_ID
- [ ] Configure production secrets management
- [ ] Enable HTTPS/TLS
- [ ] Configure rate limiting
- [ ] Set up log aggregation
- [ ] Configure automated backups

## Monitoring

### Metrics Collection

Use Prometheus + Grafana:

```yaml
# docker-compose.yml additions
  prometheus:
    image: prom/prometheus
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml
    ports:
      - "9090:9090"

  grafana:
    image: grafana/grafana
    ports:
      - "3000:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
```

### Log Aggregation

Configure centralized logging:

```bash
# CloudWatch (AWS)
RUST_LOG=info,service=debug cargo run | aws logs put-log-events ...

# Stackdriver (GCP)
gcloud logging write my-log "Service started"

# Self-hosted: ELK Stack
```

## Backup Strategy

### Move Contract State
- Sui blockchain handles data persistence
- No manual backup needed for on-chain data

### Walrus Blobs
- Walrus provides redundancy automatically
- Keep metadata backup (blob_id mappings)

### Application State
- If using PostgreSQL cache, backup regularly:
```bash
pg_dump walrus_kyc > backup.sql
```

## Security Hardening

1. **Use secrets management**:
   - AWS Secrets Manager
   - GCP Secret Manager
   - HashiCorp Vault

2. **Enable HTTPS** with valid TLS certificate

3. **Configure CORS** properly:
```env
CORS_ALLOWED_ORIGINS=https://yourdomain.com,https://app.yourdomain.com
```

4. **Rate limiting** (add middleware)

5. **Input validation** (already implemented)

6. **Regular security updates**:
```bash
cargo audit
```

## Troubleshooting

### Service won't start
- Check `.env` file exists and is valid
- Verify Sui RPC is accessible
- Check Walrus endpoints are correct

### Transactions failing
- Ensure sufficient SUI balance for gas
- Verify package ID matches deployed contracts
- Check network connectivity to Sui RPC

### Walrus upload errors
- Verify publisher URL is correct
- Check network connectivity
- Ensure sufficient storage quota

## Support

- GitHub Issues: https://github.com/Legatia/walrus_kyc_contract_helper/issues
- Sui Discord: https://discord.gg/sui
- Walrus Documentation: https://docs.walrus.site
