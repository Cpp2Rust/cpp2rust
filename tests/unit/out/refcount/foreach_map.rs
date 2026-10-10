extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let m: Value<BTreeMap<i32, Value<(Value<i32>, Value<f64>)>>> =
        Rc::new(RefCell::new(BTreeMap::new()));
    let mut i: i32 = 0;
    let mut k: i32 = 100;
    'loop_: while (i < 100) {
        (m.as_pointer() as Ptr<BTreeMap<i32, Value<(Value<i32>, Value<f64>)>>>)
            .with_mut(|__v: &mut BTreeMap<i32, Value<(Value<i32>, Value<f64>)>>| {
                __v.entry(i.clone())
                    .or_insert_with(|| {
                        Rc::new(RefCell::new((
                            Rc::new(RefCell::new(i)),
                            Rc::new(RefCell::new(<f64>::default())),
                        )))
                    })
                    .borrow()
                    .1
                    .as_pointer()
            })
            .write(((k as f64) / 2_f64));
        {
            i.prefix_inc();
            k.prefix_dec()
        };
    }
    let mut sum: f64 = 0_f64;
    'loop_: for __begin1 in RefcountMapIter::begin(m.as_pointer()) {
        let __begin1: Value<RefcountMapIter<i32, f64>> = Rc::new(RefCell::new(__begin1));
        let i: Ptr<(Value<i32>, Value<f64>)> = (*__begin1.borrow()).deref();
        sum += (*(*i.upgrade().deref()).1.borrow());
    }
    'loop_: for __begin1 in RefcountMapIter::begin(m.as_pointer()) {
        let __begin1: Value<RefcountMapIter<i32, f64>> = Rc::new(RefCell::new(__begin1));
        let i: Value<(Value<i32>, Value<f64>)> = Rc::new(RefCell::new((
            Rc::new(RefCell::new(
                (*(*__begin1.borrow()).deref().upgrade().deref())
                    .0
                    .borrow()
                    .clone(),
            )),
            Rc::new(RefCell::new(
                (*(*__begin1.borrow()).deref().upgrade().deref())
                    .1
                    .borrow()
                    .clone(),
            )),
        )));
        sum += ((*(*i.borrow()).0.borrow()) as f64);
    }
    assert!((sum == 7475_f64));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
