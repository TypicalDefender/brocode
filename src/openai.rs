use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Deserialize)]
struct ResponseChoice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<ResponseChoice>,
}

/// Generate a commit message using OpenAI API
pub async fn generate_commit_message(
    api_key: &str,
    model: &str,
    temperature: f32,
    max_tokens: u32,
    system_prompt: &str,
    diff: &str,
) -> Result<String> {
    let client = Client::new();
    
    // Prepare the prompt with the diff
    let user_prompt = format!("Here are the changes that need to be committed:\n```diff\n{}\n```\n\nPlease write a concise and descriptive commit message explaining what these changes do.", diff);
    
    let request = OpenAIRequest {
        model: model.to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: user_prompt,
            },
        ],
        temperature,
        max_tokens,
    };
    
    let response = client.post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .context("Failed to send request to OpenAI API")?;
    
    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(anyhow!("OpenAI API error: {}", error_text));
    }
    
    let response_data: OpenAIResponse = response.json().await
        .context("Failed to parse OpenAI API response")?;
    
    response_data.choices.first()
        .map(|choice| choice.message.content.clone())
        .ok_or_else(|| anyhow!("No completion choices returned from OpenAI"))
}
