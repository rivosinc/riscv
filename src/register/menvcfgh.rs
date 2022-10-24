//! menvcfgh register

#[cfg(riscv32)]
use bit_field::BitField;

/// menvcfgh register
#[derive(Clone, Copy, Debug)]
pub struct Menvcfgh {
    bits: usize,
}

impl Menvcfgh {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// PBMTE controls whether the Svpbmt extension is available for use in S-mode and G-stage
    /// address translation
    #[cfg(riscv32)]
    #[inline]
    pub fn pbmte(&self) -> bool {
        self.bits.get_bit(30)
    }

    /// STimeCmp Enable
    #[cfg(riscv32)]
    #[inline]
    pub fn stce(&self) -> bool {
        self.bits.get_bit(31)
    }
}

/// Reads the CSR
#[inline]
pub fn read() -> Menvcfgh {
    Menvcfgh {
        bits: unsafe { _read() },
    }
}

read_csr_rv32!(0x31A);
write_csr_rv32!(0x31A);
set!(0x31A);
clear!(0x31A);

#[cfg(riscv32)]
set_clear_csr!(
    /// PBMTE controls whether the Svpbmt extension is available for use in S-mode and G-stage
    /// address translation
    , set_pbmte, clear_pbmte, 1 << 30);

#[cfg(riscv32)]
set_clear_csr!(
    /// STimeCmp Enable
    , set_stce, clear_stce, 1 << 31);
