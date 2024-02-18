use axum::{
    extract::{Query, State}, http::StatusCode, response::Json, routing::{get, post}, Router
};
use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::{net::SocketAddr, time::Duration};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tokio::signal;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

table! {
    log_items (ts) {
        ts -> TimestamptzSqlite,
        device_id -> Text,
    }
}

#[derive(serde::Serialize, Selectable, Queryable)]
#[diesel(check_for_backend(Sqlite))]
struct LogItem {
    ts: time::OffsetDateTime,
    device_id: String,
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = log_items, check_for_backend(Sqlite))]
struct LogItemAppend {
    device_id: String,
}

#[derive(serde::Serialize)]
struct LogItemAppendResponse {
    status: String,
}

#[derive(serde::Deserialize)]
struct Pagination {
    size: i64,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = std::env::var("DATABASE_URL").unwrap();

    // set up connection pool
    let manager = deadpool_diesel::sqlite::Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    let pool = deadpool_diesel::sqlite::Pool::builder(manager)
        .build()
        .unwrap();

    // run the migrations on server startup
    {
        let conn = pool.get().await.unwrap();
        conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
            .await
            .unwrap()
            .unwrap();
    }

    let app = Router::new()
        .route("/", get(root))
        .route("/log", get(get_logs))
        .route("/log", post(add_log))
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::new(Duration::from_secs(5))
        ))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], 8080)))
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, world!"
}

async fn get_logs(
    State(pool): State<deadpool_diesel::sqlite::Pool>,
    Query(pagination): Query<Pagination>
) -> Result<Json<Vec<LogItem>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(move |conn|
            log_items::table
            .select(LogItem::as_select())
            .order_by(log_items::ts.desc())
            .limit(pagination.size)
            .load(conn)
        )
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

async fn add_log(
    State(pool): State<deadpool_diesel::sqlite::Pool>,
    Json(body): Json<LogItemAppend>
) -> Result<Json<LogItemAppendResponse>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    conn
        .interact(|conn| {
            diesel::insert_into(log_items::table)
                .values(body)
                .execute(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(LogItemAppendResponse { status: "ok".to_string() }))
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

/// Utility to implement ctrl-c shutdown
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}