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
    let m: Value<Map<i32, (Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(Map::new()));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    let mut k: i32 = 100;
    'loop_: while ((*i.borrow()) < 100) {
        Map::subscript(m.as_pointer(), i.as_pointer()).write(((k as f64) / 2_f64));
        {
            (*i.borrow_mut()).prefix_inc();
            k.prefix_dec()
        };
    }
    let mut sum: f64 = 0_f64;
    'loop_: for i in IterRange::new(
        m.as_pointer()
            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
        m.as_pointer()
            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.end()),
    ) {
        let i: Ptr<(Value<i32>, Value<f64>)> = i.iter_deref();
        sum += (*(*i.upgrade().deref()).1.borrow());
    }
    'loop_: for i in IterRange::new(
        m.as_pointer()
            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
        m.as_pointer()
            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.end()),
    ) {
        let i: Value<(Value<i32>, Value<f64>)> = Rc::new(RefCell::new(match (i.iter_deref(),) {
            (__tmp_0,) => (
                Rc::new(RefCell::new(
                    (*__tmp_0.upgrade().deref()).0.borrow().clone(),
                )),
                Rc::new(RefCell::new(
                    (*__tmp_0.upgrade().deref()).1.borrow().clone(),
                )),
            ),
        }));
        sum += ((*(*i.borrow()).0.borrow()) as f64);
    }
    assert!((sum == 7475_f64));
    'loop_: for i in IterRange::new(
        m.as_pointer()
            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
        m.as_pointer()
            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.end()),
    ) {
        let i: Value<(Value<i32>, Value<f64>)> = Rc::new(RefCell::new(match (i.iter_deref(),) {
            (__tmp_1,) => (
                Rc::new(RefCell::new(
                    (*__tmp_1.upgrade().deref()).0.borrow().clone(),
                )),
                Rc::new(RefCell::new(
                    (*__tmp_1.upgrade().deref()).1.borrow().clone(),
                )),
            ),
        }));
        (*(*i.borrow()).1.borrow_mut()) = (-1_i32 as f64);
        assert!(((*(*i.borrow()).1.borrow()) == (-1_i32 as f64)));
    }
    'loop_: for i in IterRange::new(
        m.as_pointer()
            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
        m.as_pointer()
            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.end()),
    ) {
        let i: Ptr<(Value<i32>, Value<f64>)> = i.iter_deref();
        assert!(((*(*i.upgrade().deref()).1.borrow()) != (-1_i32 as f64)));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
