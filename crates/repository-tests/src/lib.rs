//! Repository Contract Tests
//!
//! This crate provides reusable test suites for repository implementations.
//! Any implementation of repository traits should pass these tests.
//!
//! # Usage
//!
//! ```ignore
//! use sawa_repository_tests::test_all_repositories;
//!
//! test_all_repositories! {
//!     product: InMemoryProductRepository::new(),
//!     product_instance: InMemoryProductInstanceRepository::new(),
//!     // ... other repos
//! }
//! ```

pub mod macros;
pub mod suites;

// Re-export tokio for test macro users
pub use tokio;
