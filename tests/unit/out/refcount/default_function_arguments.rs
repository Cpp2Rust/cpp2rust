extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(mut a: i32, mut b: i32) -> i32 {
    return (a + b);
}
pub fn baz_1(mut a: Ptr<i32>, mut b: Ptr<i32>) -> bool {
    return ({ a } == { b });
}
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct Bar {
    #[offset(0)]
    pub v: i32,
}
impl Bar {
    pub fn new(mut v: i32) -> Self {
        Self { v: v }
    }
}
impl Default for Bar {
    fn default() -> Self {
        { Bar::new({ 1 }) }
    }
}
thread_local!(
    pub static counter_2: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn next_3() -> i32 {
    return (*(*counter_2.with(Value::clone).borrow_mut()).prefix_inc());
}
pub fn lazy_4(mut x: i32) -> i32 {
    return x;
}
pub fn by_ref_5(b: Ptr<Bar>) -> i32 {
    return b.with(|__s| __s.v);
}
thread_local!(
    pub static global_bar_6: Value<Bar> = Rc::new(RefCell::new(Bar::new({ 9 })));
);
pub fn by_global_ref_7(b: Ptr<Bar>) -> i32 {
    return b.with(|__s| __s.v);
}
pub fn shifted_8(mut x: i32, mut y: i32) -> i32 {
    return (x + y);
}
pub fn first_9(r: Ptr<i32>) -> i32 {
    return (elem!(r, 0).read());
}
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Holder_int_ {
    #[offset(0)]
    pub v: i32,
}
impl Holder_int_ {
    pub fn new(mut v: i32) -> Self {
        Self { v: v }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ foo_0(1, 10,) }) == 11));
    assert!((({ foo_0(1, 2,) }) == 3));
    let a: Value<i32> = Rc::new(RefCell::new(0));
    assert!(((({ baz_1((a.as_pointer()), Ptr::<i32>::null(),) }) as i32) == (false as i32)));
    assert!(
        ((({
            let _a: Ptr<i32> = (a.as_pointer());
            let _b: Ptr<i32> = (a.as_pointer());
            baz_1(_a, _b)
        }) as i32)
            == (true as i32))
    );
    let mut b: Bar = Bar::new({ 1 });
    assert!((b.v == 1));
    assert!(({ Bar::new({ 2 },).v } == 2));
    let mut arr: [Bar; 3] = [Bar::new({ 1 }), Bar::new({ 1 }), Bar::new({ 1 })];
    assert!((arr[(0) as usize].v == 1));
    assert!((arr[(2) as usize].v == 1));
    assert!((({ lazy_4(5,) }) == 5));
    assert!((counter_2.with(|rc| *rc.borrow()) == 0));
    assert!((({ lazy_4(({ next_3() }),) }) == 1));
    assert!((counter_2.with(|rc| *rc.borrow()) == 1));
    assert!(
        (({
            let _b: Value<Bar> = Rc::new(RefCell::new(Bar::new({ 7 })));
            by_ref_5(_b.as_pointer())
        }) == 7)
    );
    assert!(
        (({
            let _b: Value<Bar> = Rc::new(RefCell::new(Bar::new({ 3 })));
            by_ref_5(_b.as_pointer())
        }) == 3)
    );
    assert!((({ by_global_ref_7(global_bar_6.with(|v| v.as_pointer()),) }) == 9));
    assert!(
        (({
            let _b: Value<Bar> = Rc::new(RefCell::new(Bar::new({ 4 })));
            by_global_ref_7(_b.as_pointer())
        }) == 4)
    );
    let mut h: Holder_int_ = Holder_int_::new({ 4 });
    assert!((h.v == 4));
    assert!((({ shifted_8(3, -1_i32,) }) == 2));
    assert!((({ shifted_8(3, 4,) }) == 7));
    assert!(
        (({
            let _r: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([4, 5, 6])));
            first_9(_r.decay())
        }) == 4)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = counter_2.with(|_| ());
    let _ = global_bar_6.with(|_| ());
}
