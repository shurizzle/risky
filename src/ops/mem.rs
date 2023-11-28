use crate::{
    mem::{I16, I32, U16, U32, U64},
    num::{As, Bitcast, Unsigned},
};

pub trait Lb {
    fn lb(value: i8) -> Self;
}

pub trait Lbu {
    fn lbu(value: u8) -> Self;
}

pub trait Lh {
    fn lh(value: I16) -> Self;
}

pub trait Lhu {
    fn lhu(value: U16) -> Self;
}

pub trait Lw {
    fn lw(value: I32) -> Self;
}

pub trait Lwu {
    fn lwu(value: U32) -> Self;
}

pub trait Ld {
    fn ld(value: U64) -> Self;
}

pub trait Sb {
    fn sb(self) -> u8;
}

pub trait Sh {
    fn sh(self) -> U16;
}

pub trait Sw {
    fn sw(self) -> U32;
}

pub trait Sd {
    fn sd(self) -> U64;
}

pub trait Imm {
    fn imm(value: u32) -> Self;
}

impl<T> Lb for T
where
    T: Unsigned,
    <T as Unsigned>::Signed: From<i8>,
{
    #[inline(always)]
    fn lb(value: i8) -> Self {
        Bitcast::bitcast(<<T as Unsigned>::Signed as From<i8>>::from(value))
    }
}

impl<T> Lbu for T
where
    T: Unsigned + From<u8>,
{
    #[inline(always)]
    fn lbu(value: u8) -> Self {
        value.into()
    }
}

impl<T> Lh for T
where
    T: Unsigned,
    <T as Unsigned>::Signed: From<i16>,
{
    #[inline(always)]
    fn lh(value: I16) -> Self {
        Bitcast::bitcast(<<T as Unsigned>::Signed as From<i16>>::from(value.as_i16()))
    }
}

impl<T> Lhu for T
where
    T: From<u16>,
{
    #[inline(always)]
    fn lhu(value: U16) -> Self {
        value.as_u16().into()
    }
}

impl<T> Lw for T
where
    T: Unsigned,
    <T as Unsigned>::Signed: From<i32>,
    T: From<u32>,
{
    #[inline(always)]
    fn lw(value: I32) -> Self {
        Bitcast::bitcast(<<T as Unsigned>::Signed as From<i32>>::from(value.as_i32()))
    }
}

impl<T> Lwu for T
where
    T: From<u32>,
{
    #[inline(always)]
    fn lwu(value: U32) -> Self {
        value.as_u32().into()
    }
}

impl<T> Ld for T
where
    T: From<u64>,
{
    #[inline(always)]
    fn ld(value: U64) -> Self {
        value.as_u64().into()
    }
}

impl<T> Sb for T
where
    T: As<u8>,
{
    #[inline(always)]
    fn sb(self) -> u8 {
        self.r#as()
    }
}

impl<T> Sh for T
where
    T: As<u16>,
{
    #[inline(always)]
    fn sh(self) -> U16 {
        U16::new(self.r#as())
    }
}

impl<T> Sw for T
where
    T: As<u32>,
{
    #[inline(always)]
    fn sw(self) -> U32 {
        U32::new(self.r#as())
    }
}

impl<T> Sd for T
where
    T: As<u64>,
{
    #[inline(always)]
    fn sd(self) -> U64 {
        U64::new(self.r#as())
    }
}

impl Imm for u32 {
    #[inline(always)]
    fn imm(value: u32) -> Self {
        value
    }
}

impl Imm for u64 {
    #[inline(always)]
    fn imm(value: u32) -> Self {
        unsafe { core::mem::transmute(core::mem::transmute::<_, i32>(value) as i64) }
    }
}
