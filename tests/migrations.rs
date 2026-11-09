//! Migrations that move data: a database from before them comes out whole.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

use std::borrow::Cow;

use sqlx::PgPool;
use sqlx::migrate::Migrator;

/// Kompleto's Firms move onto the engine's accounts (2026-11-08).
const FIRMS_ON_ACCOUNTS: i64 = 20261108130000;

/// The migrations before `version`, or from it on.
fn migrations(before: bool, version: i64) -> Migrator {
    let mut migrator = sqlx::migrate!();
    migrator.migrations = Cow::Owned(
        migrator
            .migrations
            .iter()
            .filter(|m| (m.version < version) == before)
            .cloned()
            .collect(),
    );
    migrator.ignore_missing = true;
    migrator
}

#[sqlx::test(migrations = false)]
async fn firms_move_onto_accounts_with_their_people_and_requests_and_everyone_logs_in_again(pool: PgPool) {
    migrations(true, FIRMS_ON_ACCOUNTS).run(&pool).await.unwrap();
    let firm = uuid::Uuid::now_v7();
    sqlx::query(
        "insert into firms (id, name, login_email, cut_off_offset_days, created_at, closed_at, erase_after)
         values ($1, 'Santos Bookkeeping', 'bea@santos.test', 7, now(), now(), now() + interval '7 days')",
    )
    .bind(firm)
    .execute(&pool)
    .await
    .unwrap();
    let (owner, staff) = (uuid::Uuid::now_v7(), uuid::Uuid::now_v7());
    sqlx::query(
        "insert into firm_members (id, firm_id, email, role, created_at)
         values ($1, $3, 'bea@santos.test', 'owner', now()), ($2, $3, 'carlo@santos.test', 'staff', now())",
    )
    .bind(owner)
    .bind(staff)
    .bind(firm)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "insert into sessions (session_hash, firm_id, member_id, created_at, expires_at)
         values ('\\x01', $1, $2, now(), now() + interval '30 days')",
    )
    .bind(firm)
    .bind(staff)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "insert into account_requests (id, firm_name, email, accepted_at, created_at)
         values ($1, 'Reyes CPA', 'rina@reyes.test', now(), now())",
    )
    .bind(uuid::Uuid::now_v7())
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "insert into clients (id, firm_id, business_name, contact_first_name, email, language, forms, created_at, updated_at)
         values ($1, $2, 'Aling Nena''s Store', 'Nena', 'nena@store.test', 'taglish', '{2550Q}', now(), now())",
    )
    .bind(uuid::Uuid::now_v7())
    .bind(firm)
    .execute(&pool)
    .await
    .unwrap();

    migrations(false, FIRMS_ON_ACCOUNTS).run(&pool).await.unwrap();

    let (name, closed): (String, bool) =
        sqlx::query_as("select name, closed_at is not null from accounts where tenant_id = $1")
            .bind(firm)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!((name.as_str(), closed), ("Santos Bookkeeping", true));
    let members: Vec<(uuid::Uuid, String)> =
        sqlx::query_as("select id, role from account_members where tenant_id = $1 order by role")
            .bind(firm)
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(members, [(owner, "owner".to_owned()), (staff, "staff".to_owned())]);
    let offset: i16 = sqlx::query_scalar("select cut_off_offset_days from firms where id = $1")
        .bind(firm)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(offset, 7, "Kompleto's own setting stays");
    let requests: Vec<String> = sqlx::query_scalar("select name from account_signup_requests")
        .fetch_all(&pool)
        .await
        .unwrap();
    assert_eq!(requests, ["Reyes CPA"]);
    let sessions: i64 = sqlx::query_scalar("select count(*) from account_sessions")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(sessions, 0, "everyone logs in again once");

    // Erasing the account removes the Firm's settings and Clients with it.
    sqlx::query("delete from accounts where tenant_id = $1")
        .bind(firm)
        .execute(&pool)
        .await
        .unwrap();
    let left: i64 = sqlx::query_scalar("select (select count(*) from firms) + (select count(*) from clients)")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(left, 0);
}
