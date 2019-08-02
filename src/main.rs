use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use clap::{App, Arg};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("targets")
             .required(true)
             .multiple(true)
             .index(1))
        .get_matches();

    // "targets" is required, so we can safely unwrap here
    let targets = matches.values_of("targets").unwrap();
    for target in targets {
        let file_path = Path::new(target);

        let mut file = match File::open(&file_path) {
            Ok(file) => file,
            Err(why) => {
                println!("Couldn't open file {}: {}", file_path.display(),
                    why.description());
                continue;
            },
        };
        let mut lines = 0;
        let mut characters = 0;
        let reader = BufReader::new(&mut file);
        for line in reader.lines() {
            if let Ok(line) = line {
                lines += 1;
                characters += line.chars().count();
            }
        }
        let file_size = match file.metadata() {
            Err(_) => 0,
            Ok(meta) => meta.len()
        };
        println!("{}: {} lines, {} characters, {} bytes",
            target,
            lines,
            characters,
            file_size);
    }
}
