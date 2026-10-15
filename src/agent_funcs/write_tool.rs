use std::fs::write;
pub async fn write_tool(file_path: &str, contents: &str) -> Result<(), Box<dyn std::error::Error>> {
    write(file_path, contents)?;
    Ok(())
}
