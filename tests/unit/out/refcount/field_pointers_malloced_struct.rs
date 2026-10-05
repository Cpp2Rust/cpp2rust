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
pub fn bump_0(mut s: Ptr<S>) -> i32 {
    {
        field!(s, b).with_mut(|__v| *__v = *__v + 10)
    };
    return s.with(|__s| __s.b);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut s: Ptr<S> = libcc2rs::calloc_refcount(1_usize, 12usize).reinterpret_cast::<S>();
    assert!(!((s).is_null()));
    field!(s, b).write(1);
    let __rhs = ({ bump_0((s).clone()) });
    field!(s, a).write(__rhs);
    assert!((s.with(|__s| __s.a) == 11));
    assert!((s.with(|__s| __s.b) == 11));
    field!(s, a).write(1);
    field!(s, b).write(2);
    field!(s, c).write(0);
    if ({ s.with(|__s| __s.a) } < { s.with(|__s| __s.b) })
        && (field!(s, c).with_mut(|__v| __v.postfix_inc()) == 0)
    {
        field!(s, a).write(5);
    }
    assert!((s.with(|__s| __s.a) == 5) && (s.with(|__s| __s.c) == 1));
    if ({ s.with(|__s| __s.a) } < { s.with(|__s| __s.b) })
        && (field!(s, c).with_mut(|__v| __v.postfix_inc()) == 0)
    {
        field!(s, a).write(6);
    }
    assert!((s.with(|__s| __s.a) == 5) && (s.with(|__s| __s.c) == 1));
    let mut x: i32 = ({ s.with(|__s| __s.a) } + {
        ({
            field!(s, b).write(3);
            s.with(|__s| __s.b)
        })
    });
    assert!((x == 8) && (s.with(|__s| __s.b) == 3));
    let mut y: i32 = 0;
    let __rhs = ({
        y = 99;
        y
    });
    field!(s, c).write(__rhs);
    assert!((s.with(|__s| __s.c) == 99) && (y == 99));
    let __rhs = ({ bump_0((s).clone()) });
    {
        field!(s, a).with_mut(|__v| *__v = *__v + __rhs)
    };
    assert!(
        ((s.with(|__s| __s.a) == 18) && (s.with(|__s| __s.b) == 13)) && (s.with(|__s| __s.c) == 99)
    );
    libcc2rs::free_refcount((s).to_any());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
