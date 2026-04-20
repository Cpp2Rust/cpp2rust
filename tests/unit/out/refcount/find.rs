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
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    (*v.borrow_mut()).push(1);
    (*v.borrow_mut()).push(2);
    (*v.borrow_mut()).push(3);
    let v_begin: Value<Ptr<i32>> = Rc::new(RefCell::new((v.as_pointer() as Ptr<i32>)));
    let v_end: Value<Ptr<i32>> = Rc::new(RefCell::new((v.as_pointer() as Ptr<i32>).to_end()));
    let it: Value<Ptr<i32>> = Rc::new(RefCell::new(
        (*v_begin.borrow()).clone().offset(
            (*v_begin.borrow())
                .clone()
                .clone()
                .into_iter()
                .enumerate()
                .position(|(index_0, value_0)| {
                    index_0 < (*v_end.borrow()).clone().get_offset() as usize && value_0.read() == 2
                })
                .unwrap_or((*v_end.borrow()).clone().get_offset() as usize) as isize,
        ),
    ));
    let v_result_true: Value<bool> = Rc::new(RefCell::new(
        (*it.borrow()) != (v.as_pointer() as Ptr<i32>).to_end(),
    ));
    let m: Value<BTreeMap<i32, Value<f64>>> = Rc::new(RefCell::new(BTreeMap::new()));
    (m.as_pointer() as Ptr<BTreeMap<i32, Value<f64>>>)
        .with_mut(|__v: &mut BTreeMap<i32, Value<f64>>| {
            __v.entry(1.clone())
                .or_insert_with(|| Rc::new(RefCell::new(<f64>::default())))
                .as_pointer()
        })
        .write(1_f64);
    (m.as_pointer() as Ptr<BTreeMap<i32, Value<f64>>>)
        .with_mut(|__v: &mut BTreeMap<i32, Value<f64>>| {
            __v.entry(2.clone())
                .or_insert_with(|| Rc::new(RefCell::new(<f64>::default())))
                .as_pointer()
        })
        .write(2_f64);
    (m.as_pointer() as Ptr<BTreeMap<i32, Value<f64>>>)
        .with_mut(|__v: &mut BTreeMap<i32, Value<f64>>| {
            __v.entry(3.clone())
                .or_insert_with(|| Rc::new(RefCell::new(<f64>::default())))
                .as_pointer()
        })
        .write(3_f64);
    let m_begin: Value<RefcountMapIter<i32, f64>> = Rc::new(RefCell::new(RefcountMapIter::begin(
        (m.as_pointer() as Ptr<BTreeMap<i32, Value<f64>>>),
    )));
    let m_end: Value<RefcountMapIter<i32, f64>> = Rc::new(RefCell::new(RefcountMapIter::end(
        (m.as_pointer() as Ptr<BTreeMap<i32, Value<f64>>>),
    )));
    let m_result_true: Value<bool> =
        Rc::new(RefCell::new((*m_begin.borrow()) != (*m_end.borrow())));
    return ((((*v_result_true.borrow()) && (*m_result_true.borrow()))
        && (v.as_pointer() as Ptr<i32>).offset(
            (v.as_pointer() as Ptr<i32>)
                .clone()
                .into_iter()
                .enumerate()
                .position(|(index_0, value_0)| {
                    index_0 < (v.as_pointer() as Ptr<i32>).get_offset() as usize
                        && value_0.read() == 2
                })
                .unwrap_or((v.as_pointer() as Ptr<i32>).get_offset() as usize) as isize,
        ) == (v.as_pointer() as Ptr<i32>)) as i32);
}
