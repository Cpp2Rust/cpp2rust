extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sum_0(args_0: i32, args_1: i32, args_2: i32) -> i32 {
    let args_0: Value<i32> = Rc::new(RefCell::new(args_0));
    let args_1: Value<i32> = Rc::new(RefCell::new(args_1));
    let args_2: Value<i32> = Rc::new(RefCell::new(args_2));
    return ((*args_0.borrow()) + ((*args_1.borrow()) + ((*args_2.borrow()) + 0)));
}
pub fn sum_1(args_0: i32, args_1: i32) -> i32 {
    let args_0: Value<i32> = Rc::new(RefCell::new(args_0));
    let args_1: Value<i32> = Rc::new(RefCell::new(args_1));
    return ((*args_0.borrow()) + ((*args_1.borrow()) + 0));
}
pub fn sum_2(args: i32) -> i32 {
    let args: Value<i32> = Rc::new(RefCell::new(args));
    return ((*args.borrow()) + 0);
}
pub fn first_3(x: i32, args_1: i32, args_2: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let args_1: Value<i32> = Rc::new(RefCell::new(args_1));
    let args_2: Value<i32> = Rc::new(RefCell::new(args_2));
    return ((*x.borrow()) + ({ sum_1((*args_1.borrow()), (*args_2.borrow())) }));
}
pub fn first_4(x: i32, args: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let args: Value<i32> = Rc::new(RefCell::new(args));
    return ((*x.borrow()) + ({ sum_2((*args.borrow())) }));
}
pub fn sizeof_pack_5() -> i32 {
    return (0 as i32);
}
pub fn sizeof_pack_6(args: i32) -> i32 {
    let args: Value<i32> = Rc::new(RefCell::new(args));
    return (1 as i32);
}
pub fn sizeof_pack_7(args_0: i32, args_1: i32) -> i32 {
    let args_0: Value<i32> = Rc::new(RefCell::new(args_0));
    let args_1: Value<i32> = Rc::new(RefCell::new(args_1));
    return (2 as i32);
}
pub fn sizeof_pack_8(args_0: i32, args_1: i32, args_2: i32) -> i32 {
    let args_0: Value<i32> = Rc::new(RefCell::new(args_0));
    let args_1: Value<i32> = Rc::new(RefCell::new(args_1));
    let args_2: Value<i32> = Rc::new(RefCell::new(args_2));
    return (3 as i32);
}
pub fn sizeof_pack_9(args_0: i32, args_1: i32, args_2: i32, args_3: i32) -> i32 {
    let args_0: Value<i32> = Rc::new(RefCell::new(args_0));
    let args_1: Value<i32> = Rc::new(RefCell::new(args_1));
    let args_2: Value<i32> = Rc::new(RefCell::new(args_2));
    let args_3: Value<i32> = Rc::new(RefCell::new(args_3));
    return (4 as i32);
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ sum_0(1, 2, 3,) }) == 6));
    assert!((({ sum_1(4, 5,) }) == 9));
    assert!((({ first_3(10, 1, 2,) }) == 13));
    assert!((({ first_4(10, 0,) }) == 10));
    assert!((({ sizeof_pack_5() }) == 0));
    assert!((({ sizeof_pack_6(1,) }) == 1));
    assert!((({ sizeof_pack_7(1, 2,) }) == 2));
    assert!((({ sizeof_pack_8(1, 2, 3,) }) == 3));
    assert!((({ sizeof_pack_9(1, 2, 3, 4,) }) == 4));
    return 0;
}
