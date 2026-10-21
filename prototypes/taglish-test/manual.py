"""Throwaway prototype: the blind Taglish test, run by hand in claude.ai.

Answers one question for the Kompleto spec: is Haiku 5.5's Taglish good enough,
or do we need Sonnet 5.5? The owner has no API credits, so instead of calling
the API (run.py) the owner pastes each prompt into claude.ai once per model.
All clients are fictional, so no customer data goes to the consumer app.

    python3 -I prototypes/taglish-test/manual.py kit
        writes out/kit.md: the prompts to paste, one per case.
    python3 -I prototypes/taglish-test/manual.py blind out/answers.json
        reads the pasted answers and writes out/blind-test.html: each case's two
        emails, assembled the way Kompleto assembles them, in random order.

answers.json maps "<case>-haiku" and "<case>-sonnet" to the model's JSON reply.
Delete this folder once the result is recorded in docs/product/p1-spec.md.
"""

import importlib.util
import json
import random
import sys
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
    elif sys.argv[1:2] == ["blind"] and len(sys.argv) == 3:
        blind(sys.argv[2])
    else:
        sys.exit(__doc__)
