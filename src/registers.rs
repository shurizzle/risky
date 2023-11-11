#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Register {
    X1 = 0,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    X30,
    X31,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ZeroOrRegister {
    Zero,
    Register(Register),
}

#[derive(Debug)]
pub struct Registers(pub [u32; 31]);

// assert registers size
const _: [(); 0] = [(); {
    (Register::X31 as usize + 1) * core::mem::size_of::<u32>() - core::mem::size_of::<Registers>()
}];

impl Registers {
    #[inline(always)]
    pub fn new() -> Self {
        Self([0u32; 31])
    }

    #[inline(always)]
    #[cfg(debug_assertions)]
    pub fn get(&self, reg: Register) -> u32 {
        self.0[reg as usize]
    }

    #[inline(always)]
    #[cfg(not(debug_assertions))]
    pub fn get(&self, reg: Register) -> u32 {
        unsafe { *self.0.get_unchecked(reg as usize) }
    }

    #[inline(always)]
    #[cfg(debug_assertions)]
    pub fn get_mut(&mut self, reg: Register) -> &mut u32 {
        &mut self.0[reg as usize]
    }

    #[inline(always)]
    #[cfg(not(debug_assertions))]
    pub fn get_mut(&mut self, reg: Register) -> &mut u32 {
        unsafe { self.0.get_unchecked_mut(reg as usize) }
    }
}

impl Default for Registers {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Register {
    #[inline(always)]
    pub fn fetch(&self, regs: &Registers) -> u32 {
        regs.get(*self)
    }

    #[inline(always)]
    pub fn fetch_mut<'a>(&self, regs: &'a mut Registers) -> &'a mut u32 {
        regs.get_mut(*self)
    }
}

impl ZeroOrRegister {
    #[inline(always)]
    pub fn fetch(&self, regs: &Registers) -> u32 {
        match *self {
            Self::Zero => 0,
            Self::Register(reg) => regs.get(reg),
        }
    }

    #[inline(always)]
    pub fn fetch_mut<'a>(&self, regs: &'a mut Registers) -> Option<&'a mut u32> {
        match *self {
            Self::Zero => None,
            Self::Register(reg) => Some(regs.get_mut(reg)),
        }
    }

    #[inline(always)]
    pub unsafe fn decode_unchecked(reg: u8) -> Self {
        debug_assert!(reg < 32, "invalid register number");
        if reg == 0 {
            Self::Zero
        } else {
            Self::Register(unsafe { core::mem::transmute(reg.wrapping_sub(1)) })
        }
    }

    #[inline]
    pub fn decode(reg: u8) -> Option<Self> {
        if reg < 32 {
            Some(unsafe { Self::decode_unchecked(reg) })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registers() {
        let mut regs = Registers::default();
        let x1 = regs.get_mut(Register::X1);
        *x1 = 20;
        let val = regs.get(Register::X1);
        assert_eq!(val, 20)
    }

    #[test]
    fn test_register() {
        let mut regs = Registers::default();
        let x1 = Register::X1;
        let x1_mut = x1.fetch_mut(&mut regs);
        *x1_mut = 20;
        assert_eq!(20, x1.fetch(&regs));
    }

    #[test]
    fn test_zero_or_register() {
        let mut regs = Registers::default();
        let reg = Register::X1;
        let x1 = reg.fetch_mut(&mut regs);
        *x1 = 20;
        match ZeroOrRegister::decode(1) {
            Some(ZeroOrRegister::Register(r)) => {
                assert_eq!(r, reg);
                assert_eq!(20, r.fetch(&regs));
            }
            _ => unreachable!(),
        };
        match ZeroOrRegister::decode(0) {
            Some(ZeroOrRegister::Zero) => {}
            _ => unreachable!(),
        }
    }
}
