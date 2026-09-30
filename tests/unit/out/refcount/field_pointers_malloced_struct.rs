extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct S {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: i32,
    #[offset(8)]
    pub c: i32,
}
pub fn bump_0(s: Ptr<S>) -> i32 {
    let s: Value<Ptr<S>> = Rc::new(RefCell::new(s));
    {
        let _ptr = field!((*s.borrow()), b);
        _ptr.write(_ptr.read() + 10)
    };
    return (*s.borrow()).with(|__s| __s.b);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<Ptr<S>> = Rc::new(RefCell::new(
        libcc2rs::calloc_refcount(1_usize, 12usize).reinterpret_cast::<S>(),
    ));
    assert!(!((*s.borrow()).is_null()));
    field!((*s.borrow()), b).write(1);
    let __rhs = ({ bump_0((*s.borrow()).clone()) });
    field!((*s.borrow()), a).write(__rhs);
    assert!(((*s.borrow()).with(|__s| __s.a) == 11));
    assert!(((*s.borrow()).with(|__s| __s.b) == 11));
    field!((*s.borrow()), a).write(1);
    field!((*s.borrow()), b).write(2);
    field!((*s.borrow()), c).write(0);
    if ({
        let _lhs = (*s.borrow()).with(|__s| __s.a);
        _lhs < (*s.borrow()).with(|__s| __s.b)
    }) && (field!((*s.borrow()), c).with_mut(|__v| __v.postfix_inc()) == 0)
    {
        field!((*s.borrow()), a).write(5);
    }
    assert!(((*s.borrow()).with(|__s| __s.a) == 5) && ((*s.borrow()).with(|__s| __s.c) == 1));
    if ({
        let _lhs = (*s.borrow()).with(|__s| __s.a);
        _lhs < (*s.borrow()).with(|__s| __s.b)
    }) && (field!((*s.borrow()), c).with_mut(|__v| __v.postfix_inc()) == 0)
    {
        field!((*s.borrow()), a).write(6);
    }
    assert!(((*s.borrow()).with(|__s| __s.a) == 5) && ((*s.borrow()).with(|__s| __s.c) == 1));
    let x: Value<i32> = Rc::new(RefCell::new({
        let _lhs = (*s.borrow()).with(|__s| __s.a);
        _lhs + ({
            field!((*s.borrow()), b).write(3);
            (*s.borrow()).with(|__s| __s.b)
        })
    }));
    assert!(((*x.borrow()) == 8) && ((*s.borrow()).with(|__s| __s.b) == 3));
    let y: Value<i32> = Rc::new(RefCell::new(0));
    field!((*s.borrow()), c).write(
        ({
            (*y.borrow_mut()) = 99;
            (*y.borrow())
        }),
    );
    assert!(((*s.borrow()).with(|__s| __s.c) == 99) && ((*y.borrow()) == 99));
    let __rhs = ({ bump_0((*s.borrow()).clone()) });
    {
        let _ptr = field!((*s.borrow()), a);
        _ptr.write(_ptr.read() + __rhs)
    };
    assert!(
        (((*s.borrow()).with(|__s| __s.a) == 18) && ((*s.borrow()).with(|__s| __s.b) == 13))
            && ((*s.borrow()).with(|__s| __s.c) == 99)
    );
    libcc2rs::free_refcount((*s.borrow()).to_any());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
