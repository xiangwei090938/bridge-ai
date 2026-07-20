export interface ToolInfo {
  id: string;
  name: string;
  category: string;
  is_installed: boolean;
  install_path: string | null;
  config_path: string | null;
  api_protocols: string[];
  has_config: boolean;
  sync_supported: boolean;
  launch_uri: string | null;
}

export interface UserModel {
  internalId: string;
  name: string;
  modelId: string;
  baseUrl: string;
  apiKey: string;
  anthropicUrl: string | null;
  type: string;
}

export interface ProviderEntry {
  name: string;
  url: string | null;
  baseUrl: string | null;
  anthropicUrl: string | null;
  modelId: string | null;
  modelIds: string[] | null;
  region: string | null;
}

export interface ProviderDirectory {
  providers: ProviderEntry[];
}

export interface SyncResult {
  tool_id: string;
  tool_name: string;
  success: boolean;
  message: string;
  backed_up: boolean;
}

export interface LicenseInfo {
  is_premium: boolean;
  activated_at: string | null;
  expires_at: string | null;
  days_remaining: number;
}

export interface SkillInfo {
  id: string;
  name: string;
  description: string;
  author: string;
  version: string;
  size_bytes: number;
  downloads: number;
  category: string;
  tags: string[];
  requires_premium: boolean;
  icon: string;
}

export interface Conversation {
  id: string;
  title: string;
  model_id: string;
  created_at: string;
  updated_at: string;
  message_count: number;
}

export interface Message {
  id: string;
  role: string;
  content: string;
  created_at: string;
}