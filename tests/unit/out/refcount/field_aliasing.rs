extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct In {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(32)]
pub struct S {
    #[offset(0)]
    #[byte_size(8)]
    pub in_: In,
    #[offset(8)]
    pub total: i32,
    #[offset(12)]
    pub n: i32,
    #[offset(16)]
    #[byte_size(16)]
    pub arr: Value<Box<[i32]>>,
}
impl Clone for S {
    fn clone(&self) -> Self {
        Self {
            in_: self.in_.clone(),
            total: self.total.clone(),
            n: self.n.clone(),
            arr: Rc::new(RefCell::new((*self.arr.borrow()).clone())),
        }
    }
}
impl Default for S {
    fn default() -> Self {
        S {
            in_: <In>::default(),
            total: 0_i32,
            n: 0_i32,
            arr: Rc::new(RefCell::new((0..4).map(|_| 0_i32).collect::<Box<[i32]>>())),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Node {
    #[offset(0)]
    pub x: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub self_: Ptr<Node>,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p: Value<Ptr<S>> = Rc::new(RefCell::new(
        libcc2rs::calloc_refcount(1_usize, 32usize).reinterpret_cast::<S>(),
    ));
    assert!((((!((*p.borrow()).is_null())) as i32) != 0));
    let q: Value<Ptr<S>> = Rc::new(RefCell::new((*p.borrow()).clone()));
    field!(field!((*p.borrow()), in_), x).write(1);
    field!(field!((*p.borrow()), in_), y).write(2);
    field!((*p.borrow()), total)
        .write(({ (*q.borrow()).with(|__s| __s.in_.x) } + { (*q.borrow()).with(|__s| __s.in_.y) }));
    assert!(((((*q.borrow()).with(|__s| __s.total) == 3) as i32) != 0));
    let ip: Value<Ptr<In>> = Rc::new(RefCell::new((field_ptr!((*p.borrow()), in_))));
    field!((*ip.borrow()), x).write(((*p.borrow()).with(|__s| __s.total) + 1));
    assert!(
        (((((((*q.borrow()).with(|__s| __s.in_.x) == 4) as i32) != 0)
            && ((((*q.borrow()).with(|__s| __s.in_.y) == 2) as i32) != 0)) as i32)
            != 0)
    );
    (array_field_ptr!((*p.borrow()), arr) as Ptr<i32>)
        .offset(((*p.borrow()).with(|__s| __s.n)) as isize)
        .write({ (*p.borrow()).with(|__s| __s.total) });
    {
        let _ptr = field!((*p.borrow()), n);
        _ptr.write(_ptr.read() + 1)
    };
    (array_field_ptr!((*p.borrow()), arr) as Ptr<i32>)
        .offset(((*p.borrow()).with(|__s| __s.n)) as isize)
        .write((*q.borrow()).with(|__s| __s.in_.x));
    assert!(
        (((((((((((array_field_ptr!((*q.borrow()), arr) as Ptr::<i32>)
            .offset((0) as isize)
            .read())
            == 3) as i32)
            != 0)
            && (((((array_field_ptr!((*q.borrow()), arr) as Ptr::<i32>)
                .offset((1) as isize)
                .read())
                == 4) as i32)
                != 0)) as i32)
            != 0)
            && ((((*q.borrow()).with(|__s| __s.n) == 1) as i32) != 0)) as i32)
            != 0)
    );
    libcc2rs::free_refcount((*p.borrow()).to_any());
    let s: Value<Node> = <Value<Node>>::default();
    (*s.borrow_mut()).x = 1;
    (*s.borrow_mut()).self_ = { (s.as_pointer()) };
    field!({ (*s.borrow()).self_.clone() }, x).write({ ({ (*s.borrow()).x } + 1) });
    assert!(((({ (*s.borrow()).x } == 2) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
