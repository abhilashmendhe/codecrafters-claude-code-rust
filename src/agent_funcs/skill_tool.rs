use std::collections::HashMap;

use crate::skills::skill_config::SkillConfig;

pub async fn skill_tool(name: &str, args: &Option<String>, skills: &HashMap<String, SkillConfig>) {
    println!("running skill tool");
    println!("name: {}",name);
    println!("args: {:?}",args);
}
