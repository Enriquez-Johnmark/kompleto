// Cloudflare Email Worker: hands each email sent to the reply domain to
// Kompleto's inbound endpoint (G1), signed with INBOUND_SECRET so the app
// knows it came from us. Replies never touch Resend's sending quota.
//
// Settings (wrangler.toml [vars] and `wrangler secret put`):
//   INBOUND_URL       https://app.getkompleto.com/hooks/inbound-email
//   INBOUND_SECRET    the same value as the app's INBOUND_SECRET (a secret)
//   FALLBACK_ADDRESS  a verified Email Routing destination, e.g. the owner's
//                     inbox: gets any email Kompleto couldn't take, so a
//                     Contact's reply is never lost.

/** Hex HMAC-SHA256 of `bytes` under `secret`: the X-Kompleto-Signature. */
export async function signature(secret, bytes) {
  const key = await crypto.subtle.importKey(
    "raw",
    new TextEncoder().encode(secret),
    { name: "HMAC", hash: "SHA-256" },
    false,
    ["sign"],
  );
  const mac = new Uint8Array(await crypto.subtle.sign("HMAC", key, bytes));
  return Array.from(mac, (b) => b.toString(16).padStart(2, "0")).join("");
}

/** Posts `raw` to Kompleto; true once it took it. Retries only what may
 *  pass on a second try (the app busy or down), at most `attempts` times. */
export async function deliver(raw, env, fetchFn = fetch, attempts = 3) {
  const sig = await signature(env.INBOUND_SECRET, raw);
  for (let attempt = 1; attempt <= attempts; attempt++) {
    let status;
    try {
      const response = await fetchFn(env.INBOUND_URL, {
        method: "POST",
        headers: { "content-type": "message/rfc822", "x-kompleto-signature": sig },
        body: raw,
      });
      status = response.status;
    } catch (err) {
      status = 0; // the app is unreachable
    }
    if (status >= 200 && status < 300) return true;
    // A refused signature, an unreadable or oversized email won't change.
    if (status >= 400 && status < 500 && status !== 429) return false;
    if (attempt < attempts) await new Promise((r) => setTimeout(r, 500 * attempt));
  }
  return false;
}

export default {
  async email(message, env) {
    const raw = new Uint8Array(await new Response(message.raw).arrayBuffer());
    if (await deliver(raw, env)) return;
    if (env.FALLBACK_ADDRESS) {
      await message.forward(env.FALLBACK_ADDRESS);
      return;
    }
    throw new Error("Kompleto didn't take this email and no FALLBACK_ADDRESS is set");
  },
};
