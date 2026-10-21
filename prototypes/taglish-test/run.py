"""Throwaway prototype: blind Taglish quality test, Claude Haiku 5.5 vs Sonnet 5.5.

Answers one question for the Kompleto spec: is Haiku's Taglish good enough,
or do we need Sonnet? Not product code. Delete once the decision is recorded.

Without API credits, use manual.py instead: the same test by hand in claude.ai,
with the app's real prompt (this script's older prompt predates M3).

Usage:
    ANTHROPIC_API_KEY=sk-ant-... python3 -I prototypes/taglish-test/run.py [N]

Writes out/blind-test.html (open it in a browser, pick the better draft in each
pair, then press "Reveal") and out/results.json (raw drafts, usage, the key).
All client names and businesses are fictional.
"""

import json
import os
import random
import sys
import urllib.error
import urllib.request
from pathlib import Path

API_URL = "https://api.anthropic.com/v1/messages"
MODELS = {
    # Thinking settings per docs/research/llm-options.md (API shape notes).
    "claude-haiku-5-5": {"thinking": {"type": "disabled"}, "price": (0.10, 0.50)},
    "claude-sonnet-5-5": {"thinking": {"type": "between_tools"}, "price": (2.00, 10.00)},
}
OUT = Path(__file__).resolve().parent / "out"

SYSTEM = """You draft short reminder emails that a Philippine bookkeeper sends to a small-business client.
Write natural, polite Taglish the way a Filipino bookkeeper really texts a client: mostly Tagalog with
English accounting terms, using "po" where respectful. Rules:
- 80 to 150 words in the body.
- Mention only the missing items you are given; never invent amounts, TINs or other documents.
- Include the placeholder {{UPLOAD_LINK}} exactly once, and no other links.
- Sign off with the firm name you are given.
- Match the requested tone; later reminders are firmer but stay polite."""

SCHEMA = {
    "type": "object",
    "properties": {
        "subject": {"type": "string"},
        "body": {"type": "string"},
    },
    "required": ["subject", "body"],
    "additionalProperties": False,
}

BUSINESSES = [
    ("Aling Nena's Sari-Sari Store", "Nena"), ("Dr. Reyes Dental Clinic", "Doc Mike"),
    ("JM Print & Design", "JM"), ("Bautista Hardware", "Ramon"), ("Kusina ni Lola", "Liza"),
    ("Cruz Freelance Design", "Bea"), ("Golden Grains Rice Trading", "Mang Tonyo"),
    ("Sunrise Water Refilling", "Carlo"), ("Villanueva Tutorial Center", "Ms. Joy"),
    ("Tan Auto Supply", "Kevin"),
]
FIRMS = ["Santos Bookkeeping", "DLC Accounting Services", "Mendoza Tax & Books"]
ITEMS = [
    "sales receipts for September", "expense receipts for September", "GCash transaction screenshots",
    "BDO bank statement for September", "BIR Form 2307 from Puregold", "BIR Form 2307 from Jollibee Foods",
    "BIR Form 2307 from SM Supermalls", "official receipts for purchases", "Maya transaction history",
]
FORMS = [("2550Q", "October 25"), ("1601-EQ", "October 31"), ("0619-E", "November 10"), ("1701Q", "November 15")]
STAGES = [
    (1, "first request, friendly", 12), (2, "first reminder, friendly", 7),
    (3, "second reminder, a bit firmer", 3), (4, "last reminder before cut-off, firm but polite", 1),
]


def scenarios(n, rng):
    out = []
    for i in range(n):
        biz, contact = rng.choice(BUSINESSES)
        form, due = rng.choice(FORMS)
        stage, tone, days = rng.choice(STAGES)
        out.append({
            "id": i + 1, "firm": rng.choice(FIRMS), "business": biz, "contact": contact,
            "missing": rng.sample(ITEMS, rng.randint(1, 3)), "form": form, "bir_due": due,
            "days_to_cutoff": days, "reminder_number": stage, "tone": tone,
        })
    return out


def user_prompt(s):
    items = "\n".join(f"- {m}" for m in s["missing"])
    return (f"Firm: {s['firm']}\nClient business: {s['business']}\nContact first name: {s['contact']}\n"
            f"Missing items:\n{items}\nNeeded for: BIR Form {s['form']}, due {s['bir_due']}\n"
            f"Days until our cut-off: {s['days_to_cutoff']}\nThis is message {s['reminder_number']} of 4; "
            f"tone: {s['tone']}\nLanguage: Taglish")


def call(key, model, s):
    body = {
        "model": model, "max_tokens": 800, "system": SYSTEM,
        "messages": [{"role": "user", "content": user_prompt(s)}],
        "thinking": MODELS[model]["thinking"],
        "output_config": {"format": {"type": "json_schema", "schema": SCHEMA}},
    }
    req = urllib.request.Request(API_URL, data=json.dumps(body).encode(), method="POST", headers={
        "x-api-key": key, "anthropic-version": "2023-06-01", "content-type": "application/json"})
    try:
        with urllib.request.urlopen(req, timeout=120) as r:
            resp = json.load(r)
    except urllib.error.HTTPError as e:
        sys.exit(f"{model}: HTTP {e.code}: {e.read().decode(errors='replace')}")
    text = next((c["text"] for c in resp["content"] if c.get("type") == "text"), "")
    if resp.get("stop_reason") != "end_turn":
        return {"subject": "(failed)", "body": f"stop_reason={resp.get('stop_reason')}"}, resp["usage"]
    return json.loads(text), resp["usage"]


def esc(t):
    return t.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")


def page(pairs):
    cards = []
    for p in pairs:
        s = p["scenario"]
        cols = "".join(
            f'<label class="d"><input type="radio" name="q{s["id"]}" value="{side}"> <b>{side}</b>'
            f'<p class="s">{esc(p[side]["subject"])}</p><pre>{esc(p[side]["body"])}</pre></label>'
            for side in ("A", "B"))
        cards.append(f'<section><h2>#{s["id"]} · {esc(s["business"])} · message {s["reminder_number"]}/4</h2>'
                     f'<div class="row">{cols}</div></section>')
    key = {str(p["scenario"]["id"]): {"A": p["A"]["model"], "B": p["B"]["model"]} for p in pairs}
    return f"""<!doctype html><html><head><meta charset="utf-8"><meta name="viewport" content="width=device-width">
<title>Taglish Blind Test</title><style>
body{{font:15px system-ui;margin:16px;max-width:1100px;background:#fff;color:#111}}
.row{{display:grid;grid-template-columns:1fr 1fr;gap:12px}}@media(max-width:700px){{.row{{grid-template-columns:1fr}}}}
.d{{border:1px solid #ccc;border-radius:8px;padding:10px;display:block}}pre{{white-space:pre-wrap;font:inherit}}
.s{{font-weight:600}}button{{font-size:16px;padding:8px 16px}}</style></head><body>
<h1>Taglish blind test</h1><p>In each pair, pick the draft you'd rather send to a client. Then press Reveal.</p>
{''.join(cards)}<button onclick="reveal()">Reveal</button><pre id="r"></pre>
<script>const KEY=JSON.parse(atob("{__import__('base64').b64encode(json.dumps(key).encode()).decode()}"));
function reveal(){{const t={{}};let n=0;for(const id in KEY){{const c=document.querySelector('input[name=q'+id+']:checked');
if(!c)continue;n++;const m=KEY[id][c.value];t[m]=(t[m]||0)+1;}}
document.getElementById('r').textContent='Answered '+n+' of '+Object.keys(KEY).length+'\\n'+JSON.stringify(t,null,2);}}</script>
</body></html>"""


def main():
    key = os.environ.get("ANTHROPIC_API_KEY") or sys.exit("Set ANTHROPIC_API_KEY first.")
    n = int(sys.argv[1]) if len(sys.argv) > 1 else 30
    rng = random.Random(2026)
    OUT.mkdir(exist_ok=True)
    pairs, cost = [], {m: 0.0 for m in MODELS}
    for s in scenarios(n, rng):
        drafts = []
        for model in MODELS:
            draft, usage = call(key, model, s)
            pin, pout = MODELS[model]["price"]
            cost[model] += usage["input_tokens"] / 1e6 * pin + usage["output_tokens"] / 1e6 * pout
            drafts.append({**draft, "model": model, "usage": usage})
        rng.shuffle(drafts)
        pairs.append({"scenario": s, "A": drafts[0], "B": drafts[1]})
        print(f"scenario {s['id']}/{n} done", flush=True)
    (OUT / "results.json").write_text(json.dumps({"pairs": pairs, "cost_usd": cost}, ensure_ascii=False, indent=2))
    (OUT / "blind-test.html").write_text(page(pairs))
    print(f"Cost (USD): {json.dumps(cost)}\nOpen {OUT / 'blind-test.html'}")


if __name__ == "__main__":
    main()
