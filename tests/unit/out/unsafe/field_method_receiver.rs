extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, Default)]
pub struct Counter {
    pub n: i32,
}
impl Counter {
    pub unsafe fn get(&self) -> i32 {
        return self.n;
    }
    pub unsafe fn add(&mut self, mut k: i32) {
        self.n += k;
    }
    pub unsafe fn self_(&mut self) -> *mut Counter {
        return (self as *mut Counter);
    }
    pub unsafe fn take(&mut self, mut other: *mut Counter) {
        self.n += (*other).n;
        (*other).n = 0;
    }
}
#[repr(C)]
#[derive(Clone, VaArg, Default)]
pub struct S {
    pub tag: i32,
    pub c: Counter,
    pub arr: [Counter; 2],
    pub v: Vec<i32>,
}
impl S {
    pub unsafe fn bump(&mut self) {
        (unsafe {
            let _k: i32 = self.tag;
            Counter::add(&mut self.c, _k)
        });
    }
}
pub unsafe fn run_0(mut o: *mut S) {
    (unsafe { Counter::add(&mut (*o).c, 2) });
    assert!(((unsafe { Counter::get(&(*o).c,) }) == (2)));
    (unsafe { Counter::add(&mut (*o).arr[(1) as usize], 5) });
    assert!(((unsafe { Counter::get(&(*o).arr[(1) as usize],) }) == (5)));
    (unsafe { Counter::add(&mut (*(unsafe { Counter::self_(&mut (*o).c) })), 1) });
    assert!(((unsafe { Counter::get(&(*o).c,) }) == (3)));
    assert!(((unsafe { Counter::self_(&mut (*o).c,) }) == (&mut (*o).c as *mut Counter)));
    (unsafe {
        let _other: *mut Counter = (&mut (*o).arr[(1) as usize] as *mut Counter);
        Counter::take(&mut (*o).arr[(0) as usize], _other)
    });
    assert!(
        ((unsafe { Counter::get(&(*o).arr[(0) as usize],) }) == (5))
            && ((unsafe { Counter::get(&(*o).arr[(1) as usize],) }) == (0))
    );
    (unsafe {
        let _other: *mut Counter = (&mut (*o).c as *mut Counter);
        Counter::take(&mut (*o).c, _other)
    });
    assert!(((unsafe { Counter::get(&(*o).c,) }) == (0)));
    (unsafe { S::bump(&mut (*o)) });
    assert!(((unsafe { Counter::get(&(*o).c,) }) == (1)));
    {
        let __a1 = (unsafe { Counter::get(&(*o).c) });
        (*o).v.push(__a1)
    };
    assert!((((*o).v.len()) == (1_usize)) && (((&mut (*o)).v[(0_usize)]) == (1)));
    assert!((((*o).tag) == (1)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut local: S = <S>::default();
    local.tag = 1;
    (unsafe { run_0((&mut local as *mut S)) });
    let mut heap: *mut S = (Box::leak(Box::new(<S>::default())) as *mut S);
    (*heap).tag = 1;
    (unsafe { run_0(heap) });
    ::std::mem::drop(Box::from_raw(heap));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
