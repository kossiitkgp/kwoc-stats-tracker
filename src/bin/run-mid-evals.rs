use kwoc_stats_tracker::{db, env, github, stats, slack};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if dotenvy::dotenv().is_err() {
        println!("Failed to load .env file");
        return Ok(());
    }

    let env = env::Env::new();

    let database = db::Database::new(&env).await?;

    let gh = github::GitHub::new(&env)?;

    let (project_count, pr_count) =
        stats::update_stats(&env, &database, &gh, env.mid_evals_time).await?;

    let msg = format!(
        "[KWoC-Stats] Updated {} projects with {} new pull requests",
        project_count, pr_count
    );
    println!("{}", msg);
    let _ = slack::send_slack_message(&env.slack_webhook_url, &msg).await;

    let mut students = database.get_students().await?;
    for student in &mut students {
        if student.pull_count > 0 {
            println!(
                "✅ {} has {} pull requests",
                student.username, student.pull_count
            );
            student.passed_mid_evals = true;
        } else {
            println!("❌ {} has no pull requests", student.username);
        }
    }

    println!("\nDo you want to update the database? (y/n)");
    let mut input = String::new();
    let mut passed_students = vec![];
    std::io::stdin().read_line(&mut input)?;
    if input.trim() == "y" {
        for student in &mut students {
            if student.passed_mid_evals {
                println!("✅ {} passed the mid-evals", student.username);
                passed_students.push(student.username.clone());
                database.update_student(student.clone()).await?;
            }
        }
    }

    let msg = format!(
        "[KWoC-Stats] Mid Evaluation completed, {} students passed the mid-evals:\n{}",
        passed_students.len(),
        passed_students.join(", ")
    );
    println!("{}", msg);
    let _ = slack::send_slack_message(&env.slack_webhook_url, &msg).await;

    Ok(())
}
