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
    pub SQL_DB: SqlDbConfig,
    pub MONGO_DB: MongoDbConfig,
    pub SERVER: ServerConfig,
}

impl ConfigLoader for Config {
    fn load() -> InternalResult<Config> {
        dotenv().ok();
        Ok(Config {
            SQL_DB: SqlDbConfig::load()?,
            MONGO_DB: MongoDbConfig::load()?,
            SERVER: ServerConfig::load()?,
        })
    }
}

#[allow(non_snake_case)]
pub struct SqlDbConfig {
    pub TEST_URL: String,
    pub URL: String,
}

impl ConfigLoader for SqlDbConfig {
    fn load() -> InternalResult<Self>
    where
        Self: Sized,
    {
        let port = get_env("SQL_DB_PORT")?;
        let host = get_env("SQL_DB_HOST")?;
        let user = get_env("SQL_DB_USER")?;
        let password = get_env("SQL_DB_PASSWORD")?;
        let name = get_env("SQL_DB_NAME")?;
        let protocol = get_env("SQL_DB_PROTOCOL")?;
        let url = format!("{protocol}://{user}:{password}@{host}:{port}/{name}");
        Ok(SqlDbConfig {
            URL: url,
            TEST_URL: get_env("SQL_DB_TEST_URL")?,
        })
    }
}

#[allow(non_snake_case)]
pub struct MongoDbConfig {
    pub URL: String, 
    pub NAME: String,
    pub TEST_URL: String
}

impl ConfigLoader for MongoDbConfig {
    fn load() -> InternalResult<Self> where Self: Sized {
        let port = get_env("MONGO_DB_PORT")?;
        let host = get_env("MONGO_DB_HOST")?;
        
        let url = format!("mongodb://{host}:{port}");


        Ok(MongoDbConfig {
            URL: url,
            NAME: get_env("MONGO_DB_NAME")?,
            TEST_URL: "".to_string(),
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
            .map_err(|_| InternalError::ConfigParseImpossible("PORT"))?;
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