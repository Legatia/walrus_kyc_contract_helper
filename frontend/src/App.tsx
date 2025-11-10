import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom';
import { FileText, Plus, Home } from 'lucide-react';
import { Marketplace } from './pages/Marketplace';
import { CreateTemplate } from './pages/CreateTemplate';
import { TemplateDetail } from './pages/TemplateDetail';
import { InstanceDetail } from './pages/InstanceDetail';
import { Verify } from './pages/Verify';

function App() {
  return (
    <Router>
      <div className="min-h-screen bg-gray-50">
        {/* Navigation */}
        <nav className="bg-white border-b border-gray-200 sticky top-0 z-50">
          <div className="max-w-7xl mx-auto px-4">
            <div className="flex items-center justify-between h-16">
              <Link to="/" className="flex items-center gap-2 text-xl font-bold text-gray-900">
                <FileText className="h-6 w-6 text-primary-600" />
                <span>Walrus KYC</span>
              </Link>

              <div className="flex items-center gap-4">
                <Link
                  to="/marketplace"
                  className="flex items-center gap-2 text-gray-700 hover:text-gray-900 transition-colors"
                >
                  <Home className="h-5 w-5" />
                  <span className="hidden sm:inline">Marketplace</span>
                </Link>
                <Link
                  to="/create-template"
                  className="flex items-center gap-2 btn-primary"
                >
                  <Plus className="h-5 w-5" />
                  <span className="hidden sm:inline">Create Template</span>
                </Link>
              </div>
            </div>
          </div>
        </nav>

        {/* Routes */}
        <Routes>
          <Route path="/" element={<Marketplace />} />
          <Route path="/marketplace" element={<Marketplace />} />
          <Route path="/create-template" element={<CreateTemplate />} />
          <Route path="/templates/:templateId" element={<TemplateDetail />} />
          <Route path="/instances/:instanceId" element={<InstanceDetail />} />
          <Route path="/verify/:instanceId" element={<Verify />} />
        </Routes>

        {/* Footer */}
        <footer className="bg-white border-t border-gray-200 mt-12">
          <div className="max-w-7xl mx-auto px-4 py-8">
            <div className="text-center text-gray-600">
              <p className="mb-2">
                Powered by{' '}
                <a
                  href="https://walrus.site"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-primary-600 hover:text-primary-700 font-medium"
                >
                  Walrus
                </a>{' '}
                &amp;{' '}
                <a
                  href="https://sui.io"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-primary-600 hover:text-primary-700 font-medium"
                >
                  Sui Blockchain
                </a>
              </p>
              <p className="text-sm">
                Decentralized contract management with cryptographic verification
              </p>
            </div>
          </div>
        </footer>
      </div>
    </Router>
  );
}

export default App;
