unsafe fn f1(a0: *mut *mut libc::ifaddrs) -> i32 {
    libc::getifaddrs(a0)
}

unsafe fn f2(a0: *mut libc::ifaddrs) {
    libc::freeifaddrs(a0)
}
