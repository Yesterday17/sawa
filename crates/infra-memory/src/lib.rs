//! Sawa Infrastructure - In-Memory Implementations
//!
//! This crate provides in-memory repository implementations for testing and development.
//!
//! # Usage
//!
//! These implementations are useful for:
//! - Unit testing service layer
//! - Integration testing  
//! - Local development without database
//! - Prototyping
//!
//! # Example
//!
//! ```ignore
//! use sawa_infra_memory::repositories::*;
//!
//! let product_repo = InMemoryProductRepository::new();
//! let variant_repo = InMemoryProductVariantRepository::new();
//! // ... etc
//! ```

mod repositories;
pub use repositories::*;
