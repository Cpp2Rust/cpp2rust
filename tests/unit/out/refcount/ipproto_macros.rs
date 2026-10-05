extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut tcp: i32 = libc::IPPROTO_TCP;
    let mut udp: i32 = libc::IPPROTO_UDP;
    let mut ip: i32 = libc::IPPROTO_IP;
    let mut ip6: i32 = libc::IPPROTO_IPV6;
    assert!(((((tcp + udp) + ip) + ip6) == 64));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
