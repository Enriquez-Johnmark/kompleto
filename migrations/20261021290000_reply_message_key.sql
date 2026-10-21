-- A redelivered email is recognised by its key and never kept twice;
-- handled_at is set once its reminders were stopped and the Firm told.
alter table replies add column message_key text;
alter table replies add column handled_at timestamptz;
update replies set handled_at = received_at;
create unique index replies_message_key on replies (firm_id, message_key);
