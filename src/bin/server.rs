#[macro_use]
extern crate tracing;

use akashi_api::app::App;
use akashi_api::config::DatabaseConfig;
use akashi_api::database::PgDbClient;
use anyhow::Context;
use sqlx::postgres::PgConnectOptions;
use sqlx::{ConnectOptions, PgPool};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::signal::windows::ctrl_c;
use tracing::log::LevelFilter;

const CORE_THREADS: usize = 2;

fn main() -> anyhow::Result<()> {
    akashi_api::util::tracing::init();

    let config = akashi_api::config::Server::from_environment()?;

    let mut builder = tokio::runtime::Builder::new_multi_thread();
    builder.enable_all();
    builder.worker_threads(CORE_THREADS);
    if let Some(threads) = config.max_blocking_threads {
        builder.max_blocking_threads(threads);
    }

    let rt = builder.build()?;

    rt.block_on(async_main(config))
}

async fn async_main(config: akashi_api::config::Server) -> anyhow::Result<()> {
    let _span = info_span!("server.run");

    let database = initialize_db(&config.db_config).await?;

    let app = Arc::new(App::new(config, database));

    let axum_router = akashi_api::build_handler(app.clone());
    let make_service = axum_router.into_make_service_with_connect_info::<SocketAddr>();

    // Create a `TcpListener` using tokio.
    let listener = TcpListener::bind((app.config.ip, app.config.port)).await?;
    let addr = listener.local_addr()?;

    // Do not change this line! Removing the line or changing its contents in any way will break
    // the test suite :)
    info!("Listening at http://{addr}");

    // Run the server with graceful shutdown
    axum::serve(listener, make_service)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("Server has gracefully shutdown!");
    Ok(())
}

// TODO: make use of the max_pool_size config
async fn initialize_db(config: &DatabaseConfig) -> anyhow::Result<PgDbClient> {
    let mut opts: PgConnectOptions = config.url.parse()?;
    opts = opts.log_statements(LevelFilter::Trace);
    let db = PgPool::connect_with(opts)
        .await
        .context("Cannot connect to database")?;

    sqlx::migrate!()
        .run(&db)
        .await
        .context("Cannot run database migrations")?;

    Ok(PgDbClient::new(db))
}

async fn shutdown_signal() {
    let interrupt = async {
        ctrl_c()
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    let terminate = async {
        ctrl_c()
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = interrupt => {},
        _ = terminate => {},
    }
}
