-- Kompleto's Firms and their Clients. A Firm's id is its engine tenant_id.
-- Every Firm-owned table carries firm_id, and child rows reference
-- (client_id, firm_id) so a row can never point at another Firm's Client.

create table firms (
    id                   uuid primary key,
    name                 text not null,
    login_email          text not null,
    cut_off_offset_days  smallint not null default 5 check (cut_off_offset_days between 0 and 30),
    created_at           timestamptz not null
);

create unique index firms_login_email on firms (lower(login_email));

create table clients (
    id                   uuid primary key,
    firm_id              uuid not null references firms (id) on delete cascade,
    business_name        text not null,
    contact_first_name   text not null,
    email                text not null,
    language             text not null check (language in ('taglish', 'english')),
    forms                text[] not null,
    -- null: the Firm's default.
    cut_off_offset_days  smallint check (cut_off_offset_days between 0 and 30),
    reminder_days        smallint[] not null default '{7,3,1}',
    paused               boolean not null default false,
    created_at           timestamptz not null,
    updated_at           timestamptz not null,
    unique (id, firm_id)
);

create unique index clients_business_name on clients (firm_id, lower(business_name));
