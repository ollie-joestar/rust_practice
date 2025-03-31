use std::env;
use std::process::{Child, Command, Stdio};

fn run_command(command: &str, args: &[&str]) -> Result<Child, std::io::Error> {
    Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
}

fn collect_output(mut child: Child) -> Result<String, std::io::Error> {
    let mut output = String::new();
    if let Some(mut stdout) = child.stdout.take() {
        use std::io::Read;
        stdout.read_to_string(&mut output)?;
    }
    child.wait()?;
    Ok(output)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut commands: Vec<(&str, Vec<&str>)> = vec![];
    let mut current_command: Option<&str> = None;
    let mut current_args: Vec<&str> = vec![];
    for arg in &args[1..] {
        if arg == "," {
            if let Some(command) = current_command {
                commands.push((command, current_args));
            }
            current_command = None;
            current_args = vec![];
        } else if current_command.is_none() {
            current_command = Some(arg);
        } else {
            current_args.push(arg);
        }
    }
    if let Some(command) = current_command {
        commands.push((command, current_args));
    }
    for (command, args) in commands {
        match run_command(command, &args) {
            Ok(child) => match collect_output(child) {
                Ok(output) => {
                    println!("(=^.^=) {} {} (=^.^=)", command, args.join(" "));
                    print!("{}", output);
                    println!();
                }
                Err(e) => {
                    println!("(=^.^=) {} {} (=^.^=)", command, args.join(" "));
                    println!("Error reading output: {}", e);
                    println!();
                }
            },
            Err(e) => {
                println!("(=^.^=) {} {} (=^.^=)", command, args.join(" "));
                println!("Error: {}", e);
                println!();
            }
        }
    }
}
