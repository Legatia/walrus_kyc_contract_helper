export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
}

export interface Template {
  id: string;
  name: string;
  description: string;
  creator: string;
  price: number;
  usage_count: number;
  rating: number;
  preview_url: string;
  category: string;
  variables?: string[];
}

export interface CreateTemplateRequest {
  name: string;
  description: string;
  template_pdf: string; // Base64 encoded
  category: string;
  variables: string[];
  price_per_use: number;
  royalty_percentage: number;
  is_public: boolean;
}

export interface CreateTemplateResponse {
  template_id: string;
  blob_id: string;
  sui_object_id: string;
  marketplace_url: string;
}

export interface CreateInstanceRequest {
  variable_data: Record<string, string>;
  required_signers: SignerRequest[];
  payment_coin_id: string;
}

export interface SignerRequest {
  sui_address: string;
  role: string;
}

export interface CreateInstanceResponse {
  instance_id: string;
  generated_blob_id: string;
  document_url: string;
  payment_tx: string;
  status: string;
}

export interface SignInstanceRequest {
  signer_address: string;
  signature: string;
}

export interface SignInstanceResponse {
  signed_at: string;
  remaining_signers: string[];
  fully_signed: boolean;
}

export interface ContractInstance {
  id: string;
  template_id: string;
  status: 'pending_signatures' | 'fully_signed' | 'completed';
  variable_data: Record<string, string>;
  required_signers: string[];
  document_url: string;
}
