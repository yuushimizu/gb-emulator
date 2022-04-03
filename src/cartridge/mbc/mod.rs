mod mbc1;

pub use super::context::Context;
pub use mbc1::Mbc1;

use std::fmt::Debug;

pub trait Mbc
where
    Self: Debug,
{
    fn read(&self, context: &Context, address: u16) -> u8;

    fn write(&mut self, context: &mut Context, address: u16, value: u8);
}
