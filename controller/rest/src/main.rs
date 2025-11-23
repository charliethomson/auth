use clap::{Parser, ValueEnum};
use poem::{
    EndpointExt, Route, Server,
    listener::TcpListener,
    middleware::{Cors, Tracing},
};
use poem_openapi::OpenApiService;
use tracing_subscriber::{Registry, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    api::{Api, ApiRepositories, DebugApi, ManageApi, SwaggerApi},
    services::ApiServices,
};

mod api;
mod models;
mod services;
mod util;

pub const PRODUCT_IDENTIFIER: &str = "dev.thmsn.auth.rest";

#[derive(Clone, Copy, Debug, ValueEnum)]
enum Environment {
    Local,
    Development,
    Production,
}

#[derive(Parser)]
struct Args {
    #[arg(value_enum, long, env, default_value_t = Environment::Local)]
    environment: Environment,

    #[arg(long, env, default_value_t = 8080)]
    port: u16,
    #[arg(long, env, default_value = "0.0.0.0")]
    address: String,

    #[arg(long, env, default_value = "thedefaultsigningkeyisverycool")]
    signing_key: String,

    #[arg(long, env)]
    database_url: String,

    #[arg(long, env)]
    hostname: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let build_info = libbuildinfo::load_build_info!().expect("Failed to load build info");
    let args = Args::parse();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer::<Registry>()
                .event_format(tracing_subscriber::fmt::format().pretty())
                .with_span_events(fmt::format::FmtSpan::ACTIVE)
                .with_target(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_file(true)
                .with_line_number(true),
        )
        .init();

    let swagger_uri = |api_prefix: &str| match args.environment {
        Environment::Local => {
            format!("http://{}:{}{}", args.hostname, args.port, api_prefix)
        }
        _ => {
            format!("https://{}{}", args.hostname, api_prefix)
        }
    };
    let bind_uri = format!("{}:{}", args.address, args.port);

    let services = ApiServices::new(&args, &build_info).await?;
    let repositories = ApiRepositories::new(&args, &build_info).await?;

    let version = build_info
        .package
        .version
        .clone()
        .unwrap_or("Unknown version".to_string());

    macro_rules! oai_service {
        ($e:expr,$t:ty) => {{
            let api_prefix = <$t as SwaggerApi>::base_uri();
            let api_docs_prefix = format!("{api_prefix}/docs");
            let api_spec_prefix = format!("{api_prefix}/openapi");
            let service = OpenApiService::new($e, <$t as SwaggerApi>::name(), &version)
                .server(swagger_uri(&api_prefix));
            let ui = service.scalar();
            let spec_service = service.spec_endpoint();

            (
                api_prefix,
                service,
                api_spec_prefix,
                spec_service,
                api_docs_prefix,
                ui,
            )
        }};
    }

    let (api_prefix, api_service, api_spec_prefix, api_spec_service, api_docs_prefix, api_ui) =
        oai_service!(Api, Api);
    let (
        manage_api_prefix,
        manage_api_service,
        manage_api_spec_prefix,
        manage_api_spec_service,
        manage_api_docs_prefix,
        manage_api_ui,
    ) = oai_service!(ManageApi, ManageApi);
    let (
        debug_api_prefix,
        debug_api_service,
        debug_api_spec_prefix,
        debug_api_spec_service,
        debug_api_docs_prefix,
        debug_api_ui,
    ) = oai_service!(DebugApi, DebugApi);

    Server::new(TcpListener::bind(bind_uri))
        .run(
            Route::new()
                .nest(api_prefix, api_service)
                .nest(api_docs_prefix, api_ui)
                .nest(api_spec_prefix, api_spec_service)
                .nest(manage_api_prefix, manage_api_service)
                .nest(manage_api_docs_prefix, manage_api_ui)
                .nest(manage_api_spec_prefix, manage_api_spec_service)
                .nest(debug_api_prefix, debug_api_service)
                .nest(debug_api_docs_prefix, debug_api_ui)
                .nest(debug_api_spec_prefix, debug_api_spec_service)
                .data(services)
                .data(repositories)
                .with(Tracing)
                .with(
                    Cors::new()
                        .allow_origin_regex(".*")
                        .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                        .allow_headers(vec!["Content-Type", "Authorization", "Accept"])
                        .allow_credentials(true)
                ),
        )
        .await?;

    Ok(())
}
