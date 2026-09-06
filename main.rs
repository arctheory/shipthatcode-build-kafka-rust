use std::io::{self, BufRead, Write};

// Read PARTITIONS <n>, then KEY <string> lines. Print sum-of-bytes(key) mod n for each KEY.

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut _out = stdout.lock();
    for raw in stdin.lock().lines() {
        let line = match raw { Ok(s) => s, Err(_) => break };
        let line = line.trim_end_matches('\r');
        if line.trim().is_empty() { continue; }
        // TODO: parse line, update state, emit output via writeln!
        let _ = line;
    }
}
