//! Kompleto: a document and BIR-deadline chaser for Philippine bookkeepers,
//! built on a reusable engine (draft → approve → send → stop on reply → track).

pub mod config;
pub mod kompleto;

/// The engine, Nudgewright, under the name the code has always used.
pub use nudgewright as engine;
