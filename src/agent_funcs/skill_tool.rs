use std::collections::HashMap;

use crate::skills::{read_skill_body::read_internal_skill_body, skill_config::SkillConfig};

pub async fn skill_tool(
    name: &str,
    args: &Option<String>,
    skills: &HashMap<String, SkillConfig>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut only_skills = vec![];
    let mut only_args = vec![];
    let skill_name = if skills.contains_key(name) {
        name.to_string()
    } else {
        "".to_string()
    };
    only_skills.push(skill_name.as_str());
    let ext_args = if let Some(args) = args { args } else { "" };
    only_args.push(ext_args);
    let skill_messages = read_internal_skill_body(&name, &only_args, skills)?;
    Ok(skill_messages)
}
