use serde_json::{Value, json};

use crate::skills::{read_skill_body::read_skill_body, read_skills::read_skills_folder_contents};

pub fn process_extract_skills(prompt: &str) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let (content, skills) = read_skills_folder_contents()?;
    let messages = if prompt.starts_with("/") {
        let mut only_skills = vec![];
        let mut only_args = vec![];
        for skar in prompt.split_whitespace() {
            if skar.starts_with("/") {
                only_skills.push(skar);
            } else {
                only_args.push(skar);
            }
        }

        let skill_messages = read_skill_body(only_skills, only_args, skills)?;

        let mut messages = vec![json!({
            "role": "system",
            "content": content
        })];
        messages.extend(skill_messages);
        messages
    } else {
        vec![
            json!({
                "role": "system",
                "content": content
            }),
            json!({
                "role": "user",
                "content": prompt
            }),
        ]
    };
    Ok(messages)
}
