-- How many of a Reply's attachments were kept as Uploads ("Sent by email").
alter table replies add column attachments integer not null default 0;
