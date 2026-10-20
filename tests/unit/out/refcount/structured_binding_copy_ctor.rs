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
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Default)]
#[byte_size(8)]
pub struct Counted {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
impl Counted {
    pub fn new(mut x: i32, mut y: i32) -> Self {
        Self { x: x, y: y }
    }
    pub fn copy_from(other: Ptr<Counted>) -> Self {
        let __this: Counted = Self {
            x: (other.with(|__s| __s.x) * 2),
            y: (other.with(|__s| __s.y) * 2),
        };
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
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
    assert!(({ (*__decomp_2.borrow()).x } == 2));
    assert!(({ (*__decomp_2.borrow()).y } == 4));
    let __decomp_3: Ptr<Counted> = s.as_pointer();
    assert!((copies_0.with(|rc| *rc.borrow()) == 1));
    assert!((__decomp_3.with(|__s| __s.x) == 1));
    assert!((__decomp_3.with(|__s| __s.y) == 2));
    let __decomp_4: Ptr<Counted> = s.as_pointer();
    assert!((copies_0.with(|rc| *rc.borrow()) == 1));
    assert!((__decomp_4.with(|__s| __s.x) == 1));
    let __decomp_5: Value<Counted> = Rc::new(RefCell::new(({ make_counted_1() })));
    assert!((copies_0.with(|rc| *rc.borrow()) == 1));
    assert!(({ (*__decomp_5.borrow()).x } == 5));
    assert!(({ (*__decomp_5.borrow()).y } == 6));
    return 0;
}
pub trait CountedImpl {
    fn copy_assign(&self, _a0: Ptr<Counted>) -> Ptr<Counted>;
}
impl CountedImpl for Ptr<Counted> {
    fn copy_assign(&self, _a0: Ptr<Counted>) -> Ptr<Counted> {
        field!((*self), x).write({ _a0.with(|__s| __s.x) });
        field!((*self), y).write({ _a0.with(|__s| __s.y) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = copies_0.with(|_| ());
}
