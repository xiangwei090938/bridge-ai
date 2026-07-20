import { create } from "zustand";

interface ProviderState {
  id: string;
  display_name: string;
  provider_type: string;
  base_url: string;
  is_enabled: boolean;
  models: string[];
}

interface Conversation {
  id: string;
  title: string;
  model_id: string;
  created_at: string;
  updated_at: string;
  message_count: number;
}

interface AppStore {
  initialized: boolean;
  setInitialized: (v: boolean) => void;
  onboardingCompleted: boolean;
  setOnboardingCompleted: (v: boolean) => void;
  theme: string;
  setTheme: (v: string) => void;
  providers: ProviderState[];
  setProviders: (v: ProviderState[]) => void;
  conversations: Conversation[];
  setConversations: (v: Conversation[]) => void;
  currentConversationId: string | undefined;
  setCurrentConversationId: (v: string | undefined) => void;
}

export const useAppStore = create<AppStore>((set) => ({
  initialized: false,
  setInitialized: (v) => set({ initialized: v }),
  onboardingCompleted: false,
  setOnboardingCompleted: (v) => set({ onboardingCompleted: v }),
  theme: "dark",
  setTheme: (v) => set({ theme: v }),
  providers: [],
  setProviders: (v) => set({ providers: v }),
  conversations: [],
  setConversations: (v) => set({ conversations: v }),
  currentConversationId: undefined,
  setCurrentConversationId: (v) => set({ currentConversationId: v }),
}));
