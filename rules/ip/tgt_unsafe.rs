fn t1() -> ::libc::sockaddr_in {
    unsafe { std::mem::zeroed() }
}

fn t2() -> ::libc::in_addr {
    unsafe { std::mem::zeroed() }
}

fn t3() -> ::libc::sockaddr_in6 {
    unsafe { std::mem::zeroed() }
}

fn t4() -> ::libc::in6_addr {
    unsafe { std::mem::zeroed() }
}

unsafe fn f1() -> i32 {
    libc::IPPROTO_TCP
}

unsafe fn f2() -> i32 {
    libc::IPPROTO_UDP
}

unsafe fn f3() -> i32 {
    libc::IPPROTO_IP
}

unsafe fn f4() -> i32 {
    libc::IPPROTO_IPV6
}

#[cfg(target_os = "linux")]
unsafe fn f5() -> i32 {
    libc::IPPROTO_MPTCP
}

unsafe fn f6() -> i32 {
    libc::TCP_NODELAY
}
