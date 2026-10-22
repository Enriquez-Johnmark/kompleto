-- The audit log stays append-only, except that erasing a closed account
-- (I2) deletes that one tenant's entries: the transaction names the tenant
-- in `engine.erasing_tenant` first, and only its rows can go.
create or replace function audit_events_append_only() returns trigger language plpgsql as $$
begin
    if tg_op = 'DELETE' and old.tenant_id::text = current_setting('engine.erasing_tenant', true) then
        return old;
    end if;
    raise exception 'audit_events is append-only';
end;
$$;
