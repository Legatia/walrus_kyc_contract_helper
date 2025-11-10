# Walrus KYC Frontend

Modern React + TypeScript frontend for the Walrus KYC contract management system.

## Features

- **Template Marketplace**: Browse and purchase contract templates
- **Template Creation**: Upload PDF templates with variable placeholders like `{{customer_name}}`
- **Instance Creation**: Fill template variables and generate contracts
- **Digital Signing**: Sign documents on the Sui blockchain
- **Verification**: Verify document authenticity with QR codes

## Tech Stack

- **React 18** - UI framework
- **TypeScript** - Type safety
- **Vite** - Build tool and dev server
- **Tailwind CSS** - Styling
- **React Router** - Client-side routing
- **Axios** - API communication
- **Lucide React** - Icons
- **QRCode.react** - QR code generation

## Getting Started

### Prerequisites

- Node.js 18+ and npm

### Installation

```bash
npm install
```

### Development

```bash
npm run dev
```

The app will be available at http://localhost:3000

The Vite dev server automatically proxies API requests to http://localhost:8080

### Build for Production

```bash
npm run build
```

The production build will be in the `dist/` directory.

### Preview Production Build

```bash
npm run preview
```

## Project Structure

```
frontend/
├── src/
│   ├── components/     # Reusable UI components
│   ├── pages/          # Page components
│   │   ├── Marketplace.tsx
│   │   ├── CreateTemplate.tsx
│   │   ├── TemplateDetail.tsx
│   │   ├── InstanceDetail.tsx
│   │   └── Verify.tsx
│   ├── lib/           # Utilities and API client
│   │   ├── api.ts
│   │   └── utils.ts
│   ├── types/         # TypeScript type definitions
│   ├── App.tsx        # Main app component with routing
│   ├── main.tsx       # Entry point
│   └── index.css      # Global styles
├── index.html
├── package.json
├── vite.config.ts
├── tailwind.config.js
└── tsconfig.json
```

## API Integration

The frontend communicates with the Rust backend via REST API:

- `POST /api/v1/templates` - Create new template
- `GET /api/v1/marketplace/templates` - Browse marketplace
- `GET /api/v1/marketplace/templates/:id` - Get template details
- `POST /api/v1/templates/:id/instances` - Create instance
- `GET /api/v1/instances/:id/document` - Download PDF
- `POST /api/v1/instances/:id/sign` - Sign document

## Key Pages

### Marketplace
Browse available contract templates with filtering and sorting.

### Create Template
Upload a PDF template with variables like `{{customer_name}}`. The system automatically detects variables and stores the template on Walrus.

### Template Detail
View template information and create instances by filling in variable values and specifying required signers.

### Instance Detail
View contract instance, download PDF, and sign the document on the blockchain.

### Verify
Verify document authenticity using QR codes. Shows all blockchain signatures and Walrus storage details.

## Environment Variables

Create a `.env` file for custom configuration:

```env
VITE_API_URL=http://localhost:8080/api/v1
```

## License

MIT
