//! mstateen0
//!
//! This register is defined by the Smstateen extension, version 1.0.0.

use bit_field::BitField;

/// mstateen0 register
#[derive(Clone, Copy, Debug)]
pub struct Mstateen0 {
    bits: usize,
}

impl Mstateen0 {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// hstateen0, hstateen0h, and the sstateen0 CSRs
    #[cfg(riscv64)]
    #[inline]
    pub fn se0(&self) -> bool {
        self.bits.get_bit(63)
    }

    /// henvcfg, henvcfgh, and the senvcfg CSRs
    #[cfg(riscv64)]
    #[inline]
    pub fn envcfg(&self) -> bool {
        self.bits.get_bit(62)
    }

    /// Sscsrind extension
    #[cfg(riscv64)]
    #[inline]
    pub fn csrind(&self) -> bool {
        self.bits.get_bit(60)
    }

    /// AIA state
    #[cfg(riscv64)]
    #[inline]
    pub fn aia(&self) -> bool {
        self.bits.get_bit(59)
    }

    /// IMSIC state
    #[cfg(riscv64)]
    #[inline]
    pub fn imsic(&self) -> bool {
        self.bits.get_bit(58)
    }

    /// scontext and hcontext, specified by Sdtrig
    #[cfg(riscv64)]
    #[inline]
    pub fn context(&self) -> bool {
        self.bits.get_bit(57)
    }

    /// Privileged Specification Version 1.13: hedelegh
    #[cfg(riscv64)]
    #[inline]
    pub fn p1p13(&self) -> bool {
        self.bits.get_bit(56)
    }

    /// jvt, specified by the Zcmt extension
    #[inline]
    pub fn jvt(&self) -> bool {
        self.bits.get_bit(2)
    }

    /// fcsr, specified by Zfinx and related extensions
    #[inline]
    pub fn fcsr(&self) -> bool {
        self.bits.get_bit(1)
    }

    /// Custom state
    #[inline]
    pub fn c(&self) -> bool {
        self.bits.get_bit(0)
    }
}

read_csr_as!(Mstateen0, 0x30c);
set!(0x30c);
clear!(0x30c);

#[cfg(riscv64)]
set_clear_csr!(
    /// hstateen0, hstateen0h, and the sstateen0 CSRs
    , set_se0, clear_se0, 1 << 63);
#[cfg(riscv64)]
set_clear_csr!(
    /// henvcfg, henvcfgh, and the senvcfg CSRs
    , set_envcfg, clear_envcfg, 1 << 62);
#[cfg(riscv64)]
set_clear_csr!(
    /// Sscsrind extensions
    , set_csrind, clear_csrind, 1 << 60);
#[cfg(riscv64)]
set_clear_csr!(
    /// AIA state
    , set_aia, clear_aia, 1 << 59);
#[cfg(riscv64)]
set_clear_csr!(
    /// IMSIC state
    , set_imsic, clear_imsic, 1 << 58);
#[cfg(riscv64)]
set_clear_csr!(
    /// scontext and hcontext, specified by Sdtrig
    , set_context, clear_context, 1 << 57);
#[cfg(riscv64)]
set_clear_csr!(
    /// jvt, specified by the Zcmt extension
    , set_p1p13, clear_p1p13, 1 << 56);
set_clear_csr!(
    /// jvt, specified by the Zcmt extension
    , set_jvt, clear_jvt, 1 << 2);
set_clear_csr!(
    /// fcsr, specified by Zfinx and related extensions
    , set_fcsr, clear_fcsr, 1 << 1);
set_clear_csr!(
    /// Custom state
    , set_c, clear_c, 1 << 0);
