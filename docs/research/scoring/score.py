"""Compute weighted opportunity scores under three weight profiles.

Scores: 16 criteria (C1..C16), each "nT" where n in 1..5 and T in E/J.
Judgement (J) scores must be <= 4 (enforced).
"""
import sys

W = {
    "Balanced":      [8, 6, 8, 8, 9, 8, 8, 6, 5, 5, 5, 5, 5, 6, 4, 4],
    "Revenue-first": [8, 6, 10, 10, 12, 8, 10, 4, 3, 3, 4, 4, 6, 8, 2, 2],
    "Builder-first": [6, 5, 6, 6, 6, 6, 5, 9, 7, 6, 5, 7, 5, 5, 6, 10],
}
for k, w in W.items():
    assert sum(w) == 100, (k, sum(w))

# id, name, scores C1..C16
O = [
 ("B", "Owner-approved lead & quote follow-up, PH service SMEs",
  "4E 4E 4J 3E 3E 3E 2J 4J 3E 4J 4J 4J 4J 4J 4J 4J"),
 ("A", "Clinic/dental inquiry→booking + reminders (PH)",
  "4E 5E 4J 4E 4E 2E 2J 4J 3E 3J 2E 4J 3J 4E 4J 4J"),
 ("G", "Travel/tour inquiry→quote→follow-up (PH)",
  "3J 4J 4J 2J 3E 4E 3J 3J 3E 4J 4J 3J 3J 3J 4J 4J"),
 ("C", "AR collections / payment-reminder agent on Xero/QBO (intl)",
  "4E 3E 5E 4E 4E 2E 3J 4J 3J 3J 3J 4J 4J 4E 3J 4J"),
 ("B2", "Lead & quote follow-up, one US/AU service vertical",
  "4E 4E 4J 4E 4E 2E 2J 4J 3J 4J 4J 4J 4J 4J 4J 4J"),
 ("K", "Prosumer email triage & drafting",
  "4E 5E 3J 4E 4E 2E 2J 3J 3E 3J 3E 3J 3J 3E 3J 3J"),
 ("P", "Outbound prospecting / cold-email agent (Harvey's domain)",
  "3J 4J 3J 3J 3J 2J 3J 4J 3J 2J 2J 4J 3J 3J 3J 4J"),
 ("D", "Generic Messenger FAQ/inquiry bot (PH)",
  "3E 5E 3J 4E 2E 1E 3J 4J 3E 3J 3J 4J 3J 3J 3J 2J"),
 ("E", "Voice AI receptionist",
  "4E 5E 4E 5E 4E 1E 2J 3J 2E 2E 3J 3J 2E 4E 3J 2J"),
 ("F", "SME document intake & extraction",
  "3E 4J 3E 4E 2E 2J 2J 3J 3J 2J 2J 3J 3J 3J 4J 3J"),
 ("M", "Filipino freelancer admin (invoices, BIR, follow-up)",
  "3E 3J 3J 2J 2J 3J 4J 3J 3J 3J 3J 3J 3J 3J 3J 3J"),
 ("L", "OFW household admin",
  "3J 3J 3J 2E 2E 3J 4J 3J 2J 2J 2J 3J 2J 3J 3J 3J"),
 ("H", "Workflow glue between SME apps",
  "3J 4J 3J 4E 3E 1E 2J 3J 3J 3J 3J 3J 2J 4J 4J 3J"),
 ("N2", "Infra: agent approval/permission layer",
  "4J 4J 3J 2E 2E 2E 3J 4J 4J 3J 4J 4J 4J 3J 3J 4J"),
 ("N1", "Infra: durable agent execution",
  "4E 4J 4J 4E 3E 1E 2J 2J 4J 2J 4J 2J 3J 4J 4J 4J"),
 ("N4", "Infra: agent observability",
  "3J 4J 3J 3E 3E 1E 2J 3J 4J 3J 4J 3J 3J 4J 3J 3J"),
 ("N3", "Infra: integration platform for agents",
  "4E 4J 4J 4E 4E 1E 1J 2J 1E 3J 2J 2J 2J 4J 4J 3J"),
 ("J", "General personal agent (consumer)",
  "3J 5E 2J 3E 2E 1E 2J 2J 2E 2J 2J 3J 3J 2E 4J 4J"),
 ("I", "SME research/reporting assistant",
  "2J 2J 2J 2J 2J 3J 2J 4J 4J 4J 4J 4J 3J 2J 3J 3J"),
  # New ideas, Philippines (new-ideas-philippines.md). S1 C4 lowered 5E->4E: Taxumo's 144k users is a vendor claim.
 ("S1", "Seller tax desk: BIR compliance agent for Shopee/Lazada/TikTok/FB sellers",
  "4E 3J 4J 4E 4E 3J 4J 4J 4J 3J 2J 4J 3J 5E 4E 4J"),
 ("S4", "Marketplace inbox, review & returns agent (PH sellers)",
  "4E 5E 3J 5E 3E 1E 4J 3J 2E 3J 4J 3J 3J 4J 4J 3J"),
 ("S3", "Live-seller 'mine-to-paid' order desk",
  "3E 4J 3J 4E 3E 2E 4J 3J 2E 4J 3J 3J 3J 4J 4J 4J"),
 ("S2", "GCash/Maya payment-proof checker & paid-orders ledger",
  "4E 4E 3J 3J 2J 2J 4J 4J 2J 2J 2J 4J 4J 4J 4J 3J"),
 ("S5", "Kasambahay employer compliance",
  "2J 2E 2J 2J 2J 4J 3J 4J 3J 3J 2J 4J 3J 4J 3J 3J"),
  # New ideas, marketplaces (new-ideas-marketplaces.md)
 ("MK2", "Approval-first support reply drafter (Shopify App Store)",
  "3J 4J 3J 5E 4E 2E 3E 4J 3J 4J 3J 4J 3J 4J 4J 4J"),
 ("MK1", "Shopify order-exception agent",
  "3J 4J 3J 4E 3E 3J 3E 3J 3J 4J 3J 3J 3J 4J 4J 4J"),
 ("MK3", "WordPress content-decay refresh agent",
  "3J 2J 3J 3E 3E 3E 3E 3J 3J 4J 4J 3J 3J 3J 3J 4J"),
 ("MK5", "Atlassian Forge release-notes agent",
  "2J 2J 3J 4E 2E 2E 3E 3J 4E 4J 4J 4J 4J 4J 3J 3J"),
 ("MK4", "Apify AI-extraction Actors",
  "2J 3J 3J 4E 3E 2E 4E 4J 4J 3J 2J 4J 3E 3J 3J 2J"),
  # PH industry sweep: sectors (ph-industries-sectors.md)
 ("CON1", "Progress-billing & collections desk for small contractors",
  "4E 3J 4J 3E 4E 3E 3J 4J 4J 3J 4J 4J 4J 3J 4J 4J"),
 ("EDU1", "Tuition & promissory-note follow-through for small schools",
  "4E 3J 4J 3E 3J 3E 3J 4J 4E 4J 3E 4J 4J 4J 4J 4J"),
 ("DIS1", "Taglish chat-order desk for non-FMCG distributors",
  "3J 4J 4J 4E 3J 3E 2J 4J 2E 3J 4J 4J 4J 4J 4J 4J"),
 ("LOG1", "Customs-broker document chasing & client updates",
  "3J 4J 3J 2J 3J 3J 3J 4J 4J 4J 4J 4J 4J 4J 3J 4J"),
 ("COOP1", "Member-loan collections for small cooperatives",
  "3J 4J 4J 3E 3J 2E 3J 4J 4J 3J 2J 4J 4J 4J 3J 4J"),
  # PH industry sweep: services (ph-industries-services.md)
 ("P1", "Bookkeeper document, 2307 & BIR-deadline chaser",
  "4E 4E 4J 4E 4E 3E 3E 4J 4E 4J 2E 4J 4J 5E 4J 4J"),
 ("P2", "Applicant-document desk for DMW-licensed recruitment agencies",
  "4E 4J 4J 3E 3E 4E 2E 3J 2E 3J 1E 3J 3J 4J 3J 4J"),
 ("P3", "Dues & notices clerk for HOAs / small condo corporations",
  "3E 3J 3J 4E 2E 3E 2J 4J 3E 3J 3E 4J 3J 4J 3J 4J"),
 ("HR1", "New-hire document chaser for high-turnover PH SMEs",
  "3J 3J 3J 3J 2J 3J 2J 4J 3J 4J 2E 4J 3J 3J 4J 4J"),
 ("HR2", "Payroll-input & remittance-deadline chaser for bookkeepers (no pay computation)",
  "4E 4E 4J 3E 3J 3J 3E 4J 4J 4J 2E 4J 4J 5E 4J 4J"),
 ("HR3", "Messenger HR helpdesk for spreadsheet-run SMEs",
  "2J 4J 2J 2J 2J 2E 2J 3J 3J 3J 2E 3J 3J 4J 3J 4J"),
 ("HR4", "Separation desk: 30-day final pay & 3-day COE clocks",
  "4E 2J 3J 2J 2J 3J 2J 4J 4J 3J 2E 4J 4J 2J 3J 3J"),
 ("HR5", "AI add-on in a global HR marketplace (BambooHR/Gusto/etc.)",
  "3J 3J 3J 3J 3J 1E 2E 3J 1E 4J 2J 3J 3J 4J 4J 4J"),
 ("HR6", "AI applicant screener",
  "3J 3J 3J 3E 2J 2E 2J 3J 2J 2J 1E 3J 3J 3J 3J 3J"),
 ("ERP1", "2307 receivable desk: request, read & match received 2307s",
  "4E 3E 4J 3E 3E 4E 3J 4J 4J 4J 3J 4J 4J 3J 4J 4J"),
 ("ERP2", "Supplier-reply & PO follow-up agent for Odoo",
  "3J 4J 3J 3E 3E 2E 3E 4J 4E 3J 4J 4J 3J 2E 3J 4J"),
 ("ERP3", "EIS e-invoicing bridge for small-business ERPs",
  "5E 4E 4J 4E 3J 3E 3J 2J 1E 1J 2E 2J 2J 4J 3J 2J"),
 ("ERP4", "ERPNext bill-capture agent with BIR tax codes (Frappe Cloud)",
  "3J 4J 3J 2J 2J 4E 2J 4J 4J 3J 3J 4J 3J 4E 3J 4J"),
 ("ERP5", "'Ask your ERP' assistant for Odoo",
  "2J 3J 2J 4E 3E 1E 3E 4J 4E 4J 3J 4J 3J 2E 3J 3J"),
 ("MKT1", "Shopify PH COD & courier desk (J&T/LBC/Ninja Van)",
  "4J 4J 3J 3E 2E 3E 2E 3J 2J 4J 3J 3J 3J 4J 4J 4J"),
 ("MKT2", "HubSpot Free/Starter reply & stale-deal drafter",
  "3J 4J 3J 4E 3E 2E 2E 4J 3E 4J 3J 4J 3J 4J 3J 4J"),
 ("MKT3", "Zoho CRM Taglish follow-up drafter (Viber/SMS)",
  "3J 4J 3J 2J 3J 3J 2E 4J 3J 4J 3J 4J 3J 4J 3J 4J"),
 ("MKT4", "Freshdesk Growth-plan reply drafter",
  "3J 4J 3J 4E 3E 3E 2J 4J 3E 4J 3J 4J 3J 4J 3J 4J"),
 ("MKT5", "monday.com stale-item chaser",
  "2J 4J 2J 3E 3J 2E 3E 4J 3J 4J 4J 4J 4J 4J 3J 4J"),
 ("P-R", "Rust Harvey: open-source approval-first outbound engine + paid hosted tier",
  "3E 4E 3J 5E 4E 1E 3J 3J 2E 3J 2E 3J 2E 4E 4J 4E"),
]

rows = []
for oid, name, s in O:
    toks = s.split()
    assert len(toks) == 16, (oid, len(toks))
    vals = []
    for t in toks:
        n, tag = int(t[:-1]), t[-1]
        assert tag in "EJ" and 1 <= n <= 5, (oid, t)
        assert not (tag == "J" and n > 4), (oid, t, "J capped at 4")
        vals.append(n)
    tot = {k: sum(w * v for w, v in zip(ws, vals)) / 5 for k, ws in W.items()}
    ev = sum(t.endswith("E") for t in toks)
    ones = [f"C{i+1}" for i, v in enumerate(vals) if v == 1 and i + 1 in (5, 6, 11)]
    rows.append((oid, name, toks, tot, ev, ones))

rank = {}
for k in W:
    for i, r in enumerate(sorted(rows, key=lambda r: -r[3][k])):
        rank[(r[0], k)] = i + 1

mode = sys.argv[1] if len(sys.argv) > 1 else "summary"
if mode == "summary":
    print("| Rank (Bal.) | ID | Opportunity | Balanced | Revenue-first | Builder-first | Evidence-backed criteria | Disqualifier |")
    print("|---|---|---|---|---|---|---|---|")
    for r in sorted(rows, key=lambda r: -r[3]["Balanced"]):
        oid, name, toks, tot, ev, ones = r
        cells = " | ".join(f"{tot[k]:.1f} (#{rank[(oid,k)]})" for k in W)
        print(f"| {rank[(oid,'Balanced')]} | {oid} | {name} | {cells} | {ev}/16 | {', '.join(ones) or '—'} |")
else:
    print("| ID | " + " | ".join(f"C{i}" for i in range(1, 17)) + " |")
    print("|---|" + "---|" * 16)
    for r in sorted(rows, key=lambda r: -r[3]["Balanced"]):
        print(f"| {r[0]} | " + " | ".join(r[2]) + " |")
