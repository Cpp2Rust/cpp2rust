extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(a: i32, b: Option<i32>) -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(a));
    let b: Value<i32> = Rc::new(RefCell::new(b.unwrap_or(10)));
    return ((*a.borrow()) + (*b.borrow()));
}
pub fn baz_1(a: Ptr<i32>, b: Option<Ptr<i32>>) -> bool {
    let a: Value<Ptr<i32>> = Rc::new(RefCell::new(a));
    let b: Value<Ptr<i32>> = Rc::new(RefCell::new(b.unwrap_or(Ptr::<i32>::null())));
    return ({ (*a.borrow()).clone() } == { (*b.borrow()).clone() });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct Bar {
    #[offset(0)]
    pub v: i32,
}
impl Bar {
    pub fn new(v: Option<i32>) -> Self {
        let v: Value<i32> = Rc::new(RefCell::new(v.unwrap_or(1)));
        let __this: Value<Bar> = Rc::new(RefCell::new(Self { v: (*v.borrow()) }));
        let this: Ptr<Bar> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Bar {
    fn default() -> Self {
        { Bar::new(None) }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ foo_0(1, None,) }) == 11));
    assert!((({ foo_0(1, Some(2),) }) == 3));
    let a: Value<i32> = Rc::new(RefCell::new(0));
    assert!(((({ baz_1((a.as_pointer()), None,) }) as i32) == (false as i32)));
    assert!(
        ((({
            let _a: Ptr<i32> = (a.as_pointer());
            let _b: Ptr<i32> = (a.as_pointer());
            baz_1(_a, Some(_b))
        }) as i32)
            == (true as i32))
    );
    let b: Value<Bar> = Rc::new(RefCell::new(Bar::new(None)));
    assert!(({ (*b.borrow()).v } == 1));
    assert!(({ Bar::new({ Some(2) },).v } == 2));
    let arr: Value<Box<[Bar]>> = Rc::new(RefCell::new(Box::new([
        Bar::new(None),
        Bar::new(None),
        Bar::new(None),
    ])));
    assert!(({ (*arr.borrow())[(0) as usize].v } == 1));
    assert!(({ (*arr.borrow())[(2) as usize].v } == 1));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
