use super::Operator;
use crate::cpu::{
    instruction::operand::{register, Read, ReadWrite},
    registers::Flags,
};

fn and_u8(format: String, lhs: impl ReadWrite<u8>, rhs: impl Read<u8>) -> Operator {
    Operator::new(format, move |context| {
        let (current, writer) = lhs.prepare_and_read(context);
        let n = rhs.read(context);
        let result = current & n;
        context.set_flags(Flags {
            z: result == 0,
            n: false,
            h: true,
            c: false,
        });
        writer.write(context, result);
    })
}

pub fn and(rhs: impl Read<u8>) -> Operator {
    and_u8(format!("AND {}", rhs), register::A, rhs)
}

fn or_u8(format: String, lhs: impl ReadWrite<u8>, rhs: impl Read<u8>) -> Operator {
    Operator::new(format, move |context| {
        let (current, writer) = lhs.prepare_and_read(context);
        let n = rhs.read(context);
        let result = current | n;
        context.set_flags(Flags {
            z: result == 0,
            n: false,
            h: false,
            c: false,
        });
        writer.write(context, result);
    })
}

pub fn or(rhs: impl Read<u8>) -> Operator {
    or_u8(format!("OR {}", rhs), register::A, rhs)
}

fn xor_u8(format: String, lhs: impl ReadWrite<u8>, rhs: impl Read<u8>) -> Operator {
    Operator::new(format, move |context| {
        let (current, writer) = lhs.prepare_and_read(context);
        let n = rhs.read(context);
        let result = current ^ n;
        context.set_flags(Flags {
            z: result == 0,
            n: false,
            h: false,
            c: false,
        });
        writer.write(context, result);
    })
}

pub fn xor(rhs: impl Read<u8>) -> Operator {
    xor_u8(format!("XOR {}", rhs), register::A, rhs)
}
