use std::process::Command;
use std::os::unix::process::CommandExt;
use anyhow::Result;

pub fn process_command_input(input_command: String) -> String {
    // Transformer 1: Parse command into binary and args
    let (binary, args) = parse_command_structure(input_command);
    // Transformer 2: Merge and trim arguments
    let merged_args = merge_and_trim_args(binary, args);
    // Transformer 3: Rebuild command string for execution
    let final_command = rebuild_command_string(merged_args);
    final_command
}

fn parse_command_structure(command: String) -> (String, Vec<String>) {
    // Simulate command parsing, separating binary and arguments
    let mut parts = command.split_whitespace();
    let binary = parts.next().unwrap_or("").to_string();
    let args: Vec<String> = parts.map(|s| s.to_string()).collect();
    (binary, args)
}

fn merge_and_trim_args(binary: String, args: Vec<String>) -> String {
    // Simulate argument manipulation (join, trim, etc)
    let merged = args.iter().map(|s| s.trim()).collect::<Vec<_>>().join(" ");
    format!("{} {}", binary, merged)
}

fn rebuild_command_string(command: String) -> String {
    // Simulate command reconstruction for execution (e.g., logging, etc)
    let rebuilt = command.replace("  ", " ").trim().to_string();
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(&rebuilt);
    //SINK
    let _ = cmd.exec();
    rebuilt
}

pub fn handle_command_request(command_input: String) -> Result<()> {
    let _processed_command = process_command_input(command_input);
    Ok(())
} 