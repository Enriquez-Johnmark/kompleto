"""Throwaway prototype: the blind Taglish test, run by hand in claude.ai.

Answers one question for the Kompleto spec: is Haiku 5.5's Taglish good enough,
or do we need Sonnet 5.5? The owner has no API credits, so instead of calling
the API (run.py) the owner pastes each prompt into claude.ai once per model.
All clients are fictional, so no customer data goes to the consumer app.

    python3 -I prototypes/taglish-test/manual.py kit
        writes out/kit.md: the prompts to paste, one per case.
    python3 -I prototypes/taglish-test/manual.py api
        asks both models through the API (OWNER_ANTHROPIC_API_KEY or
        ANTHROPIC_API_KEY) with the app's exact request, read from drafting.rs,
        and writes out/answers.json and out/usage.json.
    python3 -I prototypes/taglish-test/manual.py judge
        a blind judge (Claude Opus) picks the better email in each pair, asked
        twice with the order swapped; writes out/judged.json.
    python3 -I prototypes/taglish-test/manual.py blind out/answers.json
        reads the pasted answers and writes out/blind-test.html: each case's two
        emails, assembled the way Kompleto assembles them, in random order.

answers.json maps "<case>-haiku" and "<case>-sonnet" to the model's JSON reply.
Delete this folder once the result is recorded in docs/product/p1-spec.md.
"""

import importlib.util
import json
import os
import random
import re
import sys
import urllib.request
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = HERE / "out"


def run_module():
    """run.py, for its blind-test page (loaded by path: `-I` leaves this folder off sys.path)."""
    spec = importlib.util.spec_from_file_location("run", HERE / "run.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module

# Must match SYSTEM in src/kompleto/drafting.rs, so the test judges the real prompt.
SYSTEM = """You write the wording of a short email from a Philippine bookkeeper to a small-business client \
who owes documents for a BIR filing. The email itself will list the documents and the deadline; you write only:
- subject: one line, at most 100 characters;
- opening: one to three sentences that greet the contact by first name and say documents are still needed for the period;
- closing: one short sentence of thanks.
Write in the requested language. Taglish means natural Tagalog-English as Filipino bookkeepers write to clients, \
with "po" where respectful. Match the requested tone; later reminders are firmer but stay polite.
Never include links, email addresses, amounts, account or tax numbers, or any document names."""

FIRM = "Santos Bookkeeping"
CASES = [
    ("Aling Nena's Sari-Sari Store", "Nena", ["Sales invoices issued, including cancelled ones",
     "Purchase and expense invoices and receipts"], "October 2026", "Thu 5 Nov 2026", "friendly"),
    ("Dr. Reyes Dental Clinic", "Mike", ["Bank and e-wallet (GCash, Maya) statements"],
     "October 2026", "Thu 5 Nov 2026", "friendly_reminder"),
    ("JM Print & Design", "JM", ["Sales invoices issued, including cancelled ones",
     "2307s received from customers", "Bank and e-wallet (GCash, Maya) statements"],
     "Q3 2026", "Mon 26 Oct 2026", "firmer"),
    ("Bautista Hardware", "Ramon", ["Payroll register and SSS, PhilHealth and Pag-IBIG reports"],
     "October 2026", "Thu 5 Nov 2026", "urgent_but_polite"),
    ("Kusina ni Lola", "Liza", ["Purchase and expense invoices and receipts"],
     "November 2026", "Fri 4 Dec 2026", "friendly"),
    ("Golden Grains Rice Trading", "Tonyo", ["2307s received from customers",
     "Sales invoices issued, including cancelled ones"], "Q4 2026", "Mon 25 Jan 2027", "firmer"),
    ("Sunrise Water Refilling", "Carlo", ["Bank and e-wallet (GCash, Maya) statements",
     "Purchase and expense invoices and receipts"], "November 2026", "Fri 4 Dec 2026", "urgent_but_polite"),
    ("Villanueva Tutorial Center", "Joy", ["Sales invoices issued, including cancelled ones"],
     "October 2026", "Thu 5 Nov 2026", "friendly_reminder"),
    # The prompt also covers payroll and contributions (M13).
    ("Tan Auto Supply", "Kevin", ["Timesheets and overtime for the period", "New hires and resignations"],
     "payroll on Sun 15 Nov 2026", "Wed 11 Nov 2026", "firmer"),
    ("Cruz Freelance Design", "Bea", ["SSS, PhilHealth and Pag-IBIG payment receipts"],
     "October 2026 SSS and Pag-IBIG contributions", "Tue 10 Nov 2026", "friendly_reminder"),
]


# Which of the four messages each tone belongs to, for the page's headings.
CASE_STAGE = {"friendly": 1, "friendly_reminder": 2, "firmer": 3, "urgent_but_polite": 4}


def brief(case):
    business, contact, owed, period, cut_off, tone = case
    return {"business_name": business, "contact_first_name": contact, "owed_documents": owed,
            "period": period, "cut_off": cut_off, "language": "Taglish", "tone": tone}


def kit():
    parts = ["# Taglish test kit\n",
             "For each case: open a **new chat** in claude.ai, choose **Haiku 5.5**, paste the block, "
             "and copy the JSON reply. Then do the same in a new chat with **Sonnet 5.5**. "
             "Send all 16 replies back, labelled like `3-haiku` and `3-sonnet`.\n"]
    for number, case in enumerate(CASES, 1):
        prompt = (f"{SYSTEM}\n\nAnswer with only a JSON object with three string fields: "
                  f"\"subject\", \"opening\" and \"closing\". No other text.\n\n"
                  f"Details:\n{json.dumps(brief(case), ensure_ascii=False)}")
        parts.append(f"## Case {number}\n\n```\n{prompt}\n```\n")
    OUT.mkdir(exist_ok=True)
    (OUT / "kit.md").write_text("\n".join(parts))
    print(f"Wrote {OUT / 'kit.md'}")


ROOT = HERE.parent.parent
MODELS = {"haiku": "claude-haiku-5-5", "sonnet": "claude-sonnet-5-5"}
# The engine's thinking setting per model (nudgewright src/llm.rs `body`).
THINKING = {"haiku": "disabled", "sonnet": "between_tools"}
SCHEMA = {"type": "object",
          "properties": {"subject": {"type": "string"}, "opening": {"type": "string"}, "closing": {"type": "string"}},
          "required": ["subject", "opening", "closing"], "additionalProperties": False}


def app_system():
    """SYSTEM exactly as drafting.rs has it, so the test judges the prompt the app sends."""
    rust = (ROOT / "src/kompleto/drafting.rs").read_text()
    literal = re.search(r'const SYSTEM: &str = "(.*?)";\n', rust, re.S).group(1)
    # A backslash at a line's end joins the next line, dropping its indent.
    return re.sub(r"\\\n\s*", "", literal).replace('\\"', '"')


def api():
    key = os.environ.get("ANTHROPIC_API_KEY") or os.environ.get("OWNER_ANTHROPIC_API_KEY")
    if not key:
        sys.exit("Set OWNER_ANTHROPIC_API_KEY (or ANTHROPIC_API_KEY) first.")
    system, answers, usage = app_system(), {}, {}
    for number, case in enumerate(CASES, 1):
        for name, model in MODELS.items():
            body = {"model": model, "max_tokens": 1000, "system": system,
                    "messages": [{"role": "user", "content": json.dumps(brief(case), ensure_ascii=False)}],
                    "output_config": {"format": {"type": "json_schema", "schema": SCHEMA}},
                    "thinking": {"type": THINKING[name]}}
            request = urllib.request.Request("https://api.anthropic.com/v1/messages", method="POST",
                                             data=json.dumps(body).encode(), headers={
                                                 "x-api-key": key, "anthropic-version": "2023-06-01",
                                                 "content-type": "application/json"})
            with urllib.request.urlopen(request, timeout=120) as response:
                reply = json.load(response)
            text = next(block["text"] for block in reply["content"] if block["type"] == "text")
            answers[f"{number}-{name}"] = json.loads(text)
            usage[f"{number}-{name}"] = {"model": reply["model"], **reply["usage"], "stop": reply["stop_reason"]}
            print(f"case {number} {name}: {reply['stop_reason']}")
    OUT.mkdir(exist_ok=True)
    (OUT / "answers.json").write_text(json.dumps(answers, ensure_ascii=False, indent=1))
    (OUT / "usage.json").write_text(json.dumps(usage, indent=1))
    print(f"Wrote {OUT / 'answers.json'}")


JUDGE_MODEL = "claude-opus-5-5"
JUDGE = """You are an experienced Filipino bookkeeper in Manila who writes to small-business clients every day.
You will see two reminder emails asking the same client for documents. Pick the one you would rather send.
Judge: natural Taglish as Filipino bookkeepers really write it (not stiff, not textbook Tagalog, not
English with a few Tagalog words added); respect ("po") without sounding servile; a tone that fits the
reminder's stage; and clarity. The document list, link and deadline are identical in both; judge only
the wording around them. Answer "tie" only if you truly cannot choose."""
VERDICT = {"type": "object", "properties": {"better": {"type": "string", "enum": ["first", "second", "tie"]},
           "reason": {"type": "string"}}, "required": ["better", "reason"], "additionalProperties": False}


def ask_judge(key, case, first, second):
    stage = CASE_STAGE[case[5]]
    text = (f"Reminder {stage} of 4 to {case[1]} of {case[0]}.\n\n"
            f"FIRST EMAIL\nSubject: {first['subject']}\n\n{first['body']}\n\n"
            f"SECOND EMAIL\nSubject: {second['subject']}\n\n{second['body']}")
    body = {"model": JUDGE_MODEL, "max_tokens": 1000, "system": JUDGE,
            "messages": [{"role": "user", "content": text}],
            "output_config": {"format": {"type": "json_schema", "schema": VERDICT}}}
    request = urllib.request.Request("https://api.anthropic.com/v1/messages", method="POST",
                                     data=json.dumps(body).encode(), headers={
                                         "x-api-key": key, "anthropic-version": "2023-06-01",
                                         "content-type": "application/json"})
    with urllib.request.urlopen(request, timeout=180) as response:
        reply = json.load(response)
    return json.loads(next(block["text"] for block in reply["content"] if block["type"] == "text"))


def judge():
    key = os.environ.get("ANTHROPIC_API_KEY") or os.environ.get("OWNER_ANTHROPIC_API_KEY")
    if not key:
        sys.exit("Set OWNER_ANTHROPIC_API_KEY (or ANTHROPIC_API_KEY) first.")
    answers = json.loads((OUT / "answers.json").read_text())
    results, totals = [], {"haiku": 0, "sonnet": 0, "tie": 0}
    for number, case in enumerate(CASES, 1):
        haiku, sonnet = (email(case, answers[f"{number}-{model}"]) for model in ("haiku", "sonnet"))
        # Asked twice, the order swapped: a verdict that flips with the order is a tie.
        one = ask_judge(key, case, haiku, sonnet)
        two = ask_judge(key, case, sonnet, haiku)
        picks = {"first": "haiku", "second": "sonnet", "tie": "tie"}[one["better"]], \
                {"first": "sonnet", "second": "haiku", "tie": "tie"}[two["better"]]
        winner = picks[0] if picks[0] == picks[1] else "tie"
        totals[winner] += 1
        results.append({"case": number, "business": case[0], "winner": winner, "picks": picks,
                        "reasons": [one["reason"], two["reason"]]})
        print(f"case {number}: {winner}  ({picks[0]} / {picks[1]})")
    (OUT / "judged.json").write_text(json.dumps({"judge": JUDGE_MODEL, "totals": totals, "cases": results},
                                                ensure_ascii=False, indent=1))
    print("totals:", totals)


def email(case, wording):
    """The email as Kompleto assembles it (drafting.rs `assemble`)."""
    business, contact, owed, period, cut_off, tone = case
    items = "".join(f"- {item}\n" for item in owed)
    return {"subject": wording["subject"].strip(), "body": (
        f"{wording['opening'].strip()}\n\n{items}\nI-upload po dito: https://getkompleto.com/u/…\n"
        f"Paki-send po bago {cut_off}.\n\n{wording['closing'].strip()}\n{FIRM}\n")}


def blind(path):
    answers = json.loads(Path(path).read_text())
    rng = random.Random()
    pairs = []
    for number, case in enumerate(CASES, 1):
        drafts = [{**email(case, answers[f"{number}-{model}"]), "model": model} for model in ("haiku", "sonnet")]
        rng.shuffle(drafts)
        scenario = {"id": number, "business": case[0], "reminder_number": CASE_STAGE[case[5]]}
        pairs.append({"scenario": scenario, "A": drafts[0], "B": drafts[1]})
    (OUT / "blind-test.html").write_text(run_module().page(pairs))
    print(f"Wrote {OUT / 'blind-test.html'}")


if __name__ == "__main__":
    if sys.argv[1:2] == ["kit"]:
        kit()
    elif sys.argv[1:2] == ["api"]:
        api()
    elif sys.argv[1:2] == ["judge"]:
        judge()
    elif sys.argv[1:2] == ["blind"] and len(sys.argv) == 3:
        blind(sys.argv[2])
    else:
        sys.exit(__doc__)
