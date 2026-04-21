extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let m: Value<BTreeMap<i32, Value<f64>>> = Rc::new(RefCell::new(BTreeMap::new()));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    let k: Value<i32> = Rc::new(RefCell::new(100));
    'loop_: while ((*i.borrow()) < 100) {
        (m.as_pointer() as Ptr<BTreeMap<i32, Value<f64>>>)
            .with_mut(|__v: &mut BTreeMap<i32, Value<f64>>| {
                __v.entry((*i.borrow()).clone())
                    .or_insert_with(|| Rc::new(RefCell::new(<f64>::default())))
                    .as_pointer()
            })
            .write((((*k.borrow()) as f64) / 2.0E+0));
        (*i.borrow_mut()).prefix_inc();
        (*k.borrow_mut()).prefix_dec();
    }
    let sum: Value<f64> = Rc::new(RefCell::new(0_f64));
    'loop_: for i in RefcountMapIter::begin(m.as_pointer()) {
        (*sum.borrow_mut()) += (*i.second().borrow());
    }
    'loop_: for i in RefcountMapIter::begin(m.as_pointer()) {
        let i: Value<RefcountMapIter<i32, f64>> = Rc::new(RefCell::new(i));
        (*sum.borrow_mut()) += ((*(*i.borrow()).first().borrow()) as f64);
    }
    return ((*sum.borrow()) as i32);
}
