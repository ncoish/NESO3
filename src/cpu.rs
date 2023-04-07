use crate::memory::Memory;

pub struct CPU {
    A: u8,
    X: u8,
    Y: u8,
    PC: u16,
    S: u8,
    P: u8,
    pub memory: [u8; 0x10000]
}

impl Memory for CPU {
    fn store(&mut self, address: usize, value: u8) {
        match address {
            a @ 0x0000 ..= 0x1FFF => self.memory[a % 0x0800] = value,
            a @ 0x2000 ..= 0x3FFF => self.memory[0x2000 | (a % 0x0008)] = value,
            a => self.memory[a] = value,
        }
    }

    fn load(&mut self, address: usize) -> u8 {
        match address {
            a @ 0x0000 ..= 0x1FFF => self.memory[a % 0x0800],
            a @ 0x2000 ..= 0x3FFF => self.memory[0x2000 | (a % 0x0008)],
            a => self.memory[a],
        }
    }
}