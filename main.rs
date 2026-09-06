use std::{
    collections::BTreeSet,
    io::{self, BufRead, Write},
};

// Read PARTITIONS <n>, then KEY <string> lines. Print sum-of-bytes(key) mod n for each KEY.

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut _out = stdout.lock();
    let mut offset_list: BTreeSet<u32> = BTreeSet::new();
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
        let Some(key) = parts.get(0) else {
            continue;
        };
        let value: u32 = match parts.get(1) {
            Some(_value) => _value.parse().expect("invalid number"),
            None => {
                continue;
            }
        };

        match *key {
            "SEGMENT" => {
                offset_list.insert(value);
            }
            "LOOKUP" => {
                let mut prev_offset = offset_list.iter().min().unwrap();
                for offset in offset_list.iter() {
                    if offset <= &value && offset >= prev_offset {
                        prev_offset = offset;
                    }
                }
                if prev_offset <= &value {
                    println!("{}", prev_offset);
                } else {
                    println!("NOT_FOUND");
                }
            }
            _ => {}
        }
    }
}
