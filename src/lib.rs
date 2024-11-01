#![allow(clippy::doc_lazy_continuation)]
#![doc = include_str!("../README.md")]

/// api
///
/// The api module
pub mod api;

#[cfg(feature = "build")]
pub mod build;

pub struct DisabledFeature;

impl DisabledFeature {
    #[allow(dead_code)]
    fn new() -> Self { Self{} }
}
