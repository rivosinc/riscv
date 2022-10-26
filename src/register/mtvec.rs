// Copyright (c) 2022 by Rivos Inc.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

//! mtvec register

rw_csr!(mtvec, usize);

register_bitfields![usize,
    #[cfg(target_pointer_width = "32")]
    pub mtvec [
        mode OFFSET(0) NUMBITS(2) [
            Direct = 0,
            Vectored = 1,
       ],
        base OFFSET(2) NUMBITS(30) [],
    ],
    #[cfg(target_pointer_width = "64")]
    pub mtvec [
        mode OFFSET(0) NUMBITS(2) [
            Direct = 0,
            Vectored = 1,
       ],
        base OFFSET(2) NUMBITS(62) [],
    ],
];

/// Returns true if the trap vector mode is set to `Direct`.
#[inline]
pub fn is_direct() -> bool {
    read_field(mode) == 0x0
}

/// Returns true if the trap vector mode is set to `Vectored`.
#[inline]
pub fn is_vectored() -> bool {
    read_field(mode) == 0x1
}

/// Sets the trap vector mode to `Direct`.
#[inline]
pub fn set_direct() {
    let mut local = read_local();
    local.write(mode::Direct);
    write_local(local);
}

/// Sets the trap vector mode to `Vectored`.
#[inline]
pub fn set_vectored() {
    let mut local = read_local();
    local.write(mode::Vectored);
    write_local(local);
}

/// Sets the trap vector base.
#[inline]
pub fn set_base(base_val: usize) {
    let mut local = read_local();
    local.write(base.val(base_val));
    write_local(local);
}
