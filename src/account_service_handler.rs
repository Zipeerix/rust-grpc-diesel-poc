use crate::grpc_server::account_service::account_service_server::AccountService;
use crate::grpc_server::account_service::{
    AddUserRequest, AddUserResponse, DeleteUserRequest, DeleteUserResponse,
};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct AccountServiceHandler {}

#[tonic::async_trait]
impl AccountService for AccountServiceHandler {
    async fn add_user(
        &self,
        request: Request<AddUserRequest>,
    ) -> Result<Response<AddUserResponse>, Status> {
        todo!()
    }

    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<DeleteUserResponse>, Status> {
        todo!()
    }
}
