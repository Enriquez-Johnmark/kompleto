//! Background jobs in a Postgres table (ADR 0002). A worker claims a due job
//! with `FOR UPDATE SKIP LOCKED` and holds it for a lease; if the worker
//! dies, the lease runs out and another worker picks the job up.

use std::sync::Arc;

use serde_json::Value;
use sqlx::PgPool;
use time::{Duration, OffsetDateTime};

use super::clock::Clock;
use super::ids::{JobId, TenantId};

/// A job handed to a worker.
#[derive(Debug, Clone)]
pub struct ClaimedJob {
    /// Its ID.
    pub id: JobId,
    /// What to do, e.g. `"draft_reminders"`.
    pub kind: String,
    /// The job's input.
    pub payload: Value,
    /// Whose data it concerns, if anyone's.
    pub tenant: Option<TenantId>,
    /// Which attempt this is, starting at 1.
    pub attempts: i32,
}

/// Job queue errors.
#[derive(Debug, thiserror::Error)]
pub enum JobError {
    /// Database failure.
    #[error(transparent)]
    Db(#[from] sqlx::Error),
}

/// The job queue.
#[derive(Clone)]
pub struct Jobs {
    pool: PgPool,
    clock: Arc<dyn Clock>,
}

impl Jobs {
    /// A queue on `pool`, reading time from `clock`.
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>) -> Self {
        Self { pool, clock }
    }

    /// Adds a job to run at or after `run_at`.
    pub async fn enqueue(
        &self,
        kind: &str,
        payload: &Value,
        tenant: Option<TenantId>,
        run_at: OffsetDateTime,
    ) -> Result<JobId, JobError> {
        let id = JobId::new();
        sqlx::query!(
            "insert into jobs (id, tenant_id, kind, payload, status, run_at, created_at, updated_at)
             values ($1, $2, $3, $4, 'queued', $5, $6, $6)",
            id.as_uuid(),
            tenant.map(|t| t.as_uuid()),
            kind,
            payload,
            run_at,
            self.clock.now(),
        )
        .execute(&self.pool)
        .await?;
        Ok(id)
    }

    /// Claims the next due job for `lease`, or `None` if nothing is due.
    pub async fn claim(&self, lease: Duration) -> Result<Option<ClaimedJob>, JobError> {
        let now = self.clock.now();
        let row = sqlx::query!(
            "update jobs
             set locked_until = $2, attempts = attempts + 1, updated_at = $1
             where id = (
                 select id from jobs
                 where status = 'queued' and run_at <= $1 and (locked_until is null or locked_until < $1)
                 order by run_at
                 limit 1
                 for update skip locked
             )
             returning id, kind, payload, tenant_id, attempts",
            now,
            now + lease,
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|r| ClaimedJob {
            id: r.id.into(),
            kind: r.kind,
            payload: r.payload,
            tenant: r.tenant_id.map(TenantId::from),
            attempts: r.attempts,
        }))
    }

    /// Marks a claimed job done.
    pub async fn complete(&self, id: JobId) -> Result<(), JobError> {
        sqlx::query!(
            "update jobs set status = 'done', locked_until = null, updated_at = $2 where id = $1",
            id.as_uuid(),
            self.clock.now(),
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Records a failed attempt. With `retry_at` the job runs again then;
    /// without, it is given up on (`dead`) and left for a person.
    pub async fn fail(&self, id: JobId, error: &str, retry_at: Option<OffsetDateTime>) -> Result<(), JobError> {
        let now = self.clock.now();
        sqlx::query!(
            "update jobs
             set status = case when $3::timestamptz is null then 'dead' else 'queued' end,
                 run_at = coalesce($3, run_at), locked_until = null, last_error = $2, updated_at = $4
             where id = $1",
            id.as_uuid(),
            error,
            retry_at,
            now,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
