extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn double_it_0(mut x: i32) -> i32 {
    return (x * 2);
}
pub fn triple_it_1(mut x: i32) -> i32 {
    return (x * 3);
}
thread_local!(
    pub static g_op_2: Value<FnPtr<fn(i32) -> i32>> =
        Rc::new(RefCell::new(FnPtr::<fn(i32) -> i32>::null()));
);
pub fn set_op_3(mut fn_: FnPtr<fn(i32) -> i32>) {
    g_op_2.with(|rc| *rc.borrow_mut() = (fn_).clone());
}
pub fn call_op_4(mut x: i32) -> i32 {
    if !(*g_op_2.with(Value::clone).borrow()).is_null() {
        return ({ (*g_op_2.with(Value::clone).borrow()).call(x) });
    }
    return x;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ call_op_4(5,) }) == 5));
    ({ set_op_3(FnPtr::<fn(i32) -> i32>::new(double_it_0)) });
    assert!(!((*g_op_2.with(Value::clone).borrow()).is_null()));
    assert!(
        ({ (*g_op_2.with(Value::clone).borrow()).clone() } == {
            FnPtr::<fn(i32) -> i32>::new(double_it_0)
        })
    );
    assert!((({ call_op_4(5,) }) == 10));
    ({ set_op_3(FnPtr::<fn(i32) -> i32>::new(triple_it_1)) });
    assert!(
        ({ (*g_op_2.with(Value::clone).borrow()).clone() } == {
            FnPtr::<fn(i32) -> i32>::new(triple_it_1)
        })
    );
    assert!((({ call_op_4(5,) }) == 15));
    ({ set_op_3(FnPtr::<fn(i32) -> i32>::null()) });
    assert!((*g_op_2.with(Value::clone).borrow()).is_null());
    assert!((({ call_op_4(5,) }) == 5));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = g_op_2.with(|_| ());
}
