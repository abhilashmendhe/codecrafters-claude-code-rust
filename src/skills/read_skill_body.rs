use std::collections::HashMap;
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
};

use serde_json::{Value, json};

use crate::skills::skill_config::SkillConfig;

pub fn read_skill_body(
    only_skills: Vec<&str>,
    only_args: Vec<&str>,
    // spl_prompt: Vec<&str>,
    skills: &HashMap<String, SkillConfig>,
) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let mut messages = vec![];
    for t_prompt in only_skills {
        let skill_name = &t_prompt[1..];
        messages.push(json!({
            "role": "user",
            "content": read_internal_skill_body(skill_name, &only_args, skills)?
        }));
    }
    Ok(messages)
}

pub fn read_internal_skill_body(
    skill_name: &str,
    only_args: &Vec<&str>,
    skills: &HashMap<String, SkillConfig>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut skill_body = vec![];
    let init_sent = format!(
        "Skill: {} (located at .claude/skills/{})
Paths in the instructions below are relative to that folder.\n",
        skill_name, skill_name
    );
    if let Some(skill_config) = skills.get(skill_name) {
        if skill_config.skill_exe_path().len() > 0 {
            skill_body.extend(init_sent.bytes());
        }
        let skill_file_path = skill_config.skill_path();
        let mut file = File::open(skill_file_path)?;
        file.seek(SeekFrom::Start(skill_config.body_offset()))?;
        let mut buffer = vec![];
        while let Ok(be) = file.read_to_end(&mut buffer) {
            if be == 0 {
                break;
            }

            let chunk = &buffer[..be];
            let mut pos = 0;
            let mut k;
            while pos < chunk.len() {
                if chunk[pos] == 36 {
                    k = pos;
                    while chunk[pos] != 32 && chunk[pos] != 46 && chunk[pos] != 10 {
                        pos += 1;
                    }
                    let snum = String::from_utf8_lossy(&chunk[(k + 1)..pos]).to_string();
                    if chunk[k + 1] >= 48 && chunk[k + 1] <= 57 {
                        let num = snum.parse::<usize>()?;
                        if let Some(p_arg) = only_args.get(num) {
                            skill_body.extend(p_arg.bytes());
                        } else {
                            skill_body.extend(String::from_utf8_lossy(&chunk[k..pos]).bytes());
                        }
                    } else {
                        skill_body.extend(only_args.join(" ").bytes());
                    }
                }
                skill_body.push(chunk[pos]);
                pos += 1;
            }
        }
    }
    Ok(String::from_utf8_lossy(&skill_body).to_string())
}
