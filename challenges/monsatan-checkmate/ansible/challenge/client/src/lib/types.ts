/**
 * TypeScript interfaces matching the OpenAPI schemas for the Checkmate API.
 */

/** Public profile returned by GET /api/next */
export interface UserResponse {
  username: string;
  bio: string;
  /** Chess Elo-style attractiveness rating */
  rating: number;
  /** Optional profile picture as a base64-encoded string */
  profilePicture?: string | null;
}

/**
 * Full database row returned by GET /api/user/{username}.
 * NOTE: passwordRaw is an intentional CTF vulnerability — the API
 * leaks the base64-encoded plaintext password in this endpoint.
 */
export interface UserRow {
  username: string;
  /** Base64-encoded plaintext password (intentional CTF vulnerability) */
  passwordRaw: string;
  /** Base64-encoded PBKDF2-SHA512 hash of the password */
  passwordHash: string;
  /** Base64-encoded random salt used during hashing */
  passwordSalt: string;
  bio: string;
  rating: number;
  /** Optional profile picture as a base64-encoded string */
  profilePicture?: string | null;
}

/** Private contact information, only visible to mutually matched users */
export interface ContactResponse {
  phone: string;
  email: string;
  location: string;
}

/** A single direct message in a conversation */
export interface MessageResponse {
  id: number;
  /** Username of the sender */
  sender: string;
  content: string;
  /** ISO 8601 timestamp */
  sentAt: string;
}

/** Returned after a successful login or register */
export interface TokenResponse {
  token: string;
}

/** List of usernames that mutually liked the authenticated user */
export interface MatchesResponse {
  matches: string[];
}

/** Result of a swipe action */
export interface SwipeResponse {
  /** Whether this swipe created a mutual match */
  matched: boolean;
}

/** Optional contact details supplied at registration time */
export interface ContactRequest {
  email?: string | null;
  phone?: string | null;
  location?: string | null;
}

/** Registration payload sent to POST /api/register */
export interface RegisterRequest {
  username: string;
  password: string;
  bio?: string | null;
  contact?: ContactRequest | null;
  /** Optional profile picture as a base64-encoded string */
  profilePicture?: string | null;
}
