use anyhow::Result;
use async_trait::async_trait;
use ollama_rs::{
    generation::embeddings::request::{EmbeddingsInput, GenerateEmbeddingsRequest},
    Ollama,
};

#[async_trait]
pub trait EmbeddingsService: Send + Sync {
    async fn get_text_embedding(&self, text: &str) -> Result<Vec<f32>>;
}

#[derive(Clone)]
pub struct OllamaEmbeddingsService {
    client: Ollama,
}

impl OllamaEmbeddingsService {
    pub fn new() -> Self {
        Self {
            client: Ollama::new("http://localhost", 11434),
        }
    }
}

#[async_trait]
impl EmbeddingsService for OllamaEmbeddingsService {
    async fn get_text_embedding(&self, text: &str) -> Result<Vec<f32>> {
        let request = GenerateEmbeddingsRequest::new(
            "mxbai-embed-large".to_string(),
            EmbeddingsInput::Single(text.to_string()),
        );
        let response = self.client.generate_embeddings(request).await?;
        Ok(response.embeddings[0].clone())
    }
}