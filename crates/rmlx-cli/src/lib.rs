//! Library surface for the `rmlx` CLI crate.
//!
//! Exists so host applications (Ganzo) can embed the full serve pipeline
//! in-process: [`commands::serve::run_serve`] is the same code path the
//! `rmlx serve` binary uses — loader closure, KV-calibration discovery,
//! metrics drainer, eager preload, admission control.

pub mod commands;
pub mod startup;

pub use commands::serve::run_serve;
