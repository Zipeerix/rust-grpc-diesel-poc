use crate::database::{DbPool, GrpcDbPoolWrapper};
use crate::order_service::proto::order_service_server::OrderService;
use crate::order_service::proto::*;
use tonic::{Request, Response, Status};

pub struct OrderServiceHandler {
    db: GrpcDbPoolWrapper,
}

impl OrderServiceHandler {
    pub fn new(db: DbPool) -> OrderServiceHandler {
        OrderServiceHandler {
            db: GrpcDbPoolWrapper::new(db),
        }
    }
}

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
    ) -> Result<Response<()>, Status> {
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
