extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn make_std_pair_0(x: i32) -> (Value<i32>, Value<bool>) {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return (
        Rc::new(RefCell::new(
            (*x.borrow()).try_into().expect("failed conversion"),
        )),
        Rc::new(RefCell::new(
            ((*x.borrow()) > 0).try_into().expect("failed conversion"),
        )),
    );
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let __decomp_1: Value<(Value<i32>, Value<bool>)> =
        Rc::new(RefCell::new(({ make_std_pair_0(5) })));
    let value: Ptr<i32> = ({ get_2((__decomp_1.as_pointer()).clone()) });
    let positive: Ptr<bool> = ({ get_3((__decomp_1.as_pointer()).clone()) });
    assert!(((value.read()) == 5));
    assert!(((positive.read()) as bool));
    let p: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(1.try_into().expect("failed conversion"))),
        Rc::new(RefCell::new(2.try_into().expect("failed conversion"))),
    )));
    let __decomp_4: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new((*p.borrow()).0.borrow().clone())),
        Rc::new(RefCell::new((*p.borrow()).1.borrow().clone())),
    )));
    let a: Ptr<i32> = ({ get_5((__decomp_4.as_pointer()).clone()) });
    let b: Ptr<i32> = ({ get_6((__decomp_4.as_pointer()).clone()) });
    a.write(10);
    assert!(((a.read()) == 10));
    assert!(((b.read()) == 2));
    assert!(((*(*p.borrow()).0.borrow()) == 1));
    let __decomp_7: Ptr<(Value<i32>, Value<i32>)> = p.as_pointer();
    let x: Ptr<i32> = ({ get_8((__decomp_7).clone()) });
    let y: Ptr<i32> = ({ get_9((__decomp_7).clone()) });
    x.write(3);
    {
        let _ptr = y.clone();
        _ptr.write(_ptr.read() + 4)
    };
    assert!(((*(*p.borrow()).0.borrow()) == 3));
    assert!(((*(*p.borrow()).1.borrow()) == 6));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
