// Copyright (c) 2022 by Rivos Inc.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

//! mcause register

ro_csr!(mcause, usize);

register_bitfields![usize,
    #[cfg(target_pointer_width = "32")]
    pub mcause [
        cause OFFSET(0) NUMBITS(31) [],
        is_interrupt OFFSET(31) NUMBITS(1) [],
    ],

    #[cfg(target_pointer_width = "64")]
    pub mcause [
        cause OFFSET(0) NUMBITS(63) [],
        is_interrupt OFFSET(63) NUMBITS(1) [],
    ],
];

/// Get the cause of the latest interrupt
#[inline]
pub fn cause() -> Trap {
    let cause: usize = read_field(mcause::cause);
    if is_interrupt() {
        Trap::Interrupt(Interrupt::from(cause))
    } else {
        Trap::Exception(Exception::from(cause))
    }
}

/// Returns `true` if the last trap was caused by an interrupt, otherwise
/// `false`.
#[inline]
pub fn is_interrupt() -> bool {
    is_set(mcause::is_interrupt)
}

/// Returns `true` if the last trap was caused by an exception, otherwise
/// `false`.
#[inline]
pub fn is_exception() -> bool {
    !is_interrupt()
}

/// Enum wrapping both potential trap causes
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Trap {
    Interrupt(Interrupt),
    Exception(Exception),
}

// TODO: Maybe add a feature that changes these enums to use the actual values,
// and the conversion functions to use core::mem::transmute(). Assuming that
// the target platform doesn't use the reserved or custom interrupt lines this
// should be safe. This would only be worthwhile if the compiler doesn't already
// optimize the conversions out.
/// Interrupt causes
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    SupervisorSoft,
    MachineSoft,
    SupervisorTimer,
    MachineTimer,
    SupervisorExternal,
    MachineExternal,
    Unknown,
}

/// Exception causes
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Exception {
    InstructionMisaligned,
    InstructionFault,
    IllegalInstruction,
    Breakpoint,
    LoadMisaligned,
    LoadFault,
    StoreMisaligned,
    StoreFault,
    UserEnvCall,
    SupervisorEnvCall,
    MachineEnvCall,
    InstructionPageFault,
    LoadPageFault,
    StorePageFault,
    Unknown,
}

impl Interrupt {
    #[inline]
    pub fn from(nr: usize) -> Self {
        match nr {
            1 => Interrupt::SupervisorSoft,
            3 => Interrupt::MachineSoft,
            5 => Interrupt::SupervisorTimer,
            7 => Interrupt::MachineTimer,
            9 => Interrupt::SupervisorExternal,
            11 => Interrupt::MachineExternal,
            _ => Interrupt::Unknown,
        }
    }
}

impl Exception {
    #[inline]
    pub fn from(nr: usize) -> Self {
        match nr {
            0 => Exception::InstructionMisaligned,
            1 => Exception::InstructionFault,
            2 => Exception::IllegalInstruction,
            3 => Exception::Breakpoint,
            4 => Exception::LoadMisaligned,
            5 => Exception::LoadFault,
            6 => Exception::StoreMisaligned,
            7 => Exception::StoreFault,
            8 => Exception::UserEnvCall,
            9 => Exception::SupervisorEnvCall,
            11 => Exception::MachineEnvCall,
            12 => Exception::InstructionPageFault,
            13 => Exception::LoadPageFault,
            15 => Exception::StorePageFault,
            _ => Exception::Unknown,
        }
    }
}
