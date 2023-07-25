use once_cell::sync::Lazy;

static CONFIG: Lazy<Config> = Lazy::new(|| Config::new().expect("Unable to retrieve config"));

#[derive(Debug, Clone, Default)]
pub struct DbPool {
    pub min: Option<i16>,
    pub max: Option<i16>,
}

#[derive(Debug, Clone)]
pub struct Database {
    pub url: String,
    pub pool: DbPool,
}

#[derive(Debug, Clone)]
pub struct Auth {
    pub secret: String,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub run_mode: String,
    pub port: u16,
    pub database: Database,
    pub auth: Auth,
}

impl Config {
    pub fn new() -> anyhow::Result<Self> {
        let run_mode = std::env::var("RUN_MODE").unwrap_or_else(|_| "development".to_string());
        let port = 8080;
        let database = Database {
            url: std::env::var("DATABASE_URL")?,
            pool: DbPool::default(),
        };
        let auth = Auth {
            secret: std::env::var("SECRET")?,
        };

        Ok(Config {
            run_mode,
            port,
            database,
            auth,
        })
    }

    pub fn is_dev(&self) -> bool {
        self.run_mode == "development"
    }

    pub fn get() -> &'static Config {
        &CONFIG
    }
}
