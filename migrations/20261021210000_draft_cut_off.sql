-- The Cut-off each scheduled Draft was written for, shown on the Drafts pages.
alter table scheduled_drafts add column cut_off date;
