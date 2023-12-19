use crate::models::general::llm::{APIResponse, ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::Client;
use std::env;

use reqwest::header::{HeaderMap, HeaderValue};

// Call Large Language Model (i.e. GPT-4)
pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    dotenv().ok();

    // Extract API Key information
    let api_key = env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY  must be set in .env file");
    let api_org = env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG must be set in .env file");

    // let url = "https://api.openai.com/v1/chat/completions";
    let url = "http://localhost:1234/v1/chat/completions ";

    // Create headers
    let mut headers = HeaderMap::new();

    // Create Api key header
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    // Create API Key Org header
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(&api_org)
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    // Create client
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // Create chat completion
    let chat_completion = ChatCompletion {
        model: "gpt-3.5-turbo".to_string(),
        messages,
        temperature: 0.1,
    };

    // Extract API response
    let res: APIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    //Send response
    Ok(res.choices[0].message.content.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_call_gpt() {
        let message = Message {
            role: "user".to_string(),
            content: "Where are you from? Give me a very short response".to_string(),
        };

        let messages = vec![message];

        let res = call_gpt(messages).await;

        if let Ok(response) = res {
            println!("{}", response);
        } else {
            println!("Error");
        }
    }
}
