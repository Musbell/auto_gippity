use crate::apis::call_request::call_gpt;
use crate::helpers::command_line::PrintCommand;
use crate::models::general::llm::Message;
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::fs;

const CODE_TEMPLATE_PATH: &str = "sdfvsdf";
const EXEC_MAIN_PATH: &str = "vfd";
const API_SCHEMA_PATH: &str =
    "/Users/musabello/RustroverProjects/auto_gippity/schemas/api_schema.json";

// Extend the AI function to encourage specific output
pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_func_str = ai_func(func_input);

    // Extend the string to encourage only printing the output
    let msg = format!(
        "FUNCTION {} INSTRUCTION:  You are a function printer.\
                      You ONLY print the results of functions. \
                      Nothing else. No commentary. \
                      No other information. Just print the results of functions. \
                      Here is the input to the function: {}.\
                      Print out what the function returns.",
        ai_func_str, func_input,
    );

    // return message
    Message {
        role: "system".to_string(),
        content: msg,
    }
}

// Perform call to LLM GPT
pub async fn ai_task_request(
    msg_context: &str,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> String {
    // Extend AI function
    let extended_msg = extend_ai_function(function_pass, &msg_context);

    // Print current status
    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

    // Get LLM response
    let llm_response_res = call_gpt(vec![extended_msg.clone()]).await;

    // Return success or try again
    match llm_response_res {
        Ok(llm_response) => llm_response,
        Err(_) => call_gpt(vec![extended_msg.clone()])
            .await
            .expect("Failed to get LLM response"),
    }
}

// Perform call to LLM GPT - Decode
pub async fn ai_task_request_decoded<T: DeserializeOwned>(
    msg_context: &str,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> T {
    let llm_response =
        ai_task_request(msg_context, agent_position, agent_operation, function_pass).await;
    let decoded_response: T = serde_json::from_str(&llm_response.as_str())
        .expect("Failed tomdecode AI response from serde json");
    decoded_response
}

// Check wether request url is valid
pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response: reqwest::Response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}

// Get Code Template
pub fn read_code_templete_contents() -> String {
    let path: String = String::from(CODE_TEMPLATE_PATH);
    fs::read_to_string(path).expect("msg: Failed to read code template file")
}

// Save New Backend Code
pub fn save_backend_code(contents: &String) {
    let path = String::from(EXEC_MAIN_PATH);
    fs::write(path, contents).expect("msg: Failed to write main.rs file");
}

// Save JSON API Endpoint Schema
pub fn save_api_endpoints(api_endpoints: &String) {
    let path = String::from(API_SCHEMA_PATH);
    fs::write(path, api_endpoints).expect("msg: Failed to write api_schema.json file");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn tests_extending_ai_function() {
        let extended_msg = extend_ai_function(convert_user_input_to_goal, "I need a website that lets users login and logout. It needs to look fancy and accept payments.");
        assert_eq!(extended_msg.role, "system".to_string());
    }

    #[tokio::test]
    async fn test_ai_task_request() {
        let ai_func_param = "Build me a website for a school named SHAM_LAD MEMORIAL SCHOOL";
        let res = ai_task_request(
            ai_func_param,
            "Managing Agent",
            "Define user requirements",
            convert_user_input_to_goal,
        )
        .await;

        assert!(res.len() > 20);
    }
}
