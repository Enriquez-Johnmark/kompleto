-- Every model call, for cost per Tenant per month (J1). Engine table: no
-- product references.
create table llm_calls (
    id              uuid primary key,
    tenant_id       uuid not null,
    at              timestamptz not null,
    model           text not null,
    input_tokens    integer not null,
    output_tokens   integer not null,
    -- Billionths of a US dollar; null when the model's price isn't known.
    cost_nano_usd   bigint,
    request_id      text,
    outcome         text not null check (outcome in ('ok', 'invalid', 'failed'))
);

create index llm_calls_tenant on llm_calls (tenant_id, at);
