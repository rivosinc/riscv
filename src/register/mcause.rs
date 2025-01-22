//! mcause register

/// mcause register
#[derive(Clone, Copy, Debug)]
pub struct Mcause {
    bits: usize,
}

/// Trap Cause
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Trap {
    Interrupt(Interrupt),
    Exception(Exception),
}

/// Interrupt
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    SupervisorSoft = 1,
    MachineSoft = 3,
    SupervisorTimer = 5,
    MachineTimer = 7,
    SupervisorExternal = 9,
    MachineExternal = 11,
    RasLpInterrupt = 35,
    RasHpInterrupt = 43,
    Unknown,
}

/// Exception
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Exception {
    InstructionMisaligned = 0,
    InstructionFault = 1,
    IllegalInstruction = 2,
    Breakpoint = 3,
    LoadMisaligned = 4,
    LoadFault = 5,
    StoreMisaligned = 6,
    StoreFault = 7,
    UserEnvCall = 8,
    SupervisorEnvCall = 9,
    MachineEnvCall = 11,
    InstructionPageFault = 12,
    LoadPageFault = 13,
    StorePageFault = 15,
    SoftwareCheck = 18,
    HardwareError = 19, 
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
            18 => Exception::SoftwareCheck,
            19 => Exception::HardwareError,
            _ => Exception::Unknown,
        }
    }
}
impl Mcause {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Returns the code field
    #[inline]
    pub fn code(&self) -> usize {
        match () {
            #[cfg(target_pointer_width = "32")]
            () => self.bits & !(1 << 31),
            #[cfg(target_pointer_width = "64")]
            () => self.bits & !(1 << 63),
            #[cfg(target_pointer_width = "128")]
            () => self.bits & !(1 << 127),
        }
    }

    /// Trap Cause
    #[inline]
    pub fn cause(&self) -> Trap {
        if self.is_interrupt() {
            Trap::Interrupt(Interrupt::from(self.code()))
        } else {
            Trap::Exception(Exception::from(self.code()))
        }
    }

    /// Is trap cause an interrupt.
    #[inline]
    pub fn is_interrupt(&self) -> bool {
        match () {
            #[cfg(target_pointer_width = "32")]
            () => self.bits & (1 << 31) == 1 << 31,
            #[cfg(target_pointer_width = "64")]
            () => self.bits & (1 << 63) == 1 << 63,
            #[cfg(target_pointer_width = "128")]
            () => self.bits & (1 << 127) == 1 << 127,
        }
    }

    /// Is trap cause an exception.
    #[inline]
    pub fn is_exception(&self) -> bool {
        !self.is_interrupt()
    }
}

read_csr_as!(Mcause, 0x342);
