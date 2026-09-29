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
thread_local!(
    pub static moves_1: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(VaArg, FnPtrArg, Default)]
pub struct Elem {
    pub v: Value<i32>,
}
impl Elem {
    pub fn new(v: i32) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let __this: Value<Elem> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*v.borrow()))),
        }));
        let this: Ptr<Elem> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn copy_from(other: Ptr<Elem>) -> Self {
        let __this: Value<Elem> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new(
                ((*(*other.upgrade().deref()).v.borrow()) + 100),
            )),
        }));
        let this: Ptr<Elem> = __this.as_pointer();
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(other: Ptr<Elem>) -> Self {
        let __this: Value<Elem> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new(
                ((*(*other.upgrade().deref()).v.borrow()) + 1000),
            )),
        }));
        let this: Ptr<Elem> = __this.as_pointer();
        (*(*other.upgrade().deref()).v.borrow_mut()) = -1_i32;
        (*moves_1.with(Value::clone).borrow_mut()).prefix_inc();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for Elem {
    fn clone(&self) -> Self {
        let __src: Value<Elem> = Rc::new(RefCell::new(Elem { v: self.v.clone() }));
        Elem::copy_from(__src.as_pointer())
    }
}
impl ByteRepr for Elem {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(VaArg, FnPtrArg, Default)]
pub struct Two {
    pub a: Value<Elem>,
    pub b: Value<Elem>,
}
impl Clone for Two {
    fn clone(&self) -> Self {
        let __this: Value<Two> = Rc::new(RefCell::new(Self {
            a: Rc::new(RefCell::new(Elem::copy_from({ self.a.as_pointer() }))),
            b: Rc::new(RefCell::new(Elem::copy_from({ self.b.as_pointer() }))),
        }));
        let this: Ptr<Two> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Two {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a.borrow()).to_bytes(&mut buf[0..4]);
        (*self.b.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: Rc::new(RefCell::new(<Elem>::from_bytes(&buf[0..4]))),
            b: Rc::new(RefCell::new(<Elem>::from_bytes(&buf[4..8]))),
        }
    }
}
pub fn make_two_2() -> Two {
    return Two {
        a: Rc::new(RefCell::new(Elem::new({ 1 }))),
        b: Rc::new(RefCell::new(Elem::new({ 2 }))),
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let t: Value<Two> = Rc::new(RefCell::new(Two {
        a: Rc::new(RefCell::new(Elem::new({ 1 }))),
        b: Rc::new(RefCell::new(Elem::new({ 2 }))),
    }));
    let base_copies: Value<i32> = Rc::new(RefCell::new(copies_0.with(|rc| *rc.borrow())));
    let base_moves: Value<i32> = Rc::new(RefCell::new(moves_1.with(|rc| *rc.borrow())));
    let __decomp_3: Value<Two> = Rc::new(RefCell::new((*t.borrow()).clone()));
    assert!((copies_0.with(|rc| *rc.borrow()) == ((*base_copies.borrow()) + 2)));
    assert!((moves_1.with(|rc| *rc.borrow()) == (*base_moves.borrow())));
    assert!(((*(*(*__decomp_3.borrow()).a.borrow()).v.borrow()) == 101));
    assert!(((*(*(*__decomp_3.borrow()).b.borrow()).v.borrow()) == 102));
    let __decomp_4: Ptr<Two> = t.as_pointer();
    assert!((copies_0.with(|rc| *rc.borrow()) == ((*base_copies.borrow()) + 2)));
    assert!(((*(*(*__decomp_4.upgrade().deref()).a.borrow()).v.borrow()) == 1));
    (*(*(*__decomp_4.upgrade().deref()).a.borrow()).v.borrow_mut()) = 50;
    assert!(((*(*(*t.borrow()).a.borrow()).v.borrow()) == 50));
    let __decomp_5: Value<Two> = Rc::new(RefCell::new((*t.borrow()).clone()));
    assert!((moves_1.with(|rc| *rc.borrow()) == ((*base_moves.borrow()) + 2)));
    assert!(((*(*(*__decomp_5.borrow()).a.borrow()).v.borrow()) == 1050));
    assert!(((*(*(*__decomp_5.borrow()).b.borrow()).v.borrow()) == 1002));
    assert!(((*(*(*t.borrow()).a.borrow()).v.borrow()) == -1_i32));
    let before_copies: Value<i32> = Rc::new(RefCell::new(copies_0.with(|rc| *rc.borrow())));
    let before_moves: Value<i32> = Rc::new(RefCell::new(moves_1.with(|rc| *rc.borrow())));
    let __decomp_6: Value<Two> = Rc::new(RefCell::new(({ make_two_2() })));
    assert!((copies_0.with(|rc| *rc.borrow()) == (*before_copies.borrow())));
    assert!((moves_1.with(|rc| *rc.borrow()) == (*before_moves.borrow())));
    assert!(((*(*(*__decomp_6.borrow()).a.borrow()).v.borrow()) == 1));
    let p: Value<(Value<Elem>, Value<Elem>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(
            Elem::new({ 7 }).try_into().expect("failed conversion"),
        )),
        Rc::new(RefCell::new(
            Elem::new({ 8 }).try_into().expect("failed conversion"),
        )),
    )));
    (*before_copies.borrow_mut()) = copies_0.with(|rc| *rc.borrow());
    let __decomp_7: Value<(Value<Elem>, Value<Elem>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new((*p.borrow()).0.borrow().clone())),
        Rc::new(RefCell::new((*p.borrow()).1.borrow().clone())),
    )));
    assert!((copies_0.with(|rc| *rc.borrow()) == ((*before_copies.borrow()) + 2)));
    assert!(
        ((*(*x.upgrade().deref()).v.borrow()) == ((*(*(*p.borrow()).0.borrow()).v.borrow()) + 100))
    );
    assert!(
        ((*(*y.upgrade().deref()).v.borrow()) == ((*(*(*p.borrow()).1.borrow()).v.borrow()) + 100))
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = copies_0.with(|_| ());
    let _ = moves_1.with(|_| ());
}
