-- Payroll chasing (P1, P2): off when pay_cycle is null.
alter table clients
    add column pay_cycle text check (pay_cycle in ('twice-monthly', 'monthly')),
    add column philhealth_due_day smallint not null default 20 check (philhealth_due_day between 1 and 31),
    add column pagibig_due_day smallint not null default 31 check (pagibig_due_day between 1 and 31);
