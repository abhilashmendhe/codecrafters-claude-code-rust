use core::str;
pub async fn read_tool(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    // println!("Reading tool. file path: {}", file_path);
    let content = std::fs::read(file_path)?;
    Ok(String::from_utf8_lossy(&content).to_string())
}
