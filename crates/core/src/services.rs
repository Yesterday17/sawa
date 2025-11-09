/// Service traits (Ports in Hexagonal Architecture)
///
/// These traits define the business operations for the domain.
/// Implementations (Adapters) are provided in the application layer.
///
/// # Request/Response Pattern
///
/// Following hexagonal architecture best practices:
/// - Each method takes a Request struct
/// - Returns Result<Response, SpecificError>
/// - Errors are domain-specific, not generic RepositoryError
///
/// # Naming Convention
///
/// - All service traits end with "Service"
/// - Errors are specific to each operation
mod purchase_order;
pub use purchase_order::*;

mod purchase_order_lifecycle;
pub use purchase_order_lifecycle::*;
