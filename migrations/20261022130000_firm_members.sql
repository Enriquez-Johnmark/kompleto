-- The people who log in to a Firm (M9): its owner and up to 5 staff.
create table firm_members (
    id          uuid primary key,
    firm_id     uuid not null references firms (id) on delete cascade,
    email       text not null,
    role        text not null check (role in ('owner', 'staff')),
    created_at  timestamptz not null,
    unique (id, firm_id)
);

-- An address logs in to one Firm only.
create unique index firm_members_email on firm_members (lower(email));
create unique index firm_members_one_owner on firm_members (firm_id) where role = 'owner';

-- Each existing Firm's login address becomes its owner.
insert into firm_members (id, firm_id, email, role, created_at)
select gen_random_uuid(), id, login_email, 'owner', created_at from firms;
