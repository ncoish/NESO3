mod rom_parser;
mod ppu;

fn main() {
    println!("Hello, world!");
    let filepath = std::path::Path::new("C:/Users/megab/Downloads/nestest.nes");
    let rom = rom_parser::parser::parse_rom(filepath.to_str().unwrap());
    println!("Rom data: {:?}", rom);
}
