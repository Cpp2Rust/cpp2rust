extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Test {
    pub x: Value<i32>,
}
pub trait TestImpl {
    fn inc(&self);
    fn dec(&self);
    fn as_ptr(&self) -> Ptr<i32>;
    fn update(&self, x: i32, y: i32);
}
impl Clone for Test {
    fn clone(&self) -> Self {
        let __this: Value<Test> = Rc::new(RefCell::new(Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
        }));
        let this: Ptr<Test> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Test {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn Update_0(t: Ptr<Test>) -> Ptr<Test> {
    let t: Value<Ptr<Test>> = Rc::new(RefCell::new(t));
    let x: Value<i32> = Rc::new(RefCell::new(1));
    let y: Value<i32> = Rc::new(RefCell::new(2));
    (*x.borrow_mut()).prefix_inc();
    ({ TestImpl::update(&(*t.borrow()), (*x.borrow()), (*y.borrow())) });
    (*x.borrow_mut()) = (*(*(*t.borrow()).upgrade().deref()).x.borrow());
    (*y.borrow_mut()) = (*(*(*t.borrow()).upgrade().deref()).x.borrow());
    ({
        let _x: i32 = (*x.borrow());
        let _y: i32 = (*y.borrow());
        TestImpl::update(&(*t.borrow()), _x, _y)
    });
    return (*t.borrow()).clone();
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let t1: Value<Test> = Rc::new(RefCell::new(Test {
        x: Rc::new(RefCell::new(100)),
    }));
    let t2: Value<Ptr<Test>> = Rc::new(RefCell::new(({ Update_0((t1.as_pointer())) })));
    let t3: Value<Ptr<Test>> = Rc::new(RefCell::new(Ptr::<Test>::null()));
    (*t3.borrow_mut()) = (*t2.borrow()).clone();
    (*(*(*t3.borrow()).upgrade().deref()).x.borrow_mut()) = 15;
    {
        let _ptr = ({ TestImpl::as_ptr(&(*t3.borrow())) }).clone();
        _ptr.write(_ptr.read() + 10)
    };
    assert!(
        ({
            let _lhs = {
                let _lhs = (*(*(*t3.borrow()).upgrade().deref()).x.borrow());
                _lhs + (*(*(*t2.borrow()).upgrade().deref()).x.borrow())
            };
            _lhs + (*(*t1.borrow()).x.borrow())
        } == 75)
    );
    return 0;
}
impl TestImpl for Ptr<Test> {
    fn inc(&self) {
        (*(*(*self).upgrade().deref()).x.borrow_mut()).postfix_inc();
    }
    fn dec(&self) {
        (*(*(*self).upgrade().deref()).x.borrow_mut()).postfix_dec();
    }
    fn as_ptr(&self) -> Ptr<i32> {
        return ((*(*self).upgrade().deref()).x.as_pointer());
    }
    fn update(&self, x: i32, y: i32) {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let y: Value<i32> = Rc::new(RefCell::new(y));
        (*(*(*self).upgrade().deref()).x.borrow_mut()) = ((*x.borrow()) + (*y.borrow()));
    }
}
