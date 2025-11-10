import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Upload, FileText, AlertCircle, Check } from 'lucide-react';
import { api } from '@/lib/api';
import type { CreateTemplateRequest } from '@/types';

export function CreateTemplate() {
  const navigate = useNavigate();
  const [step, setStep] = useState<'upload' | 'details' | 'success'>('upload');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string>('');

  const [pdfFile, setPdfFile] = useState<File | null>(null);
  const [pdfBase64, setPdfBase64] = useState<string>('');
  const [detectedVariables, setDetectedVariables] = useState<string[]>([]);

  const [formData, setFormData] = useState({
    name: '',
    description: '',
    category: 'nda',
    price_per_use: 5000000000, // 5 SUI in MIST
    royalty_percentage: 10,
    is_public: true,
  });

  const [createdTemplate, setCreatedTemplate] = useState<{
    template_id: string;
    blob_id: string;
  } | null>(null);

  const handleFileUpload = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    if (file.type !== 'application/pdf') {
      setError('Please upload a PDF file');
      return;
    }

    setError('');
    setPdfFile(file);

    try {
      const base64 = await api.fileToBase64(file);
      setPdfBase64(base64);

      // Extract variables from PDF content (simplified client-side detection)
      // The backend will do proper PDF parsing
      const variables = ['customer_name', 'date', 'address']; // Placeholder
      setDetectedVariables(variables);
    } catch (err) {
      setError('Failed to process PDF file');
      console.error(err);
    }
  };

  const handleSubmit = async () => {
    if (!pdfBase64) {
      setError('Please upload a PDF template');
      return;
    }

    setLoading(true);
    setError('');

    try {
      const request: CreateTemplateRequest = {
        name: formData.name,
        description: formData.description,
        template_pdf: pdfBase64,
        category: formData.category,
        variables: detectedVariables,
        price_per_use: formData.price_per_use,
        royalty_percentage: formData.royalty_percentage,
        is_public: formData.is_public,
      };

      const response = await api.createTemplate(request);
      setCreatedTemplate({
        template_id: response.template_id,
        blob_id: response.blob_id,
      });
      setStep('success');
    } catch (err: any) {
      setError(err.message || 'Failed to create template');
    } finally {
      setLoading(false);
    }
  };

  const categories = [
    { value: 'mvno', label: 'MVNO' },
    { value: 'saas', label: 'SaaS' },
    { value: 'nda', label: 'NDA' },
    { value: 'employment', label: 'Employment' },
    { value: 'service', label: 'Service Agreement' },
    { value: 'rental', label: 'Rental' },
  ];

  if (step === 'success' && createdTemplate) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center px-4">
        <div className="max-w-md w-full card text-center">
          <div className="mx-auto flex items-center justify-center h-12 w-12 rounded-full bg-green-100 mb-4">
            <Check className="h-6 w-6 text-green-600" />
          </div>
          <h2 className="text-2xl font-bold text-gray-900 mb-2">Template Created!</h2>
          <p className="text-gray-600 mb-6">
            Your template has been successfully uploaded to Walrus and is now available in
            the marketplace.
          </p>
          <div className="bg-gray-50 rounded-lg p-4 mb-6 text-left">
            <div className="text-sm">
              <div className="flex justify-between py-2">
                <span className="text-gray-600">Template ID:</span>
                <span className="font-mono text-sm">{createdTemplate.template_id.slice(0, 8)}...</span>
              </div>
              <div className="flex justify-between py-2">
                <span className="text-gray-600">Walrus Blob ID:</span>
                <span className="font-mono text-sm">{createdTemplate.blob_id.slice(0, 8)}...</span>
              </div>
            </div>
          </div>
          <div className="flex gap-3">
            <button
              onClick={() => navigate('/marketplace')}
              className="btn-secondary flex-1"
            >
              View Marketplace
            </button>
            <button
              onClick={() => navigate(`/templates/${createdTemplate.template_id}`)}
              className="btn-primary flex-1"
            >
              View Template
            </button>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50">
      <div className="bg-white border-b border-gray-200">
        <div className="max-w-4xl mx-auto px-4 py-6">
          <h1 className="text-3xl font-bold text-gray-900">Create Template</h1>
          <p className="mt-2 text-gray-600">
            Upload a PDF contract template with variables like {'{{customer_name}}'}
          </p>
        </div>
      </div>

      <div className="max-w-4xl mx-auto px-4 py-8">
        <div className="card">
          {/* Step 1: Upload PDF */}
          {step === 'upload' && (
            <div>
              <h2 className="text-xl font-semibold mb-4">Upload PDF Template</h2>

              <div className="border-2 border-dashed border-gray-300 rounded-lg p-12 text-center hover:border-primary-500 transition-colors">
                <input
                  type="file"
                  accept=".pdf"
                  onChange={handleFileUpload}
                  className="hidden"
                  id="pdf-upload"
                />
                <label htmlFor="pdf-upload" className="cursor-pointer">
                  {pdfFile ? (
                    <div>
                      <FileText className="mx-auto h-12 w-12 text-primary-600 mb-4" />
                      <p className="text-lg font-medium text-gray-900 mb-1">
                        {pdfFile.name}
                      </p>
                      <p className="text-sm text-gray-500">
                        {(pdfFile.size / 1024).toFixed(2)} KB
                      </p>
                      <p className="mt-4 text-sm text-primary-600">
                        Click to change file
                      </p>
                    </div>
                  ) : (
                    <div>
                      <Upload className="mx-auto h-12 w-12 text-gray-400 mb-4" />
                      <p className="text-lg font-medium text-gray-900 mb-1">
                        Upload PDF Template
                      </p>
                      <p className="text-sm text-gray-500">
                        Click to browse or drag and drop
                      </p>
                    </div>
                  )}
                </label>
              </div>

              {detectedVariables.length > 0 && (
                <div className="mt-6 bg-blue-50 border border-blue-200 rounded-lg p-4">
                  <h3 className="text-sm font-medium text-blue-900 mb-2">
                    Detected Variables:
                  </h3>
                  <div className="flex flex-wrap gap-2">
                    {detectedVariables.map((variable) => (
                      <span
                        key={variable}
                        className="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-blue-100 text-blue-800"
                      >
                        {`{{${variable}}}`}
                      </span>
                    ))}
                  </div>
                </div>
              )}

              {pdfFile && (
                <button
                  onClick={() => setStep('details')}
                  className="btn-primary w-full mt-6"
                >
                  Continue to Details
                </button>
              )}
            </div>
          )}

          {/* Step 2: Template Details */}
          {step === 'details' && (
            <div>
              <button
                onClick={() => setStep('upload')}
                className="text-primary-600 hover:text-primary-700 mb-4"
              >
                ← Back to Upload
              </button>

              <h2 className="text-xl font-semibold mb-6">Template Details</h2>

              <div className="space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Template Name
                  </label>
                  <input
                    type="text"
                    value={formData.name}
                    onChange={(e) =>
                      setFormData({ ...formData, name: e.target.value })
                    }
                    placeholder="e.g., Standard NDA Agreement"
                    className="input-field"
                  />
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Description
                  </label>
                  <textarea
                    value={formData.description}
                    onChange={(e) =>
                      setFormData({ ...formData, description: e.target.value })
                    }
                    placeholder="Describe what this template is for and when to use it..."
                    rows={4}
                    className="input-field"
                  />
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Category
                  </label>
                  <select
                    value={formData.category}
                    onChange={(e) =>
                      setFormData({ ...formData, category: e.target.value })
                    }
                    className="input-field"
                  >
                    {categories.map((cat) => (
                      <option key={cat.value} value={cat.value}>
                        {cat.label}
                      </option>
                    ))}
                  </select>
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Price Per Use (SUI)
                  </label>
                  <input
                    type="number"
                    value={formData.price_per_use / 1_000_000_000}
                    onChange={(e) =>
                      setFormData({
                        ...formData,
                        price_per_use: parseFloat(e.target.value) * 1_000_000_000,
                      })
                    }
                    step="0.1"
                    min="0"
                    className="input-field"
                  />
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Royalty Percentage (%)
                  </label>
                  <input
                    type="number"
                    value={formData.royalty_percentage}
                    onChange={(e) =>
                      setFormData({
                        ...formData,
                        royalty_percentage: parseInt(e.target.value),
                      })
                    }
                    min="0"
                    max="100"
                    className="input-field"
                  />
                </div>

                <div className="flex items-center gap-2">
                  <input
                    type="checkbox"
                    id="is_public"
                    checked={formData.is_public}
                    onChange={(e) =>
                      setFormData({ ...formData, is_public: e.target.checked })
                    }
                    className="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                  />
                  <label htmlFor="is_public" className="text-sm text-gray-700">
                    Make this template publicly available in the marketplace
                  </label>
                </div>
              </div>

              {error && (
                <div className="mt-4 bg-red-50 border border-red-200 rounded-lg p-4 flex items-start gap-3">
                  <AlertCircle className="h-5 w-5 text-red-600 flex-shrink-0 mt-0.5" />
                  <p className="text-sm text-red-800">{error}</p>
                </div>
              )}

              <button
                onClick={handleSubmit}
                disabled={loading || !formData.name || !formData.description}
                className="btn-primary w-full mt-6"
              >
                {loading ? 'Creating Template...' : 'Create Template'}
              </button>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
