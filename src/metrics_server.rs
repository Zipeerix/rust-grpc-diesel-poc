use hyper::{
    header::CONTENT_TYPE,
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use prometheus::{register_histogram_vec, Counter, Encoder, HistogramVec, TextEncoder};
use std::net::SocketAddr;
use std::str::FromStr;

use crate::config::Configuration;
use lazy_static::lazy_static;
use log::info;
use prometheus::{labels, opts, register_counter};

//FOR UPGRADING TO 1.5
//https://hyper.rs/guides/1/upgrading/
//https://github.com/hyperium/hyper/discussions/3471

lazy_static! {
    static ref METRICS_GET_LATENCY: HistogramVec = register_histogram_vec!(
        "metrics_request_duration_seconds",
        "Prometheus metrics request latencies in seconds.",
        &["handler"]
    )
    .unwrap();
    pub static ref GRPC_REQUEST_COUNTER: Counter = register_counter!(opts!(
        "grpc_requests_total",
        "Number of gRPC requests made.",
    ))
    .unwrap();
}

async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let encoder = TextEncoder::new();

    let metrics_get_timer = METRICS_GET_LATENCY
        .with_label_values(&["all"])
        .start_timer();

    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();

    let response = Response::builder()
        .status(200)
        .header(CONTENT_TYPE, encoder.format_type())
        .body(Body::from(buffer))
        .unwrap();

    // TODO: DONT  UNWRAP ABOVE, HANDLE ERROR

    metrics_get_timer.observe_duration();

    Ok(response)
}

pub async fn start_metrics_server(config: &Configuration) -> Result<(), hyper::Error> {
    let raw_address = config.metrics.get_address();
    let address = SocketAddr::from_str(&raw_address).expect(&format!(
        "Unable to parse metrics server address: {}",
        raw_address
    ));

    info!("Starting metrics server at http://{}", &address);

    let serve_future = Server::bind(&address).serve(make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(serve_req))
    }));

    serve_future.await
}
