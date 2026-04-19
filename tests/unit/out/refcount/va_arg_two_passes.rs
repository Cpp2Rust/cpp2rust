extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sum_then_product_0(first: i32, args: &[VaArg]) -> i32 {
    let first: Value<i32> = Rc::new(RefCell::new(first));
    let ap: Value<VaList> = <Value<VaList>>::default();
    let sum: Value<i32> = Rc::new(RefCell::new((*first.borrow())));
    let product: Value<i32> = Rc::new(RefCell::new((*first.borrow())));
    (*ap.borrow_mut()) = VaList::new(args);
    let val: Value<i32> = <Value<i32>>::default();
    'loop_: while ((({
        (*val.borrow_mut()) = ((*ap.borrow_mut()).arg::<i32>()).clone();
        (*val.borrow())
    }) as i32)
        != 0)
    {
        (*sum.borrow_mut()) += (*val.borrow());
    }
    (*ap.borrow_mut()) = VaList::new(args);
    'loop_: while ((({
        (*val.borrow_mut()) = ((*ap.borrow_mut()).arg::<i32>()).clone();
        (*val.borrow())
    }) as i32)
        != 0)
    {
        (*product.borrow_mut()) *= (*val.borrow());
    }
    return ((*sum.borrow()) + (*product.borrow()));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (({
            let _first: i32 = 2;
            sum_then_product_0(_first, &[3.into(), 4.into(), 0.into()])
        }) == 33)
    );
    return 0;
}
