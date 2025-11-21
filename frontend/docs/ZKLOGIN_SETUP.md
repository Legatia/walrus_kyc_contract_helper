# zkLogin Authentication Setup

This document explains how to enable zkLogin authentication for the Walrus Contracts marketplace.

## What is zkLogin?

zkLogin is Sui's two-factor authentication system that allows users to authenticate using:
1. OAuth credentials (Google, Facebook, Twitch)
2. A unique salt value

Users can login with familiar Web2 credentials while getting a Sui blockchain address.

## Architecture

```
User → OAuth Provider (Google/Facebook/Twitch)
      ↓
     JWT Token
      ↓
  ZK Proving Service → Generates ZK Proof
      ↓
  zkLogin Sui Address (computed from OAuth sub + salt)
      ↓
  Transaction Signatures (verified by Sui validators)
```

## Setup Steps

### 1. Register OAuth Applications

Register applications with OAuth providers to get Client IDs:

**Google:**
1. Go to [Google Cloud Console](https://console.cloud.google.com/)
2. Create a new project
3. Enable Google+ API
4. Create OAuth 2.0 Client ID (Web application type)
5. Add authorized redirect URIs: `http://localhost:5173` (dev), `https://yourdomain.com` (prod)
6. Copy the Client ID

**Facebook:**
1. Go to [Facebook Developers](https://developers.facebook.com/)
2. Create a new app
3. Add Facebook Login product
4. Configure OAuth Redirect URIs
5. Copy the App ID

**Twitch:**
1. Go to [Twitch Developer Console](https://dev.twitch.tv/console)
2. Register a new application
3. Add OAuth Redirect URLs
4. Copy the Client ID

### 2. Configure Frontend

Create `frontend/.env.local`:

```bash
# OAuth Provider Client IDs
VITE_GOOGLE_CLIENT_ID=your_google_client_id.apps.googleusercontent.com
VITE_FACEBOOK_CLIENT_ID=your_facebook_app_id
VITE_TWITCH_CLIENT_ID=your_twitch_client_id

# ZK Proving Service
VITE_PROVER_URL=https://prover-dev.mystenlabs.com/v1
# Or self-host: http://localhost:8080

# Salt Service
VITE_SALT_SERVICE_URL=https://salt.api.mystenlabs.com/get_salt
# Or use custom salt management
```

### 3. Install zkLogin Dependencies

The `@mysten/dapp-kit` package already includes zkLogin support. No additional packages needed.

### 4. Update SuiProvider

See `src/lib/SuiProvider.tsx` - the code is already configured to support zkLogin-compatible wallets.

### 5. Add zkLogin Button

The authentication flow is triggered when users connect their wallet. Many wallets now support zkLogin natively.

For custom zkLogin UI, see `src/components/ZkLoginButton.tsx` (to be implemented).

## Implementation Flow

### Frontend Flow

1. User clicks "Login with Google/Facebook/Twitch"
2. OAuth popup opens
3. User authorizes
4. Frontend receives JWT
5. Frontend generates ephemeral key pair
6. Frontend calls ZK proving service with JWT
7. Proving service returns ZK proof
8. Frontend computes zkLogin Sui address
9. User can now sign transactions

### Backend API Calls

All API requests should include the JWT in the Authorization header:

```typescript
const response = await fetch('/api/v1/templates', {
  headers: {
    'Authorization': `Bearer ${jwt_token}`,
    'Content-Type': 'application/json',
  },
});
```

The backend middleware (`service/src/auth.rs`) extracts and validates the zkLogin claims.

## Production Considerations

### Security

1. **JWT Verification**: Backend must verify JWT signatures against OAuth provider JWKs
2. **Client ID Whitelist**: Only accept JWTs from your registered client IDs
3. **Token Expiration**: Check JWT exp claim
4. **HTTPS Only**: OAuth requires HTTPS in production

### Salt Management

Three options:

1. **Mysten Labs Service** (easiest): `https://salt.api.mystenlabs.com/get_salt`
   - Only works with whitelisted client IDs
   - Need to request whitelisting from Mysten Labs

2. **User-Provided Salt** (most private):
   - User enters/stores their own salt
   - More friction, better privacy

3. **Custom Service** (most control):
   - Build your own salt server
   - Store salts securely (encrypted database)
   - Associate with user identities

### ZK Proving Service

Two options:

1. **Mysten Labs Service** (easiest): `https://prover-dev.mystenlabs.com/v1`
   - Free for development
   - May have rate limits

2. **Self-Hosted** (production):
   - Run `mysten/zklogin:prover-stable` Docker image
   - Requires minimum 16 cores, 16GB RAM
   - Compute-intensive

## Testing

### Development Mode

The current implementation works in "mock mode" without full zkLogin:
- Backend accepts any Bearer token
- Sui wallet connection works without zkLogin
- Transactions use standard Sui wallet signatures

### Enabling Full zkLogin

To enable production zkLogin:

1. Set up OAuth applications
2. Configure environment variables
3. Implement ZkLoginButton component
4. Update backend JWT verification to use real OAuth JWKs
5. Test with Google/Facebook/Twitch logins

## Resources

- [Official zkLogin Guide](https://docs.sui.io/guides/developer/cryptography/zklogin-integration)
- [zkLogin Demo](https://github.com/juzybits/polymedia-zklogin-demo)
- [zkLogin Deep Dive](https://blog.sui.io/zklogin-deep-dive/)
- [Mysten Labs zkLogin Service](https://mystenlabs.com/)

## Support

For issues with zkLogin setup:
1. Check OAuth configuration
2. Verify redirect URIs match exactly
3. Ensure JWT is being sent in Authorization header
4. Check backend logs for JWT validation errors
