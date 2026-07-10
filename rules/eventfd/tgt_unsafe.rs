#[cfg(target_os = "linux")]
unsafe fn f1() -> i32 {
    libc::EFD_CLOEXEC
}

#[cfg(target_os = "linux")]
unsafe fn f2() -> i32 {
    libc::EFD_NONBLOCK
}

#[cfg(target_os = "linux")]
unsafe fn f3() -> i32 {
    libc::EFD_SEMAPHORE
}

#[cfg(target_os = "linux")]
unsafe fn f4(a0: u32, a1: i32) -> i32 {
    libc::eventfd(a0, a1)
}
