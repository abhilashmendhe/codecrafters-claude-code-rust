use serde::Deserialize;
use serde_json::Value;

use crate::agent_funcs::read_tool::read_tool;

pub async fn execute_tool(tool: &Value) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(tool_call_type) = tool["type"].as_str() {
        if tool_call_type.eq("function") {
            let tool_call_function = &tool["function"];
            let func_name = tool_call_function["name"].as_str().unwrap();

            if func_name.eq("Read") {
                if let Some(func_args) = tool_call_function["arguments"].as_str() {
                    
                    #[derive(Deserialize, Debug)]
                    struct FuncParam {
                        file_path: String,
                    }
                    let fp = serde_json::from_str::<FuncParam>(func_args)?;
                    let content = read_tool(&fp.file_path).await?;
                    return Ok(content);
                }
            }
        }
    }
    Ok("".to_string())
}
