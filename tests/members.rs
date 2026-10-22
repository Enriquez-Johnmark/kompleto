//! M9: the people who log in to a Firm, its owner and staff (K1–K3).

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use common::clock;
use kompleto::kompleto::firms::{FirmError, Firms};
use kompleto::kompleto::members::{MemberError, Members, Role};
use sqlx::PgPool;

#[sqlx::test]
async fn k1_a_new_firm_has_its_owner_as_its_only_member(pool: PgPool) {
    let firms = Firms::new(pool.clone(), clock());

    let santos = firms.create("Santos Bookkeeping", "bea@santos.test").await.unwrap();

    let members = Members::new(pool.clone(), clock()).list(santos).await.unwrap();
    assert_eq!(members.len(), 1);
    assert_eq!(members[0].email, "bea@santos.test");
    assert_eq!(members[0].role, Role::Owner);
}

#[sqlx::test]
async fn k1_an_address_belongs_to_one_firm_only(pool: PgPool) {
    let firms = Firms::new(pool.clone(), clock());
    firms.create("Santos Bookkeeping", "bea@santos.test").await.unwrap();

    let again = firms.create("Reyes CPA", "BEA@santos.test").await;

    assert!(matches!(again, Err(FirmError::LoginEmailTaken)), "{again:?}");
}

#[sqlx::test]
async fn k1_the_owner_adds_up_to_five_staff_each_with_an_address_of_their_own(pool: PgPool) {
    let santos = Firms::new(pool.clone(), clock())
        .create("Santos Bookkeeping", "bea@santos.test")
        .await
        .unwrap();
    let members = Members::new(pool.clone(), clock());

    let add = async |email: &str| members.add_staff(santos, email, "bea@santos.test").await;
    add(" staff1@santos.test ").await.unwrap();
    let twice = add("STAFF1@santos.test").await;
    let not_an_address = add("staff two").await;
    for n in 2..=5 {
        add(&format!("staff{n}@santos.test")).await.unwrap();
    }
    let sixth = add("staff6@santos.test").await;

    assert!(matches!(sixth, Err(MemberError::TooManyStaff)), "{sixth:?}");
    assert!(matches!(twice, Err(MemberError::EmailTaken)), "{twice:?}");
    assert!(
        matches!(not_an_address, Err(MemberError::Invalid(_))),
        "{not_an_address:?}"
    );
    let listed = members.list(santos).await.unwrap();
    assert_eq!(listed.len(), 6);
    assert_eq!(listed[1].email, "staff1@santos.test");
    assert!(listed[1..].iter().all(|member| member.role == Role::Staff));
}

#[sqlx::test]
async fn k1_staff_can_be_removed_but_the_owner_cant(pool: PgPool) {
    let firms = Firms::new(pool.clone(), clock());
    let santos = firms.create("Santos Bookkeeping", "bea@santos.test").await.unwrap();
    let reyes = firms.create("Reyes CPA", "rey@reyes.test").await.unwrap();
    let members = Members::new(pool.clone(), clock());
    let ana = members
        .add_staff(santos, "ana@santos.test", "bea@santos.test")
        .await
        .unwrap();
    let owner = members.list(santos).await.unwrap()[0].id;

    let from_other_firm = members.remove_staff(reyes, ana, "rey@reyes.test").await;
    let the_owner = members.remove_staff(santos, owner, "bea@santos.test").await;
    members.remove_staff(santos, ana, "bea@santos.test").await.unwrap();

    assert!(
        matches!(from_other_firm, Err(MemberError::NotFound)),
        "{from_other_firm:?}"
    );
    assert!(matches!(the_owner, Err(MemberError::IsOwner)), "{the_owner:?}");
    assert_eq!(members.list(santos).await.unwrap().len(), 1);
}
