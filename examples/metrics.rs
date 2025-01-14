use opentelemetry::sdk::trace::Tracer;
use opentelemetry_jaeger::Uninstall;
use opentelemetry_surf::OpenTelemetryTracingMiddleware;

const SVC_NAME: &str = env!("CARGO_CRATE_NAME");

mod shared;

#[async_std::main]
async fn main() -> std::result::Result<(), http_types::Error> {
    femme::with_level(femme::LevelFilter::Info);
    shared::init_global_propagator();

    let (tracer, _uninstall) = pipeline();
    let otel_mw = OpenTelemetryTracingMiddleware::new(tracer);
    let client = create_client().with(otel_mw);

    // let res = client.get("https://httpbin.org/get").await?;
    // let res = client.get("https://httpbin.org/image/svg").await?;
    // let res = client.get("https://httpbin.org/drip?duration=3&numbytes=5&code=200&delay=1").await?;
    let res = client.get("https://httpbin.org/image/jpeg").await?;
    dbg!(res);
    Ok(())
}

fn pipeline() -> (Tracer, Uninstall) {
    opentelemetry_jaeger::new_pipeline()
        .with_service_name(SVC_NAME)
        .install()
        .expect("pipeline install failure")
}

// more custom http client setup: use isahc with metrics enabled
fn create_client() -> surf::Client {
    use http_client::isahc::IsahcClient;
    use isahc::prelude::*;

    let isahc = HttpClient::builder()
        .default_headers(&[("user-agent", "surf/isahc (with request metrics)")])
        .metrics(true)
        .build()
        .expect("isahc client could no be created");
    let http_client = IsahcClient::from_client(isahc);
    surf::Client::with_http_client(http_client)
}
