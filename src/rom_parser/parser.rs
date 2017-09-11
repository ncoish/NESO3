//! Parser

use std::error;
use std::fs::File;
use std::io::{BufReader, Read};
use ppu;

#[derive(Debug)]
pub struct HeaderConfig {
    raw_header: [u8; 16],
    format: u8,
    prg_rom_banks: u8,
    vrom_banks: u8,
    mirroring: ppu::Mirror,
    battery_ram: bool,
    trainer: bool,
    mapper_num: u8,
    ram_banks: u8,
}

#[derive(Debug)]
pub struct GameMemory {
    header: HeaderConfig,
}

fn parse_header(game_contents: &mut BufReader<File>) -> Result<HeaderConfig, Box<error::Error>> {
    let mut buf = [0; 16];
    game_contents.read_exact(&mut buf)?;
    println!("Hello: {:?}", buf);
    if !(&buf[0..3] == b"NES") {
        Err(From::from("ROM file does not begin with 'NES'"))
    } else {
        let rc1 = buf[6];
        let rc2 = buf[7];
        let vert_mirror_bit = 0b1000_0000;
        let battery_ram_bit = 0b0100_0000;
        let trainer_bit     = 0b0010_0000;
        let four_mirror_bit = 0b0001_0000;
        let header = HeaderConfig {
            raw_header: buf,
            format: buf[3],
            prg_rom_banks: buf[4],
            vrom_banks: buf[5],
            mirroring: if (rc1 & four_mirror_bit) == four_mirror_bit { 
                ppu::Mirror::FourScreen 
            } else if (rc1 & vert_mirror_bit) == vert_mirror_bit {
                ppu::Mirror::Vertical
            } else {
                ppu::Mirror::Horizontal
            },
            battery_ram: (rc1 & battery_ram_bit) == battery_ram_bit,
            trainer: (rc1 & trainer_bit) == trainer_bit,
            mapper_num: (rc2 << 4) + (rc1 & 0b0000_1111),
            ram_banks: buf[8],
        };
        Ok(header)
    }
}

pub fn parse_rom(filename: &str) -> Result<GameMemory, Box<error::Error>> {
    // Read in .nes file
    let rom_file = File::open(filename)?;
    let mut contents = BufReader::new(rom_file);
    let header = parse_header(&mut contents).unwrap();
    Ok(GameMemory { header })
}