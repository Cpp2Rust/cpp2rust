extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sum_mixed_0(count: i32, __args: &[VaArg]) -> i32 {
    let count: Value<i32> = Rc::new(RefCell::new(count));
    let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
    (*ap.borrow_mut()) = VaList::new(__args);
    let total: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((((*i.borrow()) < (*count.borrow())) as i32) != 0) {
        let tag: Value<i32> = Rc::new(RefCell::new(((*ap.borrow_mut()).arg::<i32>()).clone()));
        if ((((*tag.borrow()) == 0) as i32) != 0) {
            (*total.borrow_mut()) += ((*ap.borrow_mut()).arg::<i32>()).clone();
        } else if ((((*tag.borrow()) == 1) as i32) != 0) {
            (*total.borrow_mut()) += ((*ap.borrow_mut()).arg::<f64>() as i32).clone();
        } else {
            let val: Value<i64> = Rc::new(RefCell::new(((*ap.borrow_mut()).arg::<i64>()).clone()));
            (*total.borrow_mut()) += ((*val.borrow()) as i32);
        }
        (*i.borrow_mut()).postfix_inc();
    }
    return (*total.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (((({
            let _count: i32 = 3;
            sum_mixed_0(
                _count,
                &[
                    0.into(),
                    10.into(),
                    1.into(),
                    2.05E+1.into(),
                    2.into(),
                    30_i64.into(),
                ],
            )
        }) == 60) as i32)
            != 0)
    );
    assert!(
        (((({
            let _count: i32 = 1;
            sum_mixed_0(_count, &[0.into(), 42.into()])
        }) == 42) as i32)
            != 0)
    );
    assert!(
        (((({
            let _count: i32 = 2;
            sum_mixed_0(_count, &[1.into(), 3.7E+0.into(), 2.into(), 100_i64.into()])
        }) == 103) as i32)
            != 0)
    );
    return 0;
}
