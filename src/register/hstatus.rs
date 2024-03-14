//! hstatus register

#[cfg(target_pointer_width = "64")]
use crate::register::misa;
use bit_field::BitField;

/// Hypervisor Status Register
#[derive(Clone, Copy, Debug)]
pub struct Hstatus {
    bits: usize,
}

/// Supervisor Previous Virtual Privilege Mode
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SPVP {
    Supervisor = 1,
    User = 0,
}

impl Hstatus {
    /// Virtual Supervisor Big-endian Enable
    #[inline]
    pub fn vsbe(&self) -> bool {
        self.bits.get_bit(5)
    }

    /// Guest Virtual Address
    #[inline]
    pub fn gva(&self) -> bool {
        self.bits.get_bit(6)
    }

    /// Supervisor Previous Virtualization mode
    #[inline]
    pub fn spv(&self) -> bool {
        self.bits.get_bit(7)
    }

    /// Supervisor Previous Virtual Privilege Mode
    #[inline]
    pub fn spvp(&self) -> SPVP {
        match self.bits.get_bit(8) {
            true => SPVP::Supervisor,
            false => SPVP::User,
        }
    }

    /// Hypervisor in U-mode
    #[inline]
    pub fn hu(&self) -> bool {
        self.bits.get_bit(7)
    }

    /// Virtual Guest External Interrupt Number
    #[inline]
    pub fn vgein(&self) -> usize {
        self.bits.get_bits(12..17)
    }

    /// Virtual Mode Trap Virtual Memory
    #[inline]
    pub fn vtvm(&self) -> bool {
        self.bits.get_bit(20)
    }

    /// Virtual Mode Timeout Wait
    #[inline]
    pub fn vtw(&self) -> bool {
        self.bits.get_bit(21)
    }

    /// Virtual Mode Trap SRET
    #[inline]
    pub fn vtsr(&self) -> bool {
        self.bits.get_bit(22)
    }

    /// VS-Mode XLEN
    #[inline]
    #[cfg(target_pointer_width = "64")]
    pub fn vsxl(&self) -> misa::MXL {
        match self.bits.get_bits(32..33) {
            1 => misa::MXL::XLEN32,
            2 => misa::MXL::XLEN64,
            3 => misa::MXL::XLEN128,
            _ => unreachable!(),
        }
    }
}

read_csr_as!(Hstatus, 0x600);
write_csr!(0x600);
set!(0x600);
clear!(0x600);

set_clear_csr!(
    /// Virtual Supervisor Big-endian Enable
    , set_vsbe, clear_vsbe, 1 << 5);
set_clear_csr!(
    /// Guest Virtual Address
    , set_gva, clear_gva, 1 << 6);
set_clear_csr!(
    /// Supervisor Previous Virtualization mode
    , set_spv, clear_spv, 1 << 7);

#[inline]
/// Supervisor Previous Virtual Privilege Mode
pub unsafe fn set_spvp(spvp: SPVP) {
    match spvp {
        SPVP::Supervisor => _set(1 << 8),
        SPVP::User => _clear(1 << 8),
    }
}

set_clear_csr!(
    /// Hypervisor in U-mode
    , set_hu, clear_hu, 1 << 9);

#[inline]
/// Virtual Guest External Interrupt Number
pub unsafe fn set_vgein(vgein: usize) {
    let mut value = _read();
    value.set_bits(12..17, vgein);
    _write(value);
}

set_clear_csr!(
    /// Virtual Mode Trap Virtual Memory
    , set_vtvm, clear_vtvm, 1 << 20);
set_clear_csr!(
    /// Virtual Mode Timeout Wait
    , set_vtw, clear_vtw, 1 << 21);
set_clear_csr!(
    /// Virtual Mode Trap SRET
    , set_vtsr, clear_vtsr, 1 << 22);

#[inline]
#[cfg(target_pointer_width = "64")]
/// VS-Mode XLEN
pub unsafe fn set_vsxl(vsxl: misa::MXL) {
    let mut value = _read();
    value.set_bits(32..33, vsxl as usize);
    _write(value);
}
