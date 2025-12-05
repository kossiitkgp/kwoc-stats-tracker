use http::StatusCode;

pub async fn send_slack_message(
    webhook_url: &str,
    message: &str,
) -> anyhow::Result<()> {
    if webhook_url.is_empty() {
        return Ok(());
    }

    let client = reqwest::Client::new();
    let response = client
        .post(webhook_url)
        .json(&serde_json::json!({ "text": message }))
        .send()
        .await?;

    if response.status() != StatusCode::OK {
        return Err(anyhow::anyhow!("Slack webhook failed with status code {}", response.status()));
    }

    Ok(())
}