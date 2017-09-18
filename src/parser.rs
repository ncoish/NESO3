//! Parser

use std::error;
use std::fs::File;
use std::io::{BufReader, Read};
use ppu;
use memory::{GameMemory};

#[derive(Debug, PartialEq)]
pub enum TVSystem {
    NTSC,
    PAL,
    DUAL
}

#[derive(Debug)]
pub struct Header {
    raw_header: [u8; 16],
    prg_rom_banks: u8,
    chr_rom_banks: u8,
    mirroring: ppu::Mirror,
    battery_ram: bool,
    trainer: bool,
    mapper_num: u8,
    ram_banks: u8,
    tv_system: TVSystem,
}

// TODO: Implement check for different filetypes. (i.e. NESM\x1A for NES music files)
fn parse_header(game_contents: &mut BufReader<File>) -> Result<Header, Box<error::Error>> {
    let mut buf = [0; 16];
    game_contents.read_exact(&mut buf)?;
    if !(&buf[0..4] == b"NES\x1A") {
        Err(From::from("ROM file does not begin with 'NES' followed by MS-DOS end-of-file ($1A)"))
    } else {
        let rc1 = buf[6];
        let rc2 = buf[7];
        let vert_mirror_bit = 0b0000_0001;
        let battery_ram_bit = 0b0000_0010;
        let trainer_bit     = 0b0000_0100;
        let four_mirror_bit = 0b0000_1000;

        let header = Header {
            raw_header: buf,
            prg_rom_banks: buf[4],
            chr_rom_banks: buf[5],
            mirroring: if (rc1 & four_mirror_bit) == four_mirror_bit {
                ppu::Mirror::FourScreen
            } else if (rc1 & vert_mirror_bit) == vert_mirror_bit {
                ppu::Mirror::Vertical
            } else {
                ppu::Mirror::Horizontal
            },
            battery_ram: (rc1 & battery_ram_bit) == battery_ram_bit,
            trainer: (rc1 & trainer_bit) == trainer_bit,
            mapper_num: (rc2 & 0b1111_0000) | (rc1 >> 4),
            ram_banks: buf[8],
            tv_system: match buf[9] & 0b0000_0001 {
                0 => TVSystem::NTSC,
                1 => TVSystem::PAL,
                _ => return Err(From::from("Invalid TV System type"))
            }
        };
        Ok(header)
    }
}

pub fn parse_rom(filename: &str) -> Result<GameMemory, Box<error::Error>> {
    // Read in .nes file
    let rom_file = File::open(filename)?;
    let mut contents = BufReader::new(rom_file);
    let header = parse_header(&mut contents).unwrap();
    let trainer = match header.trainer {
        true => {
            let mut buf = [0; 512];
            contents.read_exact(&mut buf)?;
            Some(buf)
        },
        false => None
    };
    let mut prg_rom = Vec::new();
    let mut buf = [0; 16_384];
    for _ in 0..header.prg_rom_banks {
        contents.read_exact(&mut buf)?;
        prg_rom.push(buf);
    }
    let mut chr_rom = Vec::new();
    let mut buf = [0; 8_192];
    for _ in 0..header.chr_rom_banks {
        contents.read_exact(&mut buf)?;
        chr_rom.push(buf);
    }
    let game_memory = GameMemory {
        header,
        trainer,
        prg_rom,
        chr_rom,
    };
    Ok(game_memory)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DW_PATH: &'static str = "test/test_roms/Dragon Warrior (USA).nes";

    #[test]
    fn correctly_formatted_header_is_parsed_correctly() {
        let memory = parse_rom(DW_PATH).unwrap();
        let header = &memory.header;
        assert_eq!(header.mirroring, ppu::Mirror::Horizontal);
        assert_eq!(header.battery_ram, true);
        assert_eq!(header.mapper_num, 1);
        assert_eq!(header.prg_rom_banks, 4);
        assert_eq!(header.chr_rom_banks, 2);
        println!("Rom data: {:?}", memory);
    }
}
