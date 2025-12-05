use slack::send_slack_message;

mod db;
mod env;
mod github;
mod stats;
mod slack;

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

    // Normal stats run will only check for PRs that were merged before the mid evals deadline, when mid evals have not happened yet.
    let deadline = if env.mid_evals_ended {
        env.end_evals_time
    } else {
        env.mid_evals_time
    };

    let (project_count, pr_count) = stats::update_stats(&env, &database, &gh, deadline).await?;

    let msg = format!("[KWoC-Stats] Updated {} projects with {} new pull requests", project_count, pr_count);
    println!("{}", msg);
    let _ = send_slack_message(&env.slack_webhook_url, &msg).await;

    Ok(())
}
