import axios, { AxiosInstance } from 'axios';
import type {
  ApiResponse,
  Template,
  CreateTemplateRequest,
  CreateTemplateResponse,
  CreateInstanceRequest,
  CreateInstanceResponse,
  SignInstanceRequest,
  SignInstanceResponse,
} from '@/types';
import { getAuthToken } from './zklogin';

class ApiClient {
  private client: AxiosInstance;

  constructor(baseURL: string = '/api/v1') {
    this.client = axios.create({
      baseURL,
      headers: {
        'Content-Type': 'application/json',
      },
    });

    // Add request interceptor to include auth token
    this.client.interceptors.request.use((config) => {
      const token = getAuthToken();
      if (token) {
        config.headers.Authorization = `Bearer ${token}`;
      }
      return config;
    });
  }

  // Template endpoints
  async createTemplate(data: CreateTemplateRequest): Promise<CreateTemplateResponse> {
    const response = await this.client.post<ApiResponse<CreateTemplateResponse>>(
      '/templates',
      data
    );
    if (!response.data.success || !response.data.data) {
      throw new Error(response.data.error || 'Failed to create template');
    }
    return response.data.data;
  }

  async browseMarketplace(params?: {
    category?: string;
    sort?: string;
    page?: number;
    limit?: number;
  }): Promise<{ templates: Template[]; total: number; page: number }> {
    const response = await this.client.get<
      ApiResponse<{ templates: Template[]; total: number; page: number }>
    >('/marketplace/templates', { params });
    if (!response.data.success || !response.data.data) {
      throw new Error(response.data.error || 'Failed to fetch templates');
    }
    return response.data.data;
  }

  async getTemplate(templateId: string): Promise<Template> {
    const response = await this.client.get<ApiResponse<Template>>(
      `/marketplace/templates/${templateId}`
    );
    if (!response.data.success || !response.data.data) {
      throw new Error(response.data.error || 'Failed to fetch template');
    }
    return response.data.data;
  }

  // Instance endpoints
  async createInstance(
    templateId: string,
    data: CreateInstanceRequest
  ): Promise<CreateInstanceResponse> {
    const response = await this.client.post<ApiResponse<CreateInstanceResponse>>(
      `/templates/${templateId}/instances`,
      data
    );
    if (!response.data.success || !response.data.data) {
      throw new Error(response.data.error || 'Failed to create instance');
    }
    return response.data.data;
  }

  async downloadInstance(instanceId: string): Promise<Blob> {
    const response = await this.client.get(`/instances/${instanceId}/document`, {
      responseType: 'blob',
    });
    return response.data;
  }

  async signInstance(
    instanceId: string,
    data: SignInstanceRequest
  ): Promise<SignInstanceResponse> {
    const response = await this.client.post<ApiResponse<SignInstanceResponse>>(
      `/instances/${instanceId}/sign`,
      data
    );
    if (!response.data.success || !response.data.data) {
      throw new Error(response.data.error || 'Failed to sign instance');
    }
    return response.data.data;
  }

  // Helper to convert file to base64
  async fileToBase64(file: File): Promise<string> {
    return new Promise((resolve, reject) => {
      const reader = new FileReader();
      reader.readAsDataURL(file);
      reader.onload = () => {
        const result = reader.result as string;
        // Remove data URL prefix (data:application/pdf;base64,)
        const base64 = result.split(',')[1];
        resolve(base64);
      };
      reader.onerror = (error) => reject(error);
    });
  }
}

export const api = new ApiClient();
