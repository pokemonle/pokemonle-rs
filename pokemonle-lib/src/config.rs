use crate::prelude::*;
use dotenvy::dotenv;
use tracing::info;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,

    pub crypto_key: String,
}

const ENV_VAR_DATABASE_URL: &str = "DATABASE_URL";
const ENV_VAR_CRYPTO_KEY: &str = "CRYPTO_KEY";

#[inline]
fn read_and_return_env_var(env_var: &'static str) -> Result<String> {
    let res =
        std::env::var(env_var).map_err(|err| Error::ConfigReadNonExistEnvVar(env_var, err))?;

    match res.is_empty() {
        true => Err(Error::ConfigReadEmptyEnvVar(env_var)),
        false => Ok(res),
    }
}

impl Config {
    pub fn new() -> Result<Config> {
        match dotenv() {
            Ok(_) => info!("Loaded .env file"),
            Err(_) => info!("No .env file found"),
        }

        let database_url =
            read_and_return_env_var(ENV_VAR_DATABASE_URL).unwrap_or("pokemonle.db".to_string());

        let crypto_key =
            read_and_return_env_var(ENV_VAR_CRYPTO_KEY).unwrap_or("123456".to_string());

        Ok(Config {
            database_url,
            crypto_key,
        })
    }
}
