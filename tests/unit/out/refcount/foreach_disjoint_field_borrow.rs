extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(32)]
pub struct S {
    #[offset(0)]
    #[byte_size(24)]
    pub v: Value<Vec<i32>>,
    #[offset(24)]
    pub a: i32,
}
impl Clone for S {
    fn clone(&self) -> Self {
        Self {
            v: Rc::new(RefCell::new((*self.v.borrow()).clone())),
            a: self.a.clone(),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(<S>::default()));
    {
        let __a1 = 1;
        (*{ (*s.borrow()).v.clone() }.borrow_mut()).push(__a1)
    };
    'loop_: for mut e in { (*s.borrow()).v.clone() }.as_pointer() as Ptr<i32> {
        let e: Value<i32> = Rc::new(RefCell::new(e.read()));
        (*s.borrow_mut()).a.postfix_inc();
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
