use crate::{
    decode::{U12, U5},
    num::{As, Bitcast, Unsigned},
};

pub trait Add {
    fn add(self, other: Self) -> Self;
}

pub trait Sub {
    fn sub(self, other: Self) -> Self;
}

pub trait Sll: ShiftBits {
    fn sll(self, other: Self) -> Self;
}

pub trait Slt {
    fn slt(self, other: Self) -> Self;
}

pub trait Sltu {
    fn sltu(self, other: Self) -> Self;
}

pub trait Xor {
    fn xor(self, other: Self) -> Self;
}

pub trait Srl: ShiftBits {
    fn srl(self, other: Self) -> Self;
}

pub trait Sra: ShiftBits {
    fn sra(self, other: Self) -> Self;
}

pub trait Or {
    fn or(self, other: Self) -> Self;
}

pub trait And {
    fn and(self, other: Self) -> Self;
}

pub trait Addi {
    fn addi(self, other: U12) -> Self;
}

pub trait Slti {
    fn slti(self, other: U12) -> Self;
}

pub trait Sltiu {
    fn sltiu(self, other: U12) -> Self;
}

pub trait Xori {
    fn xori(self, other: U12) -> Self;
}

pub trait Ori {
    fn ori(self, other: U12) -> Self;
}

pub trait Andi {
    fn andi(self, other: U12) -> Self;
}

pub trait Slli {
    fn slli(self, other: U5) -> Self;
}

pub trait Srli {
    fn srli(self, other: U5) -> Self;
}

pub trait Srai {
    fn srai(self, other: U5) -> Self;
}

pub trait ShiftBits {
    type Type;
}

pub trait Shift: ShiftBits + Sll + Srl + Sra {}

pub trait BaseMath:
    Add
    + Sub
    + Sll
    + Slt
    + Sltu
    + Xor
    + Srl
    + Sra
    + Or
    + And
    + Addi
    + Slti
    + Sltiu
    + Xori
    + Ori
    + Andi
    + Slli
    + Srli
    + Srai
    + ShiftBits
    + Shift
{
}

pub trait Addw {
    fn addw(self, other: Self) -> Self;
}

pub trait Subw {
    fn subw(self, other: Self) -> Self;
}

pub trait Sllw {
    fn sllw(self, other: Self) -> Self;
}

pub trait Srlw {
    fn srlw(self, other: Self) -> Self;
}

pub trait Sraw {
    fn sraw(self, other: Self) -> Self;
}

pub trait Addiw {
    fn addiw(self, other: U12) -> Self;
}

pub trait Slliw {
    fn slliw(self, other: U5) -> Self;
}

pub trait Srliw {
    fn srliw(self, other: U5) -> Self;
}

pub trait Sraiw {
    fn sraiw(self, other: U5) -> Self;
}

pub trait MathW: BaseMath + Addw + Subw + Sllw + Srlw + Sraw {}

macro_rules! impl_ops {
    ($t:ty) => {
        impl Add for $t {
            #[inline(always)]
            fn add(self, other: Self) -> Self {
                <$t>::wrapping_add(self, other)
            }
        }

        impl Sub for $t {
            #[inline(always)]
            fn sub(self, other: Self) -> Self {
                <$t>::wrapping_sub(self, other)
            }
        }

        impl Sll for $t {
            #[inline(always)]
            fn sll(self, other: Self) -> Self {
                <$t>::wrapping_shl(self, other as <$t as ShiftBits>::Type)
            }
        }

        impl Srl for $t {
            #[inline(always)]
            fn srl(self, other: Self) -> Self {
                <$t>::wrapping_shr(self, other as <$t as ShiftBits>::Type)
            }
        }

        impl Sra for $t {
            #[inline(always)]
            fn sra(self, other: Self) -> Self {
                unsafe {
                    core::mem::transmute(
                        core::mem::transmute::<_, <$t as Unsigned>::Signed>(self)
                            .wrapping_shr(other as <$t as ShiftBits>::Type),
                    )
                }
            }
        }
    };
}

impl_ops!(u32);
impl_ops!(u64);

impl ShiftBits for u32 {
    type Type = u32;
}

impl ShiftBits for u64 {
    type Type = u32;
}

impl<T> Slt for T
where
    T: Unsigned,
    <T as Unsigned>::Signed: core::cmp::Ord,
    bool: As<T>,
{
    #[inline(always)]
    fn slt(self, other: Self) -> Self {
        (Bitcast::<<T as Unsigned>::Signed>::bitcast(self)
            < Bitcast::<<T as Unsigned>::Signed>::bitcast(other))
        .r#as()
    }
}

impl<T> Sltu for T
where
    T: core::cmp::Ord,
    bool: As<T>,
{
    #[inline(always)]
    fn sltu(self, other: Self) -> Self {
        (self < other).r#as()
    }
}

impl<T> Xor for T
where
    T: core::ops::BitXor<Output = T>,
{
    #[inline(always)]
    fn xor(self, other: Self) -> Self {
        core::ops::BitXor::bitxor(self, other)
    }
}

impl<T> Or for T
where
    T: core::ops::BitOr<Output = T>,
{
    #[inline(always)]
    fn or(self, other: Self) -> Self {
        core::ops::BitOr::bitor(self, other)
    }
}

impl<T> And for T
where
    T: core::ops::BitAnd<Output = T>,
{
    #[inline(always)]
    fn and(self, other: Self) -> Self {
        core::ops::BitAnd::bitand(self, other)
    }
}

impl<T> Addi for T
where
    T: Unsigned + Add,
    <T as Unsigned>::Signed: From<i16>,
{
    #[inline(always)]
    fn addi(self, other: U12) -> Self {
        Add::add(
            self,
            Bitcast::bitcast(<<T as Unsigned>::Signed>::from(other.sign_extend())),
        )
    }
}

impl<T> Slti for T
where
    T: Slt + From<u16>,
{
    #[inline(always)]
    fn slti(self, other: U12) -> Self {
        Slt::slt(self, Bitcast::<u16>::bitcast(other.sign_extend()).into())
    }
}

impl<T> Sltiu for T
where
    T: Sltu + From<U12>,
{
    #[inline(always)]
    fn sltiu(self, other: U12) -> Self {
        Sltu::sltu(self, other.into())
    }
}

impl<T> Xori for T
where
    T: Xor + From<U12>,
{
    #[inline(always)]
    fn xori(self, other: U12) -> Self {
        Xor::xor(self, other.into())
    }
}

impl<T> Ori for T
where
    T: Or + From<U12>,
{
    #[inline(always)]
    fn ori(self, other: U12) -> Self {
        Or::or(self, other.into())
    }
}

impl<T> Andi for T
where
    T: And + From<U12>,
{
    #[inline(always)]
    fn andi(self, other: U12) -> Self {
        And::and(self, other.into())
    }
}

impl<T> Slli for T
where
    T: Sll + From<U5>,
{
    #[inline(always)]
    fn slli(self, other: U5) -> Self {
        Sll::sll(self, other.into())
    }
}

impl<T: Srl + From<U5>> Srli for T {
    #[inline(always)]
    fn srli(self, other: U5) -> Self {
        Srl::srl(self, other.into())
    }
}

impl<T> Srai for T
where
    T: Sra + From<U5>,
{
    #[inline(always)]
    fn srai(self, other: U5) -> Self {
        Sra::sra(self, other.into())
    }
}

impl<T: ShiftBits + Sll + Srl + Sra> Shift for T {}
impl<T> BaseMath for T where
    T: Add
        + Sub
        + Sll
        + Slt
        + Sltu
        + Xor
        + Srl
        + Sra
        + Or
        + And
        + Addi
        + Slti
        + Sltiu
        + Xori
        + Ori
        + Andi
        + Slli
        + Srli
        + Srai
        + Shift
{
}

macro_rules! forward_mathw {
    () => {};
    (@wrap $t:ident :: $meth:ident => ($meth32:expr) (|$name:ident| $b:expr) : $bt:ty) => {
        impl $t for u64 {
            #[inline(always)]
            fn $meth(self, $name: $bt) -> Self {
                crate::ops::Imm::imm($meth32(self as u32, $b))
            }
        }
    };
    (i12 $t:ident :: $meth:ident => $meth32:expr) => {
        forward_mathw!(@wrap $t :: $meth => ($meth32) (|other| other) : U12);
    };
    (i5 $t:ident :: $meth:ident => $meth32:expr) => {
        forward_mathw!(@wrap $t :: $meth => ($meth32) (|other| other) : U5);
    };
    ($t:ident :: $meth:ident => $meth32:expr) => {
        forward_mathw!(@wrap $t :: $meth => ($meth32) (|other| other as u32) : Self);
    };
    (i12 $t:ident :: $meth:ident => $meth32:expr; $($tt:tt)*) => {
        forward_mathw!(i12 $t :: $meth => $meth32);
        forward_mathw!($($tt)*);
    };
    (i5 $t:ident :: $meth:ident => $meth32:expr; $($tt:tt)*) => {
        forward_mathw!(i5 $t :: $meth => $meth32);
        forward_mathw!($($tt)*);
    };
    ($t:ident :: $meth:ident => $meth32:expr; $($tt:tt)*) => {
        forward_mathw!($t :: $meth => $meth32);
        forward_mathw!($($tt)*);
    };
}

forward_mathw! {
        Addw::addw => Add::add;
        Subw::subw => Sub::sub;
        Sllw::sllw => Sll::sll;
        Srlw::srlw => Srl::srl;
        Sraw::sraw => Sra::sra;
    i12 Addiw::addiw => Addi::addi;
    i5  Slliw::slliw => Slli::slli;
    i5  Srliw::srliw => Srli::srli;
    i5  Sraiw::sraiw => Srai::srai;
}

impl MathW for u64 {}
