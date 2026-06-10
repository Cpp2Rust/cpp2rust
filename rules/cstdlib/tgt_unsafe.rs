// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1() {
    std::process::abort();
}

unsafe fn f2(a0: *mut ::libc::c_void) {
    libcc2rs::free_unsafe(a0)
}

unsafe fn f3(a0: usize) -> *mut ::libc::c_void {
    libcc2rs::malloc_unsafe(a0)
}

unsafe fn f4(a0: *mut ::libc::c_void, a1: usize) -> *mut ::libc::c_void {
    libcc2rs::realloc_unsafe(a0, a1)
}

unsafe fn f5(a0: usize, a1: usize) -> *mut ::libc::c_void {
    libcc2rs::calloc_unsafe(a0, a1)
}

unsafe fn f6(a0: *const u8) -> *mut u8 {
    libc::getenv(a0 as *const i8) as *mut u8
}

unsafe fn f7(a0: *const u8, a1: *const u8, a2: i32) -> i32 {
    libc::setenv(a0 as *const i8, a1 as *const i8, a2)
}

unsafe fn f8(
    a0: *const ::libc::c_void,
    a1: *const ::libc::c_void,
    a2: usize,
    a3: usize,
    a4: unsafe fn(*const ::libc::c_void, *const ::libc::c_void) -> i32,
) -> *mut ::libc::c_void {
    libc::bsearch(
        a0,
        a1,
        a2,
        a3,
        Some(std::mem::transmute::<
            *const (),
            unsafe extern "C" fn(*const ::libc::c_void, *const ::libc::c_void) -> i32,
        >(a4 as *const ())),
    )
}

unsafe fn f9(
    a0: *mut ::libc::c_void,
    a1: usize,
    a2: usize,
    a3: unsafe fn(*const ::libc::c_void, *const ::libc::c_void) -> i32,
) {
    libc::qsort(
        a0,
        a1,
        a2,
        Some(std::mem::transmute::<
            *const (),
            unsafe extern "C" fn(*const ::libc::c_void, *const ::libc::c_void) -> i32,
        >(a3 as *const ())),
    )
}

unsafe fn f10(a0: *const u8, a1: *mut u8) -> *mut u8 {
    libc::realpath(a0 as *const i8, a1 as *mut i8) as *mut u8
}
