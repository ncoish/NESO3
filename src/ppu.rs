use crate::memory::Memory;

pub struct PPU {
    pub memory: [u8; 0x4000],
    oam: OAM,
}

impl Memory for PPU {
    fn store(&mut self, address: usize, value: u8) {
        match address {
            a @ 0x2000 ..= 0x3EFF => self.memory[0x2000 | (a % 0x1000)] = value,
            a @ 0x3F00 ..= 0x3FFF => self.memory[0x3F00 | (a % 0x0020)] = value,
            a => self.memory[a] = value,
        }
    }

    fn load(&mut self, address: usize) -> u8 {
        match address {
            a @ 0x2000 ..= 0x3EFF => self.memory[0x2000 | (a % 0x1000)],
            a @ 0x3F00 ..= 0x3FFF => self.memory[0x3F00 | (a % 0x0020)],
            a => self.memory[a],
        }
    }
}

struct OAM {
    memory: [u8; 0x100]
}

impl Memory for OAM {
    fn store(&mut self, address: usize, value: u8) {
        self.memory[address] = value;
    }

    fn load(&mut self, address: usize) -> u8 {
        self.memory[address]
    }
}