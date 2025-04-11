use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let invoked_as = env::args().next().unwrap();
    let cmd_name = Path::new(&invoked_as)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();

    let mut commands: HashMap<&str, Vec<&str>> = HashMap::new();

    commands.insert("gcm", vec!["git", "checkout", "main"]);
    commands.insert("gb", vec!["git", "branch"]);
    commands.insert("gco", vec!["git", "checkout"]);
    commands.insert("gst", vec!["git", "status"]);
    commands.insert(
        "gl",
        vec!["git", "log", "--oneline", "--graph", "--decorate"],
    );
    commands.insert("gaa", vec!["git", "add", "."]);
    commands.insert("gcb", vec!["git", "checkout", "-b"]);
    commands.insert("gp", vec!["git", "push"]);
    commands.insert("gpl", vec!["git", "pull"]);
    commands.insert("gcp", vec!["git", "cherry-pick"]);

    match commands.get(cmd_name.as_str()) {
        Some(cmd) => {
            let mut command = Command::new(cmd[0]);
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
