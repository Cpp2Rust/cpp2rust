extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn next_0(mut pp: Ptr<Ptr<i32>>, calls: Ptr<i32>) -> Ptr<Ptr<i32>> {
    calls.with_mut(|__v| __v.prefix_inc());
    return (pp).clone();
}
pub fn test_1() {
    let a: Value<i32> = Rc::new(RefCell::new(1));
    let mut b: i32 = 42;
    let r: Ptr<i32> = (match ({ b }, a.as_pointer()) {
        (__tmp_1, __tmp_0) => {
            __tmp_0.write(__tmp_1);
            __tmp_0
        }
    });
    assert!(({ (r).clone() } == { (a.as_pointer()) }));
    (match ({ b }, a.as_pointer()) {
        (__tmp_3, __tmp_2) => {
            __tmp_2.write(__tmp_3);
            __tmp_2
        }
    })
    .write(5);
    assert!(({ ((*a.borrow()) == 5) }));
    assert!((b == 42));
    {
        (match ({ 2 }, a.as_pointer()) {
            (__tmp_5, __tmp_4) => {
                __tmp_4.with_mut(|__v| *__v = (((*__v) as i32) + (__tmp_5 as i32)) as i32);
                __tmp_4
            }
        })
        .with_mut(|__v| *__v = *__v * 3)
    };
    assert!(({ ((*a.borrow()) == 21) }));
    let c: Value<i32> = Rc::new(RefCell::new(0));
    (match ({ b }, c.as_pointer()) {
        (__tmp_7, __tmp_6) => {
            __tmp_6.write(__tmp_7);
            __tmp_6
        }
    })
    .write(3);
    assert!(({ ((*c.borrow()) == 3) }));
    {
        (match ({ 1 }, c.as_pointer()) {
            (__tmp_9, __tmp_8) => {
                __tmp_8.with_mut(|__v| *__v = (((*__v) as i32) - (__tmp_9 as i32)) as i32);
                __tmp_8
            }
        })
        .with_mut(|__v| *__v = *__v + 10)
    };
    assert!(({ ((*c.borrow()) == 12) }));
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
    let mut i: i32 = 0;
    (match ({ 9 }, arr.decay().offset(i.postfix_inc())) {
        (__tmp_11, __tmp_10) => {
            __tmp_10.write(__tmp_11);
            __tmp_10
        }
    })
    .write(7);
    assert!((i == 1));
    assert!(({ ((*arr.borrow())[(0) as usize] == 7) }));
    assert!(({ ((*arr.borrow())[(1) as usize] == 2) }));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(arr.decay()));
    {
        let _ptr = (match ({ 1 }, p.as_pointer()) {
            (__tmp_13, __tmp_12) => {
                __tmp_12.with_mut(|__v| *__v = (*__v).offset((__tmp_13 as isize)));
                __tmp_12
            }
        });
        _ptr.write(_ptr.read() + 1)
    };
    assert!(({ ({ (*p.borrow()).clone() } == { arr.decay().offset((2) as isize) }) }));
    let q: Value<Ptr<i32>> = Rc::new(RefCell::new(arr.decay()));
    let calls: Value<i32> = Rc::new(RefCell::new(0));
    {
        let _ptr = ({
            let _pp: Ptr<Ptr<i32>> = (q.as_pointer());
            let _calls: Ptr<i32> = calls.as_pointer();
            next_0(_pp, _calls)
        });
        _ptr.write(_ptr.read() + 1)
    };
    assert!(({ ((*calls.borrow()) == 1) }));
    assert!(({ ({ (*q.borrow()).clone() } == { arr.decay().offset((1) as isize) }) }));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_1() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
