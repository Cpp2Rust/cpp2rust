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
pub struct Pair {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
pub fn twice_0(v: Ptr<i64>) -> i64 {
    return ((v.read()) * 2_i64);
}
pub fn sum_1(p: Ptr<Pair>) -> i32 {
    return ({ p.with(|__s| __s.first) } + { p.with(|__s| __s.second) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut x: i32 = 1;
    let mut y: i32 = {
        x = 2;
        (x + 1)
    };
    assert!((x == 2));
    assert!((y == 3));
    let mut z: i32 = {
        {
            1;
            2
        };
        3
    };
    assert!((z == 3));
    let counter: Value<i32> = Rc::new(RefCell::new(0));
    let mut w: i32 = {
        {
            (*counter.borrow_mut()).postfix_inc();
            (*counter.borrow_mut()).postfix_inc()
        };
        (*counter.borrow())
    };
    assert!(((*counter.borrow()) == 2));
    assert!((w == 2));
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    if {
        {
            a = 1;
            b = 2
        };
        ((a + b) > 0)
    } {
        assert!((a == 1));
        assert!((b == 2));
    }
    let v1: Value<Vec<i32>> = Rc::new(RefCell::new(vec![1, 2]));
    let mut v2: Vec<i32> = {
        a = 5;
        (*v1.borrow()).clone()
    }
    .clone();
    {
        let __a1 = 3;
        v2.push(__a1)
    };
    assert!((a == 5));
    assert!(((*v1.borrow()).len() == 2_usize));
    assert!((v2.len() == 3_usize));
    let p1: Value<Pair> = Rc::new(RefCell::new(Pair {
        first: 1,
        second: 2,
    }));
    let mut p2: Pair = {
        b = 6;
        (*p1.borrow()).clone()
    };
    p2.first = 10;
    assert!(({ (*p1.borrow()).first } == 1));
    assert!((p2.first == 10));
    assert!(
        (({
            let _v: Value<i64> = Rc::new(RefCell::new((a as i64)));
            twice_0(_v.as_pointer())
        }) == 10_i64)
    );
    assert!(
        (({
            sum_1({
                a = 7;
                p1.as_pointer()
            })
        }) == 3)
    );
    assert!((a == 7));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
