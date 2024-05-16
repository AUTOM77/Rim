const GEMINI_FLASH: &str = "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key=";
const GEMINI_PRO: &str = "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-pro-latest:generateContent?key=";

use serde_json::json;

#[derive(Debug)]
pub struct Gemini {
    prompt: String,
    api: String,
}

impl Gemini {
    pub fn new(prompt: String, api: String) -> Self {
        Self { prompt, api }
    }

    pub fn build(prompt: String, key: String) -> Self {
        let api = format!("{}{}", GEMINI_FLASH, key);

        Self::new(prompt, api)
    }

    pub fn get_prompt(&self) -> String{
        self.prompt.clone()
    }

    pub fn get_api(&self) -> String{
        self.api.clone()
    }

    pub fn payload(&self, data: String) -> serde_json::Value{
        let payload = json!({
            "contents": [
                {   
                    "role": "user",
                    "parts": [
                        {"text": self.prompt.clone()},
                        {"inlineData": { "mimeType": "image/png", "data": data } },
                    ]
                }
            ],
            "generationConfig": {
                "temperature": 1,
                "topK": 64,
                "topP": 0.95,
                "maxOutputTokens": 8192,
                "stopSequences": []
            },
            "safetySettings": [
                {
                "category": "HARM_CATEGORY_HARASSMENT",
                "threshold": "BLOCK_NONE"
                },
                {
                "category": "HARM_CATEGORY_HATE_SPEECH",
                "threshold": "BLOCK_NONE"
                },
                {
                "category": "HARM_CATEGORY_SEXUALLY_EXPLICIT",
                "threshold": "BLOCK_NONE"
                },
                {
                "category": "HARM_CATEGORY_DANGEROUS_CONTENT",
                "threshold": "BLOCK_NONE"
                }
            ]
        });
        payload
    }
}