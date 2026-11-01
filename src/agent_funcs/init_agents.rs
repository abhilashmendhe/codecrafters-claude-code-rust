use crate::{
    agent_funcs::create_agent::create_agent,
    skills::process_extract_skills::process_extract_skills,
    utils::{args_info::Args, config::Config},
};

use async_openai::{Client, config::OpenAIConfig};

pub async fn init_agents(
    args: Args,
    config: Config,
    client: Client<OpenAIConfig>,
) -> Result<(), Box<dyn std::error::Error>> {
    // 1. read skill folder, process and extract
    let prompt = args.prompt;
    let (mut messages, skills) = process_extract_skills(&prompt)?;
    // println!("{:#?}", messages);
    // println!("{:#?}", skills);
    // return Ok(());
    // 2. run agent loop
    let content = create_agent(config, client, &mut messages, skills).await?;
    println!("{}", content);
    Ok(())
}
