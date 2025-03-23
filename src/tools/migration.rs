use may_postgres::{Client};
use serde_json::json;
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct PostgresConfig {
    host: String,
    port: u16,
    name: String,
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    name: String,
    port: u16,
    postgres: PostgresConfig,
}

fn get_all_sql(path: &str) -> Option<String> {
    let entries = fs::read_dir(path).unwrap();
    let mut sql_stmt = String::new();

    for entry in entries {
        let path = entry.unwrap().path();

        if path.is_dir() {
            get_all_sql(path.to_str().unwrap());
        } else {
            let stmt = fs::read_to_string(path).unwrap();
            sql_stmt += &stmt;
        }
    }

    Some(sql_stmt)
}

pub fn run_sql() {
    let config = fs::read_to_string("./config.json").unwrap();
    let app_config: AppConfig = serde_json::from_str(&config).unwrap();

    let db_url = format!(
        "postgresql://{}:{}@{}:{}/{}",
        app_config.postgres.username,
        app_config.postgres.password,
        app_config.postgres.host,
        app_config.postgres.port,
        app_config.postgres.name
    );

    let client = may_postgres::connect(&db_url).unwrap();
    let stmt = get_all_sql("./.project_build/db/postgres").unwrap();

    client.batch_execute(&stmt).unwrap();
}
