use std::env;
use std::time::Duration;

use sea_orm::prelude::Uuid;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

#[derive(Clone, Debug, Default)]
pub struct UroborosOauthSuperadminOptions {
    pub login: String,
    pub password: String,
}

impl UroborosOauthSuperadminOptions {
    pub fn from_env() -> Option<Self> {
        env::var("UROBOROS_SUPERADMIN_LOGIN")
            .ok()
            .map(|login| UroborosOauthSuperadminOptions {
                login,
                password: env::var("UROBOROS_SUPERADMIN_PASSWORD")
                    .unwrap_or(Uuid::default().to_string()),
            })
    }
}

#[derive(Clone, Debug)]
pub struct UroborosOauthSeverOptions {
    pub host: String,
    pub port: usize,
}

impl UroborosOauthSeverOptions {
    pub fn from_env() -> Self {
        UroborosOauthSeverOptions {
            host: env::var("UROBOROS_SERVER_HOST").unwrap_or("127.0.0.1".to_string()),
            port: env::var("UROBOROS_SERVER_PORT")
                .map(|p| p.parse::<usize>().unwrap_or(3000))
                .unwrap_or(3000),
        }
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", &self.host, &self.port)
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct UroborosOauthState {
    pub postgres: DatabaseConnection,
    pub server_options: UroborosOauthSeverOptions,
    pub superadmin_options: Option<UroborosOauthSuperadminOptions>,
}

impl UroborosOauthState {
    pub async fn from_env() -> Self {
        let postgres_url = env::var("POSTGRES_URL").unwrap_or(String::from(
            "postgres://uroboros:uroboros@127.0.0.1:5432/uroboros",
        ));

        let mut opt = ConnectOptions::new(postgres_url);

        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .set_schema_search_path("public")
            .sqlx_logging(true);

        let postgres = Database::connect(opt).await.unwrap();

        Self {
            postgres,
            server_options: UroborosOauthSeverOptions::from_env(),
            superadmin_options: UroborosOauthSuperadminOptions::from_env(),
        }
    }
}
