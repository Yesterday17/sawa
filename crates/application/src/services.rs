use sawa_core::repositories::{
    MediaRepository, ProductInstanceRepository, ProductRepository, ProductVariantRepository,
    PurchaseOrderRepository, TagRepository, UserRepository, UserTransactionRepository,
};

/// Unified service that implements all domain service traits.
///
/// This struct holds all repository dependencies and provides implementations
/// for all business operations defined in the core service traits.
///
/// In hexagonal architecture:
/// - Service traits in core are PORTS
/// - This struct is the ADAPTER that implements all ports
/// - Repositories are injected dependencies (also ports)
pub struct Service<P, PV, PI, PO, UT, U, T, M>
where
    P: ProductRepository,
    PV: ProductVariantRepository,
    PI: ProductInstanceRepository,
    PO: PurchaseOrderRepository,
    UT: UserTransactionRepository,
    U: UserRepository,
    T: TagRepository,
    M: MediaRepository,
{
    pub product: P,
    pub product_variant: PV,
    pub product_instance: PI,
    pub order: PO,
    pub transaction: UT,
    pub user: U,
    pub tag: T,
    pub media: M,
}

// Service trait implementations (core flow only)
mod product_impl;
mod product_instance_impl;
mod purchase_order_impl;
mod purchase_order_lifecycle_impl;
mod transaction_impl;
mod transaction_lifecycle_impl;
mod user_impl;
