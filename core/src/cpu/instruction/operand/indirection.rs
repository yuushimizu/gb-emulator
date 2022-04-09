use super::{Operand, Read, Write};
use crate::cpu::instruction::Context;
use std::fmt;

#[derive(Clone, Copy)]
pub struct Indirection {
    name: &'static str,
    address: fn(&mut Context) -> u16,
}

impl fmt::Debug for Indirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Indirection")
            .field("name", &self.name)
            .finish()
    }
}

impl fmt::Display for Indirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.name)
    }
}

impl Operand for Indirection {}

impl Read<u8> for Indirection {
    fn read(&self, context: &mut Context) -> u8 {
        let address = (self.address)(context);
        context.read(address)
    }
}

impl Write<u8> for Indirection {
    fn write(&self, context: &mut Context, value: u8) {
        let address = (self.address)(context);
        context.write(address, value);
    }
}

impl Read<u16> for Indirection {
    fn read(&self, context: &mut Context) -> u16 {
        let address = (self.address)(context);
        context.read16(address)
    }
}

impl Write<u16> for Indirection {
    fn write(&self, context: &mut Context, value: u16) {
        let address = (self.address)(context);
        context.write16(address, value);
    }
}

macro_rules! register {
    ($name: ident, $field: ident) => {
        pub const $name: Indirection = Indirection {
            name: stringify!($name),
            address: |context| context.registers().$field(),
        };
    };
}

register!(BC, bc);
register!(DE, bc);
register!(HL, bc);

pub const LITERAL: Indirection = Indirection {
    name: "nn",
    address: |context| context.fetch16(),
};

pub const LITERAL_8: Indirection = Indirection {
    name: "$FF00+n",
    address: |context| 0xFF00 | context.fetch() as u16,
};

pub const C: Indirection = Indirection {
    name: "C",
    address: |context| 0xFF00 | context.registers().c as u16,
};

pub const HLD: Indirection = Indirection {
    name: "HLD",
    address: |context| {
        let address = context.registers().hl();
        context.registers_mut().set_hl(address.wrapping_sub(1));
        address
    },
};

pub const HLI: Indirection = Indirection {
    name: "HLI",
    address: |context| {
        let address = context.registers().hl();
        context.registers_mut().set_hl(address.wrapping_add(1));
        address
    },
};
