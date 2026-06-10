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

unsafe fn f7(a0: *const u8) -> usize {
    libc::strlen(a0 as *const i8)
}

unsafe fn f8(a0: *const u8, a1: *const u8) -> i32 {
    libc::strcmp(a0 as *const i8, a1 as *const i8)
}

unsafe fn f9(a0: *const u8, a1: *const u8, a2: usize) -> i32 {
    libc::strncmp(a0 as *const i8, a1 as *const i8, a2 as usize)
}

unsafe fn f10(a0: *const u8, a1: i32, a2: usize) -> *mut ::libc::c_void {
    libc::memchr(a0 as *const ::libc::c_void, a1, a2 as usize)
}

unsafe fn f11(a0: *const u8, a1: i32) -> *mut u8 {
    libc::strrchr(a0 as *const i8, a1) as *mut u8
}

unsafe fn f12(a0: *const u8, a1: i32, a2: usize) -> *const ::libc::c_void {
    libc::memchr(a0 as *const ::libc::c_void, a1, a2 as usize) as *const ::libc::c_void
}

unsafe fn f13(a0: *const u8, a1: i32) -> *const u8 {
    libc::strrchr(a0 as *const i8, a1) as *const u8
}

unsafe fn f14(a0: *mut u8, a1: i32) -> *mut u8 {
    libc::strrchr(a0 as *const i8, a1) as *mut u8
}

unsafe fn f15(a0: *const u8) -> *mut u8 {
    libcc2rs::strdup_unsafe(a0)
}

unsafe fn f16(a0: *const u8, a1: *const u8) -> usize {
    libc::strcspn(a0 as *const i8, a1 as *const i8)
}

unsafe fn f17(a0: *const u8, a1: *const u8) -> usize {
    libc::strspn(a0 as *const i8, a1 as *const i8)
}

unsafe fn f18(a0: *const u8, a1: *const u8) -> *mut u8 {
    libc::strstr(a0 as *const i8, a1 as *const i8) as *mut u8
}

unsafe fn f19(a0: *const u8, a1: *const u8) -> *const u8 {
    libc::strstr(a0 as *const i8, a1 as *const i8) as *const u8
}

unsafe fn f20(a0: *mut u8, a1: *const u8) -> *mut u8 {
    libc::strstr(a0 as *const i8, a1 as *const i8) as *mut u8
}

unsafe fn f21(a0: *const u8, a1: *const u8) -> *mut u8 {
    libc::strpbrk(a0 as *const i8, a1 as *const i8) as *mut u8
}

unsafe fn f22(a0: *const u8, a1: *const u8) -> *const u8 {
    libc::strpbrk(a0 as *const i8, a1 as *const i8) as *const u8
}

unsafe fn f23(a0: *mut u8, a1: *const u8) -> *mut u8 {
    libc::strpbrk(a0 as *const i8, a1 as *const i8) as *mut u8
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

unsafe fn f27(a0: *const u8, a1: *const u8) -> i32 {
    libc::strcasecmp(a0 as *const i8, a1 as *const i8)
}

// From the man page:
//
// The GNU-specific strerror_r() returns a pointer to a string containing the error message.  This
// may be either a pointer to a string that the function stores in buf, or a  pointer to some
// (immutable) static string (in which case buf is unused)
//
// So it's not 100% correct to always return a1. But the Rust libc version only returns int.
#[cfg(target_os = "linux")]
unsafe fn f28(a0: i32, a1: *mut u8, a2: usize) -> *mut u8 {
    libc::strerror_r(a0, a1 as *mut i8, a2 as usize);
    a1
}

#[cfg(target_os = "macos")]
unsafe fn f28(a0: i32, a1: *mut u8, a2: usize) -> i32 {
    libc::strerror_r(a0, a1 as *mut i8, a2 as usize)
}
