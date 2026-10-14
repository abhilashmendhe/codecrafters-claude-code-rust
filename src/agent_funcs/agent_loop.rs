use crate::utils::{args_info::Args, config::Config};

use async_openai::{Client, config::OpenAIConfig};
use serde::Deserialize;
use serde_json::{Value, json};

pub async fn agent_loop(
    args: Args,
    config: Config,
    client: Client<OpenAIConfig>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut messages = vec![json!({
        "role": "user",
        "content": args.prompt
    })];
    
    loop {
        let response: Value = client
            .chat()
            .create_byot(json!({
                "messages": messages,
                "model": config.model_name(),
                "tools": [
                    {
                        "type": "function",
                        "function": {
                            "name": "Read",
                            "description": "Read and return the contents of a file",
                            "parameters": {
                            "type": "object",
                            "properties": {
                                "file_path": {
                                "type": "string",
                                "description": "The path to the file to read"
                                }
                            },
                            "required": ["file_path"]
                            }
                        }
                    }
                ]
            }))
            .await?;
        println!("{:#?}", response);
        break;
        let choices = &response["choices"];
        let first_choice = &choices[0];
        let message = &first_choice["message"];
        
        let tool_calls = message.get("tool_calls").and_then(|v| v.as_array());
        if tool_calls.is_none() {
            // Handle plain-text response
            if let Some(content) = message["content"].as_str() {
                println!("{}", content);
            }
            break;
        }
        let tool_calls = tool_calls.unwrap();
        let tool_call = &tool_calls[0];
        let _tool_call_id = &tool_call["id"].as_str();
        let _tool_call_type = &tool_call["type"].as_str();
        let tool_call_function = &tool_call["function"];
        let _func_name = &tool_call_function["name"].as_str();
        if let Some(func_args) = tool_call_function["arguments"].as_str() {
            #[derive(Deserialize, Debug)]
            struct FuncParam {
                file_path: String,
            }
            let fp = serde_json::from_str::<FuncParam>(func_args)?;
            let output = std::fs::read_to_string(fp.file_path)?;
            println!("{}", output);
        }
        
    }
    // #[allow(unused_variables)]
    Ok(())
}
