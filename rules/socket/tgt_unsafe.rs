fn t1() -> libc::sockaddr {
    unsafe { std::mem::zeroed() }
}

fn t2() -> libc::sockaddr_storage {
    unsafe { std::mem::zeroed() }
}

fn t3() -> libc::sockaddr_un {
    unsafe { std::mem::zeroed() }
}

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

unsafe fn f9(a0: i32, a1: *mut ::libc::c_void, a2: usize, a3: i32) -> isize {
    libc::recv(a0, a1, a2, a3)
}

unsafe fn f10(a0: i32, a1: *const ::libc::c_void, a2: usize, a3: i32) -> isize {
    libc::send(a0, a1, a2, a3)
}

unsafe fn f11(a0: i32, a1: i32, a2: i32, a3: *mut i32) -> i32 {
    libc::socketpair(a0, a1, a2, a3)
}

unsafe fn f12(a0: i32, a1: *mut ::libc::sockaddr, a2: *mut u32) -> i32 {
    libc::getsockname(a0, a1, a2)
}

unsafe fn f13(a0: i32, a1: *const ::libc::sockaddr, a2: u32) -> i32 {
    libc::connect(a0, a1, a2)
}

unsafe fn f14(a0: i32, a1: *mut ::libc::sockaddr, a2: *mut u32) -> i32 {
    libc::getpeername(a0, a1, a2)
}

#[cfg(target_os = "linux")]
unsafe fn f15(a0: i32, a1: *mut ::libc::sockaddr, a2: *mut u32, a3: i32) -> i32 {
    libc::accept4(a0, a1, a2, a3)
}

unsafe fn f16(a0: i32, a1: *const ::libc::sockaddr, a2: u32) -> i32 {
    libc::bind(a0, a1, a2)
}

unsafe fn f17(a0: i32, a1: i32) -> i32 {
    libc::listen(a0, a1)
}

unsafe fn f18(
    a0: i32,
    a1: *mut ::libc::c_void,
    a2: usize,
    a3: i32,
    a4: *mut ::libc::sockaddr,
    a5: *mut u32,
) -> isize {
    libc::recvfrom(a0, a1, a2, a3, a4, a5)
}

unsafe fn f19(
    a0: i32,
    a1: *const ::libc::c_void,
    a2: usize,
    a3: i32,
    a4: *const ::libc::sockaddr,
    a5: u32,
) -> isize {
    libc::sendto(a0, a1, a2, a3, a4, a5)
}

unsafe fn f20() -> i32 {
    libc::AF_INET
}

unsafe fn f21() -> i32 {
    libc::AF_INET6
}
