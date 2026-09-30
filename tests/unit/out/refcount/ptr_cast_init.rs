extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct header {
    #[offset(0)]
    pub tag: i32,
    #[offset(4)]
    pub size: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct view {
    #[offset(0)]
    pub tag: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let text: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::from(*b"hi\0")));
    let cp: Value<Ptr<u8>> = Rc::new(RefCell::new((text.as_pointer() as Ptr<u8>)));
    let u: Value<Ptr<u8>> = Rc::new(RefCell::new((*cp.borrow()).reinterpret_cast::<u8>()));
    assert!(((((((*u.borrow()).offset((0) as isize).read()) as i32) == ('h' as i32)) as i32) != 0));
    assert!(((((((*u.borrow()).offset((1) as isize).read()) as i32) == ('i' as i32)) as i32) != 0));
    let h: Value<header> = Rc::new(RefCell::new(header { tag: 7, size: 32 }));
    let hp: Value<Ptr<header>> = Rc::new(RefCell::new((h.as_pointer())));
    let v: Value<Ptr<view>> = Rc::new(RefCell::new((*hp.borrow()).reinterpret_cast::<view>()));
    assert!(((((*v.borrow()).with(|__s| __s.tag) == 7) as i32) != 0));
    let data: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::from(*b"hi\0")));
    let vp: Value<AnyPtr> = Rc::new(RefCell::new(
        ((data.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any(),
    ));
    let n: Value<i32> = Rc::new(RefCell::new(2));
    let sel: Value<Ptr<u8>> = Rc::new(RefCell::new(
        if ((((*n.borrow()) < 100) as i32) != 0) {
            (*vp.borrow()).clone()
        } else {
            (AnyPtr::default())
        }
        .reinterpret_cast::<u8>(),
    ));
    assert!((((!((*sel.borrow()).is_null())) as i32) != 0));
    assert!(
        ((((((*sel.borrow()).offset((0) as isize).read()) as i32) == ('h' as i32)) as i32) != 0)
    );
    (*n.borrow_mut()) = 200;
    (*sel.borrow_mut()) = if ((((*n.borrow()) < 100) as i32) != 0) {
        (*vp.borrow()).clone()
    } else {
        (AnyPtr::default())
    }
    .reinterpret_cast::<u8>();
    assert!(((((*sel.borrow()).is_null()) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
