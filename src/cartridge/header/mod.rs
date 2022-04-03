mod cartridge_type;
mod cgb_flag;
mod destination;
mod entry_point;
mod global_checksum;
mod header_checksum;
mod licensee;
mod logo;
mod ram_size;
mod rom_size;
mod sgb_flag;
mod title;
mod version;

pub use cartridge_type::{CartridgeType, MBCType};
pub use cgb_flag::CGBFlag;
pub use destination::Destination;
pub use entry_point::EntryPoint;
pub use global_checksum::GlobalChecksum;
pub use licensee::Licensee;
pub use logo::Logo;
pub use ram_size::RamSize;
pub use rom_size::RomSize;
pub use sgb_flag::SGBFlag;
pub use title::Title;

use std::result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    pub entry_point: EntryPoint,
    pub logo: Logo,
    pub cgb_flag: CGBFlag,
    pub title: Title,
    pub licensee: Licensee,
    pub sgb_flag: SGBFlag,
    pub cartridge_type: CartridgeType,
    pub rom_size: RomSize,
    pub ram_size: RamSize,
    pub destination: Destination,
    pub version: u8,
    pub header_checksum: u8,
    pub global_checksum: GlobalChecksum,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Error {
    TooSmall,
}

pub type Result<T> = result::Result<T, Error>;

const MIN_HEADER_LENGTH: usize = 0x14F + 1;

impl Header {
    pub fn load(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < MIN_HEADER_LENGTH {
            Err(Error::TooSmall)?
        }
        Ok(Self {
            entry_point: EntryPoint::load(bytes),
            logo: Logo::load(bytes),
            cgb_flag: CGBFlag::load(bytes),
            title: Title::load(bytes),
            licensee: licensee::Licensee::load(bytes),
            sgb_flag: SGBFlag::load(bytes),
            cartridge_type: CartridgeType::load(bytes),
            rom_size: RomSize::load(bytes),
            ram_size: RamSize::load(bytes),
            destination: Destination::load(bytes),
            version: version::load(bytes),
            header_checksum: header_checksum::load(bytes),
            global_checksum: GlobalChecksum::load(bytes),
        })
    }
}