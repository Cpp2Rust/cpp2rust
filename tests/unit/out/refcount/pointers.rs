extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Test {
    #[offset(0)]
    pub x: i32,
}
pub fn Update_0(mut t: Ptr<Test>) -> Ptr<Test> {
    let mut x: i32 = 1;
    let mut y: i32 = 2;
    x.prefix_inc();
    ({ TestImpl::update(&t, x, y) });
    x = t.with(|__s| (__s).x);
    y = t.with(|__s| __s.x);
    ({
        let _x: i32 = x;
        let _y: i32 = y;
        TestImpl::update(&(t), _x, _y)
    });
    return (t).clone();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let t1: Value<Test> = Rc::new(RefCell::new(Test { x: 100 }));
    let mut t2: Ptr<Test> = ({ Update_0((t1.as_pointer())) });
    let mut t3: Ptr<Test> = Ptr::<Test>::null();
    t3 = (t2).clone();
    field!(t3, x).write(15);
    {
        let _ptr = ({ TestImpl::as_ptr(&t3) });
        _ptr.write(_ptr.read() + 10)
    };
    assert!(
        (({ ({ t3.with(|__s| __s.x) } + { t2.with(|__s| __s.x) }) } + { { (*t1.borrow()).x } })
            == 75)
    );
    return 0;
}
pub trait TestImpl {
    fn inc(&self);
    fn dec(&self);
    fn as_ptr(&self) -> Ptr<i32>;
    fn update(&self, x: i32, y: i32);
}
impl TestImpl for Ptr<Test> {
    fn inc(&self) {
        field!((*self), x).with_mut(|__v| __v.postfix_inc());
    }
    fn dec(&self) {
        field!((*self), x).with_mut(|__v| __v.postfix_dec());
    }
    fn as_ptr(&self) -> Ptr<i32> {
        return (field_ptr!((*self), x));
    }
    fn update(&self, mut x: i32, mut y: i32) {
        field!((*self), x).write((x + y));
    }
}
pub fn __cpp2rust_init_globals() {}
