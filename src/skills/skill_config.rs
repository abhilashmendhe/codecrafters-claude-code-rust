use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SkillConfig {
    name: String,
    description: String,
    license: String,
    metadata: HashMap<String, String>,
    body_offset: usize,
}

impl Default for SkillConfig {
    fn default() -> Self {
        Self {
            name: Default::default(),
            description: Default::default(),
            license: Default::default(),
            metadata: Default::default(),
            body_offset: Default::default(),
        }
    }
}

impl SkillConfig {
    pub fn clear(&mut self) {
        self.name = String::new();
        self.description = String::new();
        self.license = String::new();
        self.metadata.clear();
        self.body_offset = 0;
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
    pub fn set_license(&mut self, license: String) {
        self.license = license;
    }
    pub fn add_metadata(&mut self, mk: String, mv: String) {
        self.metadata.insert(mk, mv);
    }
    pub fn set_body_offset(&mut self, body_offset: usize) {
        self.body_offset = body_offset;
    }
}
