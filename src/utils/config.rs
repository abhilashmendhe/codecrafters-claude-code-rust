#[derive(Debug, Clone)]
pub struct Config {
    base_url: String, 
    api_key: String,
    model_name: String
}

impl Config {
    pub fn new(base_url: String, api_key: String, model_name: String) -> Self {
        Self { base_url, api_key, model_name }
    }
    pub fn base_url(&self) -> &str {
        return &self.base_url;
    }
    pub fn api_key(&self) -> &str {
        return &self.api_key;
    }
    pub fn model_name(&self) -> &str {
        return &self.model_name;
    }
}