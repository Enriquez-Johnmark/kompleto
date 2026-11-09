//! The people who log in to a Firm (M9): one owner, whose address is the
//! Firm's login address, and up to 5 staff. They are the engine's account
//! members (`engine::accounts`); every query is scoped by the Firm (A2), and
//! adding or removing staff is logged as the engine's `account.member_added`
//! and `account.member_removed`.

pub use crate::engine::accounts::{MAX_STAFF, Member, MemberError, MemberId, Members, Role, is_email_address};
