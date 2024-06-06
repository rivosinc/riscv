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

    /// Cache Block Invalidate instruction Enable
    #[inline]
    pub fn mpp(&self) -> CBIE {
        match self.bits.get_bits(4..6) {
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

    /// Counter Delegation Enable (enabled by Smcdeleg extension)
    #[cfg(riscv64)]
    #[inline]
    pub fn cde(&self) -> bool {
        self.bits.get_bit(60)
    }

    /// A/D Update Enabled (enabled by Svadu extension)
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

/// Cache Block Invalidate instruction Enable
#[inline]
pub unsafe fn set_cbie(cbie: CBIE) {
    let mut value = _read();
    value.set_bits(4..6, cbie as usize);
    _write(value);
}

set_clear_csr!(
    /// Cache Block Clean and Flush instruction Enable
    , set_cbcfe, clear_cbcfe, 1 << 6);

set_clear_csr!(
    /// Cache Block Zero instruction Enable
    , set_cbze, clear_cbze, 1 << 7);

#[cfg(riscv64)]
set_clear_csr!(
    /// Counter Delegation Enable (enabled by Smcdeleg extension)
    , set_cde, clear_cde, 1 << 60);

#[cfg(riscv64)]
set_clear_csr!(
    /// A/D Update Enabled (enabled by Svadu extension)
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
