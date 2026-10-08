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
pub struct S {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let base: Value<i32> = Rc::new(RefCell::new(10));
    let add_base: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let base: Ptr<i32> = base.as_pointer();
        },
        |x: i32| -> i32 {
            return (x + (base.read()));
        },
        copy_from {
            base: (base).clone(),
        },
        move_from {
            base: (base).clone(),
        }
    )));
    assert!((({ (*add_base.borrow()).call(5,) }) == 15));
    (*base.borrow_mut()) = 100;
    assert!((({ (*add_base.borrow()).call(5,) }) == 105));
    let s: Value<S> = Rc::new(RefCell::new(S { x: 1, y: 2 }));
    let sum: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let s: Ptr<S> = s.as_pointer();
        },
        || -> i32 {
            return (s.with(|__s| __s.x) + s.with(|__s| __s.y));
        },
        copy_from { s: (s).clone() },
        move_from { s: (s).clone() }
    )));
    assert!((({ (*sum.borrow()).call() }) == 3));
    (*s.borrow_mut()).x = 50;
    assert!((({ (*sum.borrow()).call() }) == 52));
    let counter: Value<i32> = Rc::new(RefCell::new(0));
    let bump: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
        {
            let counter: Ptr<i32> = counter.as_pointer();
        },
        || {
            counter.with_mut(|__v| __v.postfix_inc());
        },
        copy_from {
            counter: (counter).clone(),
        },
        move_from {
            counter: (counter).clone(),
        }
    )));
    ({ (*bump.borrow()).call() });
    ({ (*bump.borrow()).call() });
    assert!(((*counter.borrow()) == 2));
    let arr: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([3_u16, 1_u16, 2_u16, 0_u16])));
    let swap: Value<FnPtr<fn(usize, usize)>> = Rc::new(RefCell::new(lambda!(
        {
            let arr: Ptr<u16> = arr.as_pointer();
        },
        |i: usize, j: usize| {
            let mut t: u16 = (elem!((arr), j).read());
            elem!((arr), j).write({ (elem!((arr), i).read()) });
            elem!((arr), i).write(t);
        },
        copy_from { arr: (arr).clone() },
        move_from { arr: (arr).clone() }
    )));
    ({ (*swap.borrow()).call(0_usize, 3_usize) });
    assert!((((*arr.borrow())[(0) as usize] as i32) == 0));
    assert!((((*arr.borrow())[(3) as usize] as i32) == 3));
    let total: Value<i32> = Rc::new(RefCell::new(0));
    let add: Value<FnPtr<fn(i32)>> = Rc::new(RefCell::new(lambda!(
        {
            let t: Ptr<i32> = total.as_pointer();
        },
        |x: i32| {
            {
                let __rhs = { x };
                t.with_mut(|__v| *__v = *__v + __rhs)
            };
        },
        copy_from { t: (t).clone() },
        move_from { t: (t).clone() }
    )));
    ({ (*add.borrow()).call(2) });
    ({ (*add.borrow()).call(3) });
    assert!(((*total.borrow()) == 5));
    let set_y: Value<FnPtr<fn(i32)>> = Rc::new(RefCell::new(lambda!(
        {
            let y: Ptr<i32> = field_ptr!(s.as_pointer(), y);
        },
        |v: i32| {
            y.write({ v });
        },
        copy_from { y: (y).clone() },
        move_from { y: (y).clone() }
    )));
    ({ (*set_y.borrow()).call(9) });
    assert!(({ (*s.borrow()).y } == 9));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
