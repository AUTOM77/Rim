use crate::llm::Vertex;
// use crate::llm::GPT;

use reqwest::header::{HeaderMap, AUTHORIZATION};

#[derive(Debug)]
pub struct RimClient {
    model: Vertex,
    headers: HeaderMap,
}

impl RimClient {
    pub fn new(model: Vertex, headers: HeaderMap) -> Self {
        Self { model, headers }
    }

    pub fn build(prompt: String, project: String) -> Self {
        let model = Vertex::build(prompt, project);
        let headers = HeaderMap::new();
        Self::new(model, headers)
    }

    pub fn with_auth(mut self, key: String) -> Self{
        let auth = format!("Bearer {key}");
        self.headers.insert(AUTHORIZATION, auth.parse().unwrap());
        self
    }

    pub async fn generate_caption(&self, fileUrl: String, mime: String) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let api = self.model.get_api();
        let payload = self.model.payload(fileUrl, mime);

        let client = reqwest::Client::builder().build()?;

        let response = client
            .post(api)
            .headers(self.headers.clone())
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
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

    pub async fn gs_upload(&self, data: Vec<u8>, mime: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let api = self.model.get_api();
        // let payload = self.model.payload(data, mime);

        // let raw = json
        //     .get("candidates")
        //     .and_then(|candidates| candidates.get(0))
        //     .and_then(|candidate| candidate.get("content"))
        //     .and_then(|content| content.get("parts"))
        //     .and_then(|parts| parts.get(0))
        //     .and_then(|part| part.get("text"))
        //     .and_then(|text| text.as_str())
        //     .ok_or_else(|| "Missing or invalid response data".to_string())?;
        let gs_url  = "gs://cloud-samples-data/video/animals.mp4".to_string();
        Ok(gs_url)
    }

    pub async fn delete_asset(&self, url: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
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
