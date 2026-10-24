use serde_json::{Value, json};

use crate::skills::{read_skill_body::read_skill_body, read_skills::read_skills_folder_contents};

pub fn process_extract_skills(prompt: &str) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let (content, skills) = read_skills_folder_contents()?;
    let messages = if prompt.starts_with("/") {
        let spl_prompt = prompt.split_whitespace().collect::<Vec<&str>>();
        let skill_body = read_skill_body(spl_prompt, skills)?;
        // println!("Skill body:\n{}",skill_body);
        vec![
            json!({
                "role": "system",
                "content": content
            }),
            json!({
                "role": "user",
                "content": skill_body
            }),
        ]
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
