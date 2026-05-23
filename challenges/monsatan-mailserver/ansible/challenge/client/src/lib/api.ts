/**
 * api.ts — Typed client for the Monsatan Mailserver API.
 *
 * All mail endpoints require a Bearer JWT obtained from /api/login or /api/register.
 */

const BASE_URL = "/api";

// ── Request / Response types ──────────────────────────────────────────────────

export interface AuthRequest {
  username: string;
  password: string;
}

export interface AuthResponse {
  token: string;
  /** Full email address assigned to the user, e.g. alice@monsatan.ctf */
  email: string;
}

export interface SendMailRequest {
  to: string;
  subject: string;
  body: string;
  /** Base64-encoded .tar.gz plugin files */
  plugins?: string[];
}

export interface MailSummary {
  id: number;
  from: string;
  subject: string;
  sent_at: string;
  is_read: boolean;
}

export interface MailDetail {
  id: number;
  from: string;
  to: string;
  subject: string;
  body: string;
  sent_at: string;
  is_read: boolean;
}

// ── Internal helper ───────────────────────────────────────────────────────────

/**
 * Performs a fetch and throws a descriptive Error on non-OK responses.
 */
async function request<T>(path: string, options: RequestInit = {}): Promise<T> {
  // Destructure headers out of options so that spreading `rest` below does not
  // overwrite the already-merged headers object with the raw options.headers.
  const { headers, ...rest } = options;
  const res = await fetch(`${BASE_URL}${path}`, {
    headers: {
      "Content-Type": "application/json",
      ...headers,
    },
    ...rest,
  });

  if (!res.ok) {
    const text = await res.text().catch(() => res.statusText);
    throw new Error(`${res.status}: ${text}`);
  }

  return res.json() as Promise<T>;
}

/** Returns headers with the Bearer token included. */
function authHeaders(token: string): Record<string, string> {
  return { Authorization: `Bearer ${token}` };
}

// ── Public API functions ──────────────────────────────────────────────────────

export async function apiRegister(data: AuthRequest): Promise<AuthResponse> {
  return request<AuthResponse>("/register", {
    method: "POST",
    body: JSON.stringify(data),
  });
}

export async function apiLogin(data: AuthRequest): Promise<AuthResponse> {
  return request<AuthResponse>("/login", {
    method: "POST",
    body: JSON.stringify(data),
  });
}

export async function apiListMails(token: string): Promise<MailSummary[]> {
  return request<MailSummary[]>("/mails", {
    headers: authHeaders(token),
  });
}

export async function apiGetMail(
  token: string,
  id: number,
): Promise<MailDetail> {
  return request<MailDetail>(`/mails/${id}`, {
    headers: authHeaders(token),
  });
}

export async function apiSendMail(
  token: string,
  data: SendMailRequest,
): Promise<{ id: number }> {
  return request<{ id: number }>("/mails", {
    method: "POST",
    headers: authHeaders(token),
    body: JSON.stringify(data),
  });
}
