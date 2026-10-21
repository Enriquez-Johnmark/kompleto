-- Login links (A1) and sessions. Only SHA-256 hashes of the secrets are
-- stored, so a database leak doesn't hand out working links or sessions.
create table login_tokens (
    token_hash  bytea primary key,
    firm_id     uuid not null references firms (id) on delete cascade,
    created_at  timestamptz not null,
    expires_at  timestamptz not null,
    used_at     timestamptz
);

create index login_tokens_firm on login_tokens (firm_id, created_at);

create table sessions (
    session_hash  bytea primary key,
    firm_id       uuid not null references firms (id) on delete cascade,
    created_at    timestamptz not null,
    expires_at    timestamptz not null
);
