-- Payors (H2): businesses expected to send a Client a BIR Form 2307 each
-- quarter. Names only, never TINs.
create table payors (
    id          uuid primary key,
    firm_id     uuid not null,
    client_id   uuid not null,
    name        text not null,
    created_at  timestamptz not null,
    foreign key (client_id, firm_id) references clients (id, firm_id) on delete cascade
);

create unique index payors_name on payors (client_id, lower(name));
