use crate::{
    agent_funcs::{execute_tool::execute_tool, tools_array::tools_array},
    skills::process_extract_skills::process_extract_skills,
    utils::{args_info::Args, config::Config},
};

use async_openai::{Client, config::OpenAIConfig};
use serde_json::{Value, json};

pub async fn agent_loop(
    args: Args,
    config: Config,
    client: Client<OpenAIConfig>,
) -> Result<(), Box<dyn std::error::Error>> {
    // 1. read skill folder, process and extract
    let prompt = args.prompt;
    let (mut messages, skills) = process_extract_skills(&prompt)?;
    // println!("{:#?}", messages);
    // println!("{:#?}", skills);
    // return Ok(());
    // 2. run agent loop
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
            if let Some(content) = message["content"].as_str() {
                println!("{}", content);
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
    Ok(())
}
