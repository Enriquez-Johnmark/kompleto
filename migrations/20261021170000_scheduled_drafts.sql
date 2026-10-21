-- Each Draft the scheduler created, one row per (Client, collection Period,
-- slot). ordinal numbers a Period's Drafts 0 to 3, so the database itself
-- holds every Client to at most four Drafts per Period (D1).
create table scheduled_drafts (
    firm_id     uuid not null,
    client_id   uuid not null,
    period      text not null,
    slot        text not null,
    ordinal     smallint not null check (ordinal between 0 and 3),
    -- Inserted in the same transaction as the Draft itself.
    message_id  uuid not null,
    created_at  timestamptz not null,
    primary key (client_id, period, slot),
    unique (client_id, period, ordinal),
    foreign key (client_id, firm_id) references clients (id, firm_id) on delete cascade
);
