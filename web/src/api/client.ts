import type { ErrorBody } from "@/api/generated/ErrorBody"

/** An answer from the API other than success, in the API's own words. */
export class ApiError extends Error {
  readonly status: number
  readonly problems: string[]

  constructor(status: number, body: ErrorBody) {
    super(body.message)
    this.name = "ApiError"
    this.status = status
    this.problems = body.problems
  }

  /** The session has ended; the bookkeeper must log in again. */
  get isNotLoggedIn() {
    return this.status === 401
  }
}

type Method = "GET" | "POST" | "PUT" | "DELETE"

/**
 * Calls the Rust API. Every request in the app goes through here: it sends
 * the session cookie and JSON, and turns any failure into an ApiError.
 */
export async function api<T>(method: Method, path: string, body?: unknown): Promise<T> {
  const response = await fetch(path, {
    method,
    credentials: "same-origin",
    headers: body === undefined ? undefined : { "Content-Type": "application/json" },
    body: body === undefined ? undefined : JSON.stringify(body),
  })
  if (!response.ok) throw new ApiError(response.status, await errorBody(response))
  if (response.status === 204 || response.status === 202) return undefined as T
  return (await response.json()) as T
}

async function errorBody(response: Response): Promise<ErrorBody> {
  try {
    const body = (await response.json()) as Partial<ErrorBody>
    if (typeof body.message === "string") {
      return { message: body.message, problems: body.problems ?? [] }
    }
  } catch {
    // Not JSON: a proxy or the network answered, not the API.
  }
  return {
    message: "Kompleto couldn't be reached. Check your connection and try again.",
    problems: [],
  }
}
