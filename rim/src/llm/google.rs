const GENAI_API_DISCOVERY_URL: &str = "https://generativelanguage.googleapis.com/$discovery/rest?version=v1beta&key=";
const GEMINI_0514: &str = "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-pro-preview-0514:generateContent?key=";
const GEMINI_VISION: &str = "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.0-pro-vision-001:generateContent?key=";
const GEMINI_FLASH: &str = "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key=";
const GEMINI_PRO: &str = "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-pro-latest:generateContent?key=";
const GEMINI_EXP: &str = "https://generativelanguage.googleapis.com/v1beta/models/gemini-experimental:generateContent?key=";
const GEMINI: &str = "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key=";
const GEMINI_FILE: &str = "https://generativelanguage.googleapis.com/upload/v1beta/files?key={}&alt=json&uploadType=media";
// &alt=json&uploadType=media

const VERTEX: &str = "https://{ZONE}-aiplatform.googleapis.com/v1/projects/${PROJECT}/locations/{ZONE}/publishers/google/models/${MODEL}:generateContent";
const VERTEX_PRO: &str = "https://us-central1-aiplatform.googleapis.com/v1/projects/${PROJECT}/locations/us-central1/publishers/google/models/gemini-1.5-pro-preview-0514:generateContent";
pub const VERTEX_FLASH: &str = "https://us-central1-aiplatform.googleapis.com/v1/projects/${PROJECT}/locations/us-central1/publishers/google/models/gemini-1.5-flash-preview-0514:generateContent";

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

    pub fn get_prompt(&self) -> String {
        self.prompt.clone()
    }

    pub fn get_api(&self) -> String {
        self.api.clone()
    }

    pub fn payload(&self, data: String) -> serde_json::Value {
        let payload =
            json!({
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

#[derive(Debug)]
pub struct Vertex {
    prompt: String,
    api: String,
}

impl Vertex {
    pub fn new(prompt: String, api: String) -> Self {
        Self { prompt, api}
    }

    pub fn build(prompt: String, project: String) -> Self {
        let api = VERTEX_PRO.replace("${PROJECT}", &project);
        Self::new(prompt, api)
    }

    pub fn get_prompt(&self) -> String {
        self.prompt.clone()
    }
    
    pub fn get_api(&self) -> String {
        self.api.clone()
    }

    pub fn payload(&self, fileUrl: String, mime: String) -> serde_json::Value {
        let payload =
            json!({
            "contents": [
                {   
                    "role": "user",
                    "parts": [
                        {"fileData": { "mimeType": mime, "fileUri": fileUrl } },
                        {"text": self.prompt.clone()},
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
