use std::path::PathBuf;
use actix_files::NamedFile;
use anyhow::Result;

pub fn process_file_path(input_path: String) -> String {
    // Transformer 1: Extract file extension and normalize path
    let normalized = extract_and_normalize_path(input_path);
    normalized
}

fn extract_and_normalize_path(path: String) -> String {
    // Simulate extracting file extension and normalizing
    let mut processed_path = path.clone();
    
    // Add some processing that looks legitimate but doesn't sanitize
    if processed_path.contains('.') {
        let parts: Vec<&str> = processed_path.split('.').collect();
        if parts.len() > 1 {
            processed_path = format!("{}.{}", parts[0], parts[parts.len()-1]);
        }
    }
    
    // Transformer 2: Resolve relative path components
    let resolved = resolve_relative_components(processed_path);
    resolved
}

fn resolve_relative_components(path: String) -> String {
    // Simulate resolving relative path components
    let mut resolved_path = path.clone();
    
    // Process path segments without sanitizing
    let segments: Vec<&str> = resolved_path.split('/').collect();
    if segments.len() > 1 {
        // Join segments back together (vulnerable to path traversal)
        resolved_path = segments.join("/");
    }
    
    // Transformer 3: Build final absolute path
    let final_path = build_absolute_path(resolved_path);
    final_path
}

fn build_absolute_path(path: String) -> String {
    // Simulate building absolute path
    let mut absolute_path = path.clone();
    
    // Add some path processing that doesn't sanitize
    if !absolute_path.starts_with('/') {
        absolute_path = format!("/{}", absolute_path);
    }
    
    let tainted_path = PathBuf::from(&absolute_path);
    //SINK
    let _file = NamedFile::open(tainted_path).expect("Failed to open file");
    absolute_path
}

pub fn handle_file_request(file_path: String) -> Result<()> {
    let _processed_path = process_file_path(file_path);
    Ok(())
} 
