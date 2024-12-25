use crate::account_service::proto::account_service_server::AccountServiceServer;
use crate::account_service::request_handler::AccountServiceHandler;
use crate::config::Configuration;
use crate::database::connect_to_database_and_run_migrations;
use crate::order_service::proto::order_service_server::OrderServiceServer;
use crate::order_service::request_handler::OrderServiceHandler;
use crate::{account_service, metrics_server, order_service};
use log::info;
use std::net::SocketAddr;
use std::str::FromStr;
use tonic::transport::Server;
use tonic::{Request, Status};

fn request_interceptor(req: Request<()>) -> Result<Request<()>, Status> {
    metrics_server::GRPC_REQUEST_COUNTER.inc();
    Ok(req)
}

pub async fn start_grpc_server(config: &Configuration) -> Result<(), Box<dyn std::error::Error>> {
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<OrderServiceServer<OrderServiceHandler>>()
        .await;
    health_reporter
        .set_serving::<AccountServiceServer<AccountServiceHandler>>()
        .await;

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(order_service::proto::FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(account_service::proto::FILE_DESCRIPTOR_SET)
        .build_v1()
        .expect("Failed to create reflection service");

    let raw_server_address = config.server.get_address();
    let server_address = SocketAddr::from_str(&raw_server_address).unwrap_or_else(|_| {
        panic!(
            "Unable to parse grpc server address: {}",
            &raw_server_address
        )
    });

    let db_pool = connect_to_database_and_run_migrations(config.general.database_timeout);

    let order_service = OrderServiceServer::new(OrderServiceHandler::new(db_pool.clone()));
    let account_service = AccountServiceServer::new(AccountServiceHandler::new(db_pool.clone()));

    info!("Starting gRPC server at http://{}", &server_address);

    Server::builder()
        .layer(tonic::service::interceptor(request_interceptor))
        .add_service(reflection_service)
        .add_service(health_service)
        .add_service(order_service)
        .add_service(account_service)
        .serve(server_address)
        .await?;

    Ok(())
}
