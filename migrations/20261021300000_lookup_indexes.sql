-- Delivery events find the sent message by the provider's ID, and inbound
-- mail without a reply code finds the Client by its address.
create index outbox_messages_provider_message_id on outbox_messages (provider_message_id)
    where provider_message_id is not null;
create index clients_lower_email on clients (lower(email));
