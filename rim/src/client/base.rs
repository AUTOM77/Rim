use reqwest::header::{HeaderMap, CONTENT_TYPE};
use serde_json::json;

pub trait API {
    fn get_headers(&self) -> HeaderMap;
    fn get_url(&self) -> String;
    fn get_payload(&self, user_prompt: &str, _base64: Vec<String>) -> serde_json::Value;
    fn parse_response(&self, response: serde_json::Value) -> Result<String, Box<dyn std::error::Error>>;
    fn parse_consumption(&self, response: serde_json::Value) -> Result<String, Box<dyn std::error::Error>>;

    async fn get_caption(&self, user_prompt: &str, images_base64: Vec<String>) -> Result<(String, String), Box<dyn std::error::Error>> {
        let headers = self.get_headers();
        let payload = self.get_payload(user_prompt, images_base64);
        let url = self.get_url();
        let start_time = std::time::Instant::now();

        let client = reqwest::Client::new();
        let response = client.post(&url)
            .headers(headers)
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            println!("{:#?}", response);
            return Err(format!("HTTP error: {} - {}", response.status(), response.text().await?).into());
        }
        let json: serde_json::Value = response.json().await?;

        println!("HTTP Response time: {:?}", start_time.elapsed());
        let caption = self.parse_response(json.clone())?;
        let consumption = self.parse_consumption(json)?;
        Ok((caption, consumption))
    }
}

#[derive(Debug)]
pub enum Service {
    Azure(Azure),
    Gemini(Gemini),
}

impl Service {
    pub fn from(name: &str, endpoint: String, key: String, model: String) -> Option<Self> {
        match name {
            "azure"=> Some(Service::Azure(Azure::from(endpoint, key, model))),
            "gemini" => Some(Service::Gemini(Gemini::from(endpoint, key, model))),
            _ => None
        }
    }

    pub async fn get_caption(&self, user_prompt: &str, images_base64: Vec<String>) -> Result<(String, String), Box<dyn std::error::Error>> {
        match self {
            Service::Azure(azure) => azure.get_caption(user_prompt, images_base64).await,
            Service::Gemini(gemini) => gemini.get_caption(user_prompt, images_base64).await,
        }
    }
}

#[derive(Debug)]
pub struct Azure {
    endpoint: String,
    key: String,
    model: String,
    system: String,
}

impl Azure {
    pub fn new(endpoint: String, key: String, model: String, system: String) -> Self {
        Self {
            endpoint,
            key,
            model,
            system,
        }
    }

    pub fn from(endpoint: String, key: String, model: String) -> Self {
        Self::new(endpoint, key, model, "".into())
    }
}

impl API for Azure {
    fn get_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert("api-key", self.key.parse().unwrap());
        headers
    }

    fn get_url(&self) -> String {
        format!("{}/openai/deployments/{}/chat/completions?api-version=2024-02-15-preview", self.endpoint, self.model)
    }

    fn get_payload(&self, user_prompt: &str, _base64: Vec<String>) -> serde_json::Value {
        let sys_prompt = self.system.clone();
        let mut usr_content: Vec<serde_json::Value> = Vec::new();

        let sys_msg = json!({ "role": "system", "content": sys_prompt });
        let text_content = json!({ "type": "text", "text": user_prompt });
        let image_content: Vec<serde_json::Value> = _base64
            .into_iter()
            .map(|_b64| format!("data:image/jpg;base64,{}", _b64))
            .map(|image| json!({ "type": "image_url", "image_url": { "url": image }}))
            .collect();

        usr_content.push(text_content);
        usr_content.extend(image_content);

        let usr_msg = json!({ "role": "user", "content": usr_content });

        json!({
            "messages": [sys_msg, usr_msg],
            "model": self.model,
        })
    }

    fn parse_response(&self, response: serde_json::Value) -> Result<String, Box<dyn std::error::Error>> {
        let caption = response["choices"][0]["message"]["content"]
            .as_str()
            .ok_or("Failed to parse response")?;
        Ok(caption.to_string())
    }

    fn parse_consumption(&self, response: serde_json::Value) -> Result<String, Box<dyn std::error::Error>> {
        Ok(response["usage"].to_string())
    }
}

#[derive(Debug)]
pub struct Gemini {
    endpoint: String,
    key: String,
    model: String,
    system: String,
}

impl Gemini {
    pub fn new(endpoint: String, key: String, model: String, system: String) -> Self {
        Self {
            endpoint,
            key,
            model,
            system,
        }
    }

    pub fn from(endpoint: String, key: String, model: String) -> Self {
        Self::new(endpoint, key, model, "".into())
    }
}

impl API for Gemini {
    fn get_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers
    }

    fn get_url(&self) -> String {
        format!("{}/v1beta/models/{}:generateContent?key={}", self.endpoint, self.model, self.key)
    }

    fn get_payload(&self, user_prompt: &str, _base64: Vec<String>) -> serde_json::Value {
        let mut usr_content: Vec<serde_json::Value> = Vec::new();

        let text_content = json!({"text": user_prompt });
        let image_content: Vec<serde_json::Value> = _base64
            .into_iter()
            .map(|_b64| json!({ "mimeType": "image/jpeg", "data": _b64 }))
            .map(|image| json!({ "inlineData": image }))
            .collect();

        usr_content.push(text_content);
        usr_content.extend(image_content);

        let usr_msg = json!({ "role": "user", "parts": usr_content });

        json!({
            "contents": [usr_msg]
        })
    }

    fn parse_response(&self, response: serde_json::Value) -> Result<String, Box<dyn std::error::Error>> {
        let caption = response["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .ok_or("Failed to parse response")?;
        Ok(caption.to_string())
    }

    fn parse_consumption(&self, response: serde_json::Value) -> Result<String, Box<dyn std::error::Error>> {
        Ok(response["usageMetadata"].to_string())
    }
}