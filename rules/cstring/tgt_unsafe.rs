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

unsafe fn f5(a0: *const core::ffi::c_char, a1: i32) -> *mut core::ffi::c_char {
    libc::strchr(a0, a1)
}

unsafe fn f6(a0: *const core::ffi::c_char, a1: i32) -> *const core::ffi::c_char {
    (libc::strchr(a0, a1) as *const core::ffi::c_char)
}

unsafe fn f7(a0: *const core::ffi::c_char) -> usize {
    libc::strlen(a0)
}

unsafe fn f8(a0: *const core::ffi::c_char, a1: *const core::ffi::c_char) -> i32 {
    libc::strcmp(a0, a1)
}

unsafe fn f9(a0: *const core::ffi::c_char, a1: *const core::ffi::c_char, a2: usize) -> i32 {
    libc::strncmp(a0, a1, a2 as usize)
}

unsafe fn f10(a0: *const u8, a1: i32, a2: usize) -> *mut ::libc::c_void {
    libc::memchr(a0 as *const ::libc::c_void, a1, a2 as usize)
}

unsafe fn f11(a0: *const core::ffi::c_char, a1: i32) -> *mut core::ffi::c_char {
    libc::strrchr(a0, a1)
}

unsafe fn f12(a0: *const u8, a1: i32, a2: usize) -> *const ::libc::c_void {
    libc::memchr(a0 as *const ::libc::c_void, a1, a2 as usize) as *const ::libc::c_void
}

unsafe fn f13(a0: *const core::ffi::c_char, a1: i32) -> *const core::ffi::c_char {
    (libc::strrchr(a0, a1) as *const core::ffi::c_char)
}

unsafe fn f14(a0: *mut core::ffi::c_char, a1: i32) -> *mut core::ffi::c_char {
    libc::strrchr(a0, a1)
}

unsafe fn f15(a0: *const core::ffi::c_char) -> *mut core::ffi::c_char {
    libcc2rs::strdup_unsafe(a0)
}

unsafe fn f16(a0: *const core::ffi::c_char, a1: *const core::ffi::c_char) -> usize {
    libc::strcspn(a0, a1)
}

unsafe fn f17(a0: *const core::ffi::c_char, a1: *const core::ffi::c_char) -> usize {
    libc::strspn(a0, a1)
}

unsafe fn f18(
    a0: *const core::ffi::c_char,
    a1: *const core::ffi::c_char,
) -> *mut core::ffi::c_char {
    libc::strstr(a0, a1)
}

unsafe fn f19(
    a0: *const core::ffi::c_char,
    a1: *const core::ffi::c_char,
) -> *const core::ffi::c_char {
    (libc::strstr(a0, a1) as *const core::ffi::c_char)
}

unsafe fn f20(a0: *mut core::ffi::c_char, a1: *const core::ffi::c_char) -> *mut core::ffi::c_char {
    libc::strstr(a0, a1)
}

unsafe fn f21(
    a0: *const core::ffi::c_char,
    a1: *const core::ffi::c_char,
) -> *mut core::ffi::c_char {
    libc::strpbrk(a0, a1)
}

unsafe fn f22(
    a0: *const core::ffi::c_char,
    a1: *const core::ffi::c_char,
) -> *const core::ffi::c_char {
    (libc::strpbrk(a0, a1) as *const core::ffi::c_char)
}

unsafe fn f23(a0: *mut core::ffi::c_char, a1: *const core::ffi::c_char) -> *mut core::ffi::c_char {
    libc::strpbrk(a0, a1)
}

#[cfg(target_os = "linux")]
unsafe fn f24(a0: *const u8, a1: i32, a2: usize) -> *mut ::libc::c_void {
    libc::memrchr(a0 as *const ::libc::c_void, a1, a2 as usize)
}

#[cfg(target_os = "linux")]
unsafe fn f25(a0: *const u8, a1: i32, a2: usize) -> *const ::libc::c_void {
    libc::memrchr(a0 as *const ::libc::c_void, a1, a2 as usize) as *const ::libc::c_void
}

#[cfg(target_os = "linux")]
unsafe fn f26(a0: *mut u8, a1: i32, a2: usize) -> *mut ::libc::c_void {
    libc::memrchr(a0 as *const ::libc::c_void, a1, a2 as usize)
}

unsafe fn f27(a0: *const core::ffi::c_char, a1: *const core::ffi::c_char) -> i32 {
    libc::strcasecmp(a0, a1)
}

// From the man page:
//
// The GNU-specific strerror_r() returns a pointer to a string containing the error message.  This
// may be either a pointer to a string that the function stores in buf, or a  pointer to some
// (immutable) static string (in which case buf is unused)
//
// So it's not 100% correct to always return a1. But the Rust libc version only returns int.
#[cfg(target_os = "linux")]
unsafe fn f28(a0: i32, a1: *mut core::ffi::c_char, a2: usize) -> *mut core::ffi::c_char {
    libc::strerror_r(a0, a1, a2 as usize);
    a1
}

#[cfg(target_os = "macos")]
unsafe fn f28(a0: i32, a1: *mut core::ffi::c_char, a2: usize) -> i32 {
    libc::strerror_r(a0, a1, a2 as usize)
}
