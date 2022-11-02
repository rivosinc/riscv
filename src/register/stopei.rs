//! stopei register
//!
//! The `stopei` CSR is defined in "The RISC-V Advanced Interrupt
//! Architecture" Version 0.3.2-draft
//!
//! The primary interface to the stopei CSR should be the `claim()`
//! function, which will atomically claim the highest-priority pending
//! interrupt and allow the interrupt handler to process it.

use bit_field::BitField;

/// stopei register
#[derive(Clone, Copy, Debug)]
pub struct Stopei {
    bits: usize,
}

impl Stopei {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Interrupt identity
    #[inline]
    pub fn identity(&self) -> usize {
        self.bits.get_bits(16..26)
    }

    /// Interrupt priority
    #[inline]
    pub fn priority(&self) -> usize {
        self.bits.get_bits(0..10)
    }
}

read_csr_as!(Stopei, 0x15C);
claim_csr_as!(Stopei, 0x15C);
