use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use fastembed::{RerankInitOptions, RerankerModel, TextRerank};

#[async_trait]
pub trait RerankingService: Send + Sync {
    async fn rerank(&self, query: &str, results: Vec<String>) -> Result<Vec<usize>>;
}

#[derive(Clone)]
pub struct FastEmbedRerankingService {
    client: Arc<TextRerank>,
}

impl FastEmbedRerankingService {
    pub fn new(model_dir_path: &str) -> Self {
        // ensure the directory exists
        std::fs::create_dir_all(model_dir_path).unwrap();
        let reranker_model = TextRerank::try_new(
            RerankInitOptions::new(RerankerModel::JINARerankerV1TurboEn)
                .with_cache_dir(PathBuf::from(model_dir_path))
                .with_show_download_progress(true),
        )
        .unwrap();

        Self {
            client: Arc::new(reranker_model),
        }
    }
}

#[async_trait]
impl RerankingService for FastEmbedRerankingService {
    async fn rerank(&self, query: &str, results: Vec<String>) -> Result<Vec<usize>> {
        let documents_strs: Vec<&str> = results.iter().map(|s| s.as_str()).collect();
        let reranked = self.client.rerank(query, documents_strs, false, None)?;

        Ok(reranked.into_iter().map(|r| r.index).collect())
    }
}
