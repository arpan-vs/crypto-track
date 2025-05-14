use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Tell cargo to invalidate the build if the .env file changes.
    println!("cargo:rerun-if-changed=.env");

    if Path::new(".env").exists() {
        if let Ok(lines) = read_lines(".env") {
            for line in lines {
                if let Ok(line) = line {
                    let parts: Vec<&str> = line.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        let key = parts[0].trim();
                        let value = parts[1].trim();
                        println!("cargo:rustc-env={}={}", key, value);
                    }
                }
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
