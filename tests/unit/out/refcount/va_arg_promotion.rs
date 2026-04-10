extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_promotions_0(count: i32, args: &[VaArg]) -> i32 {
    let count: Value<i32> = Rc::new(RefCell::new(count));
    let ap: Value<VaList> = Rc::new(RefCell::new(<VaList>::default()));
    (*ap.borrow_mut()) = VaList::new(args);
    let a: Value<i32> = Rc::new(RefCell::new(((*ap.borrow_mut()).arg::<i32>()).clone()));
    let b: Value<i32> = Rc::new(RefCell::new(((*ap.borrow_mut()).arg::<i32>()).clone()));
    let c: Value<f64> = Rc::new(RefCell::new(((*ap.borrow_mut()).arg::<f64>()).clone()));
    assert!(((*a.borrow()) == 65));
    assert!(((*b.borrow()) == 10));
    assert!(((*c.borrow()) == 3.0E+0));
    return (((*a.borrow()) + (*b.borrow())) + ((*c.borrow()) as i32));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<u8> = Rc::new(RefCell::new(('A' as u8)));
    let y: Value<i16> = Rc::new(RefCell::new(10_i16));
    let z: Value<f32> = Rc::new(RefCell::new(3.0E+0));
    assert!(
        (({
            let _count: i32 = 3;
            test_promotions_0(
                _count,
                &[
                    ((*x.borrow()) as i32).into(),
                    ((*y.borrow()) as i32).into(),
                    ((*z.borrow()) as f64).into(),
                ],
            )
        }) == 78)
    );
    return 0;
}
