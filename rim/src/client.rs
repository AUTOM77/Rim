use crate::llm::Gemini;
use crate::llm::GPT;

use reqwest::header::{HeaderMap, AUTHORIZATION};

#[derive(Debug)]
pub struct RimClient {
    model: Gemini,
}

impl RimClient {
    pub fn new(model: Gemini) -> Self {
        Self { model }
    }

    pub fn build(prompt: String, key: String) -> Self {
        let model = Gemini::build(prompt, key);
        Self::new(model)
    }

    pub async fn generate_caption(&self, data: String) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let api = self.model.get_api();
        let payload = self.model.payload(data);

        let client = reqwest::Client::builder().build()?;

        let response = client
            .post(api)
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            println!("{:?}", response);
            return Err(format!("API request failed with status code: {}", response.status()).into());
        }

        let json: serde_json::Value = response.json().await?;
        let raw = json
            .get("candidates")
            .and_then(|candidates| candidates.get(0))
            .and_then(|candidate| candidate.get("content"))
            .and_then(|content| content.get("parts"))
            .and_then(|parts| parts.get(0))
            .and_then(|part| part.get("text"))
            .and_then(|text| text.as_str())
            .ok_or_else(|| "Missing or invalid response data".to_string())?;
        Ok(raw.to_string())
    }

    pub fn log_api(&self) {
        println!("API: {}", self.model.get_api());
    }

    pub fn log_prompt(&self) {
        println!("Prompt: {}", self.model.get_prompt());
    }
}

// pub trait LLM {
//     fn generate_caption(&self) -> Result<String, Box<dyn std::error::Error>>;
//     fn log_prompt(&self) -> &String;
// }

// impl LLM for Gemini {
//     fn generate_caption(&self) -> Result<String, Box<dyn std::error::Error>> {
//         Ok("Gemini Caption".to_string())
//     }

//     fn log_prompt(&self) -> &String{
//         &self.get_prompt()
//     }
// }

// impl LLM for GPT {
//     fn generate_caption(&self) -> Result<String, Box<dyn std::error::Error>> {
//         Ok("GPT4V Caption".to_string())
//     }
//     fn log_prompt(&self) -> &String{
//         &self.get_prompt()
//     }
// }

// pub struct RimClient {
//     client: Box<dyn LLM>,
// }

// impl RimClient {
//     pub fn build(client_type: &str, prompt: String, keys: Vec<String>) -> Self {
//         let client: Box<dyn LLM> = match client_type {
//             "gemini" => Box::new(Gemini::build(prompt, keys)),
//             "gpt" => Box::new(GPT::build(prompt, keys)),
//             _ => panic!("Invalid client type"),
//         };
//         Self { client }
//     }

//     pub fn generate_caption(&self) -> Result<String, Box<dyn std::error::Error>> {
//         self.client.generate_caption()
//     }

//     pub fn log_prompt(&self) {
//         println!("Prompt: {}", self.client.log_prompt());
//     }
// }
