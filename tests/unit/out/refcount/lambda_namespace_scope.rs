extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static counter_0: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static inc_1: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(
        FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
            {
                return (x + 1);
            }
        }),
    ));
);
thread_local!(
    pub static bump_2: Value<FnPtr<fn() -> i32>> =
        Rc::new(RefCell::new(FnPtr::<fn() -> i32>::new(|| -> i32 {
            {
                (*counter_0.with(Value::clone).borrow_mut()).postfix_inc();
                return counter_0.with(|rc| *rc.borrow());
            }
        })));
);
pub fn apply_3(f: FnPtr<fn(i32) -> i32>, mut x: i32) -> i32 {
    let f: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(f));
    return ({ (*f.borrow()).call(x) });
}
pub fn by_ref_4(v: Ptr<i32>) -> i32 {
    return ((v.read()) * 2);
}
thread_local!(
    pub static twice_next_5: Value<FnPtr<fn(i32) -> i32>> =
        Rc::new(RefCell::new(FnPtr::<fn(i32) -> i32>::new(
            |mut x: i32| -> i32 {
                {
                    return ({
                        let _v: Value<i32> = Rc::new(RefCell::new((x + 1)));
                        by_ref_4(_v.as_pointer())
                    });
                }
            },
        )));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ (*inc_1.with(Value::clone).borrow()).call(41,) }) == 42));
    ({ (*bump_2.with(Value::clone).borrow()).call() });
    assert!((({ (*bump_2.with(Value::clone).borrow()).call() }) == 2));
    assert!((counter_0.with(|rc| *rc.borrow()) == 2));
    assert!((({ apply_3((*inc_1.with(Value::clone).borrow()).copy_from(), 1,) }) == 2));
    let copy: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(
        (*inc_1.with(Value::clone).borrow()).copy_from(),
    ));
    assert!((({ (*copy.borrow()).call(9,) }) == 10));
    let mut fp: FnPtr<fn(i32) -> i32> = (*inc_1.with(Value::clone).borrow()).clone();
    assert!((({ fp.call(-1_i32,) }) == 0));
    assert!((({ (*twice_next_5.with(Value::clone).borrow()).call(4,) }) == 10));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = counter_0.with(|_| ());
    let _ = inc_1.with(|_| ());
    let _ = bump_2.with(|_| ());
    let _ = twice_next_5.with(|_| ());
}
