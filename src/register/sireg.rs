//! sireg register
//!
//! The `sireg` CSR is defined in "The RISC-V Advanced Interrupt
//! Architecture" Version 0.3.2-draft
//!
//! Advanced Interrupt Architecture control is specified using an
//! indirect register file. In order to access to the register file,
//! software must:
//!
//! (1) Write to the `siselect` CSR with the index of the register to
//!     access
//! (2) Access the `sireg` CSR, which now contains the register to
//!     access
//!
//! The functions implemented in this module all write to the `siselect`
//! CSR to select the indirect register, then perform the read, write,
//! or modify operation requested on the `sireg` CSR.

use crate::register::siselect;
use bit_field::BitField;

/// External interrupt delivery enable register
#[derive(Clone, Copy, Debug)]
pub struct Eidelivery {
    bits: usize,
}

impl Eidelivery {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Interrupt delivery is enabled
    #[inline]
    pub fn enabled(&self) -> bool {
        self.bits.get_bit(0)
    }

    /// Interrupt delivery from a PLIC or APLIC is enabled
    #[inline]
    pub fn plic_enabled(&self) -> bool {
        self.bits.get_bit(30)
    }
}

/// Read the supervisor external interrupt delivery enable register
pub fn read_eidelivery() -> Eidelivery {
    siselect::write(siselect::Register::Eidelivery);
    Eidelivery {
        bits: unsafe { _read() },
    }
}

/// Write the supervisor external interrupt delivery enable register
pub fn write_eidelivery(value: usize) {
    siselect::write(siselect::Register::Eidelivery);
    unsafe {
        _write(value);
    }
}

/// Read the supervisor external interrupt threshold register
pub fn read_eithreshold() -> usize {
    siselect::write(siselect::Register::Eithreshold);
    unsafe { _read() }
}

/// Write the supervisor external interrupt threshold register
pub fn write_eithreshold(value: usize) {
    siselect::write(siselect::Register::Eithreshold);
    unsafe {
        _write(value);
    }
}

/// Determine the register offset and bit position for the external
/// interrupt pending and external interrupt enabled registers
#[cfg(riscv32)]
fn int_register_bit(interrupt: usize) -> (usize, usize) {
    // On 32-bit RISC-V:
    // - Each register is 32 bits wide
    // - Even and odd registers both exist
    let register = interrupt / 32;
    let bit = interrupt % 32;
    (register, bit)
}

/// Determine the register offset and bit position for the external
/// interrupt pending and external interrupt enabled registers
#[cfg(not(riscv32))]
fn int_register_bit(interrupt: usize) -> (usize, usize) {
    // On 64-bit RISC-V:
    // - Each register is 64 bits wide
    // - Only the even-numbered registers exist
    let register = (interrupt / 64) * 2;
    let bit = interrupt % 64;
    (register, bit)
}

/// Read the supervisor external interrupt pending bit for the given
/// external interrupt
pub fn read_eip(interrupt: usize) -> bool {
    let (register, bit) = int_register_bit(interrupt);
    siselect::write_usize(siselect::Register::Eip0 as usize + register);
    (unsafe { _read() } >> bit) & 1 == 1
}

/// Set the supervisor external interrupt pending bit for the given
/// external interrupt
pub fn set_eip(interrupt: usize) {
    let (register, bit) = int_register_bit(interrupt);
    siselect::write_usize(siselect::Register::Eip0 as usize + register);
    unsafe {
        _set(1 << bit);
    }
}

/// Clear the supervisor external interrupt pending bit for the given
/// external interrupt
pub fn clear_eip(interrupt: usize) {
    let (register, bit) = int_register_bit(interrupt);
    siselect::write_usize(siselect::Register::Eip0 as usize + register);
    unsafe {
        _clear(1 << bit);
    }
}

/// Read the supervisor external interrupt enable bit for the given
/// external interrupt
pub fn read_eie(interrupt: usize) -> bool {
    let (register, bit) = int_register_bit(interrupt);
    siselect::write_usize(siselect::Register::Eie0 as usize + register);
    (unsafe { _read() } >> bit) & 1 == 1
}

/// Set the supervisor external interrupt enable bit for the given
/// external interrupt
pub fn set_eie(interrupt: usize) {
    let (register, bit) = int_register_bit(interrupt);
    siselect::write_usize(siselect::Register::Eie0 as usize + register);
    unsafe {
        _set(1 << bit);
    }
}

/// Clear the supervisor external interrupt enable bit for the given
/// external interrupt
pub fn clear_eie(interrupt: usize) {
    let (register, bit) = int_register_bit(interrupt);
    siselect::write_usize(siselect::Register::Eie0 as usize + register);
    unsafe {
        _clear(1 << bit);
    }
}

read_csr!(0x151);
write_csr!(0x151);
set!(0x151);
clear!(0x151);
