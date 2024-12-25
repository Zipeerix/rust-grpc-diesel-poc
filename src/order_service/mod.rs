mod models;
mod order_repository;
pub mod request_handler;

pub mod proto {
    tonic::include_proto!("order_service");
    pub const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("order_service_descriptor");
}
