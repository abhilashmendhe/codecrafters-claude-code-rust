use serde::Deserialize;
use serde_json::Value;

use crate::agent_funcs::{read_tool::read_tool, write_tool::write_tool};

pub async fn execute_tool(tool: &Value) -> Result<String, Box<dyn std::error::Error>> {
    // println!("tool: {:#?}\n", tool);
    if let Some(tool_call_type) = tool["type"].as_str() {
        if tool_call_type.eq("function") {
            let tool_call_function = &tool["function"];
            let func_name = tool_call_function["name"].as_str().unwrap();
            // println!("{:#?}", tool_call_function);
            if func_name.eq("Read") {
                if let Some(func_args) = tool_call_function["arguments"].as_str() {
                    #[derive(Deserialize, Debug)]
                    struct FuncParam {
                        file_path: String,
                    }
                    let fp = serde_json::from_str::<FuncParam>(func_args)?;
                    let r_content = read_tool(&fp.file_path).await?;
                    return Ok(r_content);
                }
            } else if func_name.eq("Write") {
                // println!("Implementing write tool");
                if let Some(func_args) = tool_call_function["arguments"].as_str() {
                    #[derive(Deserialize, Debug)]
                    struct FuncParam {
                        file_path: String,
                        content: String,
                    }
                    let fp = serde_json::from_str::<FuncParam>(func_args)?;
                    write_tool(&fp.file_path, &fp.content).await?;
                }
            }
        }
    }
    Ok("".to_string())
}
