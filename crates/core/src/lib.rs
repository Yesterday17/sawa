//! Sawa Core - Domain Layer
//!
//! This crate contains the pure domain logic with no infrastructure dependencies.
//!
//! # Architecture
//!
//! Following hexagonal architecture (ports and adapters):
//!
//! - `models/` - Domain entities, value objects, aggregates
//! - `repositories/` - Repository trait definitions (PORTS)
//! - `services/` - Service trait definitions (PORTS)
//!
//! ## Dependencies
//!
//! This crate has ZERO infrastructure dependencies:
//! - No database clients
//! - No HTTP frameworks
//! - No external service clients
//!
//! Only domain-relevant dependencies:
//! - chrono (for DateTime)
//! - uuid (for entity IDs)
//! - iso_currency (for Price)
//! - thiserror (for error types)
//!
//! ## Implementations
//!
//! - Service implementations → `sawa-application` crate
//! - Repository implementations → `sawa-infra-*` crates

pub mod errors;
pub mod models;
pub mod repositories;
pub mod services;
pub(crate) mod utils;
