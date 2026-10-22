// Run with `node --test deploy/email-worker/`. The signature vector is the
// same one `tests/inbound.rs` checks on the Rust side.
import { test } from "node:test";
import assert from "node:assert/strict";
import worker, { deliver, signature } from "./worker.mjs";

const SECRET = "test-inbound-secret";
const RAW = new TextEncoder().encode("From: nena@example.test\r\nSubject: Re: docs\r\n\r\nHi po\r\n");
const VECTOR = "62c376e06f8b7a0bddc963a7c17ee0bf0170a0aabeacbc8dd582574788e4e950";

test("signs the way the app checks", async () => {
  const hex = await signature(SECRET, RAW);
  assert.equal(hex, VECTOR);
});

test("posts the raw email with its signature, and retries a busy app", async () => {
  const calls = [];
  const statuses = [503, 204];
  const fake = async (url, init) => {
    calls.push({ url, init });
    return { status: statuses.shift() };
  };
  const env = { INBOUND_URL: "https://app.example/hooks/inbound-email", INBOUND_SECRET: SECRET };
  assert.equal(await deliver(RAW, env, fake), true);
  assert.equal(calls.length, 2);
  assert.equal(calls[0].init.headers["x-kompleto-signature"], await signature(SECRET, RAW));
  assert.deepEqual(calls[0].init.body, RAW);
});

test("doesn't retry a refusal, and forwards what the app won't take", async () => {
  let posts = 0;
  const env = { INBOUND_URL: "https://x/hooks", INBOUND_SECRET: SECRET, FALLBACK_ADDRESS: "owner@example.test" };
  assert.equal(await deliver(RAW, env, async () => (posts++, { status: 401 })), false);
  assert.equal(posts, 1);

  const forwarded = [];
  const message = { raw: new Blob([RAW]).stream(), forward: async (to) => forwarded.push(to) };
  const realFetch = globalThis.fetch;
  globalThis.fetch = async () => ({ status: 400 });
  try {
    await worker.email(message, env);
  } finally {
    globalThis.fetch = realFetch;
  }
  assert.deepEqual(forwarded, ["owner@example.test"]);
});
