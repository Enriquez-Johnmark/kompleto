-- How one Client's Owed documents differ from the default checklist (B4).
create table client_removed_items (
    firm_id    uuid not null,
    client_id  uuid not null,
    item_key   text not null,
    primary key (client_id, item_key),
    foreign key (client_id, firm_id) references clients (id, firm_id) on delete cascade
);

create table client_extra_items (
    id         uuid primary key,
    firm_id    uuid not null,
    client_id  uuid not null,
    label      text not null,
    recurs     text not null check (recurs in ('every-period', 'quarter-end', 'year-end')),
    created_at timestamptz not null,
    foreign key (client_id, firm_id) references clients (id, firm_id) on delete cascade
);

create index client_extra_items_client on client_extra_items (client_id);
