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
    pub s: Value<Vec<u8>>,
    #[offset(64)]
    pub m: BTreeMap<i32, Value<i32>>,
}
impl Clone for S {
    fn clone(&self) -> Self {
        Self {
            tag: self.tag.clone(),
            v: Rc::new(RefCell::new((*self.v.borrow()).clone())),
            s: Rc::new(RefCell::new((*self.s.borrow()).clone())),
            m: self.m.clone(),
        }
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        112
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
pub fn append_1(s: Ptr<Vec<u8>>, t: Ptr<u8>, n: usize) {
    let s: Value<Ptr<Vec<u8>>> = Rc::new(RefCell::new(s));
    let t: Value<Ptr<u8>> = Rc::new(RefCell::new(t));
    let n: Value<usize> = Rc::new(RefCell::new(n));
    {
        ((*s.borrow()).clone() as Ptr<Vec<u8>>).with_mut(|__v: &mut Vec<u8>| {
            __v.pop();
            __v.extend(
                (*t.borrow())
                    .clone()
                    .map(|c| c.read())
                    .take((*n.borrow()) as usize),
            );
            __v.push(0);
        });
        ((*s.borrow()).clone() as Ptr<Vec<u8>>)
    };
}
pub fn put_2(m: Ptr<BTreeMap<i32, Value<i32>>>, k: i32, v: i32) {
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
pub fn run_3(h: Ptr<S>) {
    let h: Value<Ptr<S>> = Rc::new(RefCell::new(h));
    ({
        let _v: Ptr<Vec<i32>> = ((*h.borrow()).with(|__s| __s.v.clone()).as_pointer());
        let _k: i32 = (*h.borrow()).with(|__s| __s.tag);
        add_0(_v, _k)
    });
    ({
        append_1(
            ((*h.borrow()).with(|__s| __s.s.clone()).as_pointer()),
            Ptr::<u8>::from_string_literal(b"ab"),
            2_usize,
        )
    });
    ({
        let _m: Ptr<BTreeMap<i32, Value<i32>>> = (field_ptr!((*h.borrow()), m));
        let _k: i32 = (*h.borrow()).with(|__s| __s.tag);
        put_2(_m, _k, 2)
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
        (*(*h.borrow()).with(|__s| __s.s.clone()).borrow())
            .iter()
            .copied()
            .take(
                (*(*h.borrow()).with(|__s| __s.s.clone()).borrow())
                    .len()
                    .saturating_sub(1)
            )
            .eq(Ptr::<u8>::from_string_literal(b"ab").to_c_string_iterator())
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
    ({ run_3((local.as_pointer())) });
    let heap: Value<Ptr<S>> = Rc::new(RefCell::new(Ptr::alloc(<S>::default())));
    field!((*heap.borrow()), tag).write(7);
    ({ run_3((*heap.borrow()).clone()) });
    (*heap.borrow()).delete();
    return 0;
}
pub fn __cpp2rust_init_globals() {}
