extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut tcp: i32 = libc::IPPROTO_TCP;
    let mut udp: i32 = libc::IPPROTO_UDP;
    let mut ip: i32 = libc::IPPROTO_IP;
    let mut ip6: i32 = libc::IPPROTO_IPV6;
    let mut mptcp: i32 = libc::IPPROTO_MPTCP;
    return (((((tcp) + (udp)) + (ip)) + (ip6)) + (mptcp));
}
