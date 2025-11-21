/**
 * zkLogin Authentication Utilities
 *
 * This module provides utilities for integrating Sui zkLogin authentication.
 * zkLogin allows users to authenticate with OAuth providers (Google, Facebook, Twitch)
 * and get a Sui address without managing private keys directly.
 *
 * For full implementation guide, see docs/ZKLOGIN_SETUP.md
 */

export interface ZkLoginConfig {
  googleClientId?: string;
  facebookClientId?: string;
  twitchClientId?: string;
  proverUrl: string;
  saltServiceUrl: string;
}

/**
 * Load zkLogin configuration from environment variables
 */
export function getZkLoginConfig(): ZkLoginConfig {
  return {
    googleClientId: import.meta.env.VITE_GOOGLE_CLIENT_ID,
    facebookClientId: import.meta.env.VITE_FACEBOOK_CLIENT_ID,
    twitchClientId: import.meta.env.VITE_TWITCH_CLIENT_ID,
    proverUrl: import.meta.env.VITE_PROVER_URL || 'https://prover-dev.mystenlabs.com/v1',
    saltServiceUrl:
      import.meta.env.VITE_SALT_SERVICE_URL || 'https://salt.api.mystenlabs.com/get_salt',
  };
}

/**
 * Check if zkLogin is configured
 */
export function isZkLoginConfigured(): boolean {
  const config = getZkLoginConfig();
  return !!(config.googleClientId || config.facebookClientId || config.twitchClientId);
}

/**
 * OAuth Provider types
 */
export type OAuthProvider = 'google' | 'facebook' | 'twitch';

/**
 * Get OAuth client ID for provider
 */
export function getOAuthClientId(provider: OAuthProvider): string | undefined {
  const config = getZkLoginConfig();
  switch (provider) {
    case 'google':
      return config.googleClientId;
    case 'facebook':
      return config.facebookClientId;
    case 'twitch':
      return config.twitchClientId;
  }
}

/**
 * Start OAuth login flow
 *
 * This is a placeholder for the actual OAuth flow.
 * In production, this would:
 * 1. Generate ephemeral key pair
 * 2. Create nonce from ephemeral public key
 * 3. Build OAuth authorization URL with nonce
 * 4. Open OAuth popup
 * 5. Handle OAuth callback with JWT
 * 6. Generate ZK proof using JWT + salt
 * 7. Compute zkLogin Sui address
 *
 * For full implementation, see:
 * https://docs.sui.io/guides/developer/cryptography/zklogin-integration
 */
export function startOAuthLogin(provider: OAuthProvider): Promise<string> {
  console.log(`Starting zkLogin OAuth flow for ${provider}`);
  console.warn('zkLogin not fully configured. See docs/ZKLOGIN_SETUP.md');

  // In production, implement full OAuth flow
  return Promise.reject(new Error('zkLogin not configured'));
}

/**
 * Get JWT token from storage
 * In production, this would retrieve the authenticated JWT from secure storage
 */
export function getAuthToken(): string | null {
  return localStorage.getItem('zklogin_jwt');
}

/**
 * Set JWT token in storage
 */
export function setAuthToken(token: string): void {
  localStorage.setItem('zklogin_jwt', token);
}

/**
 * Clear authentication
 */
export function clearAuth(): void {
  localStorage.removeItem('zklogin_jwt');
}

/**
 * Check if user is authenticated
 */
export function isAuthenticated(): boolean {
  const token = getAuthToken();
  if (!token) return false;

  // In production, verify token expiration
  try {
    const payload = JSON.parse(atob(token.split('.')[1]));
    const exp = payload.exp * 1000; // Convert to milliseconds
    return Date.now() < exp;
  } catch {
    return false;
  }
}

/**
 * Get Sui address from JWT token
 * In production, this would extract the zkLogin-computed Sui address
 */
export function getSuiAddressFromToken(token: string): string | null {
  try {
    const payload = JSON.parse(atob(token.split('.')[1]));
    return payload.sui_address || null;
  } catch {
    return null;
  }
}
