extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return (*x.borrow());
}
pub fn foo_1(x: Ptr<i32>) -> i32 {
    let x: Value<Ptr<i32>> = Rc::new(RefCell::new(x));
    return ((*x.borrow()).read());
}
pub fn foo_2(x: Ptr<i32>, y: Ptr<i32>) -> i32 {
    let x: Value<Ptr<i32>> = Rc::new(RefCell::new(x));
    let y: Value<Ptr<i32>> = Rc::new(RefCell::new(y));
    return {
        let _lhs = ((*x.borrow()).read());
        _lhs + ((*y.borrow()).read())
    };
}
pub fn foo_3(x: Ptr<i32>, y: Ptr<i32>, z: Ptr<i32>) -> i32 {
    let x: Value<Ptr<i32>> = Rc::new(RefCell::new(x));
    let y: Value<Ptr<i32>> = Rc::new(RefCell::new(y));
    return {
        let _lhs = {
            let _lhs = ((*x.borrow()).read());
            _lhs + ((*y.borrow()).read())
        };
        _lhs + (z.read())
    };
}
pub fn bar_4(x: Ptr<i32>) -> i32 {
    return (x.read());
}
#[derive(Default)]
pub struct Foo {}
pub trait FooImpl {
    fn foo_const(&self);
    fn foo(&self);
    fn method_i32(&self, x: i32);
    fn method_i32_const(&self, x: i32);
    fn method2_i32_i32_const(&self, x: i32, y: i32);
    fn method2_f64_f64_const(&self, x: f64, y: f64);
}
impl Clone for Foo {
    fn clone(&self) -> Self {
        let __this: Value<Foo> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<Foo> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Foo {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn func_5(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return 1;
}
pub fn func_6(x: Ptr<i32>) -> i32 {
    let x: Value<Ptr<i32>> = Rc::new(RefCell::new(x));
    return 1;
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(1));
    let out: Value<i32> = Rc::new(RefCell::new(0));
    (*out.borrow_mut()) += ({ foo_0(0) });
    (*out.borrow_mut()) += ({ foo_1((x.as_pointer())) });
    (*out.borrow_mut()) += ({ bar_4(x.as_pointer()) });
    (*out.borrow_mut()) += ({
        let _x: Ptr<i32> = (x.as_pointer());
        let _y: Ptr<i32> = (x.as_pointer());
        let _z: Ptr<i32> = x.as_pointer();
        foo_3(_x, _y, _z)
    });
    (*out.borrow_mut()) += ({
        let _x: Ptr<i32> = (x.as_pointer());
        let _y: Ptr<i32> = (x.as_pointer());
        foo_2(_x, _y)
    });
    let bar: Value<i32> = Rc::new(RefCell::new(5));
    (*out.borrow_mut()) += (((*bar.borrow()) + ({ foo_0(0) })) + ({ foo_1((x.as_pointer())) }));
    let foo1: Value<Foo> = Rc::new(RefCell::new(Foo {}));
    let foo2: Value<Foo> = Rc::new(RefCell::new(Foo {}));
    ({ FooImpl::foo(&foo1.as_pointer()) });
    ({ FooImpl::method_i32(&foo1.as_pointer(), 1) });
    ({ FooImpl::foo_const(&foo2.as_pointer()) });
    ({ FooImpl::method_i32_const(&foo2.as_pointer(), 2) });
    assert!(((*out.borrow()) == 13));
    return 0;
}
impl FooImpl for Ptr<Foo> {
    fn foo_const(&self) {}
    fn foo(&self) {}
    fn method_i32(&self, x: i32) {
        let x: Value<i32> = Rc::new(RefCell::new(x));
    }
    fn method_i32_const(&self, x: i32) {
        let x: Value<i32> = Rc::new(RefCell::new(x));
    }
    fn method2_i32_i32_const(&self, x: i32, y: i32) {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let y: Value<i32> = Rc::new(RefCell::new(y));
    }
    fn method2_f64_f64_const(&self, x: f64, y: f64) {
        let x: Value<f64> = Rc::new(RefCell::new(x));
        let y: Value<f64> = Rc::new(RefCell::new(y));
    }
}
