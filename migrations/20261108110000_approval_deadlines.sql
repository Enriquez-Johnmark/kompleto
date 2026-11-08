-- Nudgewright v0.4.0 (its migration 20261024140000): a message's deadline
-- normally applies until it starts to send; a product may instead set a
-- deadline only for approval.
alter table outbox_messages add column deadline_for_approval_only boolean not null default false;
