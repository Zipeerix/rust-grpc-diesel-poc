use crate::account_service_handler::AccountServiceHandler;
use crate::config::Configuration;
use crate::database::connect_to_database_and_run_migrations;
use crate::grpc_server::account_service::account_service_server::AccountServiceServer;
use crate::grpc_server::order_service::order_service_server::OrderServiceServer;
use crate::metrics_server;
use crate::order_service_handler::OrderServiceHandler;
use log::info;
use std::net::SocketAddr;
use std::str::FromStr;
use tonic::transport::Server;
use tonic::{Request, Status};

pub mod order_service {
    tonic::include_proto!("order_service");
    pub const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("order_service_descriptor");
}

pub mod account_service {
    tonic::include_proto!("account_service");
    pub const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("account_service_descriptor");
}

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
        .register_encoded_file_descriptor_set(order_service::FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(account_service::FILE_DESCRIPTOR_SET)
        .build_v1()
        .unwrap();

    let raw_server_address = config.server.get_address();
    let server_address = SocketAddr::from_str(&raw_server_address).expect(&format!(
        "Unable to parse grpc server address: {}",
        &raw_server_address
    ));

    let db_conn = connect_to_database_and_run_migrations();

    let order_service = OrderServiceServer::new(OrderServiceHandler::new());
    let account_service = AccountServiceServer::new(AccountServiceHandler::new());

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
