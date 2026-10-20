# Kompleto

Kompleto helps Philippine bookkeepers get every client's documents in before each BIR deadline. It runs on a reusable engine that drafts messages, holds them for approval, sends them and stops when someone answers.

## People and businesses

**Firm**:
A bookkeeper or small accounting firm that pays for Kompleto; one account, one login.
_Avoid_: Customer, user, tenant, account

**Client**:
A small business whose books a Firm keeps, and whom Kompleto asks for documents.
_Avoid_: Customer, end user

**Contact**:
The person at a Client who receives reminders and uploads documents.
_Avoid_: Recipient, client user

**Payor**:
A business that pays a Client, withholds tax, and owes the Client a BIR Form 2307 for it.
_Avoid_: Customer (of the client), withholding agent

## Deadlines and documents

**BIR form**:
A return a Client must file, such as 2550Q or 1601-EQ, each with its own due dates.
_Avoid_: Tax form, filing

**Period**:
The month or quarter a BIR form covers.
_Avoid_: Cycle, term

**Due date**:
The date the BIR sets for filing a BIR form for a Period.
_Avoid_: Deadline (when ambiguous)

**Cut-off**:
The date by which the Firm wants a Client's documents for a Period, before the Due date.
_Avoid_: Internal deadline, target date

**Owed document**:
A document a Client must send for a Period, such as receipts, a bank statement or a 2307 from a named Payor.
_Avoid_: Requirement, missing file, checklist item

**Upload**:
A file a Contact sends through an Upload link, labelled as one Owed document.
_Avoid_: Attachment, submission

**Upload link**:
A private, unguessable link that lets one Client's Contact upload documents for one Period without logging in.
_Avoid_: Portal, magic link

## Messages

**Draft**:
A proposed message to a Contact, written by the model or a template, that nobody has approved yet.
_Avoid_: Suggestion, pending message

**Approval**:
A Firm's sign-off on the exact text of a Draft; any edit voids it.
_Avoid_: Confirmation, review

**Reminder**:
A sent message asking a Contact for Owed documents.
_Avoid_: Nudge, chase, follow-up

**Reply**:
A message a human Contact sends back; it stops Reminders for that Client and Period.
_Avoid_: Response

**Auto-reply**:
An automatic message such as an out-of-office notice; it does not stop Reminders.
_Avoid_: Reply

**Bounce**:
A delivery-failure notice matched to a Reminder we sent.
_Avoid_: Failed email

**Send budget**:
The maximum number of Reminders Kompleto sends in one day.
_Avoid_: Rate limit, quota

## Building blocks

**Engine**:
The reusable core every product runs on: scheduling, drafting, approval, sending, reply handling and records.
_Avoid_: Platform, framework

**Product**:
A specific offering built on the Engine, such as Kompleto.
_Avoid_: App, module
