//! A Client's payroll chasing settings (P1, P2).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use common::{clock, firm, nena};
use kompleto::kompleto::clients::{ClientError, Clients};
use kompleto::kompleto::payroll::{PayCycle, PayrollSettings};
use sqlx::PgPool;

#[sqlx::test]
async fn p1_payroll_chasing_is_off_until_set_and_reads_back(pool: PgPool) {
    let santos = firm(&pool, "Santos Bookkeeping").await;
    let clients = Clients::new(pool.clone(), clock());
    let client = clients.create(santos, &nena()).await.unwrap();
    assert_eq!(clients.payroll(santos, client).await.unwrap(), None);

    let settings = PayrollSettings {
        cycle: PayCycle::TwiceMonthly,
        philhealth_due_day: 15,
        pagibig_due_day: 31,
    };
    clients.set_payroll(santos, client, Some(settings)).await.unwrap();
    assert_eq!(clients.payroll(santos, client).await.unwrap(), Some(settings));

    clients.set_payroll(santos, client, None).await.unwrap();
    assert_eq!(clients.payroll(santos, client).await.unwrap(), None);
}

#[sqlx::test]
async fn p2_a_due_day_outside_the_month_is_refused_and_another_firms_client_is_not_found(pool: PgPool) {
    let (santos, reyes) = (firm(&pool, "Santos Bookkeeping").await, firm(&pool, "Reyes CPA").await);
    let clients = Clients::new(pool.clone(), clock());
    let client = clients.create(santos, &nena()).await.unwrap();
    let settings = PayrollSettings {
        cycle: PayCycle::Monthly,
        philhealth_due_day: 32,
        pagibig_due_day: 31,
    };

    let refused = clients.set_payroll(santos, client, Some(settings)).await;
    let elsewhere = clients
        .set_payroll(
            reyes,
            client,
            Some(PayrollSettings {
                philhealth_due_day: 20,
                ..settings
            }),
        )
        .await;

    assert!(matches!(refused, Err(ClientError::Invalid(_))), "{refused:?}");
    assert!(matches!(elsewhere, Err(ClientError::NotFound)), "{elsewhere:?}");
    assert_eq!(clients.payroll(santos, client).await.unwrap(), None);
}
