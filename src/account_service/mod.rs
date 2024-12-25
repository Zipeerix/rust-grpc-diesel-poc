mod account_repository;
mod models;
pub mod request_handler;

pub mod proto {
    tonic::include_proto!("account_service");
    pub const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("account_service_descriptor");
}
