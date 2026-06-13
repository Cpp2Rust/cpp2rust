// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(a0: *const core::ffi::c_char) -> u32 {
    libc::if_nametoindex(a0)
}
