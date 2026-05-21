unsafe fn f1() -> i32 {
    libc::MSG_NOSIGNAL
}

unsafe fn f2() -> i32 {
    libc::SOCK_STREAM
}

unsafe fn f3() -> i32 {
    libc::SOCK_DGRAM
}

#[cfg(target_os = "linux")]
unsafe fn f4() -> i32 {
    libc::SOCK_CLOEXEC
}

#[cfg(target_os = "linux")]
unsafe fn f5() -> i32 {
    libc::SOCK_NONBLOCK
}

unsafe fn f6(a0: i32, a1: i32, a2: i32) -> i32 {
    libc::socket(a0, a1, a2)
}

unsafe fn f7(a0: i32, a1: i32, a2: i32, a3: *const ::libc::c_void, a4: u32) -> i32 {
    libc::setsockopt(a0, a1, a2, a3, a4)
}

unsafe fn f8(a0: i32, a1: i32, a2: i32, a3: *mut ::libc::c_void, a4: *mut u32) -> i32 {
    libc::getsockopt(a0, a1, a2, a3, a4)
}

unsafe fn f9(a0: i32, a1: *mut ::libc::c_void, a2: u64, a3: i32) -> i64 {
    libc::recv(a0, a1, a2 as usize, a3) as i64
}

unsafe fn f10(a0: i32, a1: *const ::libc::c_void, a2: u64, a3: i32) -> i64 {
    libc::send(a0, a1, a2 as usize, a3) as i64
}

unsafe fn f11(a0: i32, a1: i32, a2: i32, a3: *mut i32) -> i32 {
    libc::socketpair(a0, a1, a2, a3)
}
