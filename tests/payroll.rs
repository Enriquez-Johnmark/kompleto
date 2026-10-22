//! A Client's payroll chasing settings (P1, P2).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use common::{clock, firm, nena};
use kompleto::kompleto::clients::{ClientError, Clients};
use kompleto::kompleto::payroll::{PayCycle, PayrollChoice};
use sqlx::PgPool;

#[sqlx::test]
async fn p1_payroll_chasing_is_off_until_set_and_off_keeps_the_due_days(pool: PgPool) {
    let santos = firm(&pool, "Santos Bookkeeping").await;
    let clients = Clients::new(pool.clone(), clock());
    let client = clients.create(santos, &nena()).await.unwrap();
    assert_eq!(clients.payroll(santos, client).await.unwrap(), PayrollChoice::default());

    let on = PayrollChoice {
        cycle: Some(PayCycle::TwiceMonthly),
        philhealth_due_day: 15,
        pagibig_due_day: 24,
    };
    clients.set_payroll(santos, client, on).await.unwrap();
    assert_eq!(clients.payroll(santos, client).await.unwrap(), on);

    let off = PayrollChoice { cycle: None, ..on };
    clients.set_payroll(santos, client, off).await.unwrap();
    assert_eq!(clients.payroll(santos, client).await.unwrap(), off);
    assert_eq!(off.settings(), None);
}

#[sqlx::test]
async fn p2_a_due_day_outside_the_month_is_refused_and_another_firms_client_is_not_found(pool: PgPool) {
    let (santos, reyes) = (firm(&pool, "Santos Bookkeeping").await, firm(&pool, "Reyes CPA").await);
    let clients = Clients::new(pool.clone(), clock());
    let client = clients.create(santos, &nena()).await.unwrap();
    let choice = PayrollChoice {
        cycle: Some(PayCycle::Monthly),
        philhealth_due_day: 32,
        pagibig_due_day: 31,
    };

    let refused = clients.set_payroll(santos, client, choice).await;
    let elsewhere = clients
        .set_payroll(
            reyes,
            client,
            PayrollChoice {
                philhealth_due_day: 20,
                ..choice
            },
        )
        .await;

    assert!(matches!(refused, Err(ClientError::Invalid(_))), "{refused:?}");
    assert!(matches!(elsewhere, Err(ClientError::NotFound)), "{elsewhere:?}");
    assert_eq!(clients.payroll(santos, client).await.unwrap(), PayrollChoice::default());
}
