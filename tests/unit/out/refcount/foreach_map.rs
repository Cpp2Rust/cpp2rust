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
    let mut i: i32 = 0;
    let mut k: i32 = 100;
    'loop_: while (i < 100) {
        (m.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<f64>)>>)
            .with_mut(|__m: &mut Map<i32, (Value<i32>, Value<f64>)>| {
                __m.get_or_insert_with(i.clone(), || {
                    (
                        Rc::new(RefCell::new(i)),
                        Rc::new(RefCell::new(<f64>::default())),
                    )
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
    let __begin1: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(
        (m.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<f64>)>>)
            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
    ));
    let __end1: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(
        (m.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<f64>)>>)
            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.end()),
    ));
    'loop_: while (*__begin1.borrow()) != (*__end1.borrow()) {
        let i: Ptr<(Value<i32>, Value<f64>)> = (*__begin1.borrow()).iter_deref();
        sum += (*(*i.upgrade().deref()).1.borrow());
        (*__begin1.borrow_mut()).iter_inc();
    }
    let __begin1: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(
        (m.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<f64>)>>)
            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
    ));
    let __end1: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(
        (m.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<f64>)>>)
            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.end()),
    ));
    'loop_: while (*__begin1.borrow()) != (*__end1.borrow()) {
        let i: Value<(Value<i32>, Value<f64>)> = Rc::new(RefCell::new((
            Rc::new(RefCell::new(
                (*(*__begin1.borrow()).iter_deref().upgrade().deref())
                    .0
                    .borrow()
                    .clone(),
            )),
            Rc::new(RefCell::new(
                (*(*__begin1.borrow()).iter_deref().upgrade().deref())
                    .1
                    .borrow()
                    .clone(),
            )),
        )));
        sum += ((*(*i.borrow()).0.borrow()) as f64);
        (*__begin1.borrow_mut()).iter_inc();
    }
    assert!((sum == 7475_f64));
    let __begin1: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(
        (m.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<f64>)>>)
            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
    ));
    let __end1: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(
        (m.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<f64>)>>)
            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.end()),
    ));
    'loop_: while (*__begin1.borrow()) != (*__end1.borrow()) {
        let i: Value<(Value<i32>, Value<f64>)> = Rc::new(RefCell::new((
            Rc::new(RefCell::new(
                (*(*__begin1.borrow()).iter_deref().upgrade().deref())
                    .0
                    .borrow()
                    .clone(),
            )),
            Rc::new(RefCell::new(
                (*(*__begin1.borrow()).iter_deref().upgrade().deref())
                    .1
                    .borrow()
                    .clone(),
            )),
        )));
        (*(*i.borrow()).1.borrow_mut()) = (-1_i32 as f64);
        assert!(((*(*i.borrow()).1.borrow()) == (-1_i32 as f64)));
        (*__begin1.borrow_mut()).iter_inc();
    }
    let __begin1: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(
        (m.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<f64>)>>)
            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
    ));
    let __end1: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(
        (m.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<f64>)>>)
            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.end()),
    ));
    'loop_: while (*__begin1.borrow()) != (*__end1.borrow()) {
        let i: Ptr<(Value<i32>, Value<f64>)> = (*__begin1.borrow()).iter_deref();
        assert!(((*(*i.upgrade().deref()).1.borrow()) != (-1_i32 as f64)));
        (*__begin1.borrow_mut()).iter_inc();
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
