use std::process::Command;

pub async fn execute_bash_tool(command: &str) -> Result<String, Box<dyn std::error::Error>> {
    // println!("{:?}", command);
    let ou = Command::new("sh").arg("-c").arg(command).output()?;
    Ok(String::from_utf8_lossy(&ou.stdout).to_string())
}
