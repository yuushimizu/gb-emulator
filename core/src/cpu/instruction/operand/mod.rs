pub mod condition;
pub mod indirection;
pub mod literal;
pub mod opcode_register;
pub mod register;
pub mod stack_pointer;

pub use condition::Condition;
pub use indirection::Indirection;
pub use literal::LITERAL;
pub use opcode_register::OpcodeRegister;
pub use register::Register;

use crate::cpu::instruction::Context;

pub trait Operand: 'static + Sync + Send + Copy {}

pub trait Value: 'static + Sized + Copy {}

impl<T: 'static + Sized + Copy> Value for T {}

pub trait DebugOperand<T: Value> {
    fn debug(&self, context: &Context) -> String;
}

pub trait Read<T: Value>: Operand + DebugOperand<T> {
    fn read(&self, context: &mut Context) -> T;
}

pub trait Write<T: Value>: Operand + DebugOperand<T> {
    fn write(&self, context: &mut Context, value: T);
}
