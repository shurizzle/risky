use crate::{
    decode::{B, I, J, R, S, U},
    instructions::{
        Auipc, Branch, Jal, Jalr, Load, Lui, Math, MathI, MathIW, MathW, ShiftI, ShiftIW, Store,
    },
    num::As,
    ops::Add,
    registers::Registers,
};

const LUI: u8 = 0b0110111;
const AUIPC: u8 = 0b0010111;
const JAL: u8 = 0b1101111;
const JALR: u8 = 0b1100111;
const BRANCH: u8 = 0b1100011;
const LOAD: u8 = 0b0000011;
const STORE: u8 = 0b0100011;
const MATHI: u8 = 0b0010011;
const MATH: u8 = 0b0110011;
const FENCE: u8 = 0b0001111;
const SYSCALL: u8 = 0b1110011;
const MATHIW: u8 = 0b0011011;
const MATHW: u8 = 0b0111011;

pub trait Isa: Sized {
    fn execute(encoded: u32, regs: &mut Registers<Self>, pc: &mut Self, memory: &mut [u8]);
}

impl Isa for u32 {
    fn execute(encoded: u32, regs: &mut Registers<Self>, pc: &mut Self, memory: &mut [u8]) {
        println!("{:#034b} - PC: {:#0x}", encoded, pc);
        let opcode = (encoded & 0b1111111) as u8;
        let f = match opcode {
            x if x > 0b1111111 => unsafe { core::hint::unreachable_unchecked() },
            LUI => lui::<Self>,
            AUIPC => auipc::<Self>,
            JAL => jal::<Self>,
            JALR => jalr::<Self>,
            BRANCH => branch::<Self>,
            LOAD => load::<Self>,
            STORE => store::<Self>,
            MATHI => mathi::<Self>,
            MATH => math::<Self>,
            FENCE => todo!("FENCE"),
            SYSCALL => todo!("SYSCALL"),
            _ => panic!("Invalid OPCode"),
        };

        f(encoded, regs, pc, memory)
    }
}

impl Isa for u64 {
    fn execute(encoded: u32, regs: &mut Registers<Self>, pc: &mut Self, memory: &mut [u8]) {
        println!("{:#034b} - PC: {:#0x}", encoded, pc);
        let opcode = (encoded & 0b1111111) as u8;
        let f = match opcode {
            x if x > 0b1111111 => unsafe { core::hint::unreachable_unchecked() },
            LUI => lui::<Self>,
            AUIPC => auipc::<Self>,
            JAL => jal::<Self>,
            JALR => jalr::<Self>,
            BRANCH => branch::<Self>,
            LOAD => load::<Self>,
            STORE => store::<Self>,
            MATHI => mathi::<Self>,
            MATH => math::<Self>,
            FENCE => todo!("FENCE"),
            SYSCALL => todo!("SYSCALL"),
            MATHIW => mathiw::<Self>,
            MATHW => mathw::<Self>,
            _ => panic!("Invalid OPCode"),
        };

        f(encoded, regs, pc, memory)
    }
}

#[inline(always)]
fn lui<T>(encoded: u32, regs: &mut Registers<T>, pc: &mut T, _: &mut [u8])
where
    T: Lui + Add + Copy,
    u8: As<T>,
{
    let instruction = U::from_u32(encoded);
    println!("{:?}", instruction);
    T::lui(instruction, regs).unwrap();
    *pc = pc.add(4.r#as());
}

#[inline(always)]
fn auipc<T>(encoded: u32, regs: &mut Registers<T>, pc: &mut T, _: &mut [u8])
where
    T: Auipc + Add + Copy,
    u8: As<T>,
{
    let instruction = U::from_u32(encoded);
    println!("{:?}", instruction);
    T::auipc(instruction, regs, *pc).unwrap();
    *pc = pc.add(4.r#as());
}

#[inline(always)]
fn jal<T>(encoded: u32, regs: &mut Registers<T>, pc: &mut T, _: &mut [u8])
where
    T: Jal + Add + Copy,
{
    let instruction = J::from_u32(encoded);
    println!("{:?}", instruction);
    T::jal(instruction, regs, pc).unwrap();
}

#[inline(always)]
fn jalr<T>(encoded: u32, regs: &mut Registers<T>, pc: &mut T, _: &mut [u8])
where
    T: Jalr + Add + Copy,
{
    let instruction = I::from_u32(encoded);
    println!("{:?}", instruction);
    T::jalr(instruction, regs, pc).unwrap();
}

#[inline(always)]
fn branch<T>(encoded: u32, regs: &mut Registers<T>, pc: &mut T, _: &mut [u8])
where
    T: Branch + Add + Copy,
{
    let instruction = B::from_u32(encoded);
    println!("{:?}", instruction);
    T::branch(instruction, regs, pc).unwrap();
}

#[inline(always)]
fn load<T>(encoded: u32, regs: &mut Registers<T>, pc: &mut T, memory: &mut [u8])
where
    T: Load + Add + Copy,
    u8: As<T>,
{
    let instruction = I::from_u32(encoded);
    println!("{:?}", instruction);
    T::load(instruction, regs, memory).unwrap();
    *pc = pc.add(4.r#as());
}

#[inline(always)]
fn store<T>(encoded: u32, regs: &mut Registers<T>, pc: &mut T, memory: &mut [u8])
where
    T: Store + Add + Copy,
    u8: As<T>,
{
    let instruction = S::from_u32(encoded);
    println!("{:?}", instruction);
    T::store(instruction, regs, memory).unwrap();
    *pc = pc.add(4.r#as());
}

#[inline(always)]
fn mathi<T>(encoded: u32, regs: &mut Registers<T>, pc: &mut T, _: &mut [u8])
where
    T: ShiftI + MathI + Add + Copy,
    u8: As<T>,
{
    let instruction = I::from_u32(encoded);
    println!("{:?}", instruction);
    if matches!(instruction.funct3.as_u8(), 0b001 | 0b101) {
        T::shifti(instruction.into(), regs)
    } else {
        T::mathi(instruction, regs)
    }
    .unwrap();
    *pc = pc.add(4.r#as());
}

#[inline(always)]
fn math<T>(encoded: u32, regs: &mut Registers<T>, pc: &mut T, _: &mut [u8])
where
    T: Math + Add + Copy,
    u8: As<T>,
{
    let instruction = R::from_u32(encoded);
    println!("{:?}", instruction);
    T::math(instruction, regs).unwrap();
    *pc = pc.add(4.r#as());
}

#[inline(always)]
fn mathiw<T>(encoded: u32, regs: &mut Registers<T>, pc: &mut T, _: &mut [u8])
where
    T: ShiftIW + MathIW + Add + Copy,
    u8: As<T>,
{
    let instruction = I::from_u32(encoded);
    println!("{:?}", instruction);
    if matches!(instruction.funct3.as_u8(), 0b000 /* ADDIW */) {
        T::mathiw(instruction, regs)
    } else {
        T::shiftiw(instruction.into(), regs)
    }
    .unwrap();
    *pc = pc.add(4.r#as());
}

#[inline(always)]
fn mathw<T>(encoded: u32, regs: &mut Registers<T>, pc: &mut T, _: &mut [u8])
where
    T: MathW + Add + Copy,
    u8: As<T>,
{
    let instruction = R::from_u32(encoded);
    println!("{:?}", instruction);
    T::mathw(instruction, regs).unwrap();
    *pc = pc.add(4.r#as());
}
