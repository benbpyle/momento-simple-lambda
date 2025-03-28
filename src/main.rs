use lambda_http::{run, service_fn, Error};
mod http_handler;
use http_handler::function_handler;
use opentelemetry::propagation::TextMapPropagator;
use opentelemetry_datadog::{new_pipeline, ApiVersion};
use serde::{Deserialize, Serialize};
use std::env;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Registry};


#[tokio::main]
async fn main() -> Result<(), Error> {
    // let telemetry_layer = tracing_opentelemetry::layer().with_tracer(init_datadog_pipeline());
    // let fmt_layer = tracing_subscriber::fmt::layer()
    //     .json()
    //     .with_target(false)
    //     .with_current_span(false)
    //     .without_time();
    //
    // Registry::default()
    //     .with(telemetry_layer)
    //     .with(fmt_layer)
    //     .with(tracing_subscriber::EnvFilter::from_default_env())
    //     .init();
    // Initialize the Lambda runtime and add OpenTelemetry tracing
    run(service_fn(function_handler)).await

}

fn init_datadog_pipeline() -> opentelemetry_sdk::trace::Tracer {
    let agent_address = env::var("AGENT_ADDRESS").expect("AGENT_ADDRESS is required");
    match new_pipeline()
        .with_service_name(env::var("FUNCTION_NAME").expect("FUNCTION_NAME is required"))
        .with_agent_endpoint(format!("http://{}:8126", agent_address))
        .with_api_version(ApiVersion::Version05)
        .install_simple()
    {
        Ok(a) => a,
        Err(e) => {
            panic!("error starting! {}", e);
        }
    }
}

