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
