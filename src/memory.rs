use std::fmt;
use parser::Header;

pub struct GameMemory {
    pub header: Header,
    pub trainer: Option<[u8; 512]>,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
}

impl fmt::Debug for GameMemory {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let fmt1 = self.header.fmt(formatter);
        write!(formatter, "header: {:?}\ntrainer:{}", fmt1, if self.trainer.is_some() {"Some"} else {"None"})
    }
}
