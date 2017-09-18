mod parser;
mod ppu;
mod memory;

fn main() {
    println!("Hello, world!");
    let filepath = std::path::Path::new("../test/test_roms/nestest.nes");
    let rom = parser::parse_rom(filepath.to_str().unwrap());
    println!("Rom data: {:?}", rom);
}
