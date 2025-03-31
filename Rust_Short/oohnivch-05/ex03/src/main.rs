use std::env;
use std::io::BufRead;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::thread::{sleep, spawn};
use std::time::Duration;

fn philosopher(receiver: Receiver<String>) {
    while let Ok(word) = receiver.recv() {
        println!("the philosopher is thinking about {}", word);
        sleep(Duration::from_secs(5));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run -- <brain_size>");
        std::process::exit(1);
    }
    let brain_size: usize = args[1].parse().expect("Invalid brain size");
    let (sender, receiver): (SyncSender<String>, Receiver<String>) = sync_channel(brain_size);
    spawn(move || philosopher(receiver));
    // STUFF TO CHANGE vvvvvv
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let word = line.expect("Failed to read line");
        match sender.try_send(word) {
            Ok(_) => {},
            Err(_) => println!("the philosophers head is full!").
        }
        //if sender.try_send(word).is_err() {
        //    eprintln!("the philosopher's head is full");
        //}
    }
    // STUFF TO CHANGE ^^^^^^
    drop(sender);
}
