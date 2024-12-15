#![allow(unused)]

use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};

fn main() {
    let mut child = Command::new("./target/debug/apery")
        .current_dir("apery_rust")
        .stdin(Stdio::piped())  // Capture stdin to send commands
        .stdout(Stdio::piped()) // Capture stdout to read responses
        .stderr(Stdio::piped()) // Capture stderr for debugging
        .spawn()
        .expect("Failed to start Shogi engine");

    let mut engine_input = child.stdin.take().expect("Failed to open stdin");
    let engine_output = child.stdout.take().expect("Failed to open stdout");

    let reader_thread = thread::spawn(move || {
        let reader = BufReader::new(engine_output);
        for line in reader.lines() {
            match line {
                Ok(output) => println!("Engine: {}", output), 
                Err(e) => eprintln!("Error reading engine output: {}", e),
            }
        }
    });

    writeln!(engine_input, "isready").expect("Failed to write to engine");
    println!("Sent: isready");

    writeln!(engine_input, "position sfen lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1").expect("Failed to write to engine");

    reader_thread.join().unwrap();
}