use bit_field::BitField;
use crate::register::pmpcfgx::Mode;
use core::num::NonZeroUsize;

#[derive(Copy, Clone, Debug)]
pub struct PmpAddr {
    bits: usize,
}

impl PmpAddr {
        #[inline]
        pub fn decode(&self, mode: Mode) -> (usize, Option<NonZeroUsize>) {
            match mode {
                Mode::OFF => (self.bits, None),
                Mode::TOR => (self.bits << 2, None),
                Mode::NA4 => (self.bits << 2, None),
                Mode::NAPOT => {
                    let (addr, size) = Self::decode_napot(self.bits);
                    (addr, Some(size.try_into().unwrap()))
                }
            }
        }

        #[inline]
        pub fn encode(&mut self, mode: Mode, addr: usize, size: Option<NonZeroUsize>) {
            self.bits = match mode {
                Mode::OFF => 0,
                Mode::TOR => addr >> 2,
                Mode::NA4 => addr >> 2,
                Mode::NAPOT => Self::encode_napot(addr, size.unwrap().into()),
            }
        }

        #[inline]
        fn encode_napot(addr: usize, size: usize) -> usize {
            // See riscv priv spec "Physical Memory Protection CSRs
            // "Each PMP address register encodes bits 33–2 of a 34-bit physical address for RV32"
            // and
            // "For RV64, each PMP address register encodes bits 55–2 of a 56-bit physical address"
            // TODO: top bits will get lost on 64bit system
            let addr = addr >> 2;

            let mut pmpaddr: usize = 0;
            pmpaddr |= addr;
            pmpaddr |= (size - 1) >> 3;

            return pmpaddr;
        }

        #[inline]
        fn decode_napot(bits: usize) -> (usize, usize) {
            let mut pmpaddr: usize = bits;
            //TODO: this will lose the high two bits if it was a 34 bit address
            let address = pmpaddr;

            // find first zero in pmpaddr
            let mut range_mask = 1;
            let mut size = 8;
            while pmpaddr.get_bit(0) != false {
                pmpaddr = pmpaddr >> 1;
                range_mask = (range_mask << 1) | 0x1;
                size = size << 1;
            }

            let address = (address & range_mask) << 2;
            return (address, size);
        }
}

impl From<usize> for PmpAddr {
    fn from(bits: usize) -> PmpAddr {
        return PmpAddr {bits: bits};
    }
}

macro_rules! reg {
    (
        $addr:expr, $csr:ident
    ) => {
        /// Physical memory protection address register
        pub mod $csr {
            use super::PmpAddr;

            read_csr!($addr);
            write_csr!($addr);

            #[inline]
            pub unsafe fn write(pmpaddr: PmpAddr) {
                _write(pmpaddr.bits);
            }

            #[inline]
            pub unsafe fn read() -> PmpAddr {
                _read().into()
            }

            #[inline]
            pub fn read_tor() -> usize {
                // See riscv priv spec "Physical Memory Protection CSRs
                // "Each PMP address register encodes bits 33–2 of a 34-bit physical address for RV32"
                // and
                // "For RV64, each PMP address register encodes bits 55–2 of a 56-bit physical address"
                unsafe {
                    return _read() << 2;
                }
            }
            #[inline]
            pub unsafe fn write_tor(addr: usize) {
                // See riscv priv spec "Physical Memory Protection CSRs
                // "Each PMP address register encodes bits 33–2 of a 34-bit physical address for RV32"
                // and
                // "For RV64, each PMP address register encodes bits 55–2 of a 56-bit physical address"
                let addr = addr >> 2;
                _write(addr);
            }

            #[inline]
            pub fn read_na4() -> usize {
                // See riscv priv spec "Physical Memory Protection CSRs
                // "Each PMP address register encodes bits 33–2 of a 34-bit physical address for RV32"
                // and
                // "For RV64, each PMP address register encodes bits 55–2 of a 56-bit physical address"
                unsafe {
                    return _read() << 2;
                }
            }
            #[inline]
            pub unsafe fn write_na4(addr: usize) {
                // See riscv priv spec "Physical Memory Protection CSRs
                // "Each PMP address register encodes bits 33–2 of a 34-bit physical address for RV32"
                // and
                // "For RV64, each PMP address register encodes bits 55–2 of a 56-bit physical address"
                let addr = addr >> 2;
                _write(addr);
            }

            #[inline]
            pub unsafe fn write_napot(addr: usize, size: usize) {
                _write(PmpAddr::encode_napot(addr, size));
            }

            #[inline]
            pub fn read_napot() -> (usize, usize) {
                unsafe { PmpAddr::decode_napot(_read()) }
            }

        }
    };
}

reg!(0x3B0, pmpaddr0);
reg!(0x3B1, pmpaddr1);
reg!(0x3B2, pmpaddr2);
reg!(0x3B3, pmpaddr3);
reg!(0x3B4, pmpaddr4);
reg!(0x3B5, pmpaddr5);
reg!(0x3B6, pmpaddr6);
reg!(0x3B7, pmpaddr7);
reg!(0x3B8, pmpaddr8);
reg!(0x3B9, pmpaddr9);
reg!(0x3BA, pmpaddr10);
reg!(0x3BB, pmpaddr11);
reg!(0x3BC, pmpaddr12);
reg!(0x3BD, pmpaddr13);
reg!(0x3BE, pmpaddr14);
reg!(0x3BF, pmpaddr15);
