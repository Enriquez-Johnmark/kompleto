-- A Firm that closed its account (I2): from closed_at nothing is sent, no
-- one logs in and its links are dead; at erase_after all its data goes.
alter table firms add column closed_at timestamptz;
alter table firms add column erase_after timestamptz;
