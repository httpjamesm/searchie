use std::future::Future;

use anyhow::Result;
use ollama_rs::{
    generation::embeddings::request::{EmbeddingsInput, GenerateEmbeddingsRequest},
    Ollama,
};
pub trait EmbeddingsService: Send + Sync {
    fn get_text_embedding<'a>(
        &'a self,
        text: &'a str,
    ) -> Box<dyn Future<Output = Result<Vec<f32>>> + Send + 'a>;
}

#[derive(Clone)]
pub struct OllamaEmbeddingsService {
    client: Ollama,
}

impl EmbeddingsService for OllamaEmbeddingsService {
    fn get_text_embedding<'a>(
        &'a self,
        text: &'a str,
    ) -> Box<dyn Future<Output = Result<Vec<f32>>> + Send + 'a> {
        Box::new(async move {
            let request = GenerateEmbeddingsRequest::new(
                "mxbai-embed-large".to_string(),
                EmbeddingsInput::Single(text.to_string()),
            );
            let response = self.client.generate_embeddings(request).await?;
            Ok(response.embeddings[0].clone())
        })
    }
}
