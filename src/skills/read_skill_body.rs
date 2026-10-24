use std::collections::HashMap;
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
};

use crate::skills::skill_config::SkillConfig;

pub fn read_skill_body(
    spl_prompt: Vec<&str>,
    skills: HashMap<String, SkillConfig>,
) -> Result<String, Box<dyn std::error::Error>> {
    let t_prompt = &spl_prompt[0][1..];
    let mut skill_body = vec![];
    if let Some(skill_config) = skills.get(t_prompt) {
        let skill_file_path = skill_config.skill_path();
        let mut file = File::open(skill_file_path)?;
        file.seek(SeekFrom::Start(skill_config.body_offset()))?;
        let mut buffer = vec![];
        while let Ok(be) = file.read_to_end(&mut buffer) {
            if be == 0 {
                break;
            }
            let chunk = &buffer[..be];
            // println!("{:?}", chunk);
            let mut pos = 0;
            let mut k;
            while pos < chunk.len() {
                if chunk[pos] == 36 {
                    k = pos + 1;
                    while chunk[pos] != 32 && chunk[pos] != 46 && chunk[pos] != 10 {
                        pos += 1;
                    }
                    let snum = String::from_utf8_lossy(&chunk[k..pos]).to_string();
                    if chunk[k] >= 48 && chunk[k] <= 57 {
                        let num = snum.parse::<usize>()?;
                        skill_body.extend(spl_prompt[num + 1].bytes());
                    } else {
                        skill_body.extend(spl_prompt[1..].join(" ").bytes());
                    }
                }
                skill_body.push(chunk[pos]);
                pos += 1;
            }
        }
    }
    Ok(String::from_utf8_lossy(&skill_body).to_string())
}
