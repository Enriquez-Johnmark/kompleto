# The model never sees documents or tax data

Requests to the LLM carry only:
- the Client's business name and the Contact's first name;
- the list of Owed documents;
- the dates;
- the language and tone.

They never carry Upload contents, TINs, peso amounts or files. Kompleto processes sensitive personal information as the Firm's processor under RA 10173 (`docs/research/p1-kill-checks.md` §4), and keeping it away from a third-party model removes a whole class of privacy and sub-processor risk. Reading documents with AI would be useful, for example to auto-label 2307s. It was deliberately left out of the first version and needs a new decision, plus updated privacy paperwork, before it is added. A test inspects outgoing model requests for these fields (criterion D6).
