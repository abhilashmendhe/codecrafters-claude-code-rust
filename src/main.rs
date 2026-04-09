use clap::Parser;
use codecrafters_claude_code::{run, utils::{args_info::Args, config::Config}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Args::parse();

    dotenv::dotenv().ok();

    let base_url = std::env::var("OPENROUTER_BASE_URL")
        .unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_string());

    let api_key = std::env::var("OPENROUTER_API_KEY").unwrap_or_else(|_| {
        eprintln!("OPENROUTER_API_KEY is not set");
        std::process::exit(1);
    });

    let model_name = std::env::var("MODEL_NAME").unwrap_or_else(|_| "arcee-ai/trinity-mini:free".to_string());
    let config = Config::new(base_url, api_key, model_name);
    run(args, config).await?;
    Ok(())
}
