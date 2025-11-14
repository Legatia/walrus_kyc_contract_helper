import { useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { Download, CheckCircle, Clock, AlertCircle, Shield } from 'lucide-react';
import QRCode from 'qrcode.react';
import { api } from '@/lib/api';
import { shortenAddress, formatDate } from '@/lib/utils';
import type { SignInstanceRequest } from '@/types';

export function InstanceDetail() {
  const { instanceId } = useParams<{ instanceId: string }>();
  const navigate = useNavigate();

  const [signing, setSigning] = useState(false);
  const [error, setError] = useState<string>('');
  const [success, setSuccess] = useState(false);

  const [signerAddress, setSignerAddress] = useState('');
  const [signatureData, setSignatureData] = useState('');

  // Mock instance data (in real app, fetch from API)
  const instance = {
    id: instanceId || '',
    template_name: 'Standard NDA Agreement',
    status: 'pending_signatures' as const,
    created_at: new Date().toISOString(),
    variable_data: {
      customer_name: 'John Doe',
      company_name: 'Acme Corp',
      date: '2025-11-10',
    },
    required_signers: [
      {
        address: '0x1234...5678',
        role: 'Customer',
        signed: false,
      },
      {
        address: '0xabcd...ef01',
        role: 'Company Representative',
        signed: false,
      },
    ],
    document_url: `/api/v1/instances/${instanceId}/document`,
  };

  const handleDownload = async () => {
    if (!instanceId) return;

    try {
      const blob = await api.downloadInstance(instanceId);
      const url = window.URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `contract-${instanceId}.pdf`;
      document.body.appendChild(a);
      a.click();
      window.URL.revokeObjectURL(url);
      document.body.removeChild(a);
    } catch (err: any) {
      setError(err.message || 'Failed to download document');
    }
  };

  const handleSign = async () => {
    if (!instanceId || !signerAddress || !signatureData) {
      setError('Please provide both signer address and signature');
      return;
    }

    setSigning(true);
    setError('');

    try {
      const request: SignInstanceRequest = {
        signer_address: signerAddress,
        signature: signatureData,
      };

      const response = await api.signInstance(instanceId, request);
      setSuccess(true);

      if (response.fully_signed) {
        setTimeout(() => {
          navigate(`/verify/${instanceId}`);
        }, 2000);
      }
    } catch (err: any) {
      setError(err.message || 'Failed to sign document');
    } finally {
      setSigning(false);
    }
  };

  if (success) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center px-4">
        <div className="max-w-md w-full card text-center">
          <div className="mx-auto flex items-center justify-center h-12 w-12 rounded-full bg-green-100 mb-4">
            <CheckCircle className="h-6 w-6 text-green-600" />
          </div>
          <h2 className="text-2xl font-bold text-gray-900 mb-2">
            Document Signed Successfully!
          </h2>
          <p className="text-gray-600 mb-6">
            Your signature has been recorded on the Sui blockchain. The final signed
            document will be available shortly.
          </p>
          <button onClick={() => navigate('/marketplace')} className="btn-primary w-full">
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
          <div className="flex items-center justify-between">
            <div>
              <h1 className="text-3xl font-bold text-gray-900">{instance.template_name}</h1>
              <p className="mt-2 text-gray-600">
                Instance ID: {shortenAddress(instance.id, 8)}
              </p>
            </div>
            <div>
              {instance.status === 'pending_signatures' ? (
                <span className="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-yellow-100 text-yellow-800">
                  <Clock className="h-4 w-4 mr-1" />
                  Pending Signatures
                </span>
              ) : (
                <span className="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-green-100 text-green-800">
                  <CheckCircle className="h-4 w-4 mr-1" />
                  Fully Signed
                </span>
              )}
            </div>
          </div>
        </div>
      </div>

      <div className="max-w-4xl mx-auto px-4 py-8">
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          {/* Main Content */}
          <div className="lg:col-span-2 space-y-6">
            {/* Document Info */}
            <div className="card">
              <h2 className="text-xl font-semibold mb-4">Contract Details</h2>
              <div className="space-y-3">
                <div>
                  <p className="text-sm text-gray-600">Created</p>
                  <p className="text-gray-900">{formatDate(instance.created_at)}</p>
                </div>
                <div>
                  <p className="text-sm text-gray-600 mb-2">Contract Data</p>
                  <div className="bg-gray-50 rounded-lg p-4 space-y-2">
                    {Object.entries(instance.variable_data).map(([key, value]) => (
                      <div key={key} className="flex justify-between">
                        <span className="text-sm font-medium text-gray-700">
                          {key.replace(/_/g, ' ').replace(/\b\w/g, (l) => l.toUpperCase())}:
                        </span>
                        <span className="text-sm text-gray-900">{value}</span>
                      </div>
                    ))}
                  </div>
                </div>
              </div>
            </div>

            {/* Signers */}
            <div className="card">
              <h2 className="text-xl font-semibold mb-4">Required Signers</h2>
              <div className="space-y-3">
                {instance.required_signers.map((signer, index) => (
                  <div
                    key={index}
                    className="flex items-center justify-between p-3 bg-gray-50 rounded-lg"
                  >
                    <div>
                      <p className="font-medium text-gray-900">{signer.role}</p>
                      <p className="text-sm text-gray-600">{signer.address}</p>
                    </div>
                    {signer.signed ? (
                      <CheckCircle className="h-5 w-5 text-green-600" />
                    ) : (
                      <Clock className="h-5 w-5 text-yellow-600" />
                    )}
                  </div>
                ))}
              </div>
            </div>

            {/* Signing Form */}
            {instance.status === 'pending_signatures' && (
              <div className="card">
                <h2 className="text-xl font-semibold mb-4">Sign Document</h2>

                <div className="space-y-4">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      Your Sui Address
                    </label>
                    <input
                      type="text"
                      value={signerAddress}
                      onChange={(e) => setSignerAddress(e.target.value)}
                      placeholder="0x..."
                      className="input-field"
                    />
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      Signature Data
                    </label>
                    <textarea
                      value={signatureData}
                      onChange={(e) => setSignatureData(e.target.value)}
                      placeholder="Paste your signature here..."
                      rows={4}
                      className="input-field"
                    />
                    <p className="mt-2 text-xs text-gray-500">
                      Sign the document hash with your Sui wallet and paste the signature
                      here.
                    </p>
                  </div>

                  {error && (
                    <div className="bg-red-50 border border-red-200 rounded-lg p-4 flex items-start gap-3">
                      <AlertCircle className="h-5 w-5 text-red-600 flex-shrink-0 mt-0.5" />
                      <p className="text-sm text-red-800">{error}</p>
                    </div>
                  )}

                  <button
                    onClick={handleSign}
                    disabled={signing || !signerAddress || !signatureData}
                    className="btn-primary w-full"
                  >
                    {signing ? 'Signing...' : 'Sign Document'}
                  </button>
                </div>
              </div>
            )}
          </div>

          {/* Sidebar */}
          <div className="lg:col-span-1 space-y-6">
            {/* Actions */}
            <div className="card">
              <button
                onClick={handleDownload}
                className="btn-primary w-full flex items-center justify-center gap-2"
              >
                <Download className="h-4 w-4" />
                Download PDF
              </button>
            </div>

            {/* QR Code */}
            <div className="card">
              <h3 className="text-lg font-semibold mb-4">Verification QR</h3>
              <div className="flex justify-center">
                <QRCode
                  value={`${window.location.origin}/verify/${instance.id}`}
                  size={200}
                />
              </div>
              <p className="text-xs text-gray-500 text-center mt-4">
                Scan to verify document authenticity
              </p>
            </div>

            {/* Blockchain Info */}
            <div className="card">
              <div className="flex items-center gap-2 mb-4">
                <Shield className="h-5 w-5 text-primary-600" />
                <h3 className="text-lg font-semibold">Blockchain Verified</h3>
              </div>
              <p className="text-sm text-gray-600">
                This document is stored on Walrus and verified on the Sui blockchain.
                All signatures are cryptographically secured.
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
