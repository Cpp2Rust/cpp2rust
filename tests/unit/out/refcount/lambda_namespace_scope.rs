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
    pub static inc_1: Value<_> = Rc::new(RefCell::new(
        (|x: i32| {
            let x: Value<i32> = Rc::new(RefCell::new(x));
            return ((*x.borrow()) + 1);
        }),
    ));
);
thread_local!(
    pub static bump_2: Value<_> = Rc::new(RefCell::new(
        (|| {
            (*counter_0.with(Value::clone).borrow_mut()).postfix_inc();
            return counter_0.with(|rc| *rc.borrow());
        }),
    ));
);
pub fn apply_3(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    let f: Value<_> = Rc::new(RefCell::new(f));
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ({ (*f.borrow_mut())((*x.borrow())) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ (*inc_1.with(Value::clone).borrow_mut())(41,) }) == 42));
    ({ (*bump_2.with(Value::clone).borrow_mut())() });
    assert!((({ (*bump_2.with(Value::clone).borrow_mut())() }) == 2));
    assert!((counter_0.with(|rc| *rc.borrow()) == 2));
    assert!((({ apply_3(inc_1.with(|rc| rc.borrow().clone()), 1,) }) == 2));
    let copy: Value<_> = Rc::new(RefCell::new(inc_1.with(|rc| rc.borrow().clone())));
    assert!((({ (*copy.borrow_mut())(9,) }) == 10));
    let fp: Value<FnPtr<fn(i32) -> i32>> =
        Rc::new(RefCell::new(({ inc_1.with(|rc| rc.borrow().clone())() })));
    assert!((({ (*fp.borrow()).call(-1_i32,) }) == 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = counter_0.with(|_| ());
    let _ = inc_1.with(|_| ());
    let _ = bump_2.with(|_| ());
}
