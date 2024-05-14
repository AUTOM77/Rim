const GEMINI_FLASH: &str = "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key=";
const GEMINI_PRO: &str = "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-pro-latest:generateContent?key=";

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
        let api = format!("{}{}", GEMINI_PRO, key);

        Self::new(prompt, api)
    }

    pub fn get_prompt(&self) -> String{
        self.prompt.clone()
    }

    pub fn get_api(&self) -> String{
        self.api.clone()
    }
}