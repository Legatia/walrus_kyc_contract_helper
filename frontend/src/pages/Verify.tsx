import { useParams, useNavigate } from 'react-router-dom';
import { Shield, CheckCircle, FileText, Clock } from 'lucide-react';
import { shortenAddress, formatDate } from '@/lib/utils';

export function Verify() {
  const { instanceId } = useParams<{ instanceId: string }>();
  const navigate = useNavigate();

  // Mock verification data (in real app, fetch from blockchain)
  const verification = {
    verified: true,
    instance_id: instanceId || '',
    template_name: 'Standard NDA Agreement',
    created_at: '2025-11-10T10:30:00Z',
    fully_signed: true,
    signed_at: '2025-11-10T15:45:00Z',
    walrus_blob_id: 'abc123def456...',
    sui_transaction: '0x7890abcdef1234...',
    signatures: [
      {
        signer: '0x1234...5678',
        role: 'Customer',
        signed_at: '2025-11-10T14:20:00Z',
        transaction: '0x1111...2222',
      },
      {
        signer: '0xabcd...ef01',
        role: 'Company Rep',
        signed_at: '2025-11-10T15:45:00Z',
        transaction: '0x3333...4444',
      },
    ],
    document_hash: 'sha256:a1b2c3d4e5f6...',
  };

  return (
    <div className="min-h-screen bg-gray-50">
      <div className="bg-white border-b border-gray-200">
        <div className="max-w-4xl mx-auto px-4 py-6">
          <h1 className="text-3xl font-bold text-gray-900">Document Verification</h1>
          <p className="mt-2 text-gray-600">
            Verify the authenticity of a blockchain-signed document
          </p>
        </div>
      </div>

      <div className="max-w-4xl mx-auto px-4 py-8">
        {/* Verification Status */}
        <div className="card mb-6">
          <div className="flex items-center gap-4 mb-6">
            {verification.verified ? (
              <>
                <div className="flex items-center justify-center h-12 w-12 rounded-full bg-green-100">
                  <CheckCircle className="h-6 w-6 text-green-600" />
                </div>
                <div>
                  <h2 className="text-2xl font-bold text-gray-900">Verified Authentic</h2>
                  <p className="text-gray-600">
                    This document is cryptographically verified on the blockchain
                  </p>
                </div>
              </>
            ) : (
              <>
                <div className="flex items-center justify-center h-12 w-12 rounded-full bg-red-100">
                  <Shield className="h-6 w-6 text-red-600" />
                </div>
                <div>
                  <h2 className="text-2xl font-bold text-gray-900">Verification Failed</h2>
                  <p className="text-gray-600">Unable to verify this document</p>
                </div>
              </>
            )}
          </div>

          <div className="bg-gray-50 rounded-lg p-4 space-y-3">
            <div className="flex justify-between">
              <span className="text-sm font-medium text-gray-700">Instance ID:</span>
              <span className="text-sm text-gray-900 font-mono">
                {shortenAddress(verification.instance_id, 8)}
              </span>
            </div>
            <div className="flex justify-between">
              <span className="text-sm font-medium text-gray-700">Template:</span>
              <span className="text-sm text-gray-900">{verification.template_name}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-sm font-medium text-gray-700">Status:</span>
              <span className="text-sm">
                {verification.fully_signed ? (
                  <span className="inline-flex items-center text-green-700">
                    <CheckCircle className="h-4 w-4 mr-1" />
                    Fully Signed
                  </span>
                ) : (
                  <span className="inline-flex items-center text-yellow-700">
                    <Clock className="h-4 w-4 mr-1" />
                    Pending Signatures
                  </span>
                )}
              </span>
            </div>
          </div>
        </div>

        {/* Blockchain Details */}
        <div className="card mb-6">
          <h3 className="text-lg font-semibold mb-4 flex items-center gap-2">
            <Shield className="h-5 w-5 text-primary-600" />
            Blockchain Details
          </h3>
          <div className="space-y-3">
            <div>
              <p className="text-sm font-medium text-gray-700 mb-1">Walrus Blob ID</p>
              <p className="text-sm text-gray-900 font-mono bg-gray-50 p-2 rounded">
                {verification.walrus_blob_id}
              </p>
            </div>
            <div>
              <p className="text-sm font-medium text-gray-700 mb-1">Sui Transaction</p>
              <p className="text-sm text-gray-900 font-mono bg-gray-50 p-2 rounded">
                {verification.sui_transaction}
              </p>
            </div>
            <div>
              <p className="text-sm font-medium text-gray-700 mb-1">Document Hash</p>
              <p className="text-sm text-gray-900 font-mono bg-gray-50 p-2 rounded">
                {verification.document_hash}
              </p>
            </div>
            <div>
              <p className="text-sm font-medium text-gray-700 mb-1">Created</p>
              <p className="text-sm text-gray-900">{formatDate(verification.created_at)}</p>
            </div>
            {verification.fully_signed && (
              <div>
                <p className="text-sm font-medium text-gray-700 mb-1">Signed</p>
                <p className="text-sm text-gray-900">{formatDate(verification.signed_at)}</p>
              </div>
            )}
          </div>
        </div>

        {/* Signatures */}
        <div className="card">
          <h3 className="text-lg font-semibold mb-4 flex items-center gap-2">
            <FileText className="h-5 w-5 text-primary-600" />
            Signature Records
          </h3>
          <div className="space-y-4">
            {verification.signatures.map((sig, index) => (
              <div key={index} className="border border-gray-200 rounded-lg p-4">
                <div className="flex items-start justify-between mb-3">
                  <div>
                    <p className="font-medium text-gray-900">{sig.role}</p>
                    <p className="text-sm text-gray-600">{sig.signer}</p>
                  </div>
                  <CheckCircle className="h-5 w-5 text-green-600" />
                </div>
                <div className="space-y-1 text-sm">
                  <div className="flex justify-between">
                    <span className="text-gray-600">Signed At:</span>
                    <span className="text-gray-900">{formatDate(sig.signed_at)}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600">Transaction:</span>
                    <span className="text-gray-900 font-mono">{sig.transaction}</span>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>

        <div className="mt-6 flex justify-center">
          <button onClick={() => navigate('/marketplace')} className="btn-primary">
            Back to Marketplace
          </button>
        </div>
      </div>
    </div>
  );
}
