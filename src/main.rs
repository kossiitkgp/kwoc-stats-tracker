mod db;
mod env;
mod github;
mod stats;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if dotenvy::dotenv().is_err() {
        println!("Failed to load .env file");
        return Ok(());
    }

    let env = env::Env::new();

    let now = chrono::Utc::now();
    if now < env.start_time {
        println!("Current time is before start time");
        return Ok(());
    }

    let database = db::Database::new(&env).await?;

    let gh = github::GitHub::new(&env)?;

    let (project_count, pr_count) = stats::update_stats(&env, &database, &gh, env.end_evals_time).await?;

    println!("Updated {} projects with {} new pull requests", project_count, pr_count);

    Ok(())
}
