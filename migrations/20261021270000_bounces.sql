-- Bounces and complaints (G3), matched to a Reminder we sent. A Client's
-- Reminders stop while its email is an address with a `bounce` or
-- `complaint` here; changing the email starts them again.
create table bounces (
    id          uuid primary key,
    firm_id     uuid not null,
    client_id   uuid not null,
    message_id  uuid not null,
    address     text not null,
    kind        text not null check (kind in ('bounce', 'soft_bounce', 'complaint')),
    reason      text,
    at          timestamptz not null,
    foreign key (client_id, firm_id) references clients (id, firm_id) on delete cascade
);

create index bounces_client on bounces (firm_id, client_id, at);
