-- Replies (G1): what Contacts send back to their Reminders. The subject
-- and text are deleted after 90 days; the row stays as "replied on <date>".
create table replies (
    id            uuid primary key,
    firm_id       uuid not null,
    client_id     uuid not null,
    period        text not null,
    from_address  text not null,
    subject       text,
    body_text     text,
    automatic     boolean not null,
    received_at   timestamptz not null,
    foreign key (client_id, firm_id) references clients (id, firm_id) on delete cascade
);

create index replies_client on replies (firm_id, client_id, received_at);
create index replies_text on replies (received_at) where body_text is not null;
