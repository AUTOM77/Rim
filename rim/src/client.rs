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

        let client = reqwest::Client::builder()
            .pool_idle_timeout(tokio::time::Duration::from_secs(1))
            .build()?;
        let response = client.post(api)
            .json(&payload)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let raw = &response["candidates"][0]["content"]["parts"][0]["text"].as_str().unwrap();
        Ok(raw.to_string())
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
