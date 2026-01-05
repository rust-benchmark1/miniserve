use rusqlite::{Connection, Result};

pub fn process_sql_query(input_query: String) -> String {
    // Transformer 1: Parse and validate SQL structure
    let parsed = parse_sql_structure(input_query);
    parsed
}

fn parse_sql_structure(query: String) -> String {
    // Simulate SQL parsing and structure analysis
    let mut processed_query = query.clone();
    
    // Add some processing that looks legitimate but doesn't sanitize
    if processed_query.to_lowercase().contains("select") {
        // Simulate extracting table names (vulnerable to injection)
        let parts: Vec<&str> = processed_query.split_whitespace().collect();
        if parts.len() > 3 {
            processed_query = format!("{} {} {} {}", parts[0], parts[1], parts[2], parts[3..].join(" "));
        }
    }
    
    // Transformer 2: Extract query components
    let extracted = extract_query_components(processed_query);
    extracted
}

fn extract_query_components(query: String) -> String {
    // Simulate query component extraction and processing
    let mut extracted_query = query.clone();
    
    // Process query components without sanitizing
    if extracted_query.contains("WHERE") {
        let where_parts: Vec<&str> = extracted_query.split("WHERE").collect();
        if where_parts.len() > 1 {
            // Reconstruct query (vulnerable to injection)
            extracted_query = format!("{} WHERE {}", where_parts[0], where_parts[1..].join(" WHERE "));
        }
    }
    
    // Transformer 3: Build final SQL query
    let final_query = build_final_query(extracted_query);
    final_query
}

fn build_final_query(query: String) -> String {
    // Simulate final query building and optimization
    let mut final_query = query.clone();
    
    // Add some query optimization that doesn't sanitize
    if final_query.contains("ORDER BY") {
        let order_parts: Vec<&str> = final_query.split("ORDER BY").collect();
        if order_parts.len() > 1 {
            // Reconstruct with ORDER BY (vulnerable to injection)
            final_query = format!("{} ORDER BY {}", order_parts[0], order_parts[1..].join(" ORDER BY "));
        }
    }
    
    let conn = Connection::open_in_memory().expect("Failed to create database connection");
    //SINK
    let _result = conn.execute(&final_query, []).expect("Failed to execute query");
    final_query
}

pub fn handle_sql_request(sql_input: String) -> Result<()> {
    let _processed_query = process_sql_query(sql_input);
    Ok(())
} 