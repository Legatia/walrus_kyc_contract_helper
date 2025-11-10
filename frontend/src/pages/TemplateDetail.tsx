import { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { FileText, Users, DollarSign, AlertCircle, Check, Plus, X } from 'lucide-react';
import { api } from '@/lib/api';
import { formatPrice } from '@/lib/utils';
import type { Template, CreateInstanceRequest, SignerRequest } from '@/types';

export function TemplateDetail() {
  const { templateId } = useParams<{ templateId: string }>();
  const navigate = useNavigate();

  const [template, setTemplate] = useState<Template | null>(null);
  const [loading, setLoading] = useState(true);
  const [creating, setCreating] = useState(false);
  const [error, setError] = useState<string>('');

  const [showCreateForm, setShowCreateForm] = useState(false);
  const [variableData, setVariableData] = useState<Record<string, string>>({});
  const [signers, setSigners] = useState<SignerRequest[]>([
    { sui_address: '', role: 'Signer' },
  ]);

  useEffect(() => {
    loadTemplate();
  }, [templateId]);

  const loadTemplate = async () => {
    if (!templateId) return;

    try {
      setLoading(true);
      const data = await api.getTemplate(templateId);
      setTemplate(data);

      // Initialize variable data
      if (data.variables) {
        const initialData: Record<string, string> = {};
        data.variables.forEach((v) => {
          initialData[v] = '';
        });
        setVariableData(initialData);
      }
    } catch (err: any) {
      setError(err.message || 'Failed to load template');
    } finally {
      setLoading(false);
    }
  };

  const addSigner = () => {
    setSigners([...signers, { sui_address: '', role: 'Signer' }]);
  };

  const removeSigner = (index: number) => {
    setSigners(signers.filter((_, i) => i !== index));
  };

  const updateSigner = (index: number, field: keyof SignerRequest, value: string) => {
    const updated = [...signers];
    updated[index] = { ...updated[index], [field]: value };
    setSigners(updated);
  };

  const handleCreateInstance = async () => {
    if (!templateId) return;

    // Validation
    const emptyVariables = Object.entries(variableData).filter(([_, v]) => !v);
    if (emptyVariables.length > 0) {
      setError('Please fill in all template variables');
      return;
    }

    const emptySigners = signers.filter((s) => !s.sui_address);
    if (emptySigners.length > 0) {
      setError('Please provide Sui addresses for all signers');
      return;
    }

    setCreating(true);
    setError('');

    try {
      const request: CreateInstanceRequest = {
        variable_data: variableData,
        required_signers: signers,
        payment_coin_id: '0x0000000000000000000000000000000000000000', // Mock payment
      };

      const response = await api.createInstance(templateId, request);
      navigate(`/instances/${response.instance_id}`);
    } catch (err: any) {
      setError(err.message || 'Failed to create instance');
    } finally {
      setCreating(false);
    }
  };

  if (loading) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-primary-600 mx-auto mb-4"></div>
          <p className="text-gray-600">Loading template...</p>
        </div>
      </div>
    );
  }

  if (!template) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-center">
          <FileText className="mx-auto h-12 w-12 text-gray-400 mb-4" />
          <h3 className="text-lg font-medium text-gray-900">Template not found</h3>
          <button onClick={() => navigate('/marketplace')} className="btn-primary mt-4">
            Back to Marketplace
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50">
      <div className="bg-white border-b border-gray-200">
        <div className="max-w-4xl mx-auto px-4 py-6">
          <button
            onClick={() => navigate('/marketplace')}
            className="text-primary-600 hover:text-primary-700 mb-4"
          >
            ← Back to Marketplace
          </button>
          <h1 className="text-3xl font-bold text-gray-900">{template.name}</h1>
          <p className="mt-2 text-gray-600">{template.description}</p>
        </div>
      </div>

      <div className="max-w-4xl mx-auto px-4 py-8">
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          {/* Template Info */}
          <div className="lg:col-span-2 space-y-6">
            <div className="card">
              <h2 className="text-xl font-semibold mb-4">Template Details</h2>

              <div className="space-y-4">
                <div className="flex items-center gap-3">
                  <DollarSign className="h-5 w-5 text-gray-400" />
                  <div>
                    <p className="text-sm text-gray-600">Price per use</p>
                    <p className="text-lg font-semibold">{formatPrice(template.price)}</p>
                  </div>
                </div>

                <div className="flex items-center gap-3">
                  <Users className="h-5 w-5 text-gray-400" />
                  <div>
                    <p className="text-sm text-gray-600">Usage count</p>
                    <p className="text-lg font-semibold">{template.usage_count} times</p>
                  </div>
                </div>

                <div className="flex items-center gap-3">
                  <span className="text-xl">⭐</span>
                  <div>
                    <p className="text-sm text-gray-600">Rating</p>
                    <p className="text-lg font-semibold">{template.rating.toFixed(1)} / 5.0</p>
                  </div>
                </div>
              </div>
            </div>

            {template.variables && template.variables.length > 0 && (
              <div className="card">
                <h2 className="text-xl font-semibold mb-4">Template Variables</h2>
                <div className="flex flex-wrap gap-2">
                  {template.variables.map((variable) => (
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
          </div>

          {/* Actions */}
          <div className="lg:col-span-1">
            <div className="card sticky top-4">
              <button
                onClick={() => setShowCreateForm(!showCreateForm)}
                className="btn-primary w-full"
              >
                {showCreateForm ? 'Cancel' : 'Use This Template'}
              </button>
            </div>
          </div>
        </div>

        {/* Create Instance Form */}
        {showCreateForm && (
          <div className="mt-6 card">
            <h2 className="text-xl font-semibold mb-6">Create Contract Instance</h2>

            <div className="space-y-6">
              {/* Variables */}
              <div>
                <h3 className="text-lg font-medium mb-4">Fill Template Variables</h3>
                <div className="space-y-4">
                  {template.variables?.map((variable) => (
                    <div key={variable}>
                      <label className="block text-sm font-medium text-gray-700 mb-2">
                        {variable.replace(/_/g, ' ').replace(/\b\w/g, (l) => l.toUpperCase())}
                      </label>
                      <input
                        type="text"
                        value={variableData[variable] || ''}
                        onChange={(e) =>
                          setVariableData({ ...variableData, [variable]: e.target.value })
                        }
                        placeholder={`Enter ${variable}`}
                        className="input-field"
                      />
                    </div>
                  ))}
                </div>
              </div>

              {/* Signers */}
              <div>
                <div className="flex items-center justify-between mb-4">
                  <h3 className="text-lg font-medium">Required Signers</h3>
                  <button
                    onClick={addSigner}
                    className="flex items-center gap-2 text-sm text-primary-600 hover:text-primary-700"
                  >
                    <Plus className="h-4 w-4" />
                    Add Signer
                  </button>
                </div>

                <div className="space-y-4">
                  {signers.map((signer, index) => (
                    <div key={index} className="flex gap-3">
                      <div className="flex-1">
                        <input
                          type="text"
                          value={signer.sui_address}
                          onChange={(e) =>
                            updateSigner(index, 'sui_address', e.target.value)
                          }
                          placeholder="Sui address (0x...)"
                          className="input-field"
                        />
                      </div>
                      <div className="w-32">
                        <input
                          type="text"
                          value={signer.role}
                          onChange={(e) => updateSigner(index, 'role', e.target.value)}
                          placeholder="Role"
                          className="input-field"
                        />
                      </div>
                      {signers.length > 1 && (
                        <button
                          onClick={() => removeSigner(index)}
                          className="text-red-600 hover:text-red-700"
                        >
                          <X className="h-5 w-5" />
                        </button>
                      )}
                    </div>
                  ))}
                </div>
              </div>

              {error && (
                <div className="bg-red-50 border border-red-200 rounded-lg p-4 flex items-start gap-3">
                  <AlertCircle className="h-5 w-5 text-red-600 flex-shrink-0 mt-0.5" />
                  <p className="text-sm text-red-800">{error}</p>
                </div>
              )}

              <div className="bg-blue-50 border border-blue-200 rounded-lg p-4">
                <p className="text-sm text-blue-900">
                  <strong>Note:</strong> You will be charged {formatPrice(template.price)}{' '}
                  to create this contract instance. The generated document will be stored on
                  Walrus and signers can sign it on the blockchain.
                </p>
              </div>

              <button
                onClick={handleCreateInstance}
                disabled={creating}
                className="btn-primary w-full"
              >
                {creating ? 'Creating Instance...' : 'Create Contract Instance'}
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
