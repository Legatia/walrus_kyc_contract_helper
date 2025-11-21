/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_API_URL?: string;
  readonly VITE_GOOGLE_CLIENT_ID?: string;
  readonly VITE_FACEBOOK_CLIENT_ID?: string;
  readonly VITE_TWITCH_CLIENT_ID?: string;
  readonly VITE_PROVER_URL?: string;
  readonly VITE_SALT_SERVICE_URL?: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
