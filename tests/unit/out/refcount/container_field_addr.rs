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
    #[offset(32)]
    pub m: BTreeMap<i32, Value<i32>>,
}
impl Clone for S {
    fn clone(&self) -> Self {
        Self {
            tag: self.tag.clone(),
            v: Rc::new(RefCell::new((*self.v.borrow()).clone())),
            m: self.m.clone(),
        }
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        80
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
pub fn put_1(m: Ptr<BTreeMap<i32, Value<i32>>>, k: i32, v: i32) {
    let m: Value<Ptr<BTreeMap<i32, Value<i32>>>> = Rc::new(RefCell::new(m));
    let k: Value<i32> = Rc::new(RefCell::new(k));
    let v: Value<i32> = Rc::new(RefCell::new(v));
    ((*m.borrow()).clone() as Ptr<BTreeMap<i32, Value<i32>>>)
        .with_mut(|__v: &mut BTreeMap<i32, Value<i32>>| {
            __v.entry((*k.borrow()))
                .or_insert_with(|| Rc::new(RefCell::new(<i32>::default())))
                .as_pointer()
        })
        .write((*v.borrow()));
}
pub fn run_2(h: Ptr<S>) {
    let h: Value<Ptr<S>> = Rc::new(RefCell::new(h));
    ({
        let _v: Ptr<Vec<i32>> = ((*h.borrow()).with(|__s| __s.v.clone()).as_pointer());
        let _k: i32 = (*h.borrow()).with(|__s| __s.tag);
        add_0(_v, _k)
    });
    ({
        let _m: Ptr<BTreeMap<i32, Value<i32>>> = (field_ptr!((*h.borrow()), m));
        let _k: i32 = (*h.borrow()).with(|__s| __s.tag);
        put_1(_m, _k, 2)
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
    assert!(
        (((field_ptr!((*h.borrow()), m) as Ptr<BTreeMap<i32, Value<i32>>>)
            .with_mut(|__v: &mut BTreeMap<i32, Value<i32>>| {
                __v.entry(7)
                    .or_insert_with(|| Rc::new(RefCell::new(<i32>::default())))
                    .as_pointer()
            })
            .read())
            == 2)
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
    ({ run_2((local.as_pointer())) });
    let heap: Value<Ptr<S>> = Rc::new(RefCell::new(Ptr::alloc(<S>::default())));
    field!((*heap.borrow()), tag).write(7);
    ({ run_2((*heap.borrow()).clone()) });
    (*heap.borrow()).delete();
    return 0;
}
pub fn __cpp2rust_init_globals() {}
