// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f1(a0: AnyPtr, a1: AnyPtr, a2: usize) -> AnyPtr {
    a0.memcpy(&a1, a2 as usize);
    a0.clone()
}

fn f2(a0: AnyPtr, a1: u8, a2: usize) -> AnyPtr {
    a0.memset((a1) as u8, a2 as usize);
    a0.clone()
}

fn f3(a0: AnyPtr, a1: AnyPtr, a2: usize) -> i32 {
    a0.memcmp(&a1, a2)
}

fn f4(a0: AnyPtr, a1: AnyPtr, a2: usize) -> AnyPtr {
    a0.memcpy(&a1, a2 as usize);
    a0.clone()
}

unsafe fn f7(a0: Ptr<core::ffi::c_char>) -> usize {
    a0.to_string_iterator().count()
}
