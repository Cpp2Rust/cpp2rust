extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn apply_0(mut x: i32, mut fn_: FnPtr<fn(i32) -> i32>) -> i32 {
    return ({ fn_.call(x) });
}
pub fn call_nttp_1(mut x: i32) -> i32 {
    return ({
        (FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
            {
                return (x - 1);
            }
        }))
        .call(x)
    });
}
pub fn call_nttp_2(mut x: i32) -> i32 {
    return ({
        (FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
            {
                return (x * 10);
            }
        }))
        .call(x)
    });
}
pub fn call_nttp_3(mut x: i32) -> i32 {
    return ({
        FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
            {
                return (x + 100);
            }
        })
        .call(x)
    });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut fresh: FnPtr<fn(i32) -> i32> = FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
        {
            return -x;
        }
    });
    assert!((({ fresh.call(5,) }) == -5_i32));
    let twice: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(FnPtr::<fn(i32) -> i32>::new(
        |mut x: i32| -> i32 {
            {
                return (x * 2);
            }
        },
    )));
    let mut named: FnPtr<fn(i32) -> i32> = (*twice.borrow()).clone();
    assert!((({ named.call(5,) }) == 10));
    assert!(({ (({ apply_0(5, (*twice.borrow()).clone(),) }) == 10) }));
    named = (fresh).clone();
    assert!((({ named.call(3,) }) == -3_i32));
    let mut p: FnPtr<fn(i32) -> i32> = FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
        {
            return (x * 3);
        }
    });
    assert!((({ p.call(2,) }) == 6));
    assert!((({ call_nttp_1(5,) }) == 4));
    assert!((({ call_nttp_2(5,) }) == 50));
    assert!((({ call_nttp_3(5,) }) == 105));
    let sum: Value<FnPtr<fn(i32, Box<[VaArg]>) -> i32>> =
        Rc::new(RefCell::new(FnPtr::<fn(i32, Box<[VaArg]>) -> i32>::new(
            |n: i32, __args: Box<[VaArg]>| -> i32 {
                {
                    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
                    let n: Value<i32> = Rc::new(RefCell::new(n));
                    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
                    (*ap.borrow_mut()) = __args.as_pointer();
                    let s: Value<i32> = Rc::new(RefCell::new(0));
                    let mut i: i32 = 0;
                    'loop_: while (i < (*n.borrow())) {
                        (*s.borrow_mut()) += (*ap.borrow_mut()).arg::<i32>();
                        i.postfix_inc();
                    }
                    return (*s.borrow());
                }
            },
        )));
    assert!(({ (({ (*sum.borrow()).call(2, Box::new([(5).into(), (5).into(),])) }) == 10) }));
    assert!(
        ({ (({ (*sum.borrow()).call(3, Box::new([(1).into(), (2).into(), (3).into(),])) }) == 6) })
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
