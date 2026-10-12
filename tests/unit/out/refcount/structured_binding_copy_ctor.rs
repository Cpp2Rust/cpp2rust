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
    let mut __decomp_2: Counted = Counted::copy_from({ s.as_pointer() });
    assert!((copies_0.with(|rc| *rc.borrow()) == 1));
    assert!((__decomp_2.x == 2));
    assert!((__decomp_2.y == 4));
    let __decomp_3: Ptr<Counted> = s.as_pointer();
    assert!((copies_0.with(|rc| *rc.borrow()) == 1));
    assert!((__decomp_3.with(|__s| __s.x) == 1));
    assert!((__decomp_3.with(|__s| __s.y) == 2));
    let __decomp_4: Ptr<Counted> = s.as_pointer();
    assert!((copies_0.with(|rc| *rc.borrow()) == 1));
    assert!((__decomp_4.with(|__s| __s.x) == 1));
    let mut __decomp_5: Counted = ({ make_counted_1() });
    assert!((copies_0.with(|rc| *rc.borrow()) == 1));
    assert!((__decomp_5.x == 5));
    assert!((__decomp_5.y == 6));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = copies_0.with(|_| ());
}
