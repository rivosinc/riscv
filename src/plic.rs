// SPDX-FileCopyrightText: Copyright (c) 2022 by Rivos, Inc.
// SPDX-FileCopyrightText: Copyright (c) 2020 by Luo Jia
// Licensed under the MIT License
// SPDX-License-Identifier: MIT
//
// Most of the code in this file is originally from luojia65/plic
// (https://github.com/luojia65/plic), which is released under the MIT license.
// See luojia65/plic/LICENSE or go to
// https://github.com/luojia65/plic/blob/main/LICENSE for full details.
//
//! Platform-Level Interrupt Controller
//!
//! Ref: [RISC-V Platform-Level Interrupt Controller Specification](https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc)

use volatile_register::RW;

pub struct Plic<const P: usize, const B: usize>();

impl<const P: usize, const B: usize> Plic<P, B> {
    const PTR: *const RegisterBlock = P as *const _;

    pub fn is_enabled(context: usize, irq: usize) -> bool {
        if irq == 0 {
            panic!();
        }

        let enables = unsafe { (*Self::PTR).enables[context][irq / 32].read() };
        return (enables & ((irq as u32) % 32)) != 0x0;
    }

    /// Enable interrupt for context
    ///
    /// # Safety
    ///
    /// This function is unsafe because it can break mask-based critical sections
    pub unsafe fn unmask(context: usize, irq: usize) {
        (*Self::PTR).enables[context][irq / 32].modify(|v| v | (0x1 << (irq % 32)));
    }

    /// Disable interrupt for context
    pub fn mask(context: usize, irq: usize) {
        unsafe {
            (*Self::PTR).enables[context][irq / 32].modify(|v| v & !(0x1 << ((irq as u32) % 32)));
        };
    }

    /// Get interrupt priority
    pub fn get_priority(irq: usize) -> Priority<B> {
        let bits = unsafe { (*Self::PTR).priority[irq].read() };
        Priority::from_bits(bits)
    }

    /// Set interrupt priority
    ///
    /// # Safety
    ///
    /// Changing priority levels can break priority-based critical sections
    /// and compromise memory safety.
    pub unsafe fn set_priority(irq: usize, prio: Priority<B>) {
        (*Self::PTR).priority[irq].write(prio.into_bits());
    }

    /// Get threshold for context
    pub fn get_threshold(context: usize) -> Priority<B> {
        let bits = unsafe { (*Self::PTR).contexts[context].threshold.read() };
        Priority::from_bits(bits)
    }

    /// Set threshold for context
    ///
    /// # Safety
    ///
    /// Changing the interrupt threshold can immediately trigger an interrupt
    pub unsafe fn set_threshold(context: usize, threshold: Priority<B>) {
        (*Self::PTR).contexts[context]
            .threshold
            .write(threshold.into_bits());
    }

    /// Claim interrupt (used by interrupt runtime)
    pub fn claim(context: usize) -> u16 {
        let bits = unsafe { (*Self::PTR).contexts[context].claim.read() };

        return bits as u16;
    }

    /// Complete interrupt (used by interrupt runtime)
    pub fn complete(context: usize, irq: usize) {
        unsafe {
            (*Self::PTR).contexts[context].claim.write(irq as u32);
        }
    }

    /// Checks if interrupt is pending
    pub fn is_pending(irq: usize) -> bool {
        unsafe { ((*Self::PTR).pending[irq / 32].read()) & (0x1 << (irq % 32)) != 0x0 }
    }
}

/// Priority of an interrupt
///
/// Type parameter B means how many bits are supported in target implementation.
/// For example if B = 3, highest priority would be 7 or 2^3 - 1, lowest would be 1.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Priority<const B: usize>(pub(crate) u32);

impl<const B: usize> Priority<B> {
    /// Priority 0 means never interrupt
    pub const fn never() -> Priority<B> {
        Priority(0)
    }
    /// Returns the lowest active priority, or priority 1.
    pub const fn lowest() -> Priority<B> {
        Priority(1)
    }
    /// Returns the highest active priority, or priority (2 << B) - 1.
    pub const fn highest() -> Priority<B> {
        if B == 32 {
            Priority(u32::MAX)
        } else {
            Priority((2 << B) - 1)
        }
    }
}

impl<const B: usize> Priority<B> {
    #[inline]
    pub fn into_bits(self) -> u32 {
        self.0
    }
    #[inline]
    pub fn from_bits(prio: u32) -> Priority<B> {
        if B == 32 {
            return Priority(prio); // always legal for B == 32
        }
        if prio < (1 << B) {
            Priority(prio)
        } else {
            panic!("invalid priority")
        }
    }
}

/// Register block
#[repr(C)]
struct RegisterBlock {
    /// 0x000000 - Interrupt source priority
    ///
    /// base + 0x000000: Reserved (interrupt source 0 does not exist)
    /// base + 0x000004: Interrupt source 1 priority
    /// base + 0x000008: Interrupt source 2 priority
    /// ...
    /// base + 0x000FFC: Interrupt source 1023 priority
    pub priority: [RW<u32>; 1024],
    /// 0x001000 - Interrupt pending
    ///
    /// base + 0x001000: Interrupt Pending bit 0-31
    /// base + 0x00107C: Interrupt Pending bit 992-1023
    pub pending: [RW<u32>; 32],
    _padding1: [u32; 992],
    /// 0x002000 - Enable bits for sources on contexts
    ///
    /// base + 0x002000: Enable bits for sources 0-31 on context 0
    /// base + 0x002004: Enable bits for sources 32-63 on context 0
    /// ...
    /// base + 0x00207F: Enable bits for sources 992-1023 on context 0
    /// base + 0x002080: Enable bits for sources 0-31 on context 1
    /// base + 0x002084: Enable bits for sources 32-63 on context 1
    /// ...
    /// base + 0x0020FF: Enable bits for sources 992-1023 on context 1
    /// base + 0x002100: Enable bits for sources 0-31 on context 2
    /// base + 0x002104: Enable bits for sources 32-63 on context 2
    /// ...
    /// base + 0x00217F: Enable bits for sources 992-1023 on context 2
    /// ...
    /// base + 0x1F1F80: Enable bits for sources 0-31 on context 15871
    /// base + 0x1F1F84: Enable bits for sources 32-63 on context 15871
    /// base + 0x1F1FFF: Enable bits for sources 992-1023 on context 15871
    /// ...
    pub enables: [Enables; 15872],
    _padding2: [u32; 14336],
    /// 0x200000 - Context configurations
    ///
    /// base + 0x200000: Priority threshold for context 0
    /// base + 0x200004: Claim/complete for context 0
    /// base + 0x200008: Reserved
    /// ...
    /// base + 0x200FFC: Reserved
    /// base + 0x201000: Priority threshold for context 1
    /// base + 0x201004: Claim/complete for context 1
    /// ...
    /// base + 0x3FFE000: Priority threshold for context 15871
    /// base + 0x3FFE004: Claim/complete for context 15871
    /// base + 0x3FFE008: Reserved
    pub contexts: [Contexts; 15872],
}

/// 0x002000 - Enable bits for sources
type Enables = [RW<u32>; 32];

/// 0x200000 - Context configurations
#[repr(C)]
struct Contexts {
    /// 0x000: Priority threshold for context
    pub threshold: RW<u32>,
    /// 0x004: Claim/complete for context
    pub claim: RW<u32>,
    _reserved: [u32; 1022],
}

#[cfg(test)]
mod tests {
    use core::mem::size_of;
    use crate::plic::{Contexts, Enables, RegisterBlock};

    #[test]
    fn sizeof_register_block() {
        assert_eq!(size_of::<RegisterBlock>(), 0x400_0000)
    }

    #[test]
    fn sizeof_enables() {
        assert_eq!(size_of::<Enables>(), 32 * 4);
    }

    #[test]
    fn sizeof_contexts() {
        assert_eq!(size_of::<Contexts>(), 1024 * 4);
    }
}
