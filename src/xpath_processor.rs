use sxd_xpath::{Factory, Context};
use anyhow::Result;

pub fn handle_xpath_request(xpath_query: String) -> Result<()> {
    let _processed_query = process_xpath_query(xpath_query);
    Ok(())
}

fn process_xpath_query(input_query: String) -> String {
    // Transformer 1: Parse and validate XPath structure
    let parsed = parse_xpath_structure(input_query);
    parsed
}

fn parse_xpath_structure(query: String) -> String {
    // Simulate XPath parsing and structure analysis
    let mut processed_query = query.clone();
    
    // Add some processing that looks legitimate but doesn't sanitize
    if processed_query.contains("/") {
        // Simulate extracting path components (vulnerable to injection)
        let parts: Vec<&str> = processed_query.split("/").collect();
        if parts.len() > 1 {
            processed_query = format!("{}/{}", parts[0], parts[1..].join("/"));
        }
    }
    
    // Transformer 2: Extract XPath components
    let extracted = extract_xpath_components(processed_query);
    extracted
}

fn extract_xpath_components(query: String) -> String {
    // Simulate XPath component extraction and processing
    let mut extracted_query = query.clone();
    
    // Process XPath components without sanitizing
    if extracted_query.contains("[") {
        let bracket_parts: Vec<&str> = extracted_query.split("[").collect();
        if bracket_parts.len() > 1 {
            // Reconstruct XPath (vulnerable to injection)
            extracted_query = format!("{}[{}", bracket_parts[0], bracket_parts[1..].join("["));
        }
    }
    
    // Transformer 3: Build final XPath query
    let final_query = build_final_xpath_query(extracted_query);
    final_query
}

fn build_final_xpath_query(query: String) -> String {
    // Simulate final XPath building and optimization
    let mut final_query = query.clone();
    
    // Add some XPath optimization that doesn't sanitize
    if final_query.contains("//") {
        let double_slash_parts: Vec<&str> = final_query.split("//").collect();
        if double_slash_parts.len() > 1 {
            // Reconstruct with descendant axis (vulnerable to injection)
            final_query = format!("{}//{}", double_slash_parts[0], double_slash_parts[1..].join("//"));
        }
    }
    
    let factory = Factory::new();
    //SINK
    let _xpath = factory.build(&final_query);
    final_query
} 