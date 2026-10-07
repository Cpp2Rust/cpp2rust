extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct pair {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: i32,
}
pub fn sum_mixed_0(count: i32, __args: &[VaArg]) -> i32 {
    let count: Value<i32> = Rc::new(RefCell::new(count));
    let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
    (*ap.borrow_mut()) = VaList::new(__args);
    let mut total: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while (((i < (*count.borrow())) as i32) != 0) {
        let mut tag: i32 = (*ap.borrow_mut()).arg::<i32>();
        if (((tag == 0) as i32) != 0) {
            total += (*ap.borrow_mut()).arg::<i32>();
        } else if (((tag == 1) as i32) != 0) {
            total += ((*ap.borrow_mut()).arg::<f64>() as i32);
        } else if (((tag == 3) as i32) != 0) {
            let mut p: pair = (*ap.borrow_mut()).arg::<pair>();
            total += (p.a * p.b);
        } else {
            let mut val: i64 = (*ap.borrow_mut()).arg::<i64>();
            total += (val as i32);
        }
        i.postfix_inc();
    }
    return total;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (((({
            sum_mixed_0(
                3,
                &[
                    (0).into(),
                    (10).into(),
                    (1).into(),
                    (2.05E+1).into(),
                    (2).into(),
                    (30_i64).into(),
                ],
            )
        }) == 60) as i32)
            != 0)
    );
    assert!((((({ sum_mixed_0(1, &[(0).into(), (42).into(),]) }) == 42) as i32) != 0));
    assert!(
        (((({
            sum_mixed_0(
                2,
                &[
                    (1).into(),
                    (3.7000000000000002E+0).into(),
                    (2).into(),
                    (100_i64).into(),
                ],
            )
        }) == 103) as i32)
            != 0)
    );
    let mut p: pair = pair { a: 7, b: 8 };
    assert!(
        (((({
            sum_mixed_0(
                2,
                &[(3).into(), ((p).clone()).into(), (0).into(), (5).into()],
            )
        }) == 61) as i32)
            != 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
