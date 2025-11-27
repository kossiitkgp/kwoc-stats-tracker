use chrono::{DateTime, Utc};
use std::env;

pub struct Env {
    pub github_token: String,
    pub start_time: DateTime<Utc>,
    #[allow(unused)]
    pub mid_evals_time: DateTime<Utc>,
    pub end_evals_time: DateTime<Utc>,
    pub db_user: String,
    pub db_password: String,
    pub db_host: String,
    pub db_port: u16,
    pub db_name: String,
    pub mid_evals_ended: bool,
}

impl Env {
    pub fn new() -> Self {
        Self {
            github_token: env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN is not set"),
            start_time: env::var("START_TIME").expect("START_TIME is not set").parse::<DateTime<Utc>>().expect("START_TIME is not valid"),
            mid_evals_time: env::var("MID_EVALS_TIME").expect("MID_EVALS_TIME is not set").parse::<DateTime<Utc>>().expect("MID_EVALS_TIME is not valid"),
            end_evals_time: env::var("END_EVALS_TIME").expect("END_EVALS_TIME is not set").parse::<DateTime<Utc>>().expect("END_EVALS_TIME is not valid"),
            db_user: env::var("DATABASE_USERNAME").expect("DATABASE_USERNAME is not set"),
            db_password: env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD is not set"),
            db_host: env::var("DATABASE_HOST").expect("DATABASE_HOST is not set"),
            db_port: env::var("DATABASE_PORT").expect("DATABASE_PORT is not set").parse::<u16>().expect("DATABASE_PORT is not valid"),
            db_name: env::var("DATABASE_NAME").expect("DATABASE_NAME is not set"),
            mid_evals_ended: env::var("MID_EVALS_ENDED").expect("MID_EVALS_ENDED is not set").parse::<bool>().expect("MID_EVALS_ENDED is not valid"),
        }
    }
}