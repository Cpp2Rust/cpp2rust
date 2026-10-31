extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct UserDefined {
    pub a: Vec<i32>,
    pub v: Vec<i32>,
}
impl UserDefined {
    pub unsafe fn move_from(_a0: *mut UserDefined) -> Self {
        let mut this = Self {
            a: std::mem::take(&mut (*_a0).a),
            v: std::mem::take(&mut (*_a0).v),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut UserDefined) -> *mut UserDefined {
        self.a = std::mem::take(&mut (*_a0).a);
        self.v = std::mem::take(&mut (*_a0).v);
        return &mut (*(self as *mut UserDefined));
    }
}
impl Default for UserDefined {
    fn default() -> Self {
        unsafe {
            UserDefined {
                a: std::array::from_fn::<_, 1, _>(|_| 0_i32).to_vec(),
                v: Default::default(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct FieldIsLibcType {
    pub addr: libc::sockaddr,
}
impl Default for FieldIsLibcType {
    fn default() -> Self {
        unsafe {
            FieldIsLibcType {
                addr: unsafe { std::mem::zeroed() },
            }
        }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p: ::libc::pollfd = unsafe { std::mem::zeroed() };
    p.fd = -1_i32;
    p.events = 0_i16;
    p.revents = 2_i16;
    assert!(((p.fd) == (-1_i32)));
    assert!(((p.events as i32) == (0)));
    assert!(((p.revents as i32) == (2)));
    let mut ia: ::libc::in_addr = unsafe { std::mem::zeroed() };
    ia.s_addr = (1 as ::libc::in_addr_t);
    assert!(((ia.s_addr) == (1 as ::libc::in_addr_t)));
    let mut t: ::libc::tm = unsafe { std::mem::zeroed() };
    t.tm_year = 124;
    t.tm_mon = 5;
    t.tm_mday = 15;
    assert!(((t.tm_year) == (124)));
    assert!(((t.tm_mon) == (5)));
    assert!(((t.tm_mday) == (15)));
    let mut st: ::libc::stat = unsafe { std::mem::zeroed() };
    st.st_size = (1024 as ::libc::off_t);
    assert!(((st.st_size) == (1024 as ::libc::off_t)));
    let mut ud: UserDefined = <UserDefined>::default();
    assert!(((ud.a[(0_usize)]) == (0)));
    assert!(((ud.v.len()) == (0_usize)));
    let mut filt: FieldIsLibcType = <FieldIsLibcType>::default();
    assert!(((filt.addr.sa_family as i32) == (0)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
