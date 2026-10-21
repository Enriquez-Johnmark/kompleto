-- Uploads (F1): what a Contact sent through an Upload link. The file itself
-- is a stored_files row (same ID), encrypted and deleted at its deadline.
create table uploads (
    id             uuid primary key,
    firm_id        uuid not null,
    client_id      uuid not null,
    link_id        uuid not null references upload_links (id) on delete cascade,
    period         text not null,
    -- One of the Period's Owed documents, or null with a note for
    -- "something else". A 2307 also names its Payor.
    owed_document  text,
    payor          text,
    note           text,
    media_type     text not null,
    size_bytes     integer not null,
    uploaded_at    timestamptz not null,
    received_at    timestamptz,
    received_by    text,
    check ((owed_document is null) = (note is not null)),
    foreign key (client_id, firm_id) references clients (id, firm_id) on delete cascade
);

create index uploads_client on uploads (firm_id, client_id, uploaded_at);
create index uploads_link on uploads (link_id);
