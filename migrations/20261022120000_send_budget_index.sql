-- The send budget counts messages started in the last 24 hours on every
-- claim; this keeps that count to the last day's rows.
create index outbox_messages_first_send_at on outbox_messages (first_send_at)
    where first_send_at is not null;
