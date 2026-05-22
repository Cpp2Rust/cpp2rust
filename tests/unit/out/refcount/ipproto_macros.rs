extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let tcp: Value<i32> = Rc::new(RefCell::new(libc::IPPROTO_TCP));
    let udp: Value<i32> = Rc::new(RefCell::new(libc::IPPROTO_UDP));
    let ip: Value<i32> = Rc::new(RefCell::new(libc::IPPROTO_IP));
    let ip6: Value<i32> = Rc::new(RefCell::new(libc::IPPROTO_IPV6));
    let mptcp: Value<i32> = Rc::new(RefCell::new(libc::IPPROTO_MPTCP));
    return (((((*tcp.borrow()) + (*udp.borrow())) + (*ip.borrow())) + (*ip6.borrow()))
        + (*mptcp.borrow()));
}
