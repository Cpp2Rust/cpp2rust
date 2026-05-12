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
    let mut p: pollfd = std::mem::zeroed::<pollfd>();
    p.fd = -1_i32;
    p.events = 0_i16;
    p.revents = 0_i16;
    let mut ia: in_addr = std::mem::zeroed::<in_addr>();
    ia.s_addr = 0_u32;
    assert!(((((ia.s_addr) == (0_u32)) as i32) != 0));
    let mut t: tm = std::mem::zeroed::<tm>();
    t.tm_year = 124;
    t.tm_mon = 5;
    t.tm_mday = 15;
    assert!(((((t.tm_year) == (124)) as i32) != 0));
    assert!(((((t.tm_mon) == (5)) as i32) != 0));
    assert!(((((t.tm_mday) == (15)) as i32) != 0));
    let mut sa: sockaddr_in = std::mem::zeroed::<sockaddr_in>();
    sa.sin_family = 2_u16;
    sa.sin_port = 8080_u16;
    assert!(((((sa.sin_family as i32) == (2)) as i32) != 0));
    assert!(((((sa.sin_port as i32) == (8080)) as i32) != 0));
    let mut st: stat = std::mem::zeroed::<stat>();
    st.st_size = 1024_i64;
    assert!(((((st.st_size) == (1024_i64)) as i32) != 0));
    assert!(((((p.fd) == (-1_i32)) as i32) != 0));
    assert!(((((p.events as i32) == (0)) as i32) != 0));
    return 0;
}
