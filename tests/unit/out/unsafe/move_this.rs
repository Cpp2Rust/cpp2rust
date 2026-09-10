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
pub struct Chain {
    pub v: i32,
}
impl Chain {
    pub unsafe fn Chain(mut v: i32) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn Chain_pconstChain(o: *const Chain) -> Self {
        let mut this = Self {
            v: (((*o).v) + (100)),
        };
        this
    }
    pub unsafe fn Chain_pmutChain(o: *mut Chain) -> Self {
        let mut this = Self {
            v: (((*o).v) + (1)),
        };
        (*o).v = 0;
        this
    }
    pub unsafe fn add_i32_lref(&mut self, mut n: i32) -> *mut Chain {
        self.v += n;
        return &mut (*(self as *mut Chain)) as *mut Chain;
    }
    pub unsafe fn add_i32_rref(&mut self, mut n: i32) -> *mut Chain {
        self.v += n;
        return (self as *mut Chain);
    }
    pub unsafe fn take(&mut self) -> Chain {
        return Chain::Chain_pmutChain({ (self as *mut Chain) });
    }
    pub unsafe fn copy(&self) -> Chain {
        return Chain::Chain_pconstChain({ &(*(self as *const Chain)) as *const Chain });
    }
    pub unsafe fn self_(&mut self) -> *mut Chain {
        return (self as *mut Chain);
    }
}
impl Clone for Chain {
    fn clone(&self) -> Self {
        unsafe { Chain::Chain_pconstChain(self as *const Chain) }
    }
}
pub unsafe fn consume_0(mut c: Chain) -> i32 {
    return c.v;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: Chain = Chain::Chain({ 1 });
    (unsafe { Chain::add_i32_lref(&mut (*(unsafe { Chain::add_i32_lref(&mut a, 1) })), 1) });
    assert!(((a.v) == (3)));
    let mut b0: Chain = Chain::Chain({ 5 });
    let mut b: Chain = Chain::Chain_pmutChain({
        (unsafe { Chain::add_i32_rref(&mut (*(unsafe { Chain::add_i32_rref(&mut b0, 1) })), 1) })
    });
    assert!(((b.v) == (8)) && ((b0.v) == (0)));
    let mut c: Chain = (unsafe { Chain::take(&mut Chain::Chain({ 10 })) });
    assert!(((c.v) == (11)));
    let mut d: Chain = (unsafe { Chain::copy(&c) });
    assert!(((d.v) == (111)) && ((c.v) == (11)));
    let mut g: Chain = Chain::Chain({ 20 });
    assert!(
        ((unsafe {
            consume_0(Chain::Chain_pmutChain({
                (unsafe { Chain::self_(&mut g) })
            }))
        }) == (21))
    );
    let mut e: Chain = Chain::Chain({ 30 });
    let mut f: Chain = (unsafe { Chain::take(&mut e) });
    assert!(((f.v) == (31)) && ((e.v) == (0)));
    return 0;
}
