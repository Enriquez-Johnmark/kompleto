-- A Draft not approved by this time expires and can never be sent (D4).
alter table outbox_messages add column expires_at timestamptz;

create index outbox_messages_expiring on outbox_messages (expires_at) where status = 'draft';
