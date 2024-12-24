use crate::grpc_server::order_service::order_service_server::OrderService;
use crate::grpc_server::order_service::{
    AddOrderRequest, AddOrderResponse, DeleteOrderRequest, DeleteOrderResponse,
    FindOrderByUserIdRequest, FindOrderRequest, FindOrderResponse, GetAllOrdersResponse,
};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct OrderServiceHandler {}

#[tonic::async_trait]
impl OrderService for OrderServiceHandler {
    async fn add_order(
        &self,
        request: Request<AddOrderRequest>,
    ) -> Result<Response<AddOrderResponse>, Status> {
        todo!()
    }

    async fn delete_order(
        &self,
        request: Request<DeleteOrderRequest>,
    ) -> Result<Response<DeleteOrderResponse>, Status> {
        todo!()
    }

    async fn find_order_by_id(
        &self,
        request: Request<FindOrderRequest>,
    ) -> Result<Response<FindOrderResponse>, Status> {
        todo!()
    }

    async fn find_order_by_user_id(
        &self,
        request: Request<FindOrderByUserIdRequest>,
    ) -> Result<Response<FindOrderResponse>, Status> {
        todo!()
    }

    async fn get_all_orders(
        &self,
        request: Request<()>,
    ) -> Result<Response<GetAllOrdersResponse>, Status> {
        todo!()
    }
}
