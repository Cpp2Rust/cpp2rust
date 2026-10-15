extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static copies_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(VaArg, FnPtrArg, Default)]
pub struct Counted {
    pub x: Value<i32>,
    pub y: Value<i32>,
}
impl Counted {
    pub fn new(x: i32, y: i32) -> Self {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let y: Value<i32> = Rc::new(RefCell::new(y));
        let __this: Value<Counted> = Rc::new(RefCell::new(Self {
            x: Rc::new(RefCell::new((*x.borrow()))),
            y: Rc::new(RefCell::new((*y.borrow()))),
        }));
        let this: Ptr<Counted> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn copy_from(other: Ptr<Counted>) -> Self {
        let __this: Value<Counted> = Rc::new(RefCell::new(Self {
            x: Rc::new(RefCell::new(((*(*other.upgrade().deref()).x.borrow()) * 2))),
            y: Rc::new(RefCell::new(((*(*other.upgrade().deref()).y.borrow()) * 2))),
        }));
        let this: Ptr<Counted> = __this.as_pointer();
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for Counted {
    fn clone(&self) -> Self {
        let __src: Value<Counted> = Rc::new(RefCell::new(Counted {
            x: self.x.clone(),
            y: self.y.clone(),
        }));
        Counted::copy_from(__src.as_pointer())
    }
}
impl ByteRepr for Counted {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x.borrow()).to_bytes(&mut buf[0..4]);
        (*self.y.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            y: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
pub fn make_counted_1() -> Counted {
    return Counted::new({ 5 }, { 6 });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<Counted> = Rc::new(RefCell::new(Counted::new({ 1 }, { 2 })));
    let __decomp_2: Value<Counted> = Rc::new(RefCell::new(Counted::copy_from({ s.as_pointer() })));
    assert!((copies_0.with(|rc| *rc.borrow()) == 1));
    assert!(((*(*__decomp_2.borrow()).x.borrow()) == 2));
    assert!(((*(*__decomp_2.borrow()).y.borrow()) == 4));
    let __decomp_3: Ptr<Counted> = s.as_pointer();
    assert!((copies_0.with(|rc| *rc.borrow()) == 1));
    assert!(((*(*__decomp_3.upgrade().deref()).x.borrow()) == 1));
    assert!(((*(*__decomp_3.upgrade().deref()).y.borrow()) == 2));
    let __decomp_4: Ptr<Counted> = s.as_pointer();
    assert!((copies_0.with(|rc| *rc.borrow()) == 1));
    assert!(((*(*__decomp_4.upgrade().deref()).x.borrow()) == 1));
    let __decomp_5: Value<Counted> = Rc::new(RefCell::new(({ make_counted_1() })));
    assert!((copies_0.with(|rc| *rc.borrow()) == 1));
    assert!(((*(*__decomp_5.borrow()).x.borrow()) == 5));
    assert!(((*(*__decomp_5.borrow()).y.borrow()) == 6));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = copies_0.with(|_| ());
}
