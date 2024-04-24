use toml::Value;
use std::fs;

pub fn load(path: &str) -> Result<(String, Vec<String>, Vec<String>), Box<dyn std::error::Error>> {
    let toml_str = fs::read_to_string(path)?;
    let toml_value: Value = toml::from_str(&toml_str)?;

    let prompt = toml_value
        .get("prompt")
        .ok_or("Missing 'prompt' key in TOML")?
        .get("value")
        .ok_or("Missing 'value' key ")?
        .as_str()
        .ok_or("Invalid type for 'prompt'")?
        .to_string();

    let gemini_keys = toml_value
        .get("gemini")
        .ok_or("Missing 'gemini' table in TOML")?
        .get("keys")
        .ok_or("Missing 'keys' key in 'gemini' table")?
        .as_array()
        .ok_or("Invalid type for 'gemini.keys'")?
        .iter()
        .map(|value| value.as_str().unwrap().to_string()) // Assuming keys are strings
        .collect();

    let gpt4v_keys = toml_value
        .get("gpt4v")
        .ok_or("Missing 'gpt4v' table in TOML")?
        .get("keys")
        .ok_or("Missing 'keys' key in 'gpt4v' table")?
        .as_array()
        .ok_or("Invalid type for 'gpt4v.keys'")?
        .iter()
        .map(|value| value.as_str().unwrap().to_string()) // Assuming keys are strings
        .collect();

    Ok((prompt, gemini_keys, gpt4v_keys))
}