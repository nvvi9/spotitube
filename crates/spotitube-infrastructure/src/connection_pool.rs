use spotitube_core::errors::SpotitubeResult;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tracing::info;

pub type SpotitubeConnectionPool = Pool<Postgres>;

pub struct SpotitubeConnectionPoolManager;

impl SpotitubeConnectionPoolManager {
    pub async fn new_pool(
        database_url: &str,
        run_migations: bool,
    ) -> SpotitubeResult<SpotitubeConnectionPool> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await?;

        if run_migations {
            info!("running migrations...");
            sqlx::migrate!().run(&pool).await?;
        }

        Ok(pool)
    }
}
