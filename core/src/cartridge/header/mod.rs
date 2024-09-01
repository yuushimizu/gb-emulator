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

pub use cartridge_type::{CartridgeType, MbcType};
pub use cgb_flag::CgbFlag;
pub use destination::Destination;
pub use entry_point::EntryPoint;
pub use global_checksum::GlobalChecksum;
pub use header_checksum::HeaderChecksum;
pub use licensee::Licensee;
pub use logo::Logo;
pub use ram_size::RamSize;
pub use rom_size::RomSize;
pub use sgb_flag::SgbFlag;
pub use title::Title;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    pub entry_point: EntryPoint,
    pub logo: Logo,
    pub cgb_flag: CgbFlag,
    pub title: Title,
    pub licensee: Licensee,
    pub sgb_flag: SgbFlag,
    pub cartridge_type: CartridgeType,
    pub rom_size: RomSize,
    pub ram_size: RamSize,
    pub destination: Destination,
    pub version: u8,
    pub header_checksum: HeaderChecksum,
    pub global_checksum: GlobalChecksum,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Error {
    TooSmall,
}

const MIN_HEADER_LENGTH: usize = 0x014F + 1;

impl Header {
    pub fn load(rom: &[u8]) -> Result<Self, Error> {
        if rom.len() < MIN_HEADER_LENGTH {
            return Err(Error::TooSmall);
        }
        Ok(Self {
            entry_point: EntryPoint::load(rom),
            logo: Logo::load(rom),
            cgb_flag: CgbFlag::load(rom),
            title: Title::load(rom),
            licensee: licensee::Licensee::load(rom),
            sgb_flag: SgbFlag::load(rom),
            cartridge_type: CartridgeType::load(rom),
            rom_size: RomSize::load(rom),
            ram_size: RamSize::load(rom),
            destination: Destination::load(rom),
            version: version::load(rom),
            header_checksum: HeaderChecksum::load(rom),
            global_checksum: GlobalChecksum::load(rom),
        })
    }
}
