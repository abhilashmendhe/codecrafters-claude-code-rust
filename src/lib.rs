use crate::{
    agent_funcs::init_agents::init_agents,
    utils::{args_info::Args, config::Config},
};
use async_openai::{Client, config::OpenAIConfig};

pub mod agent_funcs;
pub mod skills;
pub mod utils;
pub async fn run(args: Args, config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let open_ai_config = OpenAIConfig::new()
        .with_api_base(config.base_url())
        .with_api_key(config.api_key());

    let client = Client::with_config(open_ai_config);
    init_agents(args, config, client).await?;
    Ok(())
}
