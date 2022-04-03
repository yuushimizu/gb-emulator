#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RomSizeAmount {
    Unknown,
    Kb32,
    Kb64,
    Kb128,
    Kb256,
    Kb512,
    Mb1,
    Mb2,
    Mb4,
    Mb8,
    Mb1P1,
    Mb1P2,
    Mb1P5,
}

impl From<u8> for RomSizeAmount {
    fn from(code: u8) -> Self {
        use RomSizeAmount::*;
        match code {
            0x00 => Kb32,
            0x01 => Kb64,
            0x02 => Kb128,
            0x03 => Kb256,
            0x04 => Kb512,
            0x05 => Mb1,
            0x06 => Mb2,
            0x07 => Mb4,
            0x08 => Mb8,
            0x52 => Mb1P1,
            0x53 => Mb1P2,
            0x54 => Mb1P5,
            _ => Unknown,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RomSize {
    code: u8,
    amount: RomSizeAmount,
}

impl From<u8> for RomSize {
    fn from(code: u8) -> Self {
        Self {
            code,
            amount: code.into(),
        }
    }
}

const POSITION: usize = 0x0148;

impl RomSize {
    pub fn load(rom_bytes: &[u8]) -> Self {
        rom_bytes[POSITION].into()
    }
}
