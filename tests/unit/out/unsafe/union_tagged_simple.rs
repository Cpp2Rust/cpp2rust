extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum Kind {
    #[default]
    KIND_NONE = 0,
    KIND_DONE = 1,
}
impl From<i32> for Kind {
    fn from(n: i32) -> Kind {
        match n {
            0 => Kind::KIND_NONE,
            1 => Kind::KIND_DONE,
            _ => panic!("invalid Kind value: {}", n),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Event_anon_0 {
    pub obj: *mut ::libc::c_void,
    pub code: i32,
}
impl Default for Event_anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
// SAFETY: preserves unsafe C semantics; thread-safety is not enforced
unsafe impl Sync for Event_anon_0 {}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Event {
    pub kind: Kind,
    pub handle: *mut ::libc::c_void,
    pub payload: Event_anon_0,
}
// SAFETY: preserves unsafe C semantics; thread-safety is not enforced
unsafe impl Sync for Event {}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut dummy: i32 = 0;
    let mut m1: Event = <Event>::default();
    m1.kind = Kind::from((Kind::KIND_DONE as i32));
    m1.handle = ((&mut dummy as *mut i32) as *mut i32 as *mut ::libc::c_void);
    m1.payload.code = 42;
    assert!(((((m1.kind as u32) == ((Kind::KIND_DONE as i32) as u32)) as i32) != 0));
    assert!(((((m1.payload.code) == (42)) as i32) != 0));
    let mut m2: Event = <Event>::default();
    m2.kind = Kind::from((Kind::KIND_NONE as i32));
    m2.handle = ((&mut dummy as *mut i32) as *mut i32 as *mut ::libc::c_void);
    m2.payload.obj = ((&mut dummy as *mut i32) as *mut i32 as *mut ::libc::c_void);
    assert!(
        ((((m2.payload.obj) == ((&mut dummy as *mut i32) as *mut i32 as *mut ::libc::c_void))
            as i32)
            != 0)
    );
    return 0;
}
