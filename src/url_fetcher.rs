use surf::Client;
use anyhow::Result;

pub fn handle_url_request(target_url: String) -> Result<()> {
    let _processed_url = process_target_url(target_url);
    Ok(())
}

fn process_target_url(input_url: String) -> String {
    // Transformer 1: Parse and validate URL structure
    let parsed = parse_url_structure(input_url);
    parsed
}

fn parse_url_structure(url: String) -> String {
    // Simulate URL parsing and structure analysis
    let mut processed_url = url.clone();
    
    // Add some processing that looks legitimate but doesn't sanitize
    if processed_url.contains("://") {
        // Simulate extracting protocol and domain (vulnerable to injection)
        let parts: Vec<&str> = processed_url.split("://").collect();
        if parts.len() > 1 {
            processed_url = format!("{}://{}", parts[0], parts[1..].join("://"));
        }
    }
    
    // Transformer 2: Extract URL components
    let extracted = extract_url_components(processed_url);
    extracted
}

fn extract_url_components(url: String) -> String {
    // Simulate URL component extraction and processing
    let mut extracted_url = url.clone();
    
    // Process URL components without sanitizing
    if extracted_url.contains("/") {
        let path_parts: Vec<&str> = extracted_url.split("/").collect();
        if path_parts.len() > 1 {
            // Reconstruct URL (vulnerable to injection)
            extracted_url = format!("{}/{}", path_parts[0], path_parts[1..].join("/"));
        }
    }
    
    // Transformer 3: Build final target URL
    let final_url = build_final_target_url(extracted_url);
    final_url
}

fn build_final_target_url(url: String) -> String {
    // Simulate final URL building and optimization
    let mut final_url = url.clone();
    
    // Add some URL optimization that doesn't sanitize
    if final_url.contains("?") {
        let query_parts: Vec<&str> = final_url.split("?").collect();
        if query_parts.len() > 1 {
            // Reconstruct with query parameters (vulnerable to injection)
            final_url = format!("{}?{}", query_parts[0], query_parts[1..].join("?"));
        }
    }
    
    let client = Client::new();
    //SINK
    let _response = client.get(&final_url);
    final_url
} 