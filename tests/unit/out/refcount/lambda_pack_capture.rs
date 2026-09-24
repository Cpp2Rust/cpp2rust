extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sum_0() -> i32 {
    return 0;
}
pub fn sum_1(t: i32, u_1: i32, u_2: i32) -> i32 {
    let t: Value<i32> = Rc::new(RefCell::new(t));
    let u_1: Value<i32> = Rc::new(RefCell::new(u_1));
    let u_2: Value<i32> = Rc::new(RefCell::new(u_2));
    return ((*t.borrow()) + ({ sum_2((*u_1.borrow()), (*u_2.borrow())) }));
}
pub fn sum_2(t: i32, u: i32) -> i32 {
    let t: Value<i32> = Rc::new(RefCell::new(t));
    let u: Value<i32> = Rc::new(RefCell::new(u));
    return ((*t.borrow()) + ({ sum_3((*u.borrow())) }));
}
pub fn sum_3(t: i32) -> i32 {
    let t: Value<i32> = Rc::new(RefCell::new(t));
    return ((*t.borrow()) + ({ sum_0() }));
}
pub fn by_value_4() -> i32 {
    let f: Value<_> = Rc::new(RefCell::new(
        (|| {
            return ({ sum_0() });
        }),
    ));
    ();
    return ({ (*f.borrow_mut())() }).clone();
}
pub fn by_value_5(t_0: i32, t_1: i32, t_2: i32) -> i32 {
    let t_0: Value<i32> = Rc::new(RefCell::new(t_0));
    let t_1: Value<i32> = Rc::new(RefCell::new(t_1));
    let t_2: Value<i32> = Rc::new(RefCell::new(t_2));
    let f: Value<_> = Rc::new(RefCell::new(
        (|| {
            return ({ sum_1((*t_0.borrow()), (*t_1.borrow()), (*t_2.borrow())) });
        }),
    ));
    {
        (*t_0.borrow_mut()) = 0;
        {
            (*t_1.borrow_mut()) = 0;
            (*t_2.borrow_mut()) = 0
        }
    };
    return ({ (*f.borrow_mut())() }).clone();
}
pub fn by_ref_6(t_0: i32, t_1: i32, t_2: i32) -> i32 {
    let t_0: Value<i32> = Rc::new(RefCell::new(t_0));
    let t_1: Value<i32> = Rc::new(RefCell::new(t_1));
    let t_2: Value<i32> = Rc::new(RefCell::new(t_2));
    let f: Value<_> = Rc::new(RefCell::new(
        (|| {
            {
                (*t_0.borrow_mut()) *= 2;
                {
                    (*t_1.borrow_mut()) *= 2;
                    (*t_2.borrow_mut()) *= 2
                }
            };
        }),
    ));
    ({ (*f.borrow_mut())() });
    return ({ sum_1((*t_0.borrow()), (*t_1.borrow()), (*t_2.borrow())) });
}
pub fn init_pack_7(t_0: i32, t_1: i32, t_2: i32) -> i32 {
    let t_0: Value<i32> = Rc::new(RefCell::new(t_0));
    let t_1: Value<i32> = Rc::new(RefCell::new(t_1));
    let t_2: Value<i32> = Rc::new(RefCell::new(t_2));
    let f: Value<_> = Rc::new(RefCell::new(
        (|| {
            return ({ sum_1((*xs.borrow()), (*xs.borrow()), (*xs.borrow())) });
        }),
    ));
    return ({ (*f.borrow_mut())() }).clone();
}
pub fn implicit_8(t_0: i32, t_1: i32) -> i32 {
    let t_0: Value<i32> = Rc::new(RefCell::new(t_0));
    let t_1: Value<i32> = Rc::new(RefCell::new(t_1));
    return ({
        (|| {
            return ({ sum_2((*t_0.borrow()), (*t_1.borrow())) });
        })()
    });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ by_value_4() }) == 0));
    assert!((({ by_value_5(1, 2, 3,) }) == 6));
    assert!((({ by_ref_6(1, 2, 3,) }) == 12));
    assert!((({ init_pack_7(1, 2, 3,) }) == 9));
    assert!((({ implicit_8(4, 5,) }) == 9));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
