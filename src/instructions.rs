use crate::{
    decode::{Shift, B, I, J, R, S, U, U10, U12, U3, U5},
    error::Error,
    num::{As, Bitcast, One, Unsigned, Zero},
    registers::{Registers, ZeroOrRegister},
};

const OPCODE_SIZE: u8 = 4;

macro_rules! def_uconst {
    ($($(#[$meta:meta])* $v:vis const $name:ident: $t:ty = $n:expr;)*) => {
        $(
            $(#[$meta])*
            #[allow(clippy::unusual_byte_groupings)]
            $v const $name: $t = if let Some(n) = <$t>::new($n) {
                n
            } else {
                panic!(concat!("Value ", stringify!($n), " out of ", stringify!($t), " range"))
            };
        )*
    };
}

def_uconst! {
    const ADD: U10 = 0b0000000_000;
    const SUB: U10 = 0b0100000_000;
    const SLL: U10 = 0b0000000_001;
    const SLT: U10 = 0b0000000_010;
    const SLTU: U10 = 0b0000000_011;
    const XOR: U10 = 0b0000000_100;
    const SRL: U10 = 0b0000000_101;
    const SRA: U10 = 0b0100000_101;
    const OR: U10 = 0b0000000_110;
    const AND: U10 = 0b0000000_111;
    const ADDI: U3 = 0b000;
    const SLTI: U3 = 0b010;
    const SLTIU: U3 = 0b011;
    const XORI: U3 = 0b100;
    const ORI: U3 = 0b110;
    const ANDI: U3 = 0b111;
    const LB: U3 = 0b000;
    const LBU: U3 = 0b100;
    const LH: U3 = 0b001;
    const LHU: U3 = 0b101;
    const LW: U3 = 0b010;
    const SLLI: U10 = 0b0000000_001;
    const SRLI: U10 = 0b0000000_101;
    const SRAI: U10 = 0b0100000_101;
    const SB: U3 = 0b000;
    const SH: U3 = 0b001;
    const SW: U3 = 0b010;
    const BEQ: U3 = 0b000;
    const BNE: U3 = 0b001;
    const BLT: U3 = 0b100;
    const BGE: U3 = 0b101;
    const BLTU: U3 = 0b110;
    const BGEU: U3 = 0b111;
}

pub trait Math: Sized {
    fn math(instruction: R, regs: &mut Registers<Self>) -> Result<(), Error>;
}

pub trait MathI: Sized {
    fn mathi(instruction: I, regs: &mut Registers<Self>) -> Result<(), Error>;
}

pub trait ShiftI: Sized {
    fn shifti(instruction: Shift, regs: &mut Registers<Self>) -> Result<(), Error>;
}

pub trait Lui: Sized {
    fn lui(instruction: U, regs: &mut Registers<Self>) -> Result<(), Error>;
}

pub trait Auipc: Sized {
    fn auipc(instruction: U, regs: &mut Registers<Self>, pc: Self) -> Result<(), Error>;
}

pub trait Load: Sized {
    fn load(instruction: I, regs: &mut Registers<Self>, memory: &[u8]) -> Result<(), Error>;
}

pub trait Store: Sized {
    fn store(instruction: S, regs: &mut Registers<Self>, memory: &mut [u8]) -> Result<(), Error>;
}

pub trait Jal: Sized {
    fn jal(instruction: J, regs: &mut Registers<Self>, pc: &mut Self) -> Result<(), Error>;
}

pub trait Jalr: Sized {
    fn jalr(instruction: I, regs: &mut Registers<Self>, pc: &mut Self) -> Result<(), Error>;
}

pub trait Branch: Sized {
    fn branch(instruction: B, regs: &mut Registers<Self>, pc: &mut Self) -> Result<(), Error>;
}

macro_rules! impl_math {
    ($t:ty $({ $($tt:tt)* })?) => {
        impl Math for $t {
            #[inline(always)]
            fn math(instruction: R, regs: &mut Registers<Self>) -> Result<(), Error> {
                use crate::ops;

                #[deny(unreachable_patterns)]
                let f: fn(Self, Self) -> Self = match instruction.id() {
                    x if x > U10::MAX => unsafe {
                        core::hint::unreachable_unchecked()
                    },
                    ADD => ops::Add::add,
                    SUB => ops::Sub::sub,
                    SLL => ops::Sll::sll,
                    SLT => ops::Slt::slt,
                    SLTU => ops::Sltu::sltu,
                    XOR => ops::Xor::xor,
                    SRL => ops::Srl::srl,
                    SRA => ops::Sra::sra,
                    OR => ops::Or::or,
                    AND => ops::And::and,
                    $($($tt)*)?
                    _ => return Err(Error::InvalidOpCode),
                };

                let ZeroOrRegister::Register(reg) = instruction.rd.into() else {
                    return Err(Error::InvalidOpCode);
                };
                let src1 = ZeroOrRegister::from_u5(instruction.rs1).fetch(regs);
                let src2 = ZeroOrRegister::from_u5(instruction.rs2).fetch(regs);
                *regs.get_mut(reg) = f(src1, src2);
                Ok(())
            }
        }
    };
}

macro_rules! impl_mathi {
    ($t:ty $({ $($tt:tt)* })?) => {
        impl MathI for $t {
            #[inline(always)]
            fn mathi(instruction: I, regs: &mut Registers<Self>) -> Result<(), Error> {
                use crate::ops;

                #[deny(unreachable_patterns)]
                let f: fn(Self, U12) -> Self = match instruction.id() {
                    x if x > U3::MAX => unsafe {
                        core::hint::unreachable_unchecked()
                    },
                    ADDI => ops::Addi::addi,
                    SLTI => ops::Slti::slti,
                    SLTIU => ops::Sltiu::sltiu,
                    XORI => ops::Xori::xori,
                    ORI => ops::Ori::ori,
                    ANDI => ops::Andi::andi,
                    $($($tt)*)?
                    _ => return Err(Error::InvalidOpCode),
                };

                let ZeroOrRegister::Register(reg) = instruction.rd.into() else {
                    return Err(Error::InvalidOpCode);
                };
                let src1 = ZeroOrRegister::from_u5(instruction.rs1).fetch(regs);
                *regs.get_mut(reg) = f(src1, instruction.imm);
                Ok(())
            }
        }
    };
}

macro_rules! impl_shifti {
    ($t:ty $({ $($tt:tt)* })?) => {
        impl ShiftI for $t {
            #[inline(always)]
            fn shifti(instruction: Shift, regs: &mut Registers<Self>) -> Result<(), Error> {
                use crate::ops;

                #[deny(unreachable_patterns)]
                let f: fn(Self, U5) -> Self = match instruction.id() {
                    x if x > U10::MAX => unsafe {
                        core::hint::unreachable_unchecked()
                    },
                    SLLI => ops::Slli::slli,
                    SRLI => ops::Srli::srli,
                    SRAI => ops::Srai::srai,
                    $($($tt)*)?
                    _ => return Err(Error::InvalidOpCode),
                };

                let ZeroOrRegister::Register(dest_reg) = instruction.rd.into() else {
                    return Err(Error::InvalidOpCode);
                };
                let src1 = ZeroOrRegister::from_u5(instruction.rs1).fetch(regs);
                *regs.get_mut(dest_reg) = f(src1, instruction.shamt);

                Ok(())
            }
        }
    };
}

macro_rules! impl_branch {
    ($t:ty $({ $($tt:tt)* })?) => {
        impl Branch for $t {
            #[inline]
            fn branch(
                instruction: B,
                regs: &mut Registers<Self>,
                pc: &mut Self,
            ) -> Result<(), Error> {
                use crate::ops;

                #[deny(unreachable_patterns)]
                let f: fn(Self, Self) -> bool = match instruction.id() {
                    x if x > U3::MAX => unsafe { core::hint::unreachable_unchecked() },
                    BEQ => ops::Beq::beq,
                    BNE => ops::Bne::bne,
                    BLT => ops::Blt::blt,
                    BGE => ops::Bge::bge,
                    BLTU => ops::Bltu::bltu,
                    BGEU => ops::Bgeu::bgeu,
                    $($($tt)*)?
                    _ => return Err(Error::InvalidOpCode),
                };

                let src1 = ZeroOrRegister::from_u5(instruction.rs1).fetch(regs);
                let src2 = ZeroOrRegister::from_u5(instruction.rs2).fetch(regs);
                if f(src1, src2) {
                    *pc = pc.wrapping_add_signed(
                        instruction.imm.sign_extend() as <$t as Unsigned>::Signed
                    );
                } else {
                    *pc = pc.wrapping_add(OPCODE_SIZE as _);
                }
                Ok(())
            }
        }
    };
}

macro_rules! impl_load {
    (__internal $t:ty { $($cond:pat => $body:expr),* $(,)? }) => {
        impl Load for $t {
            #[inline(always)]
            fn load(instruction: I, regs: &mut Registers<Self>, memory: &[u8]) -> Result<(), Error> {
                use crate::mem::{self, Pod};
                use crate::ops;

                #[inline(always)]
                fn exec<T, F>(
                    instruction: I,
                    regs: &mut Registers<$t>,
                    memory: &[u8],
                    f: F,
                ) -> Result<(), Error>
                where
                    T: Pod,
                    F: Fn(T) -> $t,
                {
                    let ZeroOrRegister::Register(dest_reg) = ZeroOrRegister::from_u5(instruction.rd) else {
                            return Err(Error::InvalidOpCode);
                    };
                    let offset = ZeroOrRegister::from_u5(instruction.rs1)
                        .fetch(regs)
                        .wrapping_add_signed(instruction.imm.sign_extend() as <$t as Unsigned>::Signed)
                        as usize;
                    *regs.get_mut(dest_reg) = f(mem::read::<T>(memory, offset)?);
                    Ok(())
                }

                #[deny(unreachable_patterns)]
                match instruction.id() {
                    x if x > U3::MAX => unsafe {
                        core::hint::unreachable_unchecked()
                    },
                    $($cond => exec(instruction, regs, memory, $body),)*
                    _ => Err(Error::InvalidOpCode),
                }
            }
        }
    };
    ($t:ty $({ $($tt:tt)* })?) => {
        impl_load!(__internal $t {
            LB => ops::Lb::lb,
            LBU => ops::Lbu::lbu,
            LH => ops::Lh::lh,
            LHU => ops::Lhu::lhu,
            LW => ops::Lw::lw,
            $($($tt)*)?
        });
    };
}

macro_rules! impl_store {
    (__internal $t:ty { $($cond:pat => $body:expr),* $(,)? }) => {
        impl Store for $t {
            fn store(instruction: S, regs: &mut Registers<Self>, memory: &mut [u8]) -> Result<(), Error> {
                use crate::{
                    mem::{self, Pod},
                    ops,
                };

                #[inline(always)]
                fn exec<T, F>(
                    instruction: S,
                    regs: &mut Registers<$t>,
                    memory: &mut [u8],
                    f: F,
                ) -> Result<(), Error>
                where
                    T: Pod,
                    F: Fn($t) -> T,
                {
                    let src1 = ZeroOrRegister::from_u5(instruction.rs1).fetch(regs);
                    let src2 = ZeroOrRegister::from_u5(instruction.rs2).fetch(regs);
                    let offset = src1.wrapping_add_signed(instruction.imm.sign_extend() as <$t as Unsigned>::Signed) as usize;
                    mem::write(&f(src2), memory, offset)
                }

                #[deny(unreachable_patterns)]
                match instruction.id() {
                    x if x > U3::MAX => unsafe {
                        core::hint::unreachable_unchecked()
                    },
                    $($cond => exec(instruction, regs, memory, $body),)*
                    _ => Err(Error::InvalidOpCode),
                }
            }
        }
    };
    ($t:ty $({ $($tt:tt)* })?) => {
        impl_store!(__internal $t {
            SB => ops::Sb::sb,
            SH => ops::Sh::sh,
            SW => ops::Sw::sw,
            $($($tt)*)?
        });
    };
}

impl_math!(u32);
impl_mathi!(u32);
impl_shifti!(u32);
impl_branch!(u32);
impl_load!(u32);
impl_store!(u32);

impl_math!(u64);
impl_mathi!(u64);
impl_shifti!(u64);
impl_branch!(u64);
impl_load!(u64);
impl_store!(u64);

impl<T> Lui for T
where
    T: crate::ops::Imm,
{
    #[inline(always)]
    fn lui(instruction: U, regs: &mut Registers<Self>) -> Result<(), Error> {
        let dest = ZeroOrRegister::from_u5(instruction.rd)
            .fetch_mut(regs)
            .ok_or(Error::InvalidOpCode)?;
        *dest = crate::ops::Imm::imm(instruction.imm);
        Ok(())
    }
}

impl<T> Auipc for T
where
    T: crate::ops::Add + crate::ops::Imm,
{
    #[inline(always)]
    fn auipc(instruction: U, regs: &mut Registers<Self>, pc: Self) -> Result<(), Error> {
        use crate::ops;

        let dest = ZeroOrRegister::from_u5(instruction.rd)
            .fetch_mut(regs)
            .ok_or(Error::InvalidOpCode)?;
        *dest = pc.add(ops::Imm::imm(instruction.imm));

        Ok(())
    }
}

impl<T> Jal for T
where
    T: Unsigned + crate::ops::Add + Copy,
    <T as Unsigned>::Signed: From<i32>,
    u8: As<T>,
{
    #[inline(always)]
    fn jal(instruction: J, regs: &mut Registers<Self>, pc: &mut Self) -> Result<(), Error> {
        // TODO: The JAL and JALR instructions will generate an instruction-address-misaligned exception if the target
        //       address is not aligned to a four-byte boundary. (???)
        if let ZeroOrRegister::Register(reg) = instruction.rd.into() {
            *regs.get_mut(reg) = pc.add(OPCODE_SIZE.r#as());
        }

        *pc = pc.add(<T as Unsigned>::Signed::from(instruction.imm.sign_extend()).bitcast());

        Ok(())
    }
}

impl<T> Jalr for T
where
    T: Unsigned
        + crate::ops::Add
        + crate::ops::And
        + Copy
        + Zero
        + One
        + core::ops::Not<Output = T>,
    <T as Unsigned>::Signed: From<i16>,
    u8: As<T>,
{
    #[inline(always)]
    fn jalr(instruction: I, regs: &mut Registers<Self>, pc: &mut Self) -> Result<(), Error> {
        // TODO: The JAL and JALR instructions will generate an instruction-address-misaligned exception if the target
        //       address is not aligned to a four-byte boundary. (???)
        let next = ZeroOrRegister::from_u5(instruction.rs1)
            .fetch(regs)
            .add(<T as Unsigned>::Signed::from(instruction.imm.sign_extend()).bitcast())
            .and(!T::one());

        if let ZeroOrRegister::Register(reg) = ZeroOrRegister::from_u5(instruction.rd) {
            *regs.get_mut(reg) = pc.add(OPCODE_SIZE.r#as());
        }

        *pc = next;

        Ok(())
    }
}

#[inline(always)]
pub(crate) fn execute_math<T>(instruction: R, regs: &mut Registers<T>) -> Result<(), Error>
where
    T: Math,
{
    Math::math(instruction, regs)
}

#[inline(always)]
pub(crate) fn execute_mathi<T>(instruction: I, regs: &mut Registers<T>) -> Result<(), Error>
where
    T: MathI,
{
    MathI::mathi(instruction, regs)
}

#[inline(always)]
pub(crate) fn execute_shifti<T>(instruction: Shift, regs: &mut Registers<T>) -> Result<(), Error>
where
    T: ShiftI,
{
    ShiftI::shifti(instruction, regs)
}

#[inline(always)]
pub(crate) fn execute_load<T: Load>(
    instruction: I,
    regs: &mut Registers<T>,
    memory: &[u8],
) -> Result<(), Error> {
    T::load(instruction, regs, memory)
}

#[inline(always)]
pub(crate) fn execute_jal<T: Jal>(
    instruction: J,
    regs: &mut Registers<T>,
    pc: &mut T,
) -> Result<(), Error> {
    T::jal(instruction, regs, pc)
}

#[inline(always)]
pub(crate) fn execute_jalr<T: Jalr>(
    instruction: I,
    regs: &mut Registers<T>,
    pc: &mut T,
) -> Result<(), Error> {
    T::jalr(instruction, regs, pc)
}

#[inline(always)]
pub(crate) fn execute_store<T: Store>(
    instruction: S,
    regs: &mut Registers<T>,
    memory: &mut [u8],
) -> Result<(), Error> {
    T::store(instruction, regs, memory)
}

#[inline(always)]
pub(crate) fn execute_branch<T: Branch>(
    instruction: B,
    regs: &mut Registers<T>,
    pc: &mut T,
) -> Result<(), Error> {
    T::branch(instruction, regs, pc)
}

#[inline(always)]
pub(crate) fn execute_lui<T: Lui>(instruction: U, regs: &mut Registers<T>) -> Result<(), Error> {
    T::lui(instruction, regs)
}

#[inline(always)]
pub(crate) fn execute_auipc<T: Auipc>(
    instruction: U,
    regs: &mut Registers<T>,
    pc: T,
) -> Result<(), Error> {
    T::auipc(instruction, regs, pc)
}

#[allow(dead_code)]
const fn implements_instructions<
    T: Math + MathI + ShiftI + Lui + Auipc + Load + Store + Jal + Jalr + Branch,
>() {
}
const _: () = implements_instructions::<u32>();
const _: () = implements_instructions::<u64>();
