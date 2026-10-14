use crate::{
    agent_funcs::agent_loop::agent_loop,
    utils::{args_info::Args, config::Config},
};
use async_openai::{Client, config::OpenAIConfig};

pub mod agent_funcs;
pub mod utils;

pub async fn run(args: Args, config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let open_ai_config = OpenAIConfig::new()
        .with_api_base(config.base_url())
        .with_api_key(config.api_key());

    let client = Client::with_config(open_ai_config);
    agent_loop(args, config, client).await?;
    Ok(())
}
