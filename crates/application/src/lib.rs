//! Sawa Application - Service Implementation Layer
//!
//! This crate provides concrete implementations of service traits defined in sawa-core.
//!
//! # Architecture
//!
//! - Depends on: `sawa-core` (domain layer)
//! - Implements: Service traits (business logic)
//! - Uses: Repository abstractions (no concrete storage)
//!
//! ## Unified Service Pattern
//!
//! All service traits are implemented by a single `Service` struct:
//!
//! ```ignore
//! pub struct Service<P, V, I, O, T, U, Tg, M> {
//!     // All repository dependencies injected
//! }
//!
//! // Implements multiple traits:
//! impl CollectionQuery for Service { ... }
//! impl OrderManagement for Service { ... }
//! impl OrderFulfillment for Service { ... }
//! // etc.
//! ```
//!
//! This allows:
//! - Sharing repositories across all operations
//! - Single point of dependency injection
//! - Cohesive domain context

mod services;
pub use services::Service;
