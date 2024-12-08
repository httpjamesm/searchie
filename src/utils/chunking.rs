use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokenizers::Tokenizer;

pub struct Chunk {
    text: String,
    token_count: usize,
}

pub fn chunk_text(
    title: &str,
    text: &str,
    max_tokens: usize,
    overlap_tokens: usize,
    tokenizer: &Tokenizer,
) -> Result<Vec<String>> {
    let paragraphs: Vec<&str> = text
        .split("\n\n")
        .filter(|p| !p.trim().is_empty())
        .collect();

    let mut chunks = Vec::new();
    let mut current_chunk = Vec::new();
    let mut token_counts = Vec::new();
    let mut current_total_tokens = 0;
    let mut is_overlap = Vec::new();

    for paragraph in paragraphs {
        let chunk_with_title = format!("Title: {}\n{}", title, paragraph);
        let tokens = tokenizer.encode(chunk_with_title, false).unwrap();
        let token_count = tokens.get_ids().len();

        if current_total_tokens + token_count <= max_tokens {
            current_chunk.push(paragraph);
            token_counts.push(token_count);
            is_overlap.push(false);
            current_total_tokens += token_count;
        } else {
            if !current_chunk.is_empty() {
                let full_chunk = format!("Title: {}\n{}", title, current_chunk.join("\n"));
                chunks.push(full_chunk);
            }

            let mut overlap_start = current_chunk.len();
            let mut overlap_tokens_count = 0;
            while overlap_start > 0 && overlap_tokens_count < overlap_tokens {
                overlap_start -= 1;
                overlap_tokens_count += token_counts[overlap_start];
            }

            current_chunk = current_chunk.drain(overlap_start..).collect();
            token_counts = token_counts.drain(overlap_start..).collect();
            is_overlap = vec![true; current_chunk.len()];
            current_total_tokens = token_counts.iter().sum();

            current_chunk.push(paragraph);
            token_counts.push(token_count);
            is_overlap.push(false);
            current_total_tokens += token_count;
        }
    }

    if !current_chunk.is_empty() {
        let final_chunk = format!("Title: {}\n{}", title, current_chunk.join("\n"));
        chunks.push(final_chunk);
    }

    Ok(chunks)
}
