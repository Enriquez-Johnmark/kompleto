-- Encrypted files (F3), an engine table. The encrypted bytes live in a file
-- store (local disk, R2 from M6), never in Postgres, so database backups
-- never hold them. This row holds the file's wrapped key and the time by
-- which it must be deleted; deleting removes the file, then the key.
create table stored_files (
    id           uuid primary key,
    tenant_id    uuid not null,
    size_bytes   integer not null,
    wrapped_key  bytea,
    created_at   timestamptz not null,
    delete_by    timestamptz not null,
    deleted_at   timestamptz,
    check ((deleted_at is null) = (wrapped_key is not null))
);

create index stored_files_tenant on stored_files (tenant_id, created_at);
create index stored_files_due on stored_files (delete_by) where deleted_at is null;
