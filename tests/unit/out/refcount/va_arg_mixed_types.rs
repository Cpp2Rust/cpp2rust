extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, Default)]
pub struct pair {
    pub a: Value<i32>,
    pub b: Value<i32>,
}
impl Clone for pair {
    fn clone(&self) -> Self {
        Self {
            a: Rc::new(RefCell::new((*self.a.borrow()).clone())),
            b: Rc::new(RefCell::new((*self.b.borrow()).clone())),
        }
    }
}
impl ByteRepr for pair {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a.borrow()).to_bytes(&mut buf[0..4]);
        (*self.b.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            b: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
pub fn sum_mixed_0(count: i32, __args: &[VaArg]) -> i32 {
    let count: Value<i32> = Rc::new(RefCell::new(count));
    let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
    (*ap.borrow_mut()) = VaList::new(__args);
    let total: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((((*i.borrow()) < (*count.borrow())) as i32) != 0) {
        let tag: Value<i32> = Rc::new(RefCell::new((*ap.borrow_mut()).arg::<i32>()));
        if ((((*tag.borrow()) == 0) as i32) != 0) {
            (*total.borrow_mut()) += (*ap.borrow_mut()).arg::<i32>();
        } else if ((((*tag.borrow()) == 1) as i32) != 0) {
            (*total.borrow_mut()) += ((*ap.borrow_mut()).arg::<f64>() as i32);
        } else if ((((*tag.borrow()) == 3) as i32) != 0) {
            let p: Value<pair> = Rc::new(RefCell::new((*ap.borrow_mut()).arg::<pair>()));
            (*total.borrow_mut()) += ((*(*p.borrow()).a.borrow()) * (*(*p.borrow()).b.borrow()));
        } else {
            let val: Value<i64> = Rc::new(RefCell::new((*ap.borrow_mut()).arg::<i64>()));
            (*total.borrow_mut()) += ((*val.borrow()) as i32);
        }
        (*i.borrow_mut()).postfix_inc();
    }
    return (*total.borrow());
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
                &[(1).into(), (3.7E+0).into(), (2).into(), (100_i64).into()],
            )
        }) == 103) as i32)
            != 0)
    );
    let p: Value<pair> = Rc::new(RefCell::new(pair {
        a: Rc::new(RefCell::new(7)),
        b: Rc::new(RefCell::new(8)),
    }));
    assert!(
        (((({
            sum_mixed_0(
                2,
                &[(3).into(), (*p.borrow()).into(), (0).into(), (5).into()],
            )
        }) == 61) as i32)
            != 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
