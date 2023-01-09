//! mseccfg register

use bit_field::BitField;

/// mseccfg register
#[derive(Clone, Copy, Debug)]
pub struct Mseccfg {
    bits: usize,
}

impl Mseccfg {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Rule Locking Bypass
    #[inline]
    pub fn rlb(&self) -> bool {
        self.bits.get_bit(2)
    }

    /// Machine Mode Whitelist Policy
    #[inline]
    pub fn mmwp(&self) -> bool {
        self.bits.get_bit(1)
    }

    /// Machine Mode Lockdown
    #[inline]
    pub fn mml(&self) -> bool {
        self.bits.get_bit(0)
    }
}

read_csr_as!(Mseccfg, 0x747);
set!(0x747);
clear!(0x747);

set_clear_csr!(
    /// Rule Locking Bypass 
    , set_rlb, clear_rlb, 1 << 2);
set_clear_csr!(
    /// Machine Mode Whitelist Policy 
    , set_mmwp, clear_mmwp, 1 << 1);
set_clear_csr!(
    /// Machine Mode Lockdown 
    , set_mml, clear_mml, 1 << 0);
