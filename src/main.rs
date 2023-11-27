extern crate regex;
use regex::Regex;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Define the RFC3330 IP ranges as regular expressions
    let rfc3330_ranges = vec![
        r"^(0\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3})$",
        r"^(10\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3})$",
        r"^(127\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3})$",
        r"^(169\.254\.[0-9]{1,3}\.[0-9]{1,3})$",
        r"^(172\.(1[6-9]|2[0-9]|3[0-1])\.[0-9]{1,3}\.[0-9]{1,3})$",
        r"^(192\.0\.0\.[0-9]{1,3})$",
        r"^(192\.0\.2\.[0-9]{1,3})$",
        r"^(192\.88\.99\.[0-9]{1,3})$",
        r"^(192\.168\.[0-9]{1,3}\.[0-9]{1,3})$",
        r"^(198\.1[8-9]\.[0-9]{1,3}\.[0-9]{1,3})$",
        r"^(198\.51\.100\.[0-9]{1,3})$",
        r"^(203\.0\.113\.[0-9]{1,3})$",
        r"^(22[4-9]\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3})$",
        r"^(23[0-9]\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3})$",
    ];

    // Read input lines from stdin
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();

    // Process input lines sequentially
    for line in stdin_lock.lines() {
        if let Ok(line) = line {
            if !line.is_empty() {
                let is_bogon = rfc3330_ranges.iter().any(|re| {
                    let regex = Regex::new(re).unwrap();
                    regex.is_match(&line)
                });

                if !is_bogon {
                    println!("{}", line);
                }
            }
        }
    }

    Ok(())
}

