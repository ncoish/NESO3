
#[derive(Debug)]
pub enum Mapper {
    NROM,
    MMC1,
    UxROM,
    CNROM,
    MMC3,
    // TODO: Add more mappers
}

impl Mapper {
    pub fn from_number(ines_number: u8) -> Result<Self, String> {
        let value = match ines_number {
            0 => Mapper::NROM,
            1 => Mapper::MMC1,
            2 => Mapper::UxROM,
            3 => Mapper::CNROM,
            4 => Mapper::MMC3,
            n => return Err(From::from(format!("Mapper unsupported: {}", n)))
        };
        Ok(value)
    }
    pub fn get_number(&self) -> u8 {
        match *self {
            Mapper::NROM => 0,
            Mapper::MMC1 => 1,
            Mapper::UxROM => 2,
            Mapper::CNROM => 3,
            Mapper::MMC3 => 4,
        }
    }
}