use toml::Value;
use std::fs;

pub fn parse(path: &str) -> Result<(String, String, String, Vec<String>, Vec<String>), Box<dyn std::error::Error>> {
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

    let vertex_project = toml_value
        .get("vertex")
        .ok_or("Missing 'vertex' key in TOML")?
        .get("project")
        .ok_or("Missing 'project'")?
        .as_str()
        .ok_or("Invalid type for 'vertex_project'")?
        .to_string();

    let vertex_key = toml_value
        .get("vertex")
        .ok_or("Missing 'vertex' key in TOML")?
        .get("key")
        .ok_or("Missing 'key'")?
        .as_str()
        .ok_or("Invalid type for 'vertex_key'")?
        .to_string();

    // let prompts = toml_value
    //     .get("prompt")
    //     .ok_or("Missing 'prompt' table in TOML")?
    //     .get("value")
    //     .ok_or("Missing 'value' key ")?
    //     .as_array()
    //     .ok_or("Invalid type for 'prompt'")?
    //     .iter()
    //     .map(|value| value.as_str().unwrap().to_string())
    //     .collect();

    let gemini_keys = toml_value
        .get("gemini")
        .ok_or("Missing 'gemini' table in TOML")?
        .get("keys")
        .ok_or("Missing 'keys' key in 'gemini' table")?
        .as_array()
        .ok_or("Invalid type for 'gemini.keys'")?
        .iter()
        .map(|value| value.as_str().unwrap().to_string())
        .collect();

    let gpt4v_keys = toml_value
        .get("gpt4v")
        .ok_or("Missing 'gpt4v' table in TOML")?
        .get("keys")
        .ok_or("Missing 'keys' key in 'gpt4v' table")?
        .as_array()
        .ok_or("Invalid type for 'gpt4v.keys'")?
        .iter()
        .map(|value| value.as_str().unwrap().to_string())
        .collect();

    Ok((prompt, vertex_project, vertex_key, gemini_keys, gpt4v_keys))
}