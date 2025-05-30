use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::process::Command;

#[test]
fn test_load_commands() {
    let mut commands = HashMap::new();
    let content = include_str!("commands.txt");
    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        let alias = parts[0].to_string();
        if alias.starts_with("#") {
            println!("Skipping comment: {}", line);
            continue;
        }
        let command: Vec<String> = parts[1..].iter().map(|&s| s.to_string()).collect();
        if commands.contains_key(&alias) {
            panic!("Alias is already in the command: {}", alias);
        }
        commands.insert(alias, command);
    }
}

fn load_commands() -> HashMap<String, Vec<String>> {
    let mut commands = HashMap::new();
    let content = include_str!("commands.txt");

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        let alias = parts[0].to_string();
        if alias.starts_with("#") {
            println!("Skipping comment: {}", line);
            continue;
        }
        let command: Vec<String> = parts[1..].iter().map(|&s| s.to_string()).collect();
        commands.insert(alias, command);
    }
    commands
}

fn main() {
    let invoked_as = env::args().next().unwrap();
    let cmd_name = Path::new(&invoked_as)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();

    let commands = load_commands();

    match commands.get(cmd_name.as_str()) {
        Some(cmd) => {
            let mut command = Command::new(cmd[0].clone());
            command.args(&cmd[1..]);

            for arg in env::args().skip(1) {
                command.arg(arg);
            }

            let status = command.status().expect("Failed to execute command");
            if !status.success() {
                eprintln!("Command failed");
            }
        }
        None => {
            eprintln!("Unknown command: {}", cmd_name);
            std::process::exit(1);
        }
    }
}
