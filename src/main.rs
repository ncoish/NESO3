extern crate clap;

use std::io;
use clap::{Arg, App};

mod parser;
mod ppu;
mod memory;

fn main() {
    let matches = App::new("NESO3")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("NES emulator written in Rust")
        .arg(Arg::with_name("test")
            .short("t")
            .long("test")
            .help("Enter test mode to test various ROMs"))
        .get_matches();

    if matches.is_present("test") {
        let stdin = io::stdin();
        println!("Entering test mode...");
        loop {
            let mut input = String::new();
            println!("Enter ROM filepath:");
            stdin.read_line(&mut input).unwrap();
            let filepath = std::path::Path::new(input.trim());
            let rom = match parser::parse_rom(filepath.to_str().unwrap()) {
                Ok(a) => a,
                Err(e) => {
                    println!("Error: {:?}", e);
                    continue
                }
            };
            println!("Rom data: {:?}", rom);
        }
    }
}
