extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Counter {
    pub value: i32,
}
impl Counter {
    pub unsafe fn bump(&mut self) {
        self.value.prefix_inc();
    }
    pub unsafe fn get(&self) -> i32 {
        return self.value;
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Holder {
    pub c: Counter,
    pub ref_: *mut Counter,
}
impl Holder {
    pub unsafe fn Holder(c: *mut Counter) -> Self {
        let mut this = Self {
            c: <Counter>::default(),
            ref_: c,
        };
        this
    }
}
pub unsafe fn via_ref_0(r: *mut Counter) {
    (unsafe { (*r).bump() });
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut c: Counter = <Counter>::default();
    let mut p: *mut Counter = (&mut c as *mut Counter);
    (unsafe { (*p).bump() });
    (unsafe { (*p).bump() });
    let mut arr: [Counter; 2] = [<Counter>::default(); 2];
    (unsafe { arr[(0) as usize].bump() });
    (unsafe { arr[(1) as usize].bump() });
    let mut h: Holder = Holder::Holder(&mut c as *mut Counter);
    (unsafe { h.c.bump() });
    (unsafe { (*h.ref_).bump() });
    (unsafe {
        let _r: *mut Counter = &mut c as *mut Counter;
        via_ref_0(_r)
    });
    let mut sum: i32 = (((((unsafe { (*p).get() }) + (unsafe { h.c.get() }))
        + (unsafe { (*h.ref_).get() }))
        + (unsafe { arr[(0) as usize].get() }))
        + (unsafe { arr[(1) as usize].get() }));
    printf(b"%d\n\0".as_ptr() as *const i8, sum);
    return 0;
}
