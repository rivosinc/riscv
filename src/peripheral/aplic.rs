//! Advanced Platform-Level Interrupt Controller (APLIC)
//!
//! Ref: [RISC-V Advanced Interrupt Architecture (AIA)](https://github.com/riscv/riscv-aia/releases)

use volatile_register::RW;

/// APLIC register block
#[repr(C)]
pub struct Aplic {
    /// 0x0000 4 bytes domaincfg
    domaincfg: RW<u32>,
    /// 0x0004 4 bytes sourcecfg[1]
    /// 0x0008 4 bytes sourcecfg[2]
    /// ..
    /// 0x0FFC 4 bytes sourcecfg[1023]
    sourcecfg: [RW<u32>; 1023],

    _padding0: [u32; 752],

    /// 0x1BC0 4 bytes mmsiaddrcfg  (machine-level interrupt domains only)
    mmsiaddrcfg: RW<u32>,
    /// 0x1BC4 4 bytes mmsiaddrcfgh  "
    mmsiaddrcfgh: RW<u32>,
    /// 0x1BC8 4 bytes smsiaddrcfg   "
    smsiaddrcfg: RW<u32>,
    /// 0x1BCC 4 bytes smsiaddrcfgh  "
    smsiaddrcfgh: RW<u32>,

    _padding1: [u32; 12],

    /// 0x1C00 4 bytes setip[0]
    /// 0x1C04 4 bytes setip[1]
    /// ...
    /// 0x1C7C 4 bytes setip[31]
    setip: [RW<u32>; 32],

    _padding2: [u32; 23],

    /// 0x1CDC 4 bytes setipnum
    setipnum: RW<u32>,
    _padding3: [u32; 8],

    /// 0x1D00 4 bytes in_clrip[0]
    /// 0x1D04 4 bytes in_clrip[1]
    /// ...
    /// 0x1D7C 4 bytes in_clrip[31]
    in_clrip: [RW<u32>; 32],

    _padding4: [u32; 23],

    /// 0x1DDC 4 bytes clripnum
    clripnum: RW<u32>,

    _padding5: [u32; 8],

    /// 0x1E00 4 bytes setie[0]
    /// 0x1E04 4 bytes setie[1]
    /// ..
    /// 0x1E7C 4 bytes setie[31]
    setie: [RW<u32>; 32],

    _padding6: [u32; 23],

    /// 0x1EDC 4 bytes setienum
    setienum: RW<u32>,

    _padding7: [u32; 8],

    /// 0x1F00 4 bytes clrie[0]
    /// 0x1F04 4 bytes clrie[1]
    /// ...
    /// 0x1F7C 4 bytes clrie[31]
    clrie: [RW<u32>; 32],

    _padding8: [u32; 23],

    /// 0x1FDC 4 bytes clrienum
    clrienum: RW<u32>,

    _padding9: [u32; 8],

    /// 0x2000 4 bytes setipnum_le
    setipnum_le: RW<u32>,
    /// 0x2004 4 bytes setipnum_be
    setipnum_be: RW<u32>,

    _padding10: [u32; 1022],

    /// 0x3000 4 bytes genmsi
    genmsi: RW<u32>,
    /// 0x3004 4 bytes target[1]
    /// 0x3008 4 bytes target[2]
    /// ...
    /// 0x3FFC 4 bytes target[1023]
    target: [RW<u32>; 1023],
}

#[derive(PartialEq)]
#[repr(u32)]
pub enum InterruptEnable {
    Disabled = 0,
    Enabled = 1 << 8,
}

#[derive(PartialEq)]
#[repr(u32)]
pub enum DeliveryMode {
    DirectDeliveryMode = 0,
    MSIDeliveryMode = 1 << 2,
}

#[derive(PartialEq)]
#[repr(u32)]
pub enum Endian {
    LittleEndian = 0,
    BigEndian = 1,
}

#[derive(PartialEq)]
pub enum SourceModes {
    Inactive = 0,
    Detached = 1,
    EdgeRising = 4,
    EdgeFalling = 5,
    LevelHigh = 6,
    LevelLow = 7,
}

impl Aplic {
    /// Sets the domain configuration
    pub fn set_domaincfg(&mut self, ie: InterruptEnable, dm: DeliveryMode, be: Endian) {
        let domaincfg = ie as u32 | dm as u32 | be as u32;

        // Safety: Writes to the MMIO region
        unsafe {
            self.domaincfg.write(domaincfg);
        }
    }

    /// Sets up the Machine MSI address configuration
    ///
    /// Arguments:
    ///
    /// - `msi_addr` the physical address of the IMSIC used for
    ///   machine-mode interrupts
    /// - `mmsiaddrcfgh` the bit pattern representing the high-order
    ///   configuration specifying the MSI target addresses (hhxs, lhxs,
    ///   hhxw, and lhxw) as expected in the `mmsiaddrcfgh` register in
    ///   bits 28:12. The caller may also use this argument to set the
    ///   lock bit (bit 31), subject to the hardware implementation.
    ///
    /// # Safety
    ///
    /// This writes the memory address to allow hardware direct access
    /// to MSI without validating the address provided. The caller is
    /// responsible for ensuring that the MSI address and configuration
    /// is correct.
    pub unsafe fn set_mmsiaddrcfg(&mut self, msi_addr: usize, mmsiaddrcfgh: u32) {
        self.mmsiaddrcfg.write((msi_addr >> 12) as u32);
        self.mmsiaddrcfgh
            .write((msi_addr >> 44) as u32 | (mmsiaddrcfgh & 0xfffff000));
    }

    /// Sets up the Supervisor MSI address configuration
    ///
    /// Arguments:
    ///
    /// - `msi_addr` the physical address of the IMSIC used for
    ///   supervisor-mode interrupts
    /// - `smsiaddrcfgh` the bit pattern representing the high-order
    ///   configuration specifying the MSI target addresses (lhxs) as
    ///   expected in the `smsiaddrcfgh` register in bits 22:20.
    ///
    /// # Safety
    ///
    /// This writes the memory address to allow hardware direct access
    /// to MSI without validating the address provided. The caller is
    /// responsible for ensuring that the MSI address and configuration
    /// is correct.
    pub unsafe fn set_smsiaddrcfg(&mut self, msi_addr: usize, smsiaddrcfgh: u32) {
        self.smsiaddrcfg.write((msi_addr >> 12) as u32);
        self.smsiaddrcfgh
            .write((msi_addr >> 44) as u32 | (smsiaddrcfgh & 0xfffff000));
    }

    /// Delegate the interrupt to the specified child APLIC
    pub fn sourcecfg_delegate(&mut self, int: u32, child: u32) {
        assert!(int > 0 && int < 1024);
        assert!(child < 1024);
        let sourcecfg: u32 = 1 << 10 | child;

        // Safety: Writes to the MMIO region
        unsafe {
            self.sourcecfg[int as usize - 1].write(sourcecfg);
        }
    }

    /// Set the interrupt source configuration
    pub fn set_sourcecfg(&mut self, int: u32, mode: SourceModes) {
        assert!(int > 0 && int < 1024);
        let sourcecfg: u32 = mode as u32;

        // Safety: Writes to the MMIO region
        unsafe {
            self.sourcecfg[int as usize - 1].write(sourcecfg);
        }
    }

    /// Sets an interrupt target for an active source in MSI delivery mode
    ///
    /// Arguments:
    ///
    /// - `id` The interrupt id
    /// - `hart` Hart index
    /// - `guest` Guest index
    /// - `eiid` External Interrupt Identity to be signaled on the MSI.
    ///   By convention this is the same as the interrupt id.
    pub fn set_target_msi(&mut self, int: u32, hart: u32, guest: u32, eiid: u32) {
        assert!(int > 0 && int < 1024);
        assert!(hart < 16384);
        assert!(guest < 32);
        assert!(eiid < 1024);

        let target: u32 = (hart << 18) | (guest << 12) | eiid;

        // Safety: Writes to the MMIO region
        unsafe {
            self.target[int as usize - 1].write(target);
        }
    }

    /// Masks (disables) a specific interrupt id
    pub fn mask(&mut self, int: u32) {
        assert!(int > 0 && int < 1024);
        // Safety: Writes to the MMIO region
        unsafe {
            self.clrienum.write(int);
        }
    }

    /// Unmasks (enables) a specific interrupt id
    pub fn unmask(&mut self, int: u32) {
        assert!(int > 0 && int < 1024);
        // Safety: Writes to the MMIO region
        unsafe {
            self.setienum.write(int);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Aplic;
    use core::mem::size_of;
    use memoffset::offset_of;

    #[test]
    fn sizeof_register_block() {
        assert_eq!(size_of::<Aplic>(), 0x4000)
    }

    #[test]
    fn offset_of() {
        macro_rules! assert_offset {
            ($addr:literal, $struct:ident, $field:ident) => {{
                assert_eq!($addr, offset_of!($struct, $field));
            }};
        }
        assert_offset!(0x0000, Aplic, domaincfg);
        assert_offset!(0x0004, Aplic, sourcecfg);
        assert_offset!(0x1BC0, Aplic, mmsiaddrcfg);
        assert_offset!(0x1BC4, Aplic, mmsiaddrcfgh);
        assert_offset!(0x1BC8, Aplic, smsiaddrcfg);
        assert_offset!(0x1BCC, Aplic, smsiaddrcfgh);
        assert_offset!(0x1C00, Aplic, setip);
        assert_offset!(0x1CDC, Aplic, setipnum);
        assert_offset!(0x1D00, Aplic, in_clrip);
        assert_offset!(0x1DDC, Aplic, clripnum);
        assert_offset!(0x1E00, Aplic, setie);
        assert_offset!(0x1EDC, Aplic, setienum);
        assert_offset!(0x1F00, Aplic, clrie);
        assert_offset!(0x1FDC, Aplic, clrienum);
        assert_offset!(0x2000, Aplic, setipnum_le);
        assert_offset!(0x2004, Aplic, setipnum_be);
        assert_offset!(0x3000, Aplic, genmsi);
        assert_offset!(0x3004, Aplic, target);
    }
}
