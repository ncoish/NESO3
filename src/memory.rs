use std::fmt;
use crate::parser::Header;

pub struct GameMemory {
    pub header: Header,
    pub trainer: Option<[u8; 512]>,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub enum Mirroring {
    Horizontal,
    Vertical,
    FourScreen
}

impl fmt::Debug for GameMemory {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let fmt1 = self.header.fmt(formatter);
        write!(formatter, "header: {:?}\ntrainer:{}", fmt1, if self.trainer.is_some() {"Some"} else {"None"})
    }
}

pub trait Memory {
    fn store(&mut self, address: usize, value: u8);
    fn load(&mut self, address: usize) -> u8;
}