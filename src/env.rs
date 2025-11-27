use chrono::{DateTime, Utc};

pub struct Env {
    pub github_token: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub db_user: String,
    pub db_password: String,
    pub db_host: String,
    pub db_port: u16,
    pub db_name: String,
}

impl Env {
    pub fn new() -> Self {
        Self {
            github_token: std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN is not set"),
            start_time: std::env::var("START_TIME").expect("START_TIME is not set").parse::<DateTime<Utc>>().expect("START_TIME is not valid"),
            end_time: std::env::var("END_TIME").expect("END_TIME is not set").parse::<DateTime<Utc>>().expect("END_TIME is not valid"),
            db_user: std::env::var("DATABASE_USERNAME").expect("DATABASE_USERNAME is not set"),
            db_password: std::env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD is not set"),
            db_host: std::env::var("DATABASE_HOST").expect("DATABASE_HOST is not set"),
            db_port: std::env::var("DATABASE_PORT").expect("DATABASE_PORT is not set").parse::<u16>().expect("DATABASE_PORT is not valid"),
            db_name: std::env::var("DATABASE_NAME").expect("DATABASE_NAME is not set"),
        }
    }
}