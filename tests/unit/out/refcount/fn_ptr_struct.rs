extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Handler {
    #[offset(0)]
    pub tag: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub cb: FnPtr<fn(i32) -> i32>,
}
pub fn double_it_0(mut x: i32) -> i32 {
    return (x * 2);
}
pub fn negate_1(mut x: i32) -> i32 {
    return -x;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct S {}
impl S {
    pub fn pick_1(mut x: i32) -> i32 {
        return (x + 1);
    }
    pub fn pick_2(mut x: i64) -> i32 {
        return ((x as i32) + 2);
    }
    pub fn solo(mut x: i32) -> i32 {
        return (x + 3);
    }
}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(4)]
pub struct MoveOnly {
    #[offset(0)]
    pub data_: i32,
}
impl MoveOnly {
    pub fn new(mut data: i32) -> Self {
        Self { data_: data }
    }
    pub fn move_from(x: Ptr<MoveOnly>) -> Self {
        let __this: MoveOnly = Self {
            data_: x.with(|__s| __s.data_),
        };
        field!(x, data_).write(0);
        __this
    }
}
pub fn make_move_only_2() -> MoveOnly {
    return MoveOnly::new({ 4 });
}
pub fn scale_move_only_3(x: MoveOnly) -> MoveOnly {
    let x: Value<MoveOnly> = Rc::new(RefCell::new(x));
    return MoveOnly::new({ (({ MoveOnlyImpl::get(&x.as_pointer()) }) * 10) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut p1: FnPtr<fn(i32) -> i32> = (FnPtr::<fn(i32) -> i32>::new(S::pick_1));
    let mut p2: FnPtr<fn(i32) -> i32> = FnPtr::<fn(i32) -> i32>::new(S::solo);
    assert!((({ p1.call(5,) }) == 6));
    assert!((({ p2.call(5,) }) == 8));
    assert!((({ S::pick_2(5_i64,) }) == 7));
    let mut h3: Handler = Handler {
        tag: 3,
        cb: (FnPtr::<fn(i32) -> i32>::new(S::pick_1)),
    };
    assert!((({ h3.cb.call(1,) }) == 2));
    let mut h1: Handler = Handler {
        tag: 1,
        cb: FnPtr::<fn(i32) -> i32>::new(double_it_0),
    };
    let mut h2: Handler = Handler {
        tag: 2,
        cb: FnPtr::<fn(i32) -> i32>::new(negate_1),
    };
    assert!(!((h1.cb).is_null()));
    assert!((({ h1.cb.call(5,) }) == 10));
    assert!((({ h2.cb.call(7,) }) == -7_i32));
    h1.cb = FnPtr::<fn(i32) -> i32>::new(negate_1);
    assert!((({ h1.cb.call(3,) }) == -3_i32));
    assert!(({ (h1.cb).clone() } == { (h2.cb).clone() }));
    let mut mk: FnPtr<fn() -> MoveOnly> = FnPtr::<fn() -> MoveOnly>::new(make_move_only_2);
    let m: Value<MoveOnly> = Rc::new(RefCell::new(({ mk.call() })));
    assert!((({ MoveOnlyImpl::get(&m.as_pointer(),) }) == 4));
    let mut sc: FnPtr<fn(MoveOnly) -> MoveOnly> =
        FnPtr::<fn(MoveOnly) -> MoveOnly>::new(scale_move_only_3);
    let n: Value<MoveOnly> = Rc::new(RefCell::new(
        ({ sc.call(MoveOnly::move_from({ m.as_pointer() })) }),
    ));
    assert!((({ MoveOnlyImpl::get(&n.as_pointer(),) }) == 40));
    assert!((({ MoveOnlyImpl::get(&m.as_pointer(),) }) == 0));
    return 0;
}
pub trait MoveOnlyImpl {
    fn get(&self) -> i32;
}
impl MoveOnlyImpl for Ptr<MoveOnly> {
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.data_);
    }
}
pub fn __cpp2rust_init_globals() {}
