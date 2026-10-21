-- Whether the Reply came through its signed reply address. One matched only
-- by the sender's address (which can be forged) never stops Reminders.
alter table replies add column signed boolean not null default true;
