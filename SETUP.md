# Walrus Contracts - Setup Guide

Complete guide to set up and run the decentralized contract template marketplace.

## 🎯 What is Walrus Contracts?

A decentralized marketplace for contract templates powered by:
- **Walrus** - Decentralized blob storage for PDF templates
- **Sui Blockchain** - Smart contracts for payments and verification
- **Template System** - Reusable templates with variable substitution
- **E-signatures** - Blockchain-verified document signing

## ✨ Key Features

- **Template Marketplace** - Browse and purchase reusable contract templates
- **Variable Substitution** - Templates with `{{customer_name}}` style variables
- **Pay-Per-Use** - Template creators earn revenue from each usage
- **Blockchain Payments** - Direct SUI token transfers to creators
- **Secure Storage** - All documents stored on decentralized Walrus
- **Verified Signatures** - Cryptographically verified on Sui blockchain
- **QR Verification** - Mobile-friendly document authentication

## 🚀 Quick Start

### 1. Backend Setup

```bash
cd service
cargo run --release
```

The backend starts on **http://localhost:8080** with sensible testnet defaults.

### 2. Frontend Setup

```bash
cd frontend
npm install
npm run dev
```

The frontend starts on **http://localhost:3000**

That's it! No configuration needed to get started.

## 📋 Prerequisites

- **Rust** 1.75+ (for backend)
- **Node.js** 18+ and npm (for frontend)
- **Sui Wallet** browser extension
- Testnet SUI tokens (free from faucet)

## 🔑 Sui Wallet Setup

### 1. Install Sui Wallet

- Chrome: [Sui Wallet](https://chrome.google.com/webstore/detail/sui-wallet)
- Or use Ethos Wallet, Martian Wallet, etc.

### 2. Switch to Testnet

- Open wallet → Settings → Network → Testnet

### 3. Get Test Tokens

Visit [Sui Testnet Faucet](https://faucet.testnet.sui.io/) and request free SUI tokens.

## 💡 How It Works

### Template Creation

1. Creator uploads PDF template with variables like `{{customer_name}}`
2. PDF stored on Walrus decentralized storage
3. Template metadata registered on Sui blockchain
4. Set price per use and royalty percentage
5. Published to marketplace

### Using Templates

1. Browse marketplace and select template
2. Fill in variable values
3. Add required signers (Sui addresses)
4. **Pay creator** via SUI transaction
5. System generates unsigned PDF from template
6. Signers sign document on blockchain
7. Final signed PDF generated with signature blocks

### Revenue Model

- Template creators set price per use
- Users pay directly to creator's wallet
- Payments execute atomically on Sui
- Creators earn passive income from templates

## ⚙️ Configuration (Optional)

Create `service/.env` for custom settings:

```env
# API Configuration
API_HOST=0.0.0.0
API_PORT=8080

# Sui Blockchain
SUI_RPC_URL=https://fullnode.testnet.sui.io:443
SUI_PACKAGE_ID=0x...

# Walrus Storage
WALRUS_PUBLISHER_URL=https://publisher.walrus-testnet.walrus.space
WALRUS_AGGREGATOR_URL=https://aggregator.walrus-testnet.walrus.space

# Limits
MAX_UPLOAD_SIZE_MB=10
```

## 🏗️ Architecture

### Backend (Rust)
- **Axum** - Web framework
- **lopdf** - PDF parsing and manipulation
- **reqwest** - Sui RPC client
- **Walrus client** - Decentralized storage

### Frontend (React)
- **@mysten/dapp-kit** - Sui wallet integration
- **React Router** - Navigation
- **Tailwind CSS** - Styling
- **QRCode** - Document verification

### Blockchain
- **Sui** - Payments and verification
- **Walrus** - Template and document storage
- **Move** - Smart contracts (optional)

## 📝 Usage Examples

### Create a Template

```bash
# 1. Connect wallet on frontend
# 2. Click "Create Template"
# 3. Upload PDF with {{variables}}
# 4. Set price: 5 SUI per use
# 5. Set royalty: 10%
# 6. Publish to marketplace
```

### Use a Template

```bash
# 1. Browse marketplace
# 2. Select "NDA Agreement" template (5 SUI)
# 3. Fill variables:
#    - customer_name: "John Doe"
#    - date: "2025-01-15"
#    - company: "Acme Corp"
# 4. Add signers: 0xabc... (Customer), 0xdef... (Company)
# 5. Click "Pay & Create" (5 SUI sent to creator)
# 6. Document generated and stored
```

### Sign Document

```bash
# 1. Navigate to your instance
# 2. Click "Sign Document"
# 3. Approve with Sui wallet
# 4. Signature recorded on blockchain
# 5. When all sign → Final PDF generated
```

### Verify Document

```bash
# Scan QR code or visit /verify/{instance_id}
# Shows:
# - All blockchain signatures
# - Walrus blob IDs
# - Transaction digests
# - Document hash
```

## 🎨 Project Structure

```
walrus-contracts/
├── service/          # Rust backend API
│   ├── src/
│   │   ├── handlers/  # API endpoints
│   │   ├── pdf/       # PDF manipulation
│   │   └── main.rs
│   └── Cargo.toml
├── frontend/         # React + TypeScript
│   ├── src/
│   │   ├── pages/     # UI pages
│   │   ├── components/# Reusable components
│   │   └── lib/       # Utilities
│   └── package.json
├── domain/           # Shared types
├── sui-client/       # Blockchain integration
├── walrus-client/    # Storage integration
└── move/             # Smart contracts (optional)
```

## 🔧 Development

### Backend

```bash
cd service
cargo watch -x run  # Auto-reload
cargo test          # Run tests
cargo clippy        # Lint
```

### Frontend

```bash
cd frontend
npm run dev   # Dev server
npm run build # Production build
```

## 🐛 Troubleshooting

### "Wallet not connected"
- Install Sui Wallet extension
- Switch to Testnet network
- Refresh page

### "Insufficient funds"
- Get free test SUI from [faucet](https://faucet.testnet.sui.io/)

### "Payment failed"
- Check wallet is unlocked
- Check you have enough SUI
- Try transaction again

### Backend won't start
```bash
# Check port 8080
lsof -i :8080

# Try different port
API_PORT=3001 cargo run
```

## 🌐 Network URLs

### Sui Testnet
- RPC: https://fullnode.testnet.sui.io:443
- Explorer: https://suiexplorer.com/?network=testnet
- Faucet: https://faucet.testnet.sui.io/

### Walrus Testnet
- Publisher: https://publisher.walrus-testnet.walrus.space
- Aggregator: https://aggregator.walrus-testnet.walrus.space

## 🎉 Success!

Visit **http://localhost:3000**, connect your Sui wallet, and start creating templates!

## 📚 Learn More

- [Sui Documentation](https://docs.sui.io/)
- [Walrus Documentation](https://docs.walrus.site/)
- [Template Marketplace Architecture](docs/TEMPLATE_MARKETPLACE.md)
- [PDF Generation Pipeline](docs/PDF_GENERATION_PIPELINE.md)

## 📄 License

MIT OR Apache-2.0
