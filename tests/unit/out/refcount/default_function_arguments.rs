extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(mut a: i32, b: Option<i32>) -> i32 {
    let mut b: i32 = b.unwrap_or_else(|| 10);
    return (a + b);
}
pub fn baz_1(mut a: Ptr<i32>, b: Option<Ptr<i32>>) -> bool {
    let mut b: Ptr<i32> = b.unwrap_or_else(|| Ptr::<i32>::null());
    return ({ a } == { b });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct Bar {
    #[offset(0)]
    pub v: i32,
}
impl Bar {
    pub fn new(v: Option<i32>) -> Self {
        let mut v: i32 = v.unwrap_or_else(|| 1);
        Self { v: v }
    }
}
impl Default for Bar {
    fn default() -> Self {
        { Bar::new(None) }
    }
}
thread_local!(
    pub static counter_2: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn next_3() -> i32 {
    return (*counter_2.with(Value::clone).borrow_mut()).prefix_inc();
}
pub fn lazy_4(x: Option<i32>) -> i32 {
    let mut x: i32 = x.unwrap_or_else(|| ({ next_3() }));
    return x;
}
pub fn by_ref_5(b: Option<Ptr<Bar>>) -> i32 {
    let mut __b_default: Option<Value<Bar>> = None;
    let b: Ptr<Bar> = b.unwrap_or_else(|| {
        __b_default
            .insert(Rc::new(RefCell::new(Bar::new({ Some(7) }))))
            .as_pointer()
    });
    return b.with(|__s| __s.v);
}
thread_local!(
    pub static global_bar_6: Value<Bar> = Rc::new(RefCell::new(Bar::new({ Some(9) })));
);
pub fn by_global_ref_7(b: Option<Ptr<Bar>>) -> i32 {
    let b: Ptr<Bar> = b.unwrap_or_else(|| global_bar_6.with(|v| v.as_pointer()));
    return b.with(|__s| __s.v);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
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
    let mut b: Bar = Bar::new(None);
    assert!((b.v == 1));
    assert!(({ Bar::new({ Some(2) },).v } == 2));
    let mut arr: [Bar; 3] = [Bar::new(None), Bar::new(None), Bar::new(None)];
    assert!(({ arr[(0) as usize].v } == 1));
    assert!(({ arr[(2) as usize].v } == 1));
    assert!((({ lazy_4(Some(5),) }) == 5));
    assert!((counter_2.with(|rc| *rc.borrow()) == 0));
    assert!((({ lazy_4(None,) }) == 1));
    assert!((counter_2.with(|rc| *rc.borrow()) == 1));
    assert!((({ by_ref_5(None,) }) == 7));
    assert!(
        (({
            let _b: Value<Bar> = Rc::new(RefCell::new(Bar::new({ Some(3) })));
            by_ref_5(Some(_b.as_pointer()))
        }) == 3)
    );
    assert!((({ by_global_ref_7(None,) }) == 9));
    assert!(
        (({
            let _b: Value<Bar> = Rc::new(RefCell::new(Bar::new({ Some(4) })));
            by_global_ref_7(Some(_b.as_pointer()))
        }) == 4)
    );
    let mut h: Holder_int_ = Holder_int_::new({ 4 });
    assert!((h.v == 4));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = counter_2.with(|_| ());
    let _ = global_bar_6.with(|_| ());
}
