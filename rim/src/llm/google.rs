#[derive(Debug)]
pub struct Gemini {
    prompt: String,
    key: String,
}

impl Gemini {
    pub fn new(prompt: String, key: String) -> Self {
        Self { prompt, key }
    }

    pub fn get_prompt(&self) -> String{
        self.prompt.clone()
    }
}