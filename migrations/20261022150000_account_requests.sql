-- Requests for an account from the public sign-up page (M10). Not owned by
-- any Firm: only the operator's command reads them. A request is deleted
-- once it is approved or declined.
create table account_requests (
    id            uuid primary key,
    firm_name     text not null,
    email         text not null,
    -- When the requester accepted the terms and data processing agreement.
    accepted_at   timestamptz not null,
    created_at    timestamptz not null
);

create unique index account_requests_email on account_requests (lower(email));
