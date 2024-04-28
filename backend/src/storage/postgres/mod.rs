use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;
use tracing::{event, Level};

use self::config::PostgresConfig;

pub mod config;

pub struct PostgresStorage {
    pub pool: Pool,
}
impl PostgresStorage {
    pub async fn new(config: PostgresConfig) -> anyhow::Result<Self> {
        let mut pg_config = tokio_postgres::Config::new();
        pg_config.host(&config.host);
        pg_config.port(config.port);
        pg_config.user(&config.user);
        pg_config.password(&config.password);
        pg_config.dbname(&config.dbname);
        pg_config.application_name("tcc");
        let mgr_config = ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        };
        let tls_connector = TlsConnector::new()?;
        let connector = MakeTlsConnector::new(tls_connector);
        let mgr = Manager::from_config(pg_config, connector, mgr_config);
        let pool = Pool::builder(mgr).max_size(20).build().unwrap();

        let client = pool
            .get()
            .await
            .expect("should be able to get another connection");

        // check connection is working
        client.query_one("SELECT 1", &[]).await?;

        event!(Level::INFO, "Connection is working");

        Ok(Self { pool })
    }

    pub async fn get_connection(&self) -> anyhow::Result<deadpool_postgres::Client> {
        let client = self.pool.get().await?;
        Ok(client)
    }
}
