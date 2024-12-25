use crate::account_service::account_repository::{create_new_user, delete_user, validate_login};
use crate::account_service::models::NewUser;
use crate::account_service::proto::account_service_server::AccountService;
use crate::account_service::proto::*;
use crate::database::{DbPool, GrpcDbPoolWrapper};
use tonic::{Request, Response, Status};

pub struct AccountServiceHandler {
    db: GrpcDbPoolWrapper,
}

impl AccountServiceHandler {
    pub fn new(db: DbPool) -> AccountServiceHandler {
        AccountServiceHandler {
            db: GrpcDbPoolWrapper::new(db),
        }
    }
}

#[tonic::async_trait]
impl AccountService for AccountServiceHandler {
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<()>, Status> {
        let request = request.into_inner();

        let mut db_conn = self.db.get_db_connection_or_return_unavailable()?;

        validate_login(&request.email, &request.password, &mut db_conn)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(()))
    }

    async fn add_user(
        &self,
        request: Request<AddUserRequest>,
    ) -> Result<Response<AddUserResponse>, Status> {
        let request = request.into_inner();
        let proto_new_user = request
            .new_user
            .ok_or(Status::invalid_argument("User is required"))?;

        let new_user = NewUser::from(&proto_new_user);

        let mut db_conn = self.db.get_db_connection_or_return_unavailable()?;
        let created_user_id =
            create_new_user(new_user, &mut db_conn).map_err(|e| Status::internal(e.to_string()))?;
        //TODO: above return error proto object or stick to statuses

        Ok(Response::new(AddUserResponse {
            user_id: created_user_id,
        }))
    }
    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<()>, Status> {
        let request = request.into_inner();

        let mut db_conn = self.db.get_db_connection_or_return_unavailable()?;
        delete_user(request.user_id, &mut db_conn).map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(()))
    }
}
