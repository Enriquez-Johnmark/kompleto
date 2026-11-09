-- Kompleto moves onto the engine's accounts (Nudgewright v0.3.0). Each Firm
-- becomes an account with the same id, its owner and staff become account
-- members, and waiting account requests become sign-up requests. Sessions
-- and login links are not copied, so everyone logs in again once (owner,
-- 2026-11-08). The firms table keeps only Kompleto's own settings; a Firm
-- with no row there has the defaults.

insert into accounts (tenant_id, name, created_at, closed_at, erase_after)
select id, name, created_at, closed_at, erase_after from firms;

insert into account_members (id, tenant_id, email, role, created_at)
select id, firm_id, email, role, created_at from firm_members;

insert into account_signup_requests (id, name, email, accepted_at, created_at)
select id, firm_name, email, accepted_at, created_at from account_requests;

drop table sessions;
drop table login_tokens;
drop table firm_members;
drop table account_requests;

-- Erasing an account removes its Clients and everything about them.
alter table clients
    drop constraint clients_firm_id_fkey,
    add constraint clients_firm_id_fkey foreign key (firm_id) references accounts (tenant_id) on delete cascade;

alter table firms
    add constraint firms_account foreign key (id) references accounts (tenant_id) on delete cascade,
    drop column name,
    drop column login_email,
    drop column closed_at,
    drop column erase_after,
    drop column created_at;
