/**
 * Typed API client for the Checkmate backend.
 * All functions throw an Error with a human-readable message on failure.
 */

import type {
  ContactResponse,
  MatchesResponse,
  MessageResponse,
  SwipeResponse,
  TokenResponse,
  UserResponse,
  UserRow,
} from "./types";

const BASE = "/api";

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/** Build JSON + Bearer-auth headers from a JWT token. */
function authHeaders(token: string): HeadersInit {
  return {
    "Content-Type": "application/json",
    Authorization: `Bearer ${token}`,
  };
}

// ---------------------------------------------------------------------------
// Auth endpoints
// ---------------------------------------------------------------------------

/** POST /api/login — exchange credentials for a JWT token. */
export async function login(
  username: string,
  password: string,
): Promise<TokenResponse> {
  const res = await fetch(`${BASE}/login`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ username, password }),
  });
  if (!res.ok) throw new Error("Invalid username or password.");
  return res.json();
}

/** POST /api/register — create a new account and return a JWT token. */
export async function register(
  username: string,
  password: string,
  bio?: string,
  contact?: { email?: string; phone?: string; location?: string },
  /** Optional profile picture as a base64-encoded string. */
  profilePicture?: string,
): Promise<TokenResponse> {
  const res = await fetch(`${BASE}/register`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ username, password, bio, contact, profilePicture }),
  });
  if (res.status === 409) throw new Error("Username already taken.");
  if (res.status === 400)
    throw new Error("Username and password are required.");
  if (!res.ok) throw new Error("Registration failed.");
  return res.json();
}

// ---------------------------------------------------------------------------
// Discovery / swipe endpoints
// ---------------------------------------------------------------------------

/**
 * GET /api/next — fetch the next unswiped user profile.
 * Returns null when there are no more profiles to show.
 */
export async function getNext(token: string): Promise<UserResponse | null> {
  const res = await fetch(`${BASE}/next`, {
    headers: authHeaders(token),
  });
  if (res.status === 404) return null;
  if (!res.ok) throw new Error("Failed to fetch the next profile.");
  return res.json();
}

/**
 * POST /api/user/{username}/swipe — record a like or pass.
 * Returns whether this swipe created a mutual match.
 */
export async function swipe(
  token: string,
  username: string,
  liked: boolean,
): Promise<SwipeResponse> {
  const res = await fetch(
    `${BASE}/user/${encodeURIComponent(username)}/swipe`,
    {
      method: "POST",
      headers: authHeaders(token),
      body: JSON.stringify({ liked }),
    },
  );
  if (!res.ok) throw new Error("Failed to record swipe.");
  return res.json();
}

// ---------------------------------------------------------------------------
// Matches endpoint
// ---------------------------------------------------------------------------

/** GET /api/matches — list all usernames that mutually liked the current user. */
export async function getMatches(token: string): Promise<MatchesResponse> {
  const res = await fetch(`${BASE}/matches`, {
    headers: authHeaders(token),
  });
  if (!res.ok) throw new Error("Failed to fetch matches.");
  return res.json();
}

// ---------------------------------------------------------------------------
// User profile endpoints
// ---------------------------------------------------------------------------

/**
 * GET /api/user/{username} — fetch the full public profile row.
 * Note: the response intentionally includes password fields (CTF challenge).
 */
export async function getUser(
  token: string,
  username: string,
): Promise<UserRow> {
  const res = await fetch(`${BASE}/user/${encodeURIComponent(username)}`, {
    headers: authHeaders(token),
  });
  if (res.status === 404) throw new Error("User not found.");
  if (!res.ok) throw new Error("Failed to fetch user profile.");
  return res.json();
}

/**
 * GET /api/user/{username}/contact — fetch private contact info.
 * Only succeeds when the authenticated user is mutually matched with the target.
 */
export async function getContact(
  token: string,
  username: string,
): Promise<ContactResponse> {
  const res = await fetch(
    `${BASE}/user/${encodeURIComponent(username)}/contact`,
    { headers: authHeaders(token) },
  );
  if (res.status === 403) throw new Error("Not yet matched with this user.");
  if (res.status === 404) throw new Error("User not found.");
  if (!res.ok) throw new Error("Failed to fetch contact information.");
  return res.json();
}

// ---------------------------------------------------------------------------
// Messaging endpoints
// ---------------------------------------------------------------------------

/**
 * GET /api/user/{username}/dm — retrieve the full message history.
 * Requires a mutual match with the target user.
 */
export async function getMessages(
  token: string,
  username: string,
): Promise<MessageResponse[]> {
  const res = await fetch(`${BASE}/user/${encodeURIComponent(username)}/dm`, {
    headers: authHeaders(token),
  });
  if (res.status === 403) throw new Error("Not yet matched with this user.");
  if (res.status === 404) throw new Error("User not found.");
  if (!res.ok) throw new Error("Failed to fetch messages.");
  return res.json();
}

/**
 * POST /api/user/{username}/dm — send a direct message.
 * Requires a mutual match with the target user.
 * Returns the newly created message.
 */
export async function sendMessage(
  token: string,
  username: string,
  content: string,
): Promise<MessageResponse> {
  const res = await fetch(`${BASE}/user/${encodeURIComponent(username)}/dm`, {
    method: "POST",
    headers: authHeaders(token),
    body: JSON.stringify({ content }),
  });
  if (res.status === 403) throw new Error("Not yet matched with this user.");
  if (res.status === 404) throw new Error("User not found.");
  if (!res.ok) throw new Error("Failed to send message.");
  return res.json();
}
