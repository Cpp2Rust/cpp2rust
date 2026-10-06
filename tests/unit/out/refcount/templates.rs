extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(mut x: i32) -> i32 {
    return x;
}
pub fn foo_1(mut x: f64) -> f64 {
    return x;
}
pub fn bar_2(mut p: Ptr<i32>, mut flag: bool) -> Ptr<i32> {
    return if flag { p } else { Ptr::<i32>::null() };
}
pub fn bar_3(mut p: Ptr<f64>, mut flag: bool) -> Ptr<f64> {
    return if flag { p } else { Ptr::<f64>::null() };
}
pub fn func_4(mut x1: i32, mut x2: i32, mut x3: i32) -> i32 {
    return ((x1 + x2) + x3);
}
pub fn func_5(mut x1: f64, mut x2: i32, mut x3: f64) -> i32 {
    return (((x1 + (x2 as f64)) + x3) as i32);
}
thread_local!(
    pub static half_6: Value<i32> = Rc::new(RefCell::new((1 / 2)));
);
thread_local!(
    pub static half_7: Value<f64> = Rc::new(RefCell::new((1_f64 / 2_f64)));
);
thread_local!(
    pub static half_8: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(10));
    let y: Value<f64> = Rc::new(RefCell::new(((*x.borrow()) as f64)));
    assert!(
        (((((((({ foo_0((*x.borrow()),) }) as f64) + ({ foo_1((*y.borrow()),) }))
            + ((({ bar_2((x.as_pointer()), true,) }).read()) as f64))
            + (({ bar_3((y.as_pointer()), true,) }).read()))
            + (({ func_4(1, 2, 3,) }) as f64))
            + (({ func_5(2.0E+0, (*x.borrow()), (*y.borrow()),) }) as f64))
            == 68_f64)
    );
    assert!((half_6.with(|rc| *rc.borrow()) == 0));
    assert!((half_7.with(|rc| *rc.borrow()) == 5.0E-1));
    half_6.with(|rc| *rc.borrow_mut() = 7);
    assert!((half_6.with(|rc| *rc.borrow()) == 7));
    assert!((half_7.with(|rc| *rc.borrow()) == 5.0E-1));
    assert!((*half_8.with(Value::clone).borrow()).is_null());
    half_8.with(|rc| *rc.borrow_mut() = (x.as_pointer()));
    assert!((((*half_8.with(Value::clone).borrow()).read()) == 10));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = half_6.with(|_| ());
    let _ = half_7.with(|_| ());
    let _ = half_8.with(|_| ());
}
