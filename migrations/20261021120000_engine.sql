-- Engine tables. They are scoped by tenant_id and never reference product
-- tables, so the engine stays reusable (ADR 0001). Every time value is bound
-- from the application's Clock; no column defaults to now().

create table outbox_messages (
    id                  uuid primary key,
    tenant_id           uuid not null,
    status              text not null check (status in (
                            'draft', 'approved', 'sending', 'sent', 'failed',
                            'blocked', 'unconfirmed', 'expired', 'cancelled')),
    to_address          text not null,
    from_name           text not null,
    reply_to            text,
    subject             text not null,
    body_text           text not null,
    -- Hash of the exact content the approver saw (D2).
    approved_hash       bytea,
    approved_at         timestamptz,
    approved_by         text,
    send_attempts       integer not null default 0,
    -- Set once an attempt's outcome was unknown (timeout, crash): the email
    -- may have gone out, so only a duplicate-blocking provider may retry.
    may_have_sent       boolean not null default false,
    first_send_at       timestamptz,
    send_started_at     timestamptz,
    retry_at            timestamptz,
    provider_message_id text,
    sent_at             timestamptz,
    last_error          text,
    created_at          timestamptz not null,
    updated_at          timestamptz not null,
    check (status = 'draft' or status in ('expired', 'cancelled') or approved_hash is not null)
);

create index outbox_messages_tenant on outbox_messages (tenant_id, created_at);
create index outbox_messages_due on outbox_messages (approved_at) where status in ('approved', 'failed');
create index outbox_messages_sending on outbox_messages (send_started_at) where status = 'sending';

create table jobs (
    id            uuid primary key,
    tenant_id     uuid,
    kind          text not null,
    payload       jsonb not null,
    status        text not null check (status in ('queued', 'done', 'dead')),
    run_at        timestamptz not null,
    locked_until  timestamptz,
    attempts      integer not null default 0,
    last_error    text,
    created_at    timestamptz not null,
    updated_at    timestamptz not null
);

create index jobs_ready on jobs (run_at) where status = 'queued';

create table audit_events (
    id            uuid primary key,
    tenant_id     uuid not null,
    at            timestamptz not null,
    actor         text not null,
    action        text not null,
    subject_type  text not null,
    subject_id    uuid not null,
    details       jsonb not null default '{}'
);

create index audit_events_tenant on audit_events (tenant_id, at);

-- The audit log is append-only.
create function audit_events_append_only() returns trigger language plpgsql as $$
begin
    raise exception 'audit_events is append-only';
end;
$$;

create trigger audit_events_no_update_or_delete
    before update or delete on audit_events
    for each row execute function audit_events_append_only();
