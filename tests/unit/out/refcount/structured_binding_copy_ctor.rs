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
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Counted {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
impl Counted {
    pub fn new(x: i32, y: i32) -> Self {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let y: Value<i32> = Rc::new(RefCell::new(y));
        let __this: Value<Counted> = Rc::new(RefCell::new(Self {
            x: (*x.borrow()),
            y: (*y.borrow()),
        }));
        let this: Ptr<Counted> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn copy_from(other: Ptr<Counted>) -> Self {
        let __this: Value<Counted> = Rc::new(RefCell::new(Self {
            x: (other.with(|__s| __s.x) * 2),
            y: (other.with(|__s| __s.y) * 2),
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
pub fn __cpp2rust_init_globals() {
    let _ = copies_0.with(|_| ());
}
