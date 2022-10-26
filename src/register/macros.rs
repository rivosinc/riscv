// Copyright (c) 2022 by Rivos Inc.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

//! Various macros to help with creating CSR modules

/// This macro generates the constants and types for a read-write CSR.
macro_rules! rw_csr {
    // Use this if the CSR name can be matched exactly with the bitfield
    // generated using `register_bitfields!` Most CSRs can use this version.
    ($name:ident, $size:ty) => {
        paste::paste! {
            rw_csr!($name, [< $name:lower >], $size);
        }
    };
    // This version will need to be used if the CSR layout is not unique. An
    // example would be the PMP CSRs, there are multiple with the exact same
    // layout. Using this allows the same bitfield to be used multiple times.
    // Ex:
    //   rw_csr!(pmpcfg0, pmpcfg, usize);
    //   rw_csr!(pmpcfg1, pmpcfg, usize);
    //   rw_csr!(pmpcfg2, pmpcfg, usize);
    ($name:ident, $type:ty, $size:ty) => {
        csr_boilerplate!($name, $type, $size);
        csr_reads!($name, $size);
        csr_writes!($name, $size);
    };
}

/// This macro generates the constants and types for a read-only CSR.
macro_rules! ro_csr {
    ($name:ident, $size:ty) => {
        csr_boilerplate!($name, $name, $size);
        csr_reads!($name, $size);
    };
}

/// Generates the imports and types that all CSRs need
macro_rules! csr_boilerplate {
    ($name:ident, $type:ty, $size:ty) => {
        paste::paste! {
            use core::arch::asm;

            use tock_registers::LocalRegisterCopy;
            use tock_registers::register_bitfields;

            /// This allows the enums to be accessed directly from the module's
            /// namespace instead of having to go through the bitfield's as well
            pub use [< $name:lower >]::*;

            /// An alias of the CSR's index
            const INDEX: u16 = $crate::register::addresses::[< CSR_ $name:upper >];

            /// A typedef of the [`tock_registers::RegisterLongName`] associated with this CSR
            type LongName = [< $name:lower >]::Register;

            /// A typedef of the [`Field`] associated with this CSR
            pub type Field = tock_registers::fields::Field<$size, LongName>;

            /// A typedef of the [`LocalRegisterCopy`] associated with this CSR
            pub type Local = LocalRegisterCopy<$size, LongName>;
        }
    };
}

/// Generates the read functions for a CSR
macro_rules! csr_reads {
    ($name:ident, $size:ty) => {
        /// Reads the contents of a CSR.
        ///
        /// This method corresponds to the RISC-V `CSRR rd, csr`
        /// instruction where `rd = out(reg) <return value>`.
        #[inline]
        pub fn read() -> $size {
            let r: $size;
            unsafe {
                asm!("csrr {rd}, {csr}", rd = out(reg) r, csr = const INDEX);
            }
            r
        }

        /// Returns a [`tock_registers::LocalRegisterCopy`] for the CSR.
        #[inline]
        pub fn read_local() -> Local {
            Local::new(read())
        }

        /// Atomically read the specified field
        pub fn read_field(field: Field) -> $size {
            field.read(read())
        }

        /// Returns true if one or more bits in the field are set
        pub fn is_set(field: Field) -> bool {
            read_field(field) != 0
        }
    }
}

/// Generates the write functions for a CSR
macro_rules! csr_writes {
    ($name:ident, $size:ty) => {
        /// Writes the value of a CSR.
        ///
        /// This method corresponds to the RISC-V `CSRW csr, rs`
        /// instruction where `rs = in(reg) val_to_set`.
        #[inline]
        pub fn write(val_to_set: $size) {
            unsafe {
                asm!("csrw {csr}, {rs}", rs = in(reg) val_to_set, csr = const INDEX);
            }
        }

        /// Write a [`tock_registers::LocalRegisterCopy`] to the CSR
        #[inline]
        pub fn write_local(local: Local) {
            write(local.get());
        }

        /// Atomically swap the contents of a CSR
        ///
        /// Reads the current value of a CSR and replaces it with the
        /// specified value in a single instruction, returning the
        /// previous value.
        ///
        /// This method corresponds to the RISC-V `CSRRW rd, csr, rs1`
        /// instruction where `rs1 = in(reg) value_to_set` and `rd =
        /// out(reg) <return value>`.
        #[inline]
        pub fn atomic_replace(val_to_set: $size) -> $size {
            let r: $size;
            unsafe {
                asm!("csrrw {rd}, {csr}, {rs1}",
                     rd = out(reg) r,
                     csr = const INDEX,
                     rs1 = in(reg) val_to_set);
            }
            r
        }

        /// Atomically read a CSR and set bits specified in a bitmask
        ///
        /// This method corresponds to the RISC-V `CSRRS rd, csr, rs1`
        /// instruction where `rs1 = in(reg) bitmask` and `rd = out(reg)
        /// <return value>`.
        #[inline]
        pub fn read_and_set_bits(bitmask: $size) -> $size {
            let r: $size;
            unsafe {
                asm!("csrrs {rd}, {csr}, {rs1}",
                     rd = out(reg) r,
                     csr = const INDEX,
                     rs1 = in(reg) bitmask);
            }
            r
        }

        /// Atomically read a CSR and set bits specified in a bitmask
        ///
        /// This method corresponds to the RISC-V `CSRRS rd, csr, rs1`
        /// instruction where `rs1 = in(reg) bitmask` and `rd = out(reg)
        /// <return value>`.
        #[inline]
        pub fn read_and_clear_bits(bitmask: $size) -> $size {
            let r: $size;
            unsafe {
                asm!("csrrc {rd}, {csr}, {rs1}",
                     rd = out(reg) r,
                     csr = const INDEX,
                     rs1 = in(reg) bitmask);
            }
            r
        }

        /// Atomically read field and set all bits to 1
        ///
        /// This method corresponds to the RISC-V `CSRRS rd, csr, rs1`
        /// instruction, where `rs1` is the bitmask described by the
        /// [`Field`].
        ///
        /// The previous value of the field is returned.
        pub fn read_and_set_field(field: Field) -> $size {
            field.read(read_and_set_bits(field.mask << field.shift))
        }

        /// Atomically read field and set all bits to 0
        ///
        /// This method corresponds to the RISC-V `CSRRC rd, csr, rs1`
        /// instruction, where `rs1` is the bitmask described by the
        /// [`Field`].
        ///
        /// The previous value of the field is returned.
        pub fn read_and_clear_field(field: Field) -> $size {
            field.read(read_and_clear_bits(field.mask << field.shift))
        }

    }
}
