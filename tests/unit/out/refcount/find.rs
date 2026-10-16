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
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = 1;
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 2;
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 3;
        (*v.borrow_mut()).push(__a1)
    };
    let v_begin: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
        RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>)),
    ));
    let v_end: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(RandomAccessIterator::Vec(
        (v.as_pointer() as Ptr<i32>).to_end(),
    )));
    let it: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new({
        let mut __it = (*v_begin.borrow()).clone();
        while __it != (*v_end.borrow()).clone() && __it.iter_deref().with(|__v| *__v != 2) {
            __it.iter_inc();
        }
        __it
    }));
    let mut v_result_true: bool =
        (*it.borrow()) != RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>).to_end());
    let m: Value<BTreeMap<i32, Value<(Value<i32>, Value<f64>)>>> =
        Rc::new(RefCell::new(BTreeMap::new()));
    (m.as_pointer() as Ptr<BTreeMap<i32, Value<(Value<i32>, Value<f64>)>>>)
        .with_mut(|__v: &mut BTreeMap<i32, Value<(Value<i32>, Value<f64>)>>| {
            __v.entry(1.clone())
                .or_insert_with(|| {
                    Rc::new(RefCell::new((
                        Rc::new(RefCell::new(1)),
                        Rc::new(RefCell::new(<f64>::default())),
                    )))
                })
                .borrow()
                .1
                .as_pointer()
        })
        .write(1_f64);
    (m.as_pointer() as Ptr<BTreeMap<i32, Value<(Value<i32>, Value<f64>)>>>)
        .with_mut(|__v: &mut BTreeMap<i32, Value<(Value<i32>, Value<f64>)>>| {
            __v.entry(2.clone())
                .or_insert_with(|| {
                    Rc::new(RefCell::new((
                        Rc::new(RefCell::new(2)),
                        Rc::new(RefCell::new(<f64>::default())),
                    )))
                })
                .borrow()
                .1
                .as_pointer()
        })
        .write(2_f64);
    (m.as_pointer() as Ptr<BTreeMap<i32, Value<(Value<i32>, Value<f64>)>>>)
        .with_mut(|__v: &mut BTreeMap<i32, Value<(Value<i32>, Value<f64>)>>| {
            __v.entry(3.clone())
                .or_insert_with(|| {
                    Rc::new(RefCell::new((
                        Rc::new(RefCell::new(3)),
                        Rc::new(RefCell::new(<f64>::default())),
                    )))
                })
                .borrow()
                .1
                .as_pointer()
        })
        .write(3_f64);
    let m_begin: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> =
        Rc::new(RefCell::new(RefcountMapIter::begin(
            (m.as_pointer() as Ptr<BTreeMap<i32, Value<(Value<i32>, Value<f64>)>>>),
        )));
    let m_end: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> =
        Rc::new(RefCell::new(RefcountMapIter::end(
            (m.as_pointer() as Ptr<BTreeMap<i32, Value<(Value<i32>, Value<f64>)>>>),
        )));
    let mut m_result_true: bool = (*m_begin.borrow()) != (*m_end.borrow());
    assert!(
        ((v_result_true) && (m_result_true))
            && ({
                let mut __it = RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>));
                while __it != RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>))
                    && __it.iter_deref().with(|__v| *__v != 2)
                {
                    __it.iter_inc();
                }
                __it
            } == RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>)))
    );
    let w: Value<Vec<i32>> = Rc::new(RefCell::new(vec![1, 2, 3, 4, 5]));
    assert!(
        {
            let mut __it =
                RandomAccessIterator::Vec((w.as_pointer() as Ptr<i32>)).iter_add(2_isize);
            while __it != RandomAccessIterator::Vec((w.as_pointer() as Ptr<i32>).to_end())
                && __it.iter_deref().with(|__v| *__v != 4)
            {
                __it.iter_inc();
            }
            __it
        } == RandomAccessIterator::Vec((w.as_pointer() as Ptr<i32>)).iter_add(3_isize)
    );
    assert!(
        {
            let mut __it =
                RandomAccessIterator::Vec((w.as_pointer() as Ptr<i32>)).iter_add(2_isize);
            while __it != RandomAccessIterator::Vec((w.as_pointer() as Ptr<i32>).to_end())
                && __it.iter_deref().with(|__v| *__v != 1)
            {
                __it.iter_inc();
            }
            __it
        } == RandomAccessIterator::Vec((w.as_pointer() as Ptr<i32>).to_end())
    );
    assert!(
        {
            let mut __it =
                RandomAccessIterator::Vec((w.as_pointer() as Ptr<i32>)).iter_add(2_isize);
            while __it != RandomAccessIterator::Vec((w.as_pointer() as Ptr<i32>)).iter_add(4_isize)
                && __it.iter_deref().with(|__v| *__v != 5)
            {
                __it.iter_inc();
            }
            __it
        } == RandomAccessIterator::Vec((w.as_pointer() as Ptr<i32>)).iter_add(4_isize)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
