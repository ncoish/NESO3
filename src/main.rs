extern crate clap;
extern crate sdl2;
extern crate gl;

use std::io;
use clap::{Arg, App};

mod parser;
mod memory;
mod mapper;
mod cpu;
mod ppu;
mod gfx;
mod util;

fn main() {
    let matches = App::new("NESO3")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("NES emulator written in Rust")
        .arg(Arg::with_name("test")
            .short("t")
            .long("test")
            .help("Enter test mode to test various ROMs"))
        .arg(Arg::with_name("gl")
            .short("g")
            .long("gl")
            .help("Render with OpenGL"))
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

    // let cpu = cpu::CPU {
    //     memory: [0; 0x10000]
    // };

    // let ppu = ppu::PPU {
    //     memory: [0; 0x4000]
    // };

    if matches.is_present("gl") {
        gfx::do_window();
    }
}
