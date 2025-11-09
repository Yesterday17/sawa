use sawa_core::{
    repositories::*,
    services::{
        AddOrderItemError, CreateOrderError, GetOrderError, PurchaseOrderService,
        SubmitMysteryBoxResultsError,
    },
};

use super::Service;

impl<P, PV, PI, PO, UT, U, T, M> PurchaseOrderService for Service<P, PV, PI, PO, UT, U, T, M>
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
    async fn create_order(
        &self,
        _req: &sawa_core::services::CreateOrderRequest,
    ) -> Result<sawa_core::models::purchase::PurchaseOrder, CreateOrderError> {
        todo!("Implement create_order")
    }

    async fn add_order_item(
        &self,
        _req: &sawa_core::services::AddOrderItemRequest,
    ) -> Result<sawa_core::models::purchase::PurchaseOrderItemId, AddOrderItemError> {
        todo!("Implement add_order_item")
    }

    async fn submit_mystery_box_results(
        &self,
        _req: &sawa_core::services::SubmitMysteryBoxResultsRequest,
    ) -> Result<(), SubmitMysteryBoxResultsError> {
        todo!("Implement submit_mystery_box_results")
    }

    async fn get_order(
        &self,
        _req: &sawa_core::services::GetOrderRequest,
    ) -> Result<sawa_core::models::purchase::PurchaseOrder, GetOrderError> {
        todo!("Implement get_order")
    }
}
