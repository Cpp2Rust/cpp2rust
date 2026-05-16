extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone)]
pub struct UserDefined {
    pub a: Vec<i32>,
    pub v: Vec<i32>,
}
impl Default for UserDefined {
    fn default() -> Self {
        UserDefined {
            a: std::array::from_fn::<_, 1, _>(|_| Default::default()).to_vec(),
            v: <Vec<i32>>::default(),
        }
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p: pollfd = std::mem::zeroed::<pollfd>();
    p.fd = -1_i32;
    p.events = 0_i16;
    p.revents = 2_i16;
    assert!(((p.fd) == (-1_i32)));
    assert!(((p.events as i32) == (0)));
    assert!(((p.revents as i32) == (2)));
    let mut ia: in_addr = std::mem::zeroed::<in_addr>();
    ia.s_addr = 1_u32;
    assert!(((ia.s_addr) == (1_u32)));
    let mut t: tm = std::mem::zeroed::<tm>();
    t.tm_year = 124;
    t.tm_mon = 5;
    t.tm_mday = 15;
    assert!(((t.tm_year) == (124)));
    assert!(((t.tm_mon) == (5)));
    assert!(((t.tm_mday) == (15)));
    let mut st: stat = std::mem::zeroed::<stat>();
    st.st_size = 1024_i64;
    assert!(((st.st_size) == (1024_i64)));
    let mut ud: UserDefined = <UserDefined>::default();
    assert!(((ud.a[(0_u64) as usize]) == (0)));
    assert!(((ud.v.len() as u64) == (0_u64)));
    return 0;
}
