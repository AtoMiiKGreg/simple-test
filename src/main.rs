use std::env;
use config::{Config, ConfigError, Environment};
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Setting {
    pub pg: String,
}

impl Setting {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}

    fn main() {
        // Charger les variables d'environnement en fonction de la valeur de APP_ENV
        match env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string()).as_str() {
            "dev" => dotenv::from_filename("env/.env.dev").ok(),
            "preprod" => dotenv::from_filename("env/.env.preprod").ok(),
            "prod" => dotenv::from_filename("env/.env.prod").ok(),
            _ => panic!("Invalid value for APP_ENV"),
        };

        // Utiliser les variables d'environnement chargées
        let pg_user = env::var("PG.USER").unwrap();
        let pg_password = env::var("PG.PASSWORD").unwrap();
        let host = env::var("HOST").unwrap();
        let db_name = env::var("PG.DBNAME").unwrap();

        println!("TEST pg user: {}",pg_user);
        println!("TEST pg password: {}",pg_password);
        println!("TEST host: {}",host);
        println!("TEST db nam: {}",db_name);
    }

 /*   let test = dotenv().ok().unwrap();
    println!("test dotenv {}", test.display());
    let host  = env::var("HOST").unwrap();
    println!("Test host {}", host);

    let config = Setting::from_env().unwrap();

    println!("Test: {}", config.pg)*/

