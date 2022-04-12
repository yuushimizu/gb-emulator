pub mod cartridge;
pub mod cpu;
pub mod game_boy;
pub mod interrupt;
pub mod joypad;
pub mod memory;
pub mod ppu;
pub mod serial;
pub mod timer;

mod util;

pub use cartridge::Cartridge;
pub use game_boy::GameBoy;
pub use ppu::{display_size, Color, Renderer, Vec2};
pub use serial::SerialConnection;

pub const CLOCK_CYCLE: u64 = 4194304;
