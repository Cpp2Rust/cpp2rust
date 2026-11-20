unsafe fn t1() -> ::libc::sockaddr_in {
    std::mem::zeroed()
}

unsafe fn t2() -> ::libc::in_addr {
    std::mem::zeroed()
}

unsafe fn t3() -> ::libc::sockaddr_in6 {
    std::mem::zeroed()
}

unsafe fn t4() -> ::libc::in6_addr {
    std::mem::zeroed()
}

fn t5() -> ::libc::in_addr_t {
    0 as ::libc::in_addr_t
}

fn t6() -> ::libc::in_port_t {
    0 as ::libc::in_port_t
}

fn f1() -> i32 {
    libc::IPPROTO_TCP
}

fn f2() -> i32 {
    libc::IPPROTO_UDP
}

fn f3() -> i32 {
    libc::IPPROTO_IP
}

fn f4() -> i32 {
    libc::IPPROTO_IPV6
}

#[cfg(target_os = "linux")]
fn f5() -> i32 {
    libc::IPPROTO_MPTCP
}

fn f6() -> i32 {
    libc::TCP_NODELAY
}

unsafe fn f7(a0: *mut ::libc::in6_addr) -> *mut [u8; 16] {
    &raw mut (*a0).s6_addr
}
