use std::env::args;
use std::io::stdin;
use std::io::BufRead;
use std::process::Command;

fn main() {
    let mut args = args().skip(1);
    let input: Vec<String> = stdin().lock().lines().map_while(Result::ok).collect();
    let program = match args.next() {
        Some(prog) => prog,
        None => {
            eprintln!("Error: No program specified");
            return;
        }
    };
    let cmd_args: Vec<String> = args.collect();
    let mut cmd = Command::new(program);
    cmd.args(&cmd_args);
    cmd.args(input);
    let mut child = match cmd.spawn() {
        Ok(child) => child,
        Err(e) => {
            eprintln!("Error: Failed to spawn command: {}", e);
            return;
        }
    };
    let _status = match child.wait() {
        Ok(status) => status,
        Err(e) => {
            eprintln!("Error: Failed to wait for the process: {}", e);
            return;
        }
    };
}
