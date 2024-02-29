use std::env;
use std::sync::OnceLock;
use dotenv::dotenv;

use crate::error::{InternalError, InternalResult};

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        Config::load().unwrap_or_else(|ex| panic!("ERROR WHILE LOADING CONF: {ex:?}"))
    })
}

trait ConfigLoader {
    fn load() -> InternalResult<Self>
    where
        Self: Sized;
}

#[allow(non_snake_case)]
pub struct Config {
    pub DB: DbConfig,
    pub SERVER: ServerConfig,
}

impl ConfigLoader for Config {
    fn load() -> InternalResult<Config> {
        dotenv().ok();
        Ok(Config {
            DB: DbConfig::load()?,
            SERVER: ServerConfig::load()?,
        })
    }
}

#[allow(non_snake_case)]
pub struct DbConfig {
    pub TEST_URL: String,
    pub URL: String,
}

impl ConfigLoader for DbConfig {
    fn load() -> InternalResult<Self>
    where
        Self: Sized,
    {
        let port = get_env("DB_PORT")?;
        let host = get_env("DB_HOST")?;
        let user = get_env("DB_USER")?;
        let password = get_env("DB_PASSWORD")?;
        let name = get_env("DB_NAME")?;
        let protocol = get_env("DB_PROTOCOL")?;
        let url = format!("{protocol}://{user}:{password}@{host}:{port}/{name}");
        Ok(DbConfig {
            URL: url,
            TEST_URL: get_env("DB_TEST_URL")?,
        })
    }
}

#[allow(non_snake_case)]
pub struct ServerConfig {
    pub HOST: String,
    pub PORT: u16,
    pub SOCKET_ADDR: String,
}

impl ConfigLoader for ServerConfig {
    fn load() -> InternalResult<Self>
    where
        Self: Sized,
    {
        let port: u16 = get_env("SERVER_PORT")?
            .parse()
            .map_err(|_| InternalError::ConfigParseImpossible("Can not parse port to u16"))?;
        let host = get_env("SERVER_HOST")?;
        Ok(ServerConfig {
            SOCKET_ADDR: format!("{}:{port}", host.clone()),
            HOST: host,
            PORT: port,
        })
    }
}

fn get_env(name: &'static str) -> InternalResult<String> {
    env::var(name).map_err(|_| InternalError::ConfigMissingEnv(name))
}
