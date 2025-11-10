# Walrus KYC Contract Helper - Setup Guide

Complete guide to set up and run the decentralized contract management system.

## 🎯 Prerequisites

- **Rust** 1.75+ (for backend)
- **Node.js** 18+ and npm (for frontend)
- **Sui Wallet** (browser extension) for signing transactions
- **Walrus Testnet Access** (optional, for storage)

## 📦 Project Structure

```
walrus_kyc_contract_helper/
├── service/          # Rust backend API
├── frontend/         # React + TypeScript UI
├── domain/           # Shared types
├── sui-client/       # Sui blockchain integration
├── walrus-client/    # Walrus storage integration
└── move/             # Smart contracts
```

## 🚀 Quick Start

### 1. Backend Setup

```bash
cd service

# The service will use default configuration for testnet
# All required settings have sensible defaults

# Build and run
cargo run --release
```

The backend will start on **http://localhost:8080**

### 2. Frontend Setup

```bash
cd frontend

# Install dependencies (including Sui wallet packages)
npm install

# Start development server
npm run dev
```

The frontend will start on **http://localhost:3000**

## ⚙️ Configuration (Optional)

### Backend Configuration

Create `service/.env` file for custom settings:

```bash
# Copy example file
cp service/.env.example service/.env
```

Edit `.env` with your values:

```env
# API Configuration
API_HOST=0.0.0.0
API_PORT=8080

# Sui Blockchain (defaults to testnet)
SUI_RPC_URL=https://fullnode.testnet.sui.io:443
SUI_PACKAGE_ID=0x0000000000000000000000000000000000000000000000000000000000000000
SUI_PRIVATE_KEY=your_sui_private_key_here

# Walrus Storage (defaults to testnet)
WALRUS_PUBLISHER_URL=https://publisher.walrus-testnet.walrus.space
WALRUS_AGGREGATOR_URL=https://aggregator.walrus-testnet.walrus.space
WALRUS_EPOCHS=5

# Upload Limits
MAX_UPLOAD_SIZE_MB=10
```

### Frontend Configuration

The frontend uses Vite's proxy to automatically forward API calls to the backend.

Edit `frontend/vite.config.ts` if backend runs on different port:

```typescript
server: {
  port: 3000,
  proxy: {
    '/api': {
      target: 'http://localhost:8080', // Change if needed
      changeOrigin: true,
    },
  },
},
```

## 🔑 Sui Wallet Setup

### Install Sui Wallet Extension

1. Install **Sui Wallet** browser extension:
   - Chrome: [Sui Wallet](https://chrome.google.com/webstore/detail/sui-wallet)
   - Or use **Ethos Wallet**, **Martian Wallet**, etc.

2. Create a new wallet or import existing one

3. Switch to **Testnet** network:
   - Click wallet → Settings → Network → Testnet

4. Get testnet SUI tokens:
   - Visit [Sui Testnet Faucet](https://faucet.testnet.sui.io/)
   - Enter your wallet address
   - Request tokens (free for testing)

## 💰 Getting Test Tokens

### SUI Testnet Tokens

```bash
# Visit faucet
https://faucet.testnet.sui.io/

# Or use CLI
curl --location --request POST 'https://faucet.testnet.sui.io/gas' \
  --header 'Content-Type: application/json' \
  --data-raw '{
    "FixedAmountRequest": {
      "recipient": "YOUR_SUI_ADDRESS"
    }
  }'
```

## 📝 Usage Guide

### 1. Connect Wallet

1. Open frontend at http://localhost:3000
2. Click **"Connect Wallet"** button in top-right
3. Select your Sui wallet
4. Approve connection

### 2. Create Template

1. Click **"Create Template"**
2. Upload a PDF with variables like `{{customer_name}}`
3. Fill in template details:
   - Name, description, category
   - Price per use (in SUI)
   - Royalty percentage
4. Click **"Create Template"**
5. PDF stored on Walrus, metadata on Sui blockchain

### 3. Use Template (Create Instance)

1. Browse marketplace
2. Click on a template
3. Click **"Use This Template"**
4. Fill in all variables
5. Add required signers (Sui addresses)
6. Click **"Pay & Create Contract Instance"**
7. **Payment transaction** executes:
   - Your wallet prompts for approval
   - SUI transferred to template creator
   - Transaction confirmed on blockchain
8. Unsigned PDF generated and stored

### 4. Sign Document

1. Navigate to your contract instance
2. Click **"Sign Document"**
3. Enter your Sui address
4. Paste signature data
5. Click **"Sign Document"**
6. Signature recorded on blockchain
7. When all signers complete, final PDF generated

### 5. Verify Document

1. Scan QR code on signed document
2. Or visit `/verify/{instance_id}`
3. View all blockchain signatures
4. See Walrus blob IDs
5. Verify document authenticity

## 🏗️ Architecture

### Backend (Rust)

- **Axum** - Web framework
- **Tokio** - Async runtime
- **lopdf** - PDF manipulation
- **reqwest** - HTTP client for Sui RPC

### Frontend (React)

- **@mysten/dapp-kit** - Sui wallet integration
- **@mysten/sui** - Transaction building
- **React Router** - Page routing
- **Tailwind CSS** - Styling

### Blockchain

- **Sui** - Smart contract platform
- **Walrus** - Decentralized blob storage
- **Move** - Smart contract language

## 🔧 Development

### Backend Development

```bash
# Run with auto-reload
cargo watch -x run

# Run tests
cargo test

# Check for issues
cargo clippy
```

### Frontend Development

```bash
# Development server with hot reload
npm run dev

# Type checking
npm run build

# Lint
npm run lint
```

### Deploy Smart Contracts

```bash
cd move

# Build Move modules
sui move build

# Test Move modules
sui move test

# Deploy to testnet
sui client publish --gas-budget 100000000
```

## 🐛 Troubleshooting

### Backend won't start

```bash
# Check if port 8080 is in use
lsof -i :8080

# Try different port
API_PORT=3001 cargo run
```

### Frontend can't connect to backend

1. Check backend is running on port 8080
2. Check browser console for errors
3. Verify proxy settings in `vite.config.ts`

### Wallet connection fails

1. Make sure wallet extension is installed
2. Switch to Testnet network in wallet
3. Refresh the page
4. Try different wallet (Sui Wallet, Ethos, etc.)

### Payment transaction fails

1. Check you have enough SUI tokens
2. Check you're on Testnet network
3. Check wallet is unlocked
4. Try approving transaction again

### PDF upload fails

1. Check PDF is valid
2. Check file size < 10MB
3. Check PDF contains text (not just images)

## 📚 API Documentation

### REST Endpoints

```
POST   /api/v1/templates              - Create template
GET    /api/v1/marketplace/templates  - Browse marketplace
GET    /api/v1/templates/:id          - Get template details
POST   /api/v1/templates/:id/instances - Create instance
GET    /api/v1/instances/:id/document - Download PDF
POST   /api/v1/instances/:id/sign     - Sign document
```

See full API docs: [docs/API.md](docs/API.md)

## 🌐 Network URLs

### Sui Testnet
- RPC: `https://fullnode.testnet.sui.io:443`
- Explorer: https://suiexplorer.com/?network=testnet
- Faucet: https://faucet.testnet.sui.io/

### Sui Mainnet
- RPC: `https://fullnode.mainnet.sui.io:443`
- Explorer: https://suiexplorer.com/?network=mainnet

### Walrus Testnet
- Publisher: `https://publisher.walrus-testnet.walrus.space`
- Aggregator: `https://aggregator.walrus-testnet.walrus.space`

## 📄 License

MIT OR Apache-2.0

## 🤝 Support

For issues and questions:
- GitHub Issues: [Create Issue](https://github.com/Legatia/walrus_kyc_contract_helper/issues)
- Documentation: See `docs/` folder

## 🎉 Success!

You're all set! Visit http://localhost:3000 and start creating templates.
