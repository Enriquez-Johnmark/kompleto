-- Who wrote each scheduled Draft's wording: the model or the template (D5).
alter table scheduled_drafts
    add column source text not null default 'template' check (source in ('ai', 'template'));
