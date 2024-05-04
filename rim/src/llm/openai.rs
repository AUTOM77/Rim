pub struct GPT {
    prompt: String,
    keys: Vec<String>,
}

impl GPT {
    pub fn build(prompt: String, keys: Vec<String>) -> Self {
        Self { prompt, keys }
    }
    pub fn get_prompt(&self) -> &String{
        &self.prompt
    }
}