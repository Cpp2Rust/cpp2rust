extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Counter {
    pub value: Value<i32>,
}
impl Counter {
    pub fn bump(&self) {
        (*self.value.borrow_mut()).prefix_inc();
    }
    pub fn get(&self) -> i32 {
        return (*self.value.borrow());
    }
}
impl Clone for Counter {
    fn clone(&self) -> Self {
        let mut this = Self {
            value: Rc::new(RefCell::new((*self.value.borrow()))),
        };
        this
    }
}
impl ByteRepr for Counter {}
#[derive(Default)]
pub struct Holder {
    pub c: Value<Counter>,
    pub ref_: Ptr<Counter>,
}
impl Holder {
    pub fn Holder(c: Ptr<Counter>) -> Self {
        let mut this = Self {
            c: Rc::new(RefCell::new(<Counter>::default())),
            ref_: (c).clone(),
        };
        this
    }
}
impl Clone for Holder {
    fn clone(&self) -> Self {
        let mut this = Self {
            c: Rc::new(RefCell::new((*self.c.borrow()).clone())),
            ref_: (self.ref_).clone(),
        };
        this
    }
}
impl ByteRepr for Holder {}
pub fn via_ref_0(r: Ptr<Counter>) {
    ({ (*r.upgrade().deref()).bump() });
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let c: Value<Counter> = Rc::new(RefCell::new(<Counter>::default()));
    let p: Value<Ptr<Counter>> = Rc::new(RefCell::new((c.as_pointer())));
    ({ (*(*p.borrow()).upgrade().deref()).bump() });
    ({ (*(*p.borrow()).upgrade().deref()).bump() });
    let arr: Value<Box<[Counter]>> = Rc::new(RefCell::new(
        (0..2)
            .map(|_| <Counter>::default())
            .collect::<Box<[Counter]>>(),
    ));
    ({ (*arr.borrow())[(0) as usize].bump() });
    ({ (*arr.borrow())[(1) as usize].bump() });
    let h: Value<Holder> = Rc::new(RefCell::new(Holder::Holder(c.as_pointer())));
    ({ (*(*h.borrow()).c.borrow()).bump() });
    ({ (*(*h.borrow()).ref_.upgrade().deref()).bump() });
    ({
        let _r: Ptr<Counter> = c.as_pointer();
        via_ref_0(_r)
    });
    let sum: Value<i32> = Rc::new(RefCell::new({
        let _lhs = {
            let _lhs = {
                let _lhs = {
                    let _lhs = ({ (*(*p.borrow()).upgrade().deref()).get() });
                    _lhs + ({ (*(*h.borrow()).c.borrow()).get() })
                };
                _lhs + ({ (*(*h.borrow()).ref_.upgrade().deref()).get() })
            };
            _lhs + ({ (*arr.borrow())[(0) as usize].get() })
        };
        _lhs + ({ (*arr.borrow())[(1) as usize].get() })
    }));
    println!("{}", (*sum.borrow()));
    return 0;
}
