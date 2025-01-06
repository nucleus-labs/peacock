#![allow(clippy::doc_lazy_continuation)]
#![doc = include_str!("../README.md")]

/// api
///
/// The api module
pub mod api;

#[cfg(feature = "build")]
pub mod build;
