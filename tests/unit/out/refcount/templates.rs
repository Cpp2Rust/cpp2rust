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
pub fn pick_2() -> i32 {
    return 0;
}
pub fn pick_3() -> i32 {
    return 1;
}
pub fn bar_4(mut p: Ptr<i32>, mut flag: bool) -> Ptr<i32> {
    return if flag { p } else { Ptr::<i32>::null() };
}
pub fn bar_5(mut p: Ptr<f64>, mut flag: bool) -> Ptr<f64> {
    return if flag { p } else { Ptr::<f64>::null() };
}
pub fn func_6(mut x1: i32, mut x2: i32, mut x3: i32) -> i32 {
    return ((x1 + x2) + x3);
}
pub fn func_7(mut x1: f64, mut x2: i32, mut x3: f64) -> i32 {
    return (((x1 + (x2 as f64)) + x3) as i32);
}
thread_local!(
    pub static half_8: Value<i32> = Rc::new(RefCell::new((1 / 2)));
);
thread_local!(
    pub static half_9: Value<f64> = Rc::new(RefCell::new((1_f64 / 2_f64)));
);
thread_local!(
    pub static half_10: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
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
            + ((({ bar_4((x.as_pointer()), true,) }).read()) as f64))
            + (({ bar_5((y.as_pointer()), true,) }).read()))
            + (({ func_6(1, 2, 3,) }) as f64))
            + (({ func_7(2_f64, (*x.borrow()), (*y.borrow()),) }) as f64))
            == 68_f64)
    );
    assert!((half_8.with(|rc| *rc.borrow()) == 0));
    assert!((half_9.with(|rc| *rc.borrow()) == 0.5_f64));
    half_8.with(|rc| *rc.borrow_mut() = 7);
    assert!((half_8.with(|rc| *rc.borrow()) == 7));
    assert!((half_9.with(|rc| *rc.borrow()) == 0.5_f64));
    assert!((*half_10.with(Value::clone).borrow()).is_null());
    half_10.with(|rc| *rc.borrow_mut() = (x.as_pointer()));
    assert!((((*half_10.with(Value::clone).borrow()).read()) == 10));
    assert!((({ pick_3() }) == 1));
    assert!((({ pick_2() }) == 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = half_8.with(|_| ());
    let _ = half_9.with(|_| ());
    let _ = half_10.with(|_| ());
}
