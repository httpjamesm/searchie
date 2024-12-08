use anyhow::Result;
use tokenizers::Tokenizer;

/// Splits text into chunks based on token count, optionally prefixing each chunk with a name.
/// Ensures that each chunk does not exceed `max_tokens` and includes `overlap_tokens` from the previous chunk.
///
/// # Arguments
///
/// * `name` - Optional prefix for each chunk.
/// * `text` - The input text to be chunked.
/// * `max_tokens` - Maximum number of tokens allowed per chunk.
/// * `overlap_tokens` - Number of tokens to overlap between consecutive chunks.
/// * `tokenizer` - Tokenizer used to encode the text.
///
/// # Returns
///
/// A vector of text chunks.
pub fn chunk_text(
    name: Option<&str>,
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

    for paragraph in paragraphs {
        let trimmed = paragraph.trim();
        let prefixed = if let Some(name) = name {
            format!("{} - {}", name, trimmed)
        } else {
            trimmed.to_string()
        };
        let tokens = tokenizer
            .encode(prefixed.as_str(), false)
            .map_err(|e| anyhow::anyhow!("Failed to encode paragraph: {}", e))?;
        let token_count = tokens.get_ids().len();

        if current_total_tokens + token_count <= max_tokens {
            current_chunk.push(trimmed);
            token_counts.push(token_count);
            current_total_tokens += token_count;
        } else {
            if !current_chunk.is_empty() {
                let chunk_text = join_chunk(name, &current_chunk);
                chunks.push(chunk_text);
            }

            // Determine overlap
            let mut overlap_start = current_chunk.len();
            let mut overlap_token_sum = 0;
            while overlap_start > 0 && overlap_token_sum < overlap_tokens {
                overlap_start -= 1;
                overlap_token_sum += token_counts[overlap_start];
            }

            // Retain overlapping paragraphs
            current_chunk = current_chunk.drain(overlap_start..).collect();
            token_counts = token_counts.drain(overlap_start..).collect();
            current_total_tokens = token_counts.iter().sum();

            // Add the new paragraph
            current_chunk.push(trimmed);
            token_counts.push(token_count);
            current_total_tokens += token_count;
        }
    }

    // Add the final chunk if any
    if !current_chunk.is_empty() {
        let chunk_text = join_chunk(name, &current_chunk);
        chunks.push(chunk_text);
    }

    Ok(chunks)
}

/// Helper function to join paragraphs into a single chunk with optional name prefix.
fn join_chunk(name: Option<&str>, paragraphs: &[&str]) -> String {
    let joined = paragraphs.join("\n");
    if let Some(name) = name {
        format!("{} - {}", name, joined).trim().to_string()
    } else {
        joined.trim().to_string()
    }
}
