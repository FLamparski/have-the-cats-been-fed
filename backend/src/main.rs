use axum::{
    extract::State, http::StatusCode, routing::{get, post}, Router,
    response::Json
};
use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], 8080)))
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, world!"
}

async fn get_logs(
    State(pool): State<deadpool_diesel::sqlite::Pool>
) -> Result<Json<Vec<LogItem>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn| log_items::table.select(LogItem::as_select()).load(conn))
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
