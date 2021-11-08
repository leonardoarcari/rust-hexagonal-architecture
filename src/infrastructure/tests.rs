use anyhow::Result;
use rocket::futures::TryStreamExt;
use shaku::ModuleBuilder;
use sqlx::{Executor, PgPool};
use std::{
    collections::HashMap,
    sync::{Arc, Once},
};
use testcontainers::*;
use tokio::sync::OnceCell;
use walkdir::{DirEntry, WalkDir};

use crate::infrastructure::container::{connect_db, default_module, HexagonalRocketModule};

/// Synchronization variable to run tests initialization only once, globally
static TESTS_INIT: Once = Once::new();

lazy_static! {
    /// Handle to Docker CLI. Global to all threads threads
    static ref DOCKER: clients::Cli = clients::Cli::default();
}

thread_local! {
    /// Handle to Postgres container. This is made thread-local in order to trigger
    /// Container's Drop implementation and delete the container on thread exit.
    static PG_CONTAINER: PgContainer<'static> = PgContainer::new(&DOCKER);

    /// `Once` guard to run db initialization only once
    static DB_INIT: Arc<OnceCell<()>> = Arc::new(OnceCell::const_new());
}

/// Run setup process for testing.
///
/// For the moment, just `env_logger` is initialized for logging
pub fn setup() {
    TESTS_INIT.call_once(|| {
        let env = env_logger::Env::default().default_filter_or("info");
        env_logger::init_from_env(env);
    })
}

/// Get a `ModuleBuilder` with default settings for testing purpose.
///
/// This includes a temporary Postgres instance, run with Docker, unique to the invoking thread.
pub async fn testing_module() -> ModuleBuilder<HexagonalRocketModule> {
    // Build testcontainer Postgres URI
    let uri = PG_CONTAINER.with(|c| c.uri());

    // Run pending migrations
    migrate_db(&uri).await;

    // Connect to Postgres testing container
    log::info!("Connecting to Postgres...");
    let db_pool = connect_db(&uri).await;
    log::info!("Connected to Postgres...");

    default_module(db_pool).await
}

#[derive(Debug)]
struct PgContainer<'d> {
    container: Container<'d, clients::Cli, images::postgres::Postgres>,
    inner_port: u16,
    user: String,
    password: String,
}

impl<'d> PgContainer<'d> {
    fn new(docker: &'d clients::Cli) -> Self {
        let user = "azulejos".to_owned();
        let password = "azulejos-pg-pwd".to_owned();
        let inner_port: u16 = 5432;

        let mut envs = HashMap::new();
        envs.insert("POSTGRES_USER".to_owned(), user.to_owned());
        envs.insert("POSTGRES_PASSWORD".to_owned(), password.to_owned());

        let postgres_image = images::postgres::Postgres::default().with_env_vars(envs);
        log::info!("Running Postgres instance...");
        let container = docker.run(postgres_image);

        Self {
            container,
            inner_port,
            user,
            password,
        }
    }

    fn host_port(&self) -> u16 {
        log::info!("Checking Postgres testing container...");
        let id = self.container.id();
        log::info!("Postgres testing container is running with id '{}'", id);

        self.container.get_host_port(self.inner_port).unwrap()
    }

    fn uri(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user,
            self.password,
            "localhost",
            self.host_port(),
            self.user
        )
    }
}

async fn migrate_db(uri: &str) {
    let migration_fut = DB_INIT.with(|db_init| {
        let uri = uri.to_owned();
        let db_init = db_init.clone();
        tokio::spawn(async move {
            db_init
                .get_or_init(|| async {
                    let uri = uri.to_owned();
                    log::info!("Starting Postgres migration process...");
                    let db_pool = connect_db(&uri).await;

                    log::info!("Running Postgres migrations...");
                    sqlx::migrate!("./migrations").run(&db_pool).await.unwrap();

                    log::info!("Load test data...");
                    load_test_data(&db_pool).await.unwrap();
                })
                .await;
        })
    });

    migration_fut.await.unwrap();
}

async fn load_test_data(pool: &PgPool) -> Result<()> {
    for entry in WalkDir::new("./tests/sql")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(is_sql_file)
    {
        let sql = std::fs::read_to_string(entry.path())?;
        let mut conn = pool.acquire().await?;
        let mut rows = conn.execute_many(sql.as_ref());

        while let Some(row) = rows.try_next().await? {
            log::info!("Inserted: {}", row.rows_affected())
        }
    }

    Ok(())
}

fn is_sql_file(e: &DirEntry) -> bool {
    e.path().is_file()
        && e.path()
            .extension()
            .map(|ext| ext.to_string_lossy().ends_with("sql"))
            .unwrap_or(false)
}
