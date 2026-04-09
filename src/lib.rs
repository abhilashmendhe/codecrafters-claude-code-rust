use async_openai::{Client, config::OpenAIConfig};
use serde_json::{Value, json};

use crate::utils::{args_info::Args, config::Config};

pub mod utils;

pub async fn run(args: Args, config: Config) -> Result<(), Box<dyn std::error::Error>> {

    let open_ai_config = OpenAIConfig::new()
                            .with_api_base(config.base_url())
                            .with_api_key(config.api_key());

    let client = Client::with_config(open_ai_config);

    #[allow(unused_variables)]
    let response: Value = client
        .chat()
        .create_byot(json!({
            "messages": [
                {
                    "role": "user",
                    "content": args.prompt
                }
            ],
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


    // TODO: Uncomment the lines below to pass the first stage
    if let Some(content) = response["choices"][0]["message"]["content"].as_str() {
        println!("{}", content);
    }

    Ok(())
}