-- Login links and sessions belong to a person (M9), and go with them.
alter table login_tokens add column member_id uuid;
alter table sessions add column member_id uuid;

update login_tokens t set member_id = m.id
from firm_members m where m.firm_id = t.firm_id and m.role = 'owner';
update sessions s set member_id = m.id
from firm_members m where m.firm_id = s.firm_id and m.role = 'owner';

alter table login_tokens
    alter column member_id set not null,
    add foreign key (member_id, firm_id) references firm_members (id, firm_id) on delete cascade;
alter table sessions
    alter column member_id set not null,
    add foreign key (member_id, firm_id) references firm_members (id, firm_id) on delete cascade;

create index login_tokens_member on login_tokens (member_id, created_at);
create index sessions_member on sessions (member_id);
