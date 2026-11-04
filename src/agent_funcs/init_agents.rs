use crate::{
    agent_funcs::create_agent::create_agent,
    skills::{
        process_extract_skills::process_extract_skills, read_skill_body::read_internal_skill_body,
    },
    utils::{args_info::Args, config::Config},
};

use async_openai::{Client, config::OpenAIConfig};
use serde_json::json;

pub async fn init_agents(
    args: Args,
    config: Config,
    client: Client<OpenAIConfig>,
) -> Result<(), Box<dyn std::error::Error>> {
    // 1. read skill folder, process and extract
    let prompt = args.prompt;
    let (mut messages, skills, fork_skills, is_skill) = process_extract_skills(&prompt)?;
    // println!("{:#?}", messages);
    // println!("{:#?}", skills);

    // return Ok(());
    if !is_skill {
        // 2. Check if a skill is in a fork
        for fs in &fork_skills {
            let mut sub_agent_message = vec![json!({
                "role": "user",
                "content": read_internal_skill_body(&fs, &vec![], &skills)?
            })];
            // println!("Sub agent msg: {:#?}", sub_agent_message);
            let content = create_agent(&config, &client, &mut sub_agent_message, &skills).await?;
            // println!("Sub agent is forke");
            println!("{}", content);
            // messages.push(json!({
            //     "role": "user",
            //     "content": content
            // }));
        }
        // 3. run agent loop
        if fork_skills.len() == 0 {
            messages.push(json!({
                "role": "user",
                "content": prompt
            }));
            let content = create_agent(&config, &client, &mut messages, &skills).await?;
            println!("{}", content);
        }
    } else {
        let content = create_agent(&config, &client, &mut messages, &skills).await?;
        println!("{}", content);
    }
    
    Ok(())
}
