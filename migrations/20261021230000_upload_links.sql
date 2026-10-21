-- Upload links (F2): one per Client per collection Period. The link itself
-- is this ID plus an HMAC signature that is never stored, so nothing here
-- can be used to open an upload page.
create table upload_links (
    id          uuid primary key,
    firm_id     uuid not null,
    client_id   uuid not null,
    period      text not null,
    cut_off     date not null,
    expires_at  timestamptz not null,
    created_at  timestamptz not null,
    unique (client_id, period),
    foreign key (client_id, firm_id) references clients (id, firm_id) on delete cascade
);
