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
impl Foo {
    pub fn foo_const(&self) {}
    pub fn foo(&self) {}
    pub fn method_i32(&self, x: i32) {
        let x: Value<i32> = Rc::new(RefCell::new(x));
    }
    pub fn method_i32_const(&self, x: i32) {
        let x: Value<i32> = Rc::new(RefCell::new(x));
    }
    pub fn method2_i32_i32_const(&self, x: i32, y: i32) {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let y: Value<i32> = Rc::new(RefCell::new(y));
    }
    pub fn method2_f64_f64_const(&self, x: f64, y: f64) {
        let x: Value<f64> = Rc::new(RefCell::new(x));
        let y: Value<f64> = Rc::new(RefCell::new(y));
    }
}
impl Clone for Foo {
    fn clone(&self) -> Self {
        let mut this = Self {};
        this
    }
}
impl ByteRepr for Foo {}
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
    (*out.borrow_mut()) += ({
        let _x: i32 = 0;
        foo_0(_x)
    });
    (*out.borrow_mut()) += ({
        let _x: Ptr<i32> = (x.as_pointer());
        foo_1(_x)
    });
    (*out.borrow_mut()) += ({
        let _x: Ptr<i32> = x.as_pointer();
        bar_4(_x)
    });
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
    (*out.borrow_mut()) += (((*bar.borrow())
        + ({
            let _x: i32 = 0;
            foo_0(_x)
        }))
        + ({
            let _x: Ptr<i32> = (x.as_pointer());
            foo_1(_x)
        }));
    let foo1: Value<Foo> = Rc::new(RefCell::new(Foo {}));
    let foo2: Value<Foo> = Rc::new(RefCell::new(Foo {}));
    ({ (*foo1.borrow()).foo() });
    ({
        let _x: i32 = 1;
        (*foo1.borrow()).method_i32(_x)
    });
    ({ (*foo2.borrow()).foo_const() });
    ({
        let _x: i32 = 2;
        (*foo2.borrow()).method_i32_const(_x)
    });
    return (*out.borrow());
}
