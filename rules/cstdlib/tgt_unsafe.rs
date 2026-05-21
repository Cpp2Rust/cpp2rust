// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1() {
    std::process::abort();
}

unsafe fn f2(a0: *mut ::libc::c_void) {
    libcc2rs::free_unsafe(a0)
}

unsafe fn f3(a0: u64) -> *mut ::libc::c_void {
    libcc2rs::malloc_unsafe(a0)
}

unsafe fn f4(a0: *mut ::libc::c_void, a1: u64) -> *mut ::libc::c_void {
    libcc2rs::realloc_unsafe(a0, a1)
}

unsafe fn f5(a0: u64, a1: u64) -> *mut ::libc::c_void {
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
    a2: u64,
    a3: u64,
    a4: unsafe fn(*const ::libc::c_void, *const ::libc::c_void) -> i32,
) -> *mut ::libc::c_void {
    libc::bsearch(
        a0,
        a1,
        a2 as ::libc::size_t,
        a3 as ::libc::size_t,
        Some(std::mem::transmute::<
            *const (),
            unsafe extern "C" fn(*const ::libc::c_void, *const ::libc::c_void) -> i32,
        >(a4 as *const ())),
    )
}

unsafe fn f9(
    a0: *mut ::libc::c_void,
    a1: u64,
    a2: u64,
    a3: unsafe fn(*const ::libc::c_void, *const ::libc::c_void) -> i32,
) {
    libc::qsort(
        a0,
        a1 as ::libc::size_t,
        a2 as ::libc::size_t,
        Some(std::mem::transmute::<
            *const (),
            unsafe extern "C" fn(*const ::libc::c_void, *const ::libc::c_void) -> i32,
        >(a3 as *const ())),
    )
}
