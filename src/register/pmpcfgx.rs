/// Physical memory protection configuration
use bit_field::BitField;

use core::convert::From;

/// Permission enum contains all possible permission modes for pmp registers
/// NOTE: All encodings where R = 0 and W = 1 are reserved
#[derive(Clone, Copy, Debug)]
pub enum Permission {
    NONE = 0b000,
    R = 0b001,
    W = 0b010,
    RW = 0b011,
    X = 0b100,
    RX = 0b101,
    WX = 0b110,
    RWX = 0b111,
}

/// Mode enum contains all possible addressing modes for pmp registers
#[derive(Clone, Copy, Debug)]
pub enum Mode {
    OFF = 0b00,
    TOR = 0b01,
    NA4 = 0b10,
    NAPOT = 0b11,
}

/// PmpCfg struct holds a high-level representation of a single pmp configuration
#[derive(Clone, Copy, Debug)]
pub struct PmpCfg {
    pub byte: u8,
}

impl PmpCfg {
    pub fn new(mode: Mode, permission: Permission, locked: bool) -> PmpCfg {
        return PmpCfg {
            byte: (locked as u8) << 7 | (mode as u8) << 3 | (permission as u8),
        };
    }

    pub fn get_mode(&self) -> Mode {
        return unsafe { core::mem::transmute(self.byte.get_bits(3..=4)) };
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.byte.set_bits(3..=4, ((mode as u8) & 0b11) << 3);
    }

    pub fn get_permission(&self) -> Permission {
        return unsafe { core::mem::transmute(self.byte.get_bits(0..=2)) };
    }

    pub fn set_permission(&mut self, permission: Permission) {
        self.byte.set_bits(0..=2, (permission as u8) & 0b111);
    }

    pub fn set_locked(&mut self) {
        self.byte.set_bit(7, true);
    }

    pub fn check_locked(&self) -> bool {
        return self.byte.get_bit(7);
    }
}

pub struct PmpCfgCsr {
    #[cfg(riscv32)]
    cfgs: [PmpCfg; 4],

    #[cfg(riscv64)]
    cfgs: [PmpCfg; 8],
}

impl PmpCfgCsr {
    pub fn get_cfg(&self, index: usize) -> PmpCfg {
        #[cfg(riscv32)]
        {
            assert!(index < 4);
            self.cfgs[3 - index]
        }

        #[cfg(riscv64)]
        {
            assert!(index < 8);
            self.cfgs[7 - index]
        }
    }
}

impl From<usize> for PmpCfgCsr {
    fn from(item: usize) -> Self {
        // Safety: We know usize based on the target architecture, so these
        // casts will never drop data. The transmutes are safe because it is
        // guaranteed that the size of a PmpCfgCsr struct to be the word size
        // fof the target architecture.
        return unsafe {
            #[cfg(riscv32)]
            core::mem::transmute(item as u32);

            #[cfg(riscv64)]
            core::mem::transmute(item as u64)
        };
    }
}

impl From<PmpCfgCsr> for usize {
    fn from(item: PmpCfgCsr) -> Self {
        return unsafe { core::mem::transmute(item) };
    }
}

macro_rules! set_pmpcfg {
    () => {
        /// Set the pmp configuration corresponding to the index
        #[inline]
        pub unsafe fn set(index: usize, cfg: PmpCfg) {
            #[cfg(riscv32)]
            assert!(index < 4);

            #[cfg(riscv64)]
            assert!(index < 8);

            let mut value = _read();
            value.set_bits(8 * index..=8 * index + 7, cfg.byte.into());
            _write(value);
        }
    };
}

macro_rules! clear_pmpcfg {
    () => {
        /// Clear the pmp configuration corresponding to the index
        #[inline]
        pub unsafe fn clear(index: usize) {
            #[cfg(riscv32)]
            assert!(index < 4);

            #[cfg(riscv64)]
            assert!(index < 8);

            let mut value = _read();
            value.set_bits(8 * index..=8 * index + 7, 0);
            _write(value);
        }
    };
}

// TODO: See if there is some way to make the macro take just an integer
// argument
macro_rules! pmpcfg {
    (
        $addr: expr, $csr: ident
    ) => {
        /// Physical memory protection configuration
        /// Struct pmpcfg{N} contains pmp{N}cfg - pmp{N+3}cfg for RV32, and pmp{N}cfg - pmp{N+7}cfg for RV64
        pub mod $csr {
            use super::{PmpCfg, PmpCfgCsr};
            use bit_field::BitField;

            read_csr!($addr);
            write_csr!($addr);

            #[inline]
            pub fn read() -> PmpCfgCsr {
                return unsafe { PmpCfgCsr::from(_read()) };
            }

            #[inline]
            pub fn write(cfg: PmpCfgCsr) {
                unsafe {
                    _write(cfg.into());
                }
            }

            set_pmpcfg!();
            clear_pmpcfg!();
        }
    };
}

pmpcfg!(0x3A0, pmpcfg0);

#[cfg(riscv32)]
pmpcfg!(0x3A1, pmpcfg1);

pmpcfg!(0x3A2, pmpcfg2);

#[cfg(riscv32)]
pmpcfg!(0x3A3, pmpcfg3);

pmpcfg!(0x3A4, pmpcfg4);

#[cfg(riscv32)]
pmpcfg!(0x3A5, pmpcfg5);

pmpcfg!(0x3A6, pmpcfg6);

#[cfg(riscv32)]
pmpcfg!(0x3A7, pmpcfg7);

pmpcfg!(0x3A8, pmpcfg8);

#[cfg(riscv32)]
pmpcfg!(0x3A9, pmpcfg9);

pmpcfg!(0x3AA, pmpcfg10);

#[cfg(riscv32)]
pmpcfg!(0x3AB, pmpcfg11);

pmpcfg!(0x3AC, pmpcfg12);

#[cfg(riscv32)]
pmpcfg!(0x3AD, pmpcfg13);

pmpcfg!(0x3AE, pmpcfg14);

#[cfg(riscv32)]
pmpcfg!(0x3AF, pmpcfg15);
