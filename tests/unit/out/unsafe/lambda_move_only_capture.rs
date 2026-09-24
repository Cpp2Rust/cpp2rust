extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Default)]
pub struct Owner {
    pub p: *mut i32,
}
impl Owner {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self {
            p: (Box::leak(Box::new(v)) as *mut i32),
        };
        this
    }
    pub unsafe fn move_from(o: *mut Owner) -> Self {
        let mut this = Self { p: (*o).p };
        (*o).p = std::ptr::null_mut();
        this
    }
    pub unsafe fn destructor(&mut self) {
        ::std::mem::drop(Box::from_raw(self.p));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut o: Owner = Owner::new({ 5 });
    let _dtor_o = ScopedDestructorUnsafe::new(&raw mut o, Owner::destructor);
    assert!((o.p).is_null());
    assert!(
        ((unsafe {
            (|| {
                return (*h.p);
            })()
        }) == (5))
    );
    let mut g: _ = (|| {
        return (*h.p);
    });
    assert!(((unsafe { g() }) == (5)));
    let mut total: i32 = 0;
    (unsafe {
        (|| {
            total += (*h.p);
            (*h.p) = 0;
        })()
    });
    (unsafe {
        (|| {
            total += (*h.p);
            (*h.p) = 0;
        })()
    });
    assert!(((total) == (7)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
