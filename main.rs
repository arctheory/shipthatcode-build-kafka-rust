use std::io::{self, BufRead, Write};

// Read PARTITIONS <n>, then KEY <string> lines. Print sum-of-bytes(key) mod n for each KEY.

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut _out = stdout.lock();
    let mut size: u32 = 0;
    for raw in stdin.lock().lines() {
        let line = match raw {
            Ok(s) => s,
            Err(_) => break,
        };
        let line = line.trim_end_matches('\r');
        if line.trim().is_empty() {
            continue;
        }
        // TODO: parse line, update state, emit output via writeln!
        let parts: Vec<&str> = line.split(' ').collect();
        if parts.get(0).unwrap().eq(&"PARTITIONS") {
            size = parts.get(1).unwrap().parse().expect("Invalid number");
        }

        if parts.get(0).unwrap().eq(&"KEY") {
            let value = parts.get(1).unwrap();
            let hash: u32 = value.chars().map(|c| c as u32).sum();
            let value_size = hash % size;
            println!("{}", value_size);
        }
    }
}
