use crate::skills::{skill_config::SkillConfig, skill_parser::parse_skill_file};
use std::{
    collections::HashMap,
    fs::{read, read_dir},
};

pub fn read_skills_folder_contents()
-> Result<(String, HashMap<String, SkillConfig>), Box<dyn std::error::Error>> {
    let mut skills = HashMap::new();
    let mut skill_config = SkillConfig::default();
    let mut content_str = String::from("You have access to the following skills:\n");

    match read_dir("./.claude/skills") {
        Ok(read_dir_path) => {
            for dir in read_dir_path {
                let skill_file_p = format!("{}/SKILL.md", dir?.path().display());
                let content = read(skill_file_p)?;
                let skill_ext_vec = parse_skill_file(&content);
                // println!("{:?}", skill_ext_vec);
                for sev in skill_ext_vec {
                    let fv = &sev[0];
                    let sv = &sev[1];
                    if fv.starts_with("name") {
                        // skill_config.name = sv.to_string();
                        skill_config.set_name(sv.clone());
                        content_str.push_str(&format!("\n- {}:", sv.clone()));
                    } else if fv.starts_with("description") {
                        skill_config.set_description(sv.to_string());
                        content_str.push_str(&format!("{}", sv.clone()));
                    } else if fv.starts_with("  ") {
                        // println!("Metadata = {} - {}",fv, sv);
                        skill_config.add_metadata(fv.trim().to_string(), sv.clone());
                    } else if fv.starts_with("license") {
                        skill_config.set_license(sv.to_string());
                    } else if fv.starts_with("body_offset") {
                        skill_config.set_body_offset(sv.parse::<usize>()?);
                    }
                }
                skills.insert(skill_config.name(), skill_config.clone());
                skill_config.clear();
                // println!("{:?}", skill_ext_vec);
                // let skill_name = &skill_ext_vec[0][1];
                // let skill_desc = &skill_ext_vec[1][1];
                // content_str.push_str(&format!("\n- {}:{}", skill_name, skill_desc));
            }
            // println!("{:#?}",skills);
            Ok((content_str, skills))
        }
        Err(_) => Ok((content_str, skills)),
    }
}
