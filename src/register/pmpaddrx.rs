use crate::register::pmpcfgx::Mode;
use bit_field::BitField;
use core::num::NonZeroU64;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct PmpAddr {
    bits: usize,
}

pub type Size = u64;
pub type NonZeroSize = NonZeroU64;
pub type Addr = u64;

impl PmpAddr {
    #[inline]
    pub fn decode(&self, mode: Mode) -> (Option<Addr>, Option<NonZeroSize>) {
        let big_bits: Addr = self.bits as Addr;
        match mode {
            Mode::OFF => (None, None),
            Mode::TOR => (Some(big_bits << 2), None),
            Mode::NA4 => (Some(big_bits << 2), Some(4.try_into().unwrap())),
            Mode::NAPOT => {
                let (addr, size) = Self::decode_napot(self.bits);
                (Some(addr), Some(size.try_into().unwrap()))
            }
        }
    }

    #[inline]
    pub fn encode(&mut self, mode: Mode, addr: Addr, size: Option<NonZeroSize>) -> Result<(), ()> {
        self.bits = match mode {
            Mode::OFF => 0,
            Mode::TOR => {
                let addr_small: usize = (addr >> 2) as usize;
                // this check both ensures the bottom two bits are zero and that the (addr >> 2)
                // was not truncated by the casting
                if (addr_small as u64) << 2 != addr {
                    return Err(());
                }
                (addr >> 2).try_into().unwrap()
            }
            Mode::NA4 => {
                let addr_small: usize = (addr >> 2) as usize;
                if (addr_small as u64) << 2 != addr {
                    return Err(());
                }
                (addr >> 2).try_into().unwrap()
            }
            Mode::NAPOT => Self::encode_napot(addr, size.unwrap().into())?,
        };
        Ok(())
    }

    #[inline]
    fn encode_napot(addr: Addr, size: Size) -> Result<usize, ()> {
        // the size is related to the number of sequential ones in the low bits
        let encoded_size: Size = (size - 1) >> 3;

        // verify size is not too big
        if (encoded_size > usize::MAX as Size) ||
            // check size is a power of 2
            (size == (size & !(size-1))) ||
            // checks that the low bits where size is placed, are already zero
            (addr & encoded_size != 0)
        {
            return Err(());
        }
        // See riscv priv spec "Physical Memory Protection CSRs
        // "Each PMP address register encodes bits 33–2 of a 34-bit physical address for RV32"
        // and
        // "For RV64, each PMP address register encodes bits 55–2 of a 56-bit physical address"
        let addr: usize = (addr >> 2) as usize;

        let mut pmpaddr: usize = 0;
        pmpaddr |= addr;
        // verify the provided size is valid
        pmpaddr |= encoded_size as usize;

        return Ok(pmpaddr);
    }

    #[inline]
    fn decode_napot(bits: usize) -> (Addr, Size) {
        let mut pmpaddr: usize = bits;
        //TODO: this will lose the high two bits if it was a 34 bit address
        let address = pmpaddr;

        // find first zero in pmpaddr
        let mut range_mask = 1;
        let mut size: Size = 8;
        while pmpaddr.get_bit(0) != false {
            pmpaddr = pmpaddr >> 1;
            range_mask = (range_mask << 1) | 0x1;
            size = size << 1;
        }

        let address = ((address & !range_mask) as Addr) << 2;
        return (address, size);
    }
}

impl From<usize> for PmpAddr {
    fn from(bits: usize) -> PmpAddr {
        return PmpAddr { bits: bits };
    }
}

macro_rules! reg {
    (
        $addr:expr, $csr:ident
    ) => {
        /// Physical memory protection address register
        pub mod $csr {
            use super::{Addr, PmpAddr, Size};

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
            pub fn read_tor() -> Addr {
                // See riscv priv spec "Physical Memory Protection CSRs
                // "Each PMP address register encodes bits 33–2 of a 34-bit physical address for RV32"
                // and
                // "For RV64, each PMP address register encodes bits 55–2 of a 56-bit physical address"
                unsafe {
                    return (_read() as Addr) << 2;
                }
            }
            #[inline]
            pub unsafe fn write_tor(addr: Addr) {
                // See riscv priv spec "Physical Memory Protection CSRs
                // "Each PMP address register encodes bits 33–2 of a 34-bit physical address for RV32"
                // and
                // "For RV64, each PMP address register encodes bits 55–2 of a 56-bit physical address"
                let addr = addr >> 2;
                _write(addr.try_into().unwrap());
            }

            #[inline]
            pub fn read_na4() -> Addr {
                // See riscv priv spec "Physical Memory Protection CSRs
                // "Each PMP address register encodes bits 33–2 of a 34-bit physical address for RV32"
                // and
                // "For RV64, each PMP address register encodes bits 55–2 of a 56-bit physical address"
                unsafe {
                    return (_read() as Addr) << 2;
                }
            }
            #[inline]
            pub unsafe fn write_na4(addr: Addr) {
                // See riscv priv spec "Physical Memory Protection CSRs
                // "Each PMP address register encodes bits 33–2 of a 34-bit physical address for RV32"
                // and
                // "For RV64, each PMP address register encodes bits 55–2 of a 56-bit physical address"
                let addr = addr >> 2;
                _write(addr.try_into().unwrap());
            }

            #[inline]
            pub unsafe fn write_napot(addr: Addr, size: Size) {
                _write(PmpAddr::encode_napot(addr, size).unwrap());
            }

            #[inline]
            pub fn read_napot() -> (Addr, Size) {
                unsafe { PmpAddr::decode_napot(_read()) }
            }
        }
    };
}

pub unsafe fn write_tor_indexed(index: usize, addr: Addr) {
    assert!(index < 64);

    match index {
        0 => pmpaddr0::write_tor(addr),
        1 => pmpaddr1::write_tor(addr),
        2 => pmpaddr2::write_tor(addr),
        3 => pmpaddr3::write_tor(addr),
        4 => pmpaddr4::write_tor(addr),
        5 => pmpaddr5::write_tor(addr),
        6 => pmpaddr6::write_tor(addr),
        7 => pmpaddr7::write_tor(addr),
        8 => pmpaddr8::write_tor(addr),
        9 => pmpaddr9::write_tor(addr),
        10 => pmpaddr10::write_tor(addr),
        11 => pmpaddr11::write_tor(addr),
        12 => pmpaddr12::write_tor(addr),
        13 => pmpaddr13::write_tor(addr),
        14 => pmpaddr14::write_tor(addr),
        15 => pmpaddr15::write_tor(addr),
        16 => pmpaddr16::write_tor(addr),
        17 => pmpaddr17::write_tor(addr),
        18 => pmpaddr18::write_tor(addr),
        19 => pmpaddr19::write_tor(addr),
        20 => pmpaddr20::write_tor(addr),
        21 => pmpaddr21::write_tor(addr),
        22 => pmpaddr22::write_tor(addr),
        23 => pmpaddr23::write_tor(addr),
        24 => pmpaddr24::write_tor(addr),
        25 => pmpaddr25::write_tor(addr),
        26 => pmpaddr26::write_tor(addr),
        27 => pmpaddr27::write_tor(addr),
        28 => pmpaddr28::write_tor(addr),
        29 => pmpaddr29::write_tor(addr),
        30 => pmpaddr30::write_tor(addr),
        31 => pmpaddr31::write_tor(addr),
        32 => pmpaddr32::write_tor(addr),
        33 => pmpaddr33::write_tor(addr),
        34 => pmpaddr34::write_tor(addr),
        35 => pmpaddr35::write_tor(addr),
        36 => pmpaddr36::write_tor(addr),
        37 => pmpaddr37::write_tor(addr),
        38 => pmpaddr38::write_tor(addr),
        39 => pmpaddr39::write_tor(addr),
        40 => pmpaddr40::write_tor(addr),
        41 => pmpaddr41::write_tor(addr),
        42 => pmpaddr42::write_tor(addr),
        43 => pmpaddr43::write_tor(addr),
        44 => pmpaddr44::write_tor(addr),
        45 => pmpaddr45::write_tor(addr),
        46 => pmpaddr46::write_tor(addr),
        47 => pmpaddr47::write_tor(addr),
        48 => pmpaddr48::write_tor(addr),
        49 => pmpaddr49::write_tor(addr),
        50 => pmpaddr50::write_tor(addr),
        51 => pmpaddr51::write_tor(addr),
        52 => pmpaddr52::write_tor(addr),
        53 => pmpaddr53::write_tor(addr),
        54 => pmpaddr54::write_tor(addr),
        55 => pmpaddr55::write_tor(addr),
        56 => pmpaddr56::write_tor(addr),
        57 => pmpaddr57::write_tor(addr),
        58 => pmpaddr58::write_tor(addr),
        59 => pmpaddr59::write_tor(addr),
        60 => pmpaddr60::write_tor(addr),
        61 => pmpaddr61::write_tor(addr),
        62 => pmpaddr62::write_tor(addr),
        63 => pmpaddr63::write_tor(addr),
        _ => unreachable!(),
    }
}

pub unsafe fn write_napot_indexed(index: usize, addr: Addr, size: Size) {
    assert!(index < 64);

    match index {
        0 => pmpaddr0::write_napot(addr, size),
        1 => pmpaddr1::write_napot(addr, size),
        2 => pmpaddr2::write_napot(addr, size),
        3 => pmpaddr3::write_napot(addr, size),
        4 => pmpaddr4::write_napot(addr, size),
        5 => pmpaddr5::write_napot(addr, size),
        6 => pmpaddr6::write_napot(addr, size),
        7 => pmpaddr7::write_napot(addr, size),
        8 => pmpaddr8::write_napot(addr, size),
        9 => pmpaddr9::write_napot(addr, size),
        10 => pmpaddr10::write_napot(addr, size),
        11 => pmpaddr11::write_napot(addr, size),
        12 => pmpaddr12::write_napot(addr, size),
        13 => pmpaddr13::write_napot(addr, size),
        14 => pmpaddr14::write_napot(addr, size),
        15 => pmpaddr15::write_napot(addr, size),
        16 => pmpaddr16::write_napot(addr, size),
        17 => pmpaddr17::write_napot(addr, size),
        18 => pmpaddr18::write_napot(addr, size),
        19 => pmpaddr19::write_napot(addr, size),
        20 => pmpaddr20::write_napot(addr, size),
        21 => pmpaddr21::write_napot(addr, size),
        22 => pmpaddr22::write_napot(addr, size),
        23 => pmpaddr23::write_napot(addr, size),
        24 => pmpaddr24::write_napot(addr, size),
        25 => pmpaddr25::write_napot(addr, size),
        26 => pmpaddr26::write_napot(addr, size),
        27 => pmpaddr27::write_napot(addr, size),
        28 => pmpaddr28::write_napot(addr, size),
        29 => pmpaddr29::write_napot(addr, size),
        30 => pmpaddr30::write_napot(addr, size),
        31 => pmpaddr31::write_napot(addr, size),
        32 => pmpaddr32::write_napot(addr, size),
        33 => pmpaddr33::write_napot(addr, size),
        34 => pmpaddr34::write_napot(addr, size),
        35 => pmpaddr35::write_napot(addr, size),
        36 => pmpaddr36::write_napot(addr, size),
        37 => pmpaddr37::write_napot(addr, size),
        38 => pmpaddr38::write_napot(addr, size),
        39 => pmpaddr39::write_napot(addr, size),
        40 => pmpaddr40::write_napot(addr, size),
        41 => pmpaddr41::write_napot(addr, size),
        42 => pmpaddr42::write_napot(addr, size),
        43 => pmpaddr43::write_napot(addr, size),
        44 => pmpaddr44::write_napot(addr, size),
        45 => pmpaddr45::write_napot(addr, size),
        46 => pmpaddr46::write_napot(addr, size),
        47 => pmpaddr47::write_napot(addr, size),
        48 => pmpaddr48::write_napot(addr, size),
        49 => pmpaddr49::write_napot(addr, size),
        50 => pmpaddr50::write_napot(addr, size),
        51 => pmpaddr51::write_napot(addr, size),
        52 => pmpaddr52::write_napot(addr, size),
        53 => pmpaddr53::write_napot(addr, size),
        54 => pmpaddr54::write_napot(addr, size),
        55 => pmpaddr55::write_napot(addr, size),
        56 => pmpaddr56::write_napot(addr, size),
        57 => pmpaddr57::write_napot(addr, size),
        58 => pmpaddr58::write_napot(addr, size),
        59 => pmpaddr59::write_napot(addr, size),
        60 => pmpaddr60::write_napot(addr, size),
        61 => pmpaddr61::write_napot(addr, size),
        62 => pmpaddr62::write_napot(addr, size),
        63 => pmpaddr63::write_napot(addr, size),
        _ => unimplemented!(),
    }
}

pub unsafe fn write_na4_indexed(index: usize, addr: Addr, size: Size) {
    write_napot_indexed(index, addr, size);
}

reg!(0x3b0, pmpaddr0);
reg!(0x3b1, pmpaddr1);
reg!(0x3b2, pmpaddr2);
reg!(0x3b3, pmpaddr3);
reg!(0x3b4, pmpaddr4);
reg!(0x3b5, pmpaddr5);
reg!(0x3b6, pmpaddr6);
reg!(0x3b7, pmpaddr7);
reg!(0x3b8, pmpaddr8);
reg!(0x3b9, pmpaddr9);
reg!(0x3ba, pmpaddr10);
reg!(0x3bb, pmpaddr11);
reg!(0x3bc, pmpaddr12);
reg!(0x3bd, pmpaddr13);
reg!(0x3be, pmpaddr14);
reg!(0x3bf, pmpaddr15);
reg!(0x3c0, pmpaddr16);
reg!(0x3c1, pmpaddr17);
reg!(0x3c2, pmpaddr18);
reg!(0x3c3, pmpaddr19);
reg!(0x3c4, pmpaddr20);
reg!(0x3c5, pmpaddr21);
reg!(0x3c6, pmpaddr22);
reg!(0x3c7, pmpaddr23);
reg!(0x3c8, pmpaddr24);
reg!(0x3c9, pmpaddr25);
reg!(0x3ca, pmpaddr26);
reg!(0x3cb, pmpaddr27);
reg!(0x3cc, pmpaddr28);
reg!(0x3cd, pmpaddr29);
reg!(0x3ce, pmpaddr30);
reg!(0x3cf, pmpaddr31);
reg!(0x3d0, pmpaddr32);
reg!(0x3d1, pmpaddr33);
reg!(0x3d2, pmpaddr34);
reg!(0x3d3, pmpaddr35);
reg!(0x3d4, pmpaddr36);
reg!(0x3d5, pmpaddr37);
reg!(0x3d6, pmpaddr38);
reg!(0x3d7, pmpaddr39);
reg!(0x3d8, pmpaddr40);
reg!(0x3d9, pmpaddr41);
reg!(0x3da, pmpaddr42);
reg!(0x3db, pmpaddr43);
reg!(0x3dc, pmpaddr44);
reg!(0x3dd, pmpaddr45);
reg!(0x3de, pmpaddr46);
reg!(0x3df, pmpaddr47);
reg!(0x3e0, pmpaddr48);
reg!(0x3e1, pmpaddr49);
reg!(0x3e2, pmpaddr50);
reg!(0x3e3, pmpaddr51);
reg!(0x3e4, pmpaddr52);
reg!(0x3e5, pmpaddr53);
reg!(0x3e6, pmpaddr54);
reg!(0x3e7, pmpaddr55);
reg!(0x3e8, pmpaddr56);
reg!(0x3e9, pmpaddr57);
reg!(0x3ea, pmpaddr58);
reg!(0x3eb, pmpaddr59);
reg!(0x3ec, pmpaddr60);
reg!(0x3ed, pmpaddr61);
reg!(0x3ee, pmpaddr62);
reg!(0x3ef, pmpaddr63);
