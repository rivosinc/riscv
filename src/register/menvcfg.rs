//! menvcfg register

use bit_field::BitField;

/// menvcfg register
#[derive(Clone, Copy, Debug)]
pub struct Menvcfg {
    bits: usize,
}

/// Cache Block Invalidate instruction Enable
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CBIE {
    IllegalInstruction = 0,
    ExecutedFlush = 1,
    Reserved = 2,
    ExecutedInvalidate = 3,
}

impl Menvcfg {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Fence of I/O implies Memory
    #[inline]
    pub fn fiom(&self) -> bool {
        self.bits.get_bit(0)
    }

    /// Landing Pad Enable
    #[inline]
    pub fn lpe(&self) -> bool {
        self.bits.get_bit(2)
    }

    /// Shadow Stack Enable
    #[inline]
    pub fn sse(&self) -> bool {
        self.bits.get_bit(3)
    }

    /// Cache Block Invalidate instruction Enable
    #[inline]
    pub fn cbie(&self) -> CBIE {
        match self.bits.get_bits(4..5) {
            0b00 => CBIE::IllegalInstruction,
            0b01 => CBIE::ExecutedFlush,
            0b10 => CBIE::Reserved,
            0b11 => CBIE::ExecutedInvalidate,
            _ => unreachable!(),
        }
    }

    /// Cache Block Clean and Flush instruction Enable
    #[inline]
    pub fn cbcfe(&self) -> bool {
        self.bits.get_bit(6)
    }

    /// Cache Block Zero instruction Enable
    #[inline]
    pub fn cbze(&self) -> bool {
        self.bits.get_bit(7)
    }

    /// Double Trap Enable
    #[cfg(riscv64)]
    #[inline]
    pub fn dte(&self) -> bool {
        self.bits.get_bit(59)
    }

    /// PTE A/D Bits Update Enable
    #[cfg(riscv64)]
    #[inline]
    pub fn adue(&self) -> bool {
        self.bits.get_bit(61)
    }

    /// PBMTE controls whether the Svpbmt extension is available for use in S-mode and G-stage
    /// address translation
    #[cfg(riscv64)]
    #[inline]
    pub fn pbmte(&self) -> bool {
        self.bits.get_bit(62)
    }

    /// STimeCmp Enable
    #[cfg(riscv64)]
    #[inline]
    pub fn stce(&self) -> bool {
        self.bits.get_bit(63)
    }
}

read_csr_as!(Menvcfg, 0x30A);
write_csr!(0x30A);
set!(0x30A);
clear!(0x30A);

set_clear_csr!(
    /// Fence of I/O implies Memory
    , set_fiom, clear_fiom, 1 << 0);

set_clear_csr!(
    /// Landing Pad Enable
    , set_lpe, clear_lpe, 1 << 2);

set_clear_csr!(
    /// Shadow Stack Enable
    , set_sse, clear_sse, 1 << 3);

/// Cache Block Invalidate instruction Enable
#[inline]
pub unsafe fn set_cbie(cbie: CBIE) {
    let mut value = _read();
    value.set_bits(4..5, cbie as usize);
    _write(value);
}

set_clear_csr!(
    /// Cache Block Clean and Flush instruction Enable
    , set_cbcfe, clear_cbcfe, 6 << 0);

set_clear_csr!(
    /// Cache Block Zero instruction Enable
    , set_cbze, clear_cbze, 7 << 0);

#[cfg(riscv64)]
set_clear_csr!(
    /// Double Trap enable
    , set_dte, clear_dte, 1 << 59);

#[cfg(riscv64)]
set_clear_csr!(
    /// PTE A/D Bits Update Enable
    , set_adue, clear_adue, 1 << 61);

#[cfg(riscv64)]
set_clear_csr!(
    /// PBMTE controls whether the Svpbmt extension is available for use in S-mode and G-stage
    /// address translation
    , set_pbmte, clear_pbmte, 1 << 62);

#[cfg(riscv64)]
set_clear_csr!(
    /// STimeCmp Enable
    , set_stce, clear_stce, 1 << 63);
