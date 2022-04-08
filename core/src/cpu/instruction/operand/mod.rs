pub mod indirection;
pub mod literal;
pub mod opcode_register;
pub mod register;
pub mod stack_pointer;

pub use indirection::Indirection;
pub use literal::LITERAL;
pub use opcode_register::OpcodeRegister;
pub use register::Register;

use crate::cpu::CpuContext;
use std::fmt;

pub trait Operand: 'static + Sync + Send + Copy + fmt::Display + fmt::Debug {}

pub trait Value: 'static + Sized + Copy {}

impl<T: 'static + Sized + Copy> Value for T {}

pub trait Read<T: Value>: Operand {
    fn read(&self, context: &mut CpuContext) -> T;
}

pub struct Writer<T: Value> {
    write: Box<dyn FnOnce(&mut CpuContext, T)>,
}

impl<T: Value> Writer<T> {
    pub fn write(self, context: &mut CpuContext, value: T) {
        (self.write)(context, value);
    }

    pub fn new(f: impl FnOnce(&mut CpuContext, T) + 'static) -> Self {
        Self { write: Box::new(f) }
    }
}

pub trait Write<T: Value>: Operand {
    fn prepare(&self, context: &mut CpuContext) -> Writer<T>;
}

pub trait ReadWrite<T: Value>: Operand {
    fn prepare_and_read(&self, context: &mut CpuContext) -> (T, Writer<T>);
}
