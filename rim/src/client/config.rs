use std::str::FromStr;
use std::collections::HashMap;
use std::fmt;

const SUPPORTED_PROVIDERS: &[&str] = &["azure", "gemini"];

#[derive(Debug)]
pub struct Provider {
    pub endpoint: String,
    pub key: String,
    pub model: String,
}

#[derive(Debug, Clone)]
pub struct Prompt {
    pub name: String,
    pub value: String,
}

#[derive(Debug)]
pub struct Config {
    prompts: Vec<Prompt>,
    providers: HashMap<String, Vec<Provider>>,
}

impl FromStr for Config {
    type Err = Box<dyn std::error::Error>;

    fn from_str(toml_str: &str) -> Result<Self, Self::Err> {
        let toml_value: toml::Value = toml::from_str(toml_str)?;
        let mut prompts = Vec::new();

        if let Some(prompt_entries) = toml_value.get("prompt").and_then(|p| p.as_array()) {
            for prompt_entry in prompt_entries {
                if let Some(prompt_table) = prompt_entry.as_table() {
                    let name = prompt_table
                        .get("name")
                        .ok_or("Missing 'name' key in 'prompt'")?
                        .as_str()
                        .ok_or("Invalid type for 'prompt.name'")?
                        .to_string();

                    let value = prompt_table
                        .get("value")
                        .ok_or("Missing 'value' key in 'prompt'")?
                        .as_str()
                        .ok_or("Invalid type for 'prompt.value'")?
                        .to_string();

                    prompts.push(Prompt { name, value });
                } else {
                    return Err("Invalid 'prompt' entry".into());
                }
            }
        }

        let mut providers = HashMap::new();
        let mut found_valid_provider = false;

        for &key in SUPPORTED_PROVIDERS {
            if let Some(provider_table) = toml_value.get(key) {
                let provider_list = provider_table
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
                providers.insert(key.to_string(), provider_list);
            }
        }

        if !found_valid_provider {
            return Err("At least one of 'azure' or 'gemini' configurations must be valid".into());
        }

        Ok(Config { prompts, providers })
    }
}

impl Config {
    pub fn prompts(&self) -> Vec<Prompt> {
        self.prompts.clone()
    }

    pub fn get(&self, provider: &str) -> Option<&Vec<Provider>> {
        self.providers.get(provider)
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Config:")?;
        writeln!(f, "  Prompts:")?;
        for prompt in &self.prompts {
            writeln!(f, "    - Name: {}, Value: {}", prompt.name, prompt.value)?;
        }
        writeln!(f, "  Providers:")?;
        for (provider_key, providers) in &self.providers {
            writeln!(f, "    - Provider: {}", provider_key)?;
            for provider in providers {
                writeln!(f, "      - Endpoint: {}, Key: {}, Model: {}", provider.endpoint, provider.key, provider.model)?;
            }
        }
        Ok(())
    }
}
