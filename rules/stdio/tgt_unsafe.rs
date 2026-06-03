// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn t1() -> *mut ::libc::FILE {
    std::ptr::null_mut()
}

unsafe fn f1(a0: *const u8, a1: *const u8) -> *mut ::libc::FILE {
    libc::fopen(a0 as *const i8, a1 as *const i8)
}

unsafe fn f2(a0: *mut ::libc::FILE) -> i32 {
    libc::fclose(a0)
}

unsafe fn f3(a0: *mut ::libc::FILE) -> i64 {
    libc::ftell(a0) as i64
}

unsafe fn f4(a0: *mut ::libc::FILE, a1: i64, a2: i32) -> i32 {
    libc::fseek(a0, a1 as ::libc::c_long, a2)
}

unsafe fn f5(a0: *mut ::libc::c_void, a1: u64, a2: u64, a3: *mut ::libc::FILE) -> u64 {
    libcc2rs::fread_unsafe(a0, a1, a2, a3)
}

unsafe fn f6(a0: *const ::libc::c_void, a1: u64, a2: u64, a3: *mut ::libc::FILE) -> u64 {
    libcc2rs::fwrite_unsafe(a0, a1, a2, a3)
}

unsafe fn f7(a0: *mut ::libc::FILE) -> i32 {
    libc::fflush(a0)
}

unsafe fn f8() -> *mut ::libc::FILE {
    libcc2rs::stdout_unsafe()
}

unsafe fn f9() -> *mut ::libc::FILE {
    libcc2rs::stderr_unsafe()
}

unsafe fn f10() -> *mut ::libc::FILE {
    libcc2rs::stdin_unsafe()
}

unsafe fn f11(a0: i32, a1: *mut ::libc::FILE) -> i32 {
    libc::fputc(a0, a1)
}

unsafe fn f12(a0: *const u8, a1: *mut ::libc::FILE) -> i32 {
    libc::fputs(a0 as *const i8, a1)
}

unsafe fn f13(a0: *const u8) -> i32 {
    libc::puts(a0 as *const i8)
}

unsafe fn f14(a0: *mut ::libc::FILE) -> i32 {
    libc::fileno(a0)
}

unsafe fn f15(a0: *mut ::libc::FILE) -> i32 {
    libc::ferror(a0)
}

unsafe fn f16(a0: *mut ::libc::FILE) -> i32 {
    libc::feof(a0)
}

unsafe fn f17(a0: *mut u8, a1: i32, a2: *mut ::libc::FILE) -> *mut u8 {
    libc::fgets(a0 as *mut i8, a1, a2) as *mut u8
}

unsafe fn f18(a0: *const u8, a1: *const u8, a2: *mut ::libc::FILE) -> *mut ::libc::FILE {
    libc::freopen(a0 as *const i8, a1 as *const i8, a2)
}

unsafe fn f19(a0: *mut ::libc::FILE, a1: i64, a2: i32) -> i32 {
    libc::fseeko(a0, a1 as ::libc::off_t, a2)
}

unsafe fn f20(a0: i32, a1: *const u8) -> *mut ::libc::FILE {
    libc::fdopen(a0, a1 as *const i8)
}

unsafe fn f22(a0: *const i8, a1: *const i8) -> i32 {
    libc::rename(a0 as *const i8, a1 as *const i8)
}

unsafe fn f23(a0: *mut ::libc::FILE) -> i32 {
    libc::fgetc(a0)
}
