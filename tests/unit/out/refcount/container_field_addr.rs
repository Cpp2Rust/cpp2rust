extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, VaArg, FnPtrArg, Default)]
pub struct S {
    #[offset(0)]
    pub tag: i32,
    #[offset(8)]
    pub v: Value<Vec<i32>>,
}
impl Clone for S {
    fn clone(&self) -> Self {
        Self {
            tag: self.tag.clone(),
            v: Rc::new(RefCell::new((*self.v.borrow()).clone())),
        }
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        32
    }
}
pub fn add_0(v: Ptr<Vec<i32>>, k: i32) {
    let v: Value<Ptr<Vec<i32>>> = Rc::new(RefCell::new(v));
    let k: Value<i32> = Rc::new(RefCell::new(k));
    {
        let a0_clone = (*k.borrow()).clone();
        (*v.borrow()).with_mut(|__v: &mut Vec<i32>| __v.push(a0_clone))
    };
}
pub fn run_1(h: Ptr<S>) {
    let h: Value<Ptr<S>> = Rc::new(RefCell::new(h));
    ({
        let _v: Ptr<Vec<i32>> = ((*h.borrow()).with(|__s| __s.v.clone()).as_pointer());
        let _k: i32 = (*h.borrow()).with(|__s| __s.tag);
        add_0(_v, _k)
    });
    let pv: Value<Ptr<Vec<i32>>> = Rc::new(RefCell::new(
        ((*h.borrow()).with(|__s| __s.v.clone()).as_pointer()),
    ));
    {
        let __a1 = ((*(*pv.borrow()).upgrade().deref()).len() as i32);
        (*pv.borrow()).with_mut(|__v: &mut Vec<i32>| __v.push(__a1))
    };
    assert!(
        (((*(*h.borrow()).with(|__s| __s.v.clone()).borrow()).len() == 2_usize)
            && ((((*h.borrow()).with(|__s| __s.v.clone()).as_pointer() as Ptr<i32>)
                .offset(0_usize)
                .read())
                == 7))
            && ((((*h.borrow()).with(|__s| __s.v.clone()).as_pointer() as Ptr<i32>)
                .offset(1_usize)
                .read())
                == 1)
    );
    assert!(((*h.borrow()).with(|__s| __s.tag) == 7));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let local: Value<S> = Rc::new(RefCell::new(<S>::default()));
    (*local.borrow_mut()).tag = 7;
    ({ run_1((local.as_pointer())) });
    let heap: Value<Ptr<S>> = Rc::new(RefCell::new(Ptr::alloc(<S>::default())));
    field!((*heap.borrow()), tag).write(7);
    ({ run_1((*heap.borrow()).clone()) });
    (*heap.borrow()).delete();
    return 0;
}
pub fn __cpp2rust_init_globals() {}
