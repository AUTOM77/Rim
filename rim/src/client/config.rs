use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Provider {
    pub endpoint: String,
    pub key: String,
    pub model: String,
}

#[derive(Debug)]
pub struct Config {
    _prompt: String,
    providers: HashMap<String, Vec<Provider>>,
}

impl FromStr for Config {
    type Err = Box<dyn std::error::Error>;

    fn from_str(toml_str: &str) -> Result<Self, Self::Err> {
        let toml_value: toml::Value = toml::from_str(toml_str)?;

        let _prompt = toml_value
            .get("prompt")
            .ok_or("Missing 'prompt' key in TOML")?
            .get("value")
            .ok_or("Missing 'value' key in 'prompt'")?
            .as_str()
            .ok_or("Invalid type for 'prompt' value")?
            .to_string();

        let mut providers = HashMap::new();
        let mut found_valid_provider = false;

        for key in toml_value.as_table().ok_or("Invalid TOML structure")?.keys() {
            if key == "azure" || key == "gemini" {
                let provider_list = toml_value
                    .get(key)
                    .ok_or(format!("Missing '{}' table in TOML", key))?
                    .get("api")
                    .ok_or(format!("Missing 'api' key in '{}' table", key))?
                    .as_array()
                    .ok_or(format!("Invalid type for '{}.api'", key))?
                    .iter()
                    .map(|value| {
                        let pair = value.as_array().ok_or("Invalid tuple format in 'api'")?;
                        if pair.len() != 3 {
                            return Err("Invalid tuple length in 'api', expected 3 elements".into());
                        }
                        let endpoint = pair.get(0).ok_or("Missing 'endpoint' value in tuple")?.as_str().ok_or("Invalid type for 'endpoint'")?.to_string();
                        let key = pair.get(1).ok_or("Missing 'key' value in tuple")?.as_str().ok_or("Invalid type for 'key'")?.to_string();
                        let model = pair.get(2).ok_or("Missing 'model' value in tuple")?.as_str().ok_or("Invalid type for 'model'")?.to_string();
                        Ok(Provider { endpoint, key, model })
                    })
                    .collect::<Result<Vec<Provider>, Box<dyn std::error::Error>>>()?;

                if !provider_list.is_empty() {
                    found_valid_provider = true;
                }
                providers.insert(key.clone(), provider_list);
            }
        }

        if !found_valid_provider {
            return Err("At least one of 'azure' or 'gemini' configurations must be valid".into());
        }

        Ok(Config { _prompt, providers })
    }
}

impl Config {
    pub fn prompt(&self) -> &str {
        &self._prompt
    }

    pub fn get(&self, provider: &str) -> Option<&Vec<Provider>> {
        self.providers.get(provider)
    }
}