# Postgres is the only infrastructure

The job queue (a table claimed with `FOR UPDATE SKIP LOCKED`), login sessions, the outbox, the audit log and the cost log all live in Postgres, processed by workers inside the one binary. There is no Redis, message broker or separate worker service. The hosting budget is about US$8/month on one small server, and one database to back up keeps operations within a solo builder's reach. At Kompleto's scale (a few thousand jobs a month), Postgres is far from its limits.

## Consequences

If the hand-rolled queue gets costly to maintain, switch to `apalis` + `apalis-postgres` 1.0, which also runs on Postgres, before considering new infrastructure.
