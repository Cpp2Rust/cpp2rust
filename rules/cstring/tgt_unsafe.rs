// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(a0: *mut u8, a1: *const u8, a2: usize) -> *mut u8 {
    if a2 != 0 {
        ::std::ptr::copy_nonoverlapping(a1, a0, a2 as usize)
    }
    a0
}

unsafe fn f2(a0: *mut u8, a1: *const u8, a2: usize) -> *mut u8 {
    let byte_0 = a0 as *mut u8;
    for offset in 0..a2 {
        *byte_0.offset(offset as isize) = a1 as u8;
    }
    a0
}

unsafe fn f3(a0: *const u8, a1: *const u8, a2: usize) -> i32 {
    let sa = core::slice::from_raw_parts(a0 as *const u8, a2 as usize);
    let sb = core::slice::from_raw_parts(a1 as *const u8, a2 as usize);
    let mut diff = 0_i32;
    for (x, y) in sa.iter().zip(sb.iter()) {
        if x != y {
            diff = (*x as i32) - (*y as i32);
            break;
        }
    }
    diff
}

unsafe fn f4(a0: *mut u8, a1: *const u8, a2: usize) -> *mut u8 {
    if a2 != 0 {
        ::std::ptr::copy_nonoverlapping(a1, a0, a2 as usize)
    }
    a0
}

unsafe fn f5(a0: *const u8, a1: i32) -> *mut u8 {
    libc::strchr(a0 as *const i8, a1) as *mut u8
}

unsafe fn f6(a0: *const u8, a1: i32) -> *const u8 {
    libc::strchr(a0 as *const i8, a1) as *const u8
}

unsafe fn f7(a0: *const u8) -> u64 {
    libc::strlen(a0 as *const i8) as u64
}

unsafe fn f8(a0: *const u8, a1: *const u8) -> i32 {
    libc::strcmp(a0 as *const i8, a1 as *const i8)
}

unsafe fn f9(a0: *const u8, a1: *const u8, a2: usize) -> i32 {
    libc::strncmp(a0 as *const i8, a1 as *const i8, a2 as usize)
}
