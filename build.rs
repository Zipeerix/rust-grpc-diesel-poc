use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=migrations");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .build_server(true)
        .file_descriptor_set_path(out_dir.join("account_service_descriptor.bin"))
        .compile_protos(
            &[
                "proto/account-service/account-service.proto",
                "proto/account-service/request.proto",
                "proto/account-service/response.proto",
                "proto/account-service/error.proto",
            ],
            &["proto"],
        )?;
    tonic_build::configure()
        .build_server(true)
        .file_descriptor_set_path(out_dir.join("order_service_descriptor.bin"))
        .compile_protos(
            &[
                "proto/order-service/order-service.proto",
                "proto/order-service/request.proto",
                "proto/order-service/response.proto",
                "proto/order-service/error.proto",
                "proto/order-service/order.proto",
            ],
            &["proto"],
        )?;

    Ok(())
}
