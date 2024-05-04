use super::llm::google::Gemini;
use super::llm::openai::GPT;

pub trait LLM {
    fn generate_caption(&self) -> Result<String, Box<dyn std::error::Error>>;
    fn log_prompt(&self) -> &String;
}

impl LLM for Gemini {
    fn generate_caption(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok("Gemini Caption".to_string())
    }
    
    fn log_prompt(&self) -> &String{
        &self.get_prompt()
    }
}

impl LLM for GPT {
    fn generate_caption(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok("GPT4V Caption".to_string()) 
    }
    fn log_prompt(&self) -> &String{
        &self.get_prompt()
    }
}

pub struct RimClient {
    client: Box<dyn LLM>, 
}

impl RimClient {
    pub fn build(client_type: &str, prompt: String, keys: Vec<String>) -> Self {
        let client: Box<dyn LLM> = match client_type {
            "gemini" => Box::new(Gemini::build(prompt, keys)),
            "gpt" => Box::new(GPT::build(prompt, keys)),
            _ => panic!("Invalid client type"),
        };
        Self { client }
    }

    pub fn generate_caption(&self) -> Result<String, Box<dyn std::error::Error>> {
        self.client.generate_caption()
    }
    
    pub fn log_prompt(&self) {
        println!("Prompt: {}", self.client.log_prompt());
    }
}