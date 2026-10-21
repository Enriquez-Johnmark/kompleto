-- A Client's own due date for one form and Period, replacing the built-in
-- calendar's (B3).
create table due_date_overrides (
    firm_id    uuid not null,
    client_id  uuid not null,
    form       text not null,
    period     text not null,
    due_date   date not null,
    primary key (client_id, form, period),
    foreign key (client_id, firm_id) references clients (id, firm_id) on delete cascade
);
