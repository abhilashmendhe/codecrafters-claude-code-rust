use async_openai::{Client, config::OpenAIConfig};
use serde_json::{Value, json};

use crate::{
    agent_funcs::{execute_tool::execute_tool, tools_array::tools_array},
    utils::config::Config,
};

pub async fn create_agent(
    config: Config,
    client: Client<OpenAIConfig>,
    messages: &mut Vec<Value>,
    skills: std::collections::HashMap<String, crate::skills::skill_config::SkillConfig>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut content = String::new();
    loop {
        let response: Value = client
            .chat()
            .create_byot(json!({
                "messages": messages,
                "model": config.model_name(),
                "tools": tools_array()
            }))
            .await?;

        let choices = &response["choices"];
        let first_choice = &choices[0];
        let message = &first_choice["message"];
        messages.push(message.clone());

        let tool_calls = message.get("tool_calls").and_then(|v| v.as_array());
        if tool_calls.is_none() {
            // println!("\nNo tool calls\n");
            // Handle plain-text response
            if let Some(cont) = message["content"].as_str() {
                // println!("{}", content);
                content.push_str(cont);
            }
            break;
        }

        let tool_calls = tool_calls.unwrap();
        for (_i, tool) in tool_calls.into_iter().enumerate() {
            // Execute tool
            let content = execute_tool(tool, &skills).await?;
            let tool_call_id = tool["id"].as_str().unwrap();
            messages.push(json!({
                "role": "tool",
                "tool_call_id": tool_call_id,
                "content": content
            }));
        }
    }
    Ok(content)
}
