use std::cmp::Ordering;

use crate::error::Error;
use crate::libmem;
use crate::registers::{Registers, ZeroOrRegister};

pub(crate) struct RType {
    pub rd: u32,
    pub funct3: u32,
    pub rs1: u32,
    pub rs2: u32,
    pub funct7: u32,
}

pub(crate) struct IeType {
    pub rd: u32,
    pub funct3: u32,
    pub rs1: u32,
    pub immediate: u32,
}

pub(crate) struct IsType {
    pub rd: u32,
    pub funct3: u32,
    pub rs1: u32,
    pub imm_shamt: u32,
    pub imm_id: u32,
}

pub(crate) struct SType {
    pub funct3: u32,
    pub rs1: u32,
    pub rs2: u32,
    pub immediate: u32,
}

pub(crate) struct BType {
    pub funct3: u32,
    pub rs1: u32,
    pub rs2: u32,
    pub immediate: u32,
}

pub(crate) struct UType {
    pub rd: u32,
    pub immediate: u32,
}

pub(crate) struct JType {
    pub rd: u32,
    pub immediate: u32,
}

impl RType {
    pub(crate) fn new(rd: u32, funct3: u32, rs1: u32, rs2: u32, funct7: u32) -> Self {
        Self {
            rd,
            funct3,
            rs1,
            rs2,
            funct7,
        }
    }

    #[inline(always)]
    pub(crate) fn id(&self) -> u32 {
        self.funct3 + self.funct7
    }
}

impl IeType {
    pub(crate) fn new(rd: u32, funct3: u32, rs1: u32, immediate: u32) -> Self {
        Self {
            rd,
            funct3,
            rs1,
            immediate,
        }
    }

    #[inline(always)]
    pub(crate) fn id(&self) -> u32 {
        self.funct3
    }
}

impl IsType {
    pub(crate) fn new(rd: u32, funct3: u32, rs1: u32, imm_shamt: u32, imm_id: u32) -> Self {
        Self {
            rd,
            funct3,
            rs1,
            imm_shamt,
            imm_id,
        }
    }

    #[inline(always)]
    pub(crate) fn id(&self) -> u32 {
        self.funct3 + self.imm_id
    }
}

impl SType {
    pub(crate) fn new(funct3: u32, rs1: u32, rs2: u32, immediate: u32) -> Self {
        Self {
            funct3,
            rs1,
            rs2,
            immediate,
        }
    }

    pub(crate) fn id(&self) -> u32 {
        self.funct3
    }
}

impl BType {
    pub(crate) fn new(funct3: u32, rs1: u32, rs2: u32, immediate: u32) -> Self {
        Self {
            funct3,
            rs1,
            rs2,
            immediate,
        }
    }

    pub(crate) fn id(&self) -> u32 {
        self.immediate
    }
}

impl UType {
    pub(crate) fn new(rd: u32, immediate: u32) -> Self {
        Self { rd, immediate }
    }
}

impl JType {
    pub(crate) fn new(rd: u32, immediate: u32) -> Self {
        Self { rd, immediate }
    }
}

pub(crate) fn decode_r(encoded: u32) -> RType {
    let rd = bit_extract(encoded, 7, 11);
    let funct3 = bit_extract(encoded, 12, 14);
    let rs1 = bit_extract(encoded, 15, 19);
    let rs2 = bit_extract(encoded, 20, 24);
    let funct7 = bit_extract(encoded, 25, 31);
    RType::new(rd, funct3, rs1, rs2, funct7)
}

pub(crate) fn decode_ie(encoded: u32) -> IeType {
    let rd = bit_extract(encoded, 7, 11);
    let funct3 = bit_extract(encoded, 12, 14);
    let rs1 = bit_extract(encoded, 15, 19);
    let immediate = bit_extract(encoded, 20, 31);
    IeType::new(rd, funct3, rs1, immediate)
}

pub(crate) fn decode_is(encoded: u32) -> IsType {
    let rd = bit_extract(encoded, 7, 11);
    let funct3 = bit_extract(encoded, 12, 14);
    let rs1 = bit_extract(encoded, 15, 19);
    let shamt = bit_extract(encoded, 20, 24);
    let immediate = bit_extract(encoded, 25, 32);
    IsType::new(rd, funct3, rs1, shamt, immediate)
}

pub(crate) fn decode_s(encoded: u32) -> SType {
    let imm_5 = bit_extract(encoded, 7, 11);
    let funct3 = bit_extract(encoded, 12, 14);
    let rs1 = bit_extract(encoded, 15, 19);
    let rs2 = bit_extract(encoded, 20, 24);
    let imm_7 = bit_extract(encoded, 25, 32);
    let immediate = (imm_7 << 5) | imm_5;
    SType::new(funct3, rs1, rs2, immediate)
}

pub(crate) fn decode_b(encoded: u32) -> BType {
    let imm_11 = bit_extract(encoded, 7, 8);
    let imm_1_4 = bit_extract(encoded, 8, 11);
    let funct3 = bit_extract(encoded, 12, 14);
    let rs1 = bit_extract(encoded, 15, 19);
    let rs2 = bit_extract(encoded, 20, 24);
    let imm_5_10 = bit_extract(encoded, 25, 30);
    let imm_12 = bit_extract(encoded, 31, 32);
    let immediate = (((((imm_1_4 << 6) | imm_5_10) << 1) | imm_11) << 1) | imm_12;
    BType::new(funct3, rs1, rs2, immediate)
}

pub(crate) fn decode_u(encoded: u32) -> UType {
    let rd = bit_extract(encoded, 7, 11);
    let immediate = bit_extract(encoded, 12, 31);
    UType::new(rd, immediate)
}

pub(crate) fn decode_j(encoded: u32) -> JType {
    let rd = bit_extract(encoded, 7, 11);
    let imm_12_19 = bit_extract(encoded, 12, 19);
    let imm_11 = bit_extract(encoded, 20, 21);
    let imm_1_10 = bit_extract(encoded, 21, 30);
    let imm_20 = bit_extract(encoded, 31, 32);
    let immediate = (((((imm_1_10 << 1) | imm_11) << 8) | imm_12_19) << 1) | imm_20;
    JType::new(rd, immediate)
}

#[inline(always)]
pub(crate) fn execute_math(instruction: RType, regs: &mut Registers<u32>) -> Result<(), Error> {
    match instruction.id() {
        0 => {
            // add
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1 as u8) }.fetch(regs);
            let src2 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2 as u8) }.fetch(regs);
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd as u8) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            *dest = src1.wrapping_add(src2);
        }
        1 => {
            // SLL (rs2 truncated)
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1 as u8) }.fetch(regs);
            let src2 = unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2 as u8) }
                .fetch(regs)
                & 0b11111;
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd as u8) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            *dest = src1.wrapping_shl(src2);
        }
        2 | 3 => {
            // SLT/U
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1 as u8) }.fetch(regs);
            let src2 = unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2 as u8) }
                .fetch(regs)
                & 0b11111;
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd as u8) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            match src1.cmp(&src2) {
                Ordering::Less => *dest = 1,
                _ => *dest = 0,
            }
        }
        4 => {
            // XOR
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1 as u8) }.fetch(regs);
            let src2 = unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2 as u8) }
                .fetch(regs)
                & 0b11111;
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd as u8) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            *dest = src1 ^ src2
        }
        5 => {
            // SRL (rs2 truncated)
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1 as u8) }.fetch(regs);
            let src2 = unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2 as u8) }
                .fetch(regs)
                & 0b11111;
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd as u8) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            *dest = src1.wrapping_shr(src2);
        }
        6 => {
            // OR
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1 as u8) }.fetch(regs);
            let src2 = unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2 as u8) }
                .fetch(regs)
                & 0b11111;
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd as u8) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            *dest = src1 | src2
        }
        7 => {
            // AND
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1 as u8) }.fetch(regs);
            let src2 = unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2 as u8) }
                .fetch(regs)
                & 0b11111;
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd as u8) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            *dest = src1 & src2
        }
        32 => {
            // SUB
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1 as u8) }.fetch(regs);
            let src2 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2 as u8) }.fetch(regs);
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd as u8) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            *dest = src1.wrapping_sub(src2);
        }
        37 => {
            // SRA (rs2 truncated)
            let src1: i32 = unsafe {
                core::mem::transmute(
                    ZeroOrRegister::decode_unchecked(instruction.rs1 as u8).fetch(regs),
                )
            };
            let src2 = unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2 as u8) }
                .fetch(regs)
                & 0b11111;
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd as u8) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            *dest = src1.wrapping_shr(src2) as u32;
        }
        _ => return Err(Error::InvalidOpCode),
    };
    Ok(())
}

#[inline(always)]
pub(crate) fn execute_mathi(instruction: IeType, regs: &mut Registers<u32>) -> Result<(), Error> {
    match instruction.id() {
        0 => {
            // ADDI
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1 as u8) }.fetch(regs);
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd as u8) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            *dest = src1.wrapping_add(instruction.immediate);
        }
        2 => {
            // SLTI
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1 as u8) }.fetch(regs);
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd as u8) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            let immediate_signed =
                unsafe { sign_extend(core::mem::transmute(instruction.immediate), 12) };
            let src1_signed: i32 = unsafe { core::mem::transmute(src1) };
            match src1_signed.cmp(&immediate_signed) {
                Ordering::Less => *dest = 1,
                _ => *dest = 0,
            }
        }
        3 => {
            // SLTIU
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1 as u8) }.fetch(regs);
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd as u8) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            match src1.cmp(&instruction.immediate) {
                Ordering::Less => *dest = 1,
                _ => *dest = 0,
            }
        }
        4 => {
            // XORI
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1 as u8) }.fetch(regs);
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd as u8) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            *dest = src1 ^ instruction.immediate;
        }
        6 => {
            // ORI
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1 as u8) }.fetch(regs);
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd as u8) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            *dest = src1 | instruction.immediate;
        }
        7 => {
            // ANDI
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1 as u8) }.fetch(regs);
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd as u8) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            *dest = src1 & instruction.immediate;
        }
        _ => return Err(Error::InvalidOpCode),
    }
    Ok(())
}

// TODO: FIX memrxx calls (now reading from empty slice)
#[inline(always)]
pub(crate) fn execute_load(instruction: IeType, regs: &mut Registers<u32>) -> Result<(), Error> {
    match instruction.id() {
        0 | 4 => {
            // LB
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1 as u8) }.fetch(regs);
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd as u8) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            let addr = src1 + instruction.immediate;
            *dest = u32::from_le_bytes(libmem::memr32(&[], addr as usize)?);
        }
        1 | 5 => {
            // LH
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1 as u8) }.fetch(regs);
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd as u8) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            let addr = src1 + instruction.immediate;
            *dest = u16::from_le_bytes(libmem::memr16(&[], addr as usize)?) as u32;
        }
        2 => {
            // LW
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1 as u8) }.fetch(regs);
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd as u8) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            let addr = src1 + instruction.immediate;
            *dest = libmem::memr8(&[], addr as usize)? as u32;
        }
        _ => return Err(Error::InvalidOpCode),
    }
    Ok(())
}

#[inline(always)]
pub(crate) fn execute_jalr(
    instruction: IeType,
    regs: &mut Registers<u32>,
    pc: &mut u32,
) -> Result<(), Error> {
    let src1 = unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1 as u8) }.fetch(regs);
    let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd as u8) }
        .fetch_mut(regs)
        .ok_or(Error::InvalidOpCode)?;
    *dest = *pc + 4;
    *pc += src1 + sign_extend(unsafe { core::mem::transmute(instruction.immediate) }, 12) as u32;
    Ok(())
}

#[inline(always)]
pub(crate) fn execute_shifti(instruction: IsType, regs: &mut Registers<u32>) -> Result<(), Error> {
    match instruction.id() {
        1 => {
            // SLLI
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1 as u8) }.fetch(regs);
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd as u8) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            *dest = src1.wrapping_shl(instruction.imm_shamt);
        }
        5 => {
            // SRLI
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1 as u8) }.fetch(regs);
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd as u8) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            *dest = src1.wrapping_shr(instruction.imm_shamt);
        }
        68 => {
            // SRAI
            let src1: i32 = unsafe {
                core::mem::transmute(
                    ZeroOrRegister::decode_unchecked(instruction.rs1 as u8).fetch(regs),
                )
            };
            let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd as u8) }
                .fetch_mut(regs)
                .ok_or(Error::InvalidOpCode)?;
            *dest = unsafe { core::mem::transmute(src1.wrapping_shr(instruction.imm_shamt)) };
        }
        _ => return Err(Error::InvalidOpCode),
    }
    Ok(())
}

#[inline(always)]
pub(crate) fn execute_store(instruction: SType, regs: &mut Registers<u32>) -> Result<(), Error> {
    match instruction.id() {
        0 => {
            // SB
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1 as u8) }.fetch(regs);
            let src2 = unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2 as u8) }
                .fetch(regs) as u8;
            let addr = src1 + instruction.immediate;
            libmem::memw(&src2.to_le_bytes(), &mut [], addr as usize)?;
        }
        1 => {
            // SH
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1 as u8) }.fetch(regs);
            let src2 = unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2 as u8) }
                .fetch(regs) as u16;
            let addr = src1 + instruction.immediate;
            libmem::memw(&src2.to_le_bytes(), &mut [], addr as usize)?;
        }
        2 => {
            // SW
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1 as u8) }.fetch(regs);
            let src2 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2 as u8) }.fetch(regs);
            let addr = src1 + instruction.immediate;
            libmem::memw(&src2.to_le_bytes(), &mut [], addr as usize)?;
        }
        _ => return Err(Error::InvalidOpCode),
    }
    Ok(())
}

#[inline(always)]
pub(crate) fn execute_branch(
    instruction: BType,
    regs: &mut Registers<u32>,
    pc: &mut u32,
) -> Result<(), Error> {
    match instruction.id() {
        0 => {
            // BEQ
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1 as u8) }.fetch(regs);
            let src2 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2 as u8) }.fetch(regs);
            if src1 == src2 {
                let offset = unsafe {
                    sign_extend(core::mem::transmute::<u32, i32>(instruction.immediate), 13)
                };
                *pc += unsafe { core::mem::transmute::<i32, u32>(offset) };
            }
        }
        1 => {
            // BNE
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1 as u8) }.fetch(regs);
            let src2 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2 as u8) }.fetch(regs);
            match src1.cmp(&src2) {
                Ordering::Less | Ordering::Greater => {
                    let offset = unsafe {
                        sign_extend(core::mem::transmute::<u32, i32>(instruction.immediate), 13)
                    };
                    *pc += unsafe { core::mem::transmute::<i32, u32>(offset) };
                }
                _ => {}
            }
        }
        4 | 6 => {
            // BLT/U
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1 as u8) }.fetch(regs);
            let src2 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2 as u8) }.fetch(regs);
            if src1 < src2 {
                let offset = unsafe {
                    sign_extend(core::mem::transmute::<u32, i32>(instruction.immediate), 13)
                };
                *pc += unsafe { core::mem::transmute::<i32, u32>(offset) };
            }
        }
        5 | 7 => {
            // BGE/U
            let src1 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs1 as u8) }.fetch(regs);
            let src2 =
                unsafe { ZeroOrRegister::decode_unchecked(instruction.rs2 as u8) }.fetch(regs);
            match src1.cmp(&src2) {
                Ordering::Equal | Ordering::Greater => {
                    let offset = unsafe {
                        sign_extend(core::mem::transmute::<u32, i32>(instruction.immediate), 13)
                    };
                    *pc += unsafe { core::mem::transmute::<i32, u32>(offset) };
                }
                _ => {}
            }
        }
        _ => return Err(Error::InvalidOpCode),
    }
    Ok(())
}

#[inline(always)]
pub(crate) fn execute_lui(instruction: UType, regs: &mut Registers<u32>) -> Result<(), Error> {
    let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd as u8) }
        .fetch_mut(regs)
        .ok_or(Error::InvalidOpCode)?;
    *dest = instruction.immediate.wrapping_shl(12);
    Ok(())
}

#[inline(always)]
pub(crate) fn execute_auipc(
    instruction: UType,
    regs: &mut Registers<u32>,
    pc: u32,
) -> Result<(), Error> {
    let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd as u8) }
        .fetch_mut(regs)
        .ok_or(Error::InvalidOpCode)?;
    *dest = pc + instruction.immediate.wrapping_shl(12);
    Ok(())
}

#[inline(always)]
pub(crate) fn execute_jal(
    instruction: JType,
    regs: &mut Registers<u32>,
    pc: &mut u32,
) -> Result<(), Error> {
    let dest = unsafe { ZeroOrRegister::decode_unchecked(instruction.rd as u8) }
        .fetch_mut(regs)
        .ok_or(Error::InvalidOpCode)?;
    *dest = *pc + 4;
    let offset =
        unsafe { sign_extend(core::mem::transmute::<u32, i32>(instruction.immediate), 20) };
    *pc += unsafe { core::mem::transmute::<i32, u32>(offset) };
    Ok(())
}

#[non_exhaustive]
pub(crate) enum InstructionType {
    R,
    Ie,
    Is,
    S,
    B,
    U,
    J,
    Fence,
    System,
}

#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub(crate) enum InstructionKind {
    Lui,
    Auipc,
    Jal,
    Jalr,
    Branch,
    Load,
    Store,
    MathI,
    ShiftI,
    Math,
    Fence,
    System,
}

impl TryFrom<u32> for InstructionKind {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match bit_extract(value, 0, 6) {
            0b0110111 => Ok(InstructionKind::Lui),
            0b0010111 => Ok(InstructionKind::Auipc),
            0b1101111 => Ok(InstructionKind::Jal),
            0b1100111 => Ok(InstructionKind::Jalr),
            0b1100011 => Ok(InstructionKind::Branch),
            0b0000011 => Ok(InstructionKind::Load),
            0b0100011 => Ok(InstructionKind::Store),
            0b0010011 => {
                let funct3 = bit_extract(value, 12, 14);
                if funct3 == 0b001 || funct3 == 0b101 {
                    Ok(InstructionKind::ShiftI)
                } else {
                    Ok(InstructionKind::MathI)
                }
            }
            0b0110011 => Ok(InstructionKind::Math),
            0b0001111 => Ok(InstructionKind::Fence),
            0b1110011 => Ok(InstructionKind::System),
            _ => Err(Error::InvalidOpCode),
        }
    }
}

impl From<InstructionKind> for InstructionType {
    fn from(value: InstructionKind) -> Self {
        match value {
            InstructionKind::Lui | InstructionKind::Auipc => Self::U,
            InstructionKind::Jal => Self::J,
            InstructionKind::Jalr | InstructionKind::Load | InstructionKind::MathI => Self::Ie,
            InstructionKind::ShiftI => InstructionType::Is,
            InstructionKind::Branch => Self::B,
            InstructionKind::Store => Self::S,
            InstructionKind::Math => Self::R,
            InstructionKind::Fence => Self::Fence,
            InstructionKind::System => Self::System,
        }
    }
}

#[inline(always)]
const fn bit_extract(src: u32, lo: u32, hi: u32) -> u32 {
    (src >> lo) & ((2 << (hi - lo + 1)) - 1)
}

fn sign_extend(x: i32, n: u32) -> i32 {
    let other_bits: u32 = core::mem::size_of::<i32>() as u32 * 8 - n;
    x.wrapping_shl(other_bits).wrapping_shr(other_bits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_extract() {
        assert_eq!(bit_extract(240, 4, 5), 3);
    }

    #[test]
    fn test_bit_extract_single_bit() {
        let n = 136;
        let r = bit_extract(n, 3, 4);
        assert_eq!(r, 1);
    }

    #[test]
    fn test_instructionkind_from_u32_01() {
        let opcode: u32 = 0b0110111;
        let instruction_kind: InstructionKind = opcode.try_into().unwrap();
        assert_eq!(instruction_kind, InstructionKind::Lui);
    }

    #[test]
    fn test_instructionkind_from_u32_02() {
        let opcode: u32 = 0b0010111;
        let instruction_kind: InstructionKind = opcode.try_into().unwrap();
        assert_eq!(instruction_kind, InstructionKind::Auipc);
    }

    #[test]
    fn test_instructionkind_from_u32_03() {
        let opcode: u32 = 0b1101111;
        let instruction_kind: InstructionKind = opcode.try_into().unwrap();
        assert_eq!(instruction_kind, InstructionKind::Jal);
    }

    #[test]
    fn test_instructionkind_from_u32_04() {
        let opcode: u32 = 0b1100111;
        let instruction_kind: InstructionKind = opcode.try_into().unwrap();
        assert_eq!(instruction_kind, InstructionKind::Jalr);
    }

    #[test]
    fn test_instructionkind_from_u32_05() {
        let opcode: u32 = 0b1100011;
        let instruction_kind: InstructionKind = opcode.try_into().unwrap();
        assert_eq!(instruction_kind, InstructionKind::Branch);
    }

    #[test]
    fn test_instructionkind_from_u32_06() {
        let opcode: u32 = 0b0000011;
        let instruction_kind: InstructionKind = opcode.try_into().unwrap();
        assert_eq!(instruction_kind, InstructionKind::Load);
    }

    #[test]
    fn test_instructionkind_from_u32_07() {
        let opcode: u32 = 0b0100011;
        let instruction_kind: InstructionKind = opcode.try_into().unwrap();
        assert_eq!(instruction_kind, InstructionKind::Store);
    }

    #[test]
    fn test_instructionkind_from_u32_08() {
        // InstructionKind::MathI, InstructionKind::ShiftI
        let instruction: u32 = 0b000000000010011;
        let instruction_kind: InstructionKind = instruction.try_into().unwrap();
        assert_eq!(instruction_kind, InstructionKind::MathI);
    }

    #[test]
    fn test_instructionkind_from_u32_09() {
        // InstructionKind::MathI, InstructionKind::ShiftI
        let instruction: u32 = 0b101000000010011;
        println!("{}", bit_extract(instruction, 12, 14));
        let instruction_kind: InstructionKind = instruction.try_into().unwrap();
        assert_eq!(instruction_kind, InstructionKind::ShiftI);
    }

    #[test]
    fn test_instructionkind_from_u32_10() {
        let opcode: u32 = 0b0110011;
        let instruction_kind: InstructionKind = opcode.try_into().unwrap();
        assert_eq!(instruction_kind, InstructionKind::Math);
    }

    #[test]
    fn test_instructionkind_from_u32_11() {
        let opcode: u32 = 0b0001111;
        let instruction_kind: InstructionKind = opcode.try_into().unwrap();
        assert_eq!(instruction_kind, InstructionKind::Fence);
    }

    #[test]
    fn test_instructionkind_from_u32_12() {
        let opcode: u32 = 0b1110011;
        let instruction_kind: InstructionKind = opcode.try_into().unwrap();
        assert_eq!(instruction_kind, InstructionKind::System);
    }

    #[test]
    fn test_tuple_access() {
        struct T(u32, u32, u32);
        let t = T(10, 11, 12);
    }
}
