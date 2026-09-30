extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Test {
    #[offset(0)]
    pub x: i32,
}
impl ByteRepr for Test {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
pub fn Update_0(t: Ptr<Test>) -> Ptr<Test> {
    let t: Value<Ptr<Test>> = Rc::new(RefCell::new(t));
    let x: Value<i32> = Rc::new(RefCell::new(1));
    let y: Value<i32> = Rc::new(RefCell::new(2));
    (*x.borrow_mut()).prefix_inc();
    ({ TestImpl::update(&(*t.borrow()), (*x.borrow()), (*y.borrow())) });
    (*x.borrow_mut()) = (*t.borrow()).with(|__s: &Test| __s.x);
    (*y.borrow_mut()) = (*t.borrow()).with(|__s: &Test| __s.x);
    ({
        let _x: i32 = (*x.borrow());
        let _y: i32 = (*y.borrow());
        TestImpl::update(&(*t.borrow()), _x, _y)
    });
    return (*t.borrow()).clone();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let t1: Value<Test> = Rc::new(RefCell::new(Test { x: 100 }));
    let t2: Value<Ptr<Test>> = Rc::new(RefCell::new(({ Update_0((t1.as_pointer())) })));
    let t3: Value<Ptr<Test>> = Rc::new(RefCell::new(Ptr::<Test>::null()));
    (*t3.borrow_mut()) = (*t2.borrow()).clone();
    (*t3.borrow()).with_mut(|__s: &mut Test| __s.x = 15);
    {
        let _ptr = ({ TestImpl::as_ptr(&(*t3.borrow())) });
        _ptr.write(_ptr.read() + 10)
    };
    assert!(
        ({
            let _lhs = {
                let _lhs = (*t3.borrow()).with(|__s: &Test| __s.x);
                _lhs + (*t2.borrow()).with(|__s: &Test| __s.x)
            };
            _lhs + { (*t1.borrow()).x }
        } == 75)
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
        (*self).with_mut(|__s: &mut Test| __s.x.postfix_inc());
    }
    fn dec(&self) {
        (*self).with_mut(|__s: &mut Test| __s.x.postfix_dec());
    }
    fn as_ptr(&self) -> Ptr<i32> {
        return (field_ptr!((*self), x));
    }
    fn update(&self, x: i32, y: i32) {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let y: Value<i32> = Rc::new(RefCell::new(y));
        let __rhs = ((*x.borrow()) + (*y.borrow()));
        (*self).with_mut(|__s: &mut Test| __s.x = __rhs);
    }
}
pub fn __cpp2rust_init_globals() {}
