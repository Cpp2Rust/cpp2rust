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
pub struct Point {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
impl Point {
    pub fn new(mut x: i32, mut y: i32) -> Self {
        Self { x: x, y: y }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Option<Rc<dyn std::any::Any>>> = Rc::new(RefCell::new(Some(Rc::new(
        RefCell::new(Point::new({ 3 }, { 4 })),
    ))));
    assert!((*a.borrow()).is_some());
    let mut p: Ptr<Point> = (a.as_pointer()).with(|__a: &Option<Rc<dyn std::any::Any>>| match __a
        .clone()
        .map(|__r| __r.downcast::<RefCell<Point>>())
    {
        Some(Ok(__v)) => __v.as_pointer(),
        _ => Ptr::null(),
    });
    assert!(!((p).is_null()));
    assert!((p.with(|__s| __s.x) == 3));
    assert!((p.with(|__s| __s.y) == 4));
    assert!(
        ((a.as_pointer()).with(|__a: &Option<Rc<dyn std::any::Any>>| {
            match __a.clone().map(|__r| __r.downcast::<RefCell<i32>>()) {
                Some(Ok(__v)) => __v.as_pointer(),
                _ => Ptr::null(),
            }
        }))
        .is_null()
    );
    let q: Ptr<Point> = {
        let __value: Value<Point> = Rc::new(RefCell::new(Point::new({ 5 }, { 6 })));
        let __ptr = __value.as_pointer();
        (*a.borrow_mut()) = Some(__value);
        __ptr
    };
    assert!((q.with(|__s| __s.x) == 5));
    assert!(
        ((a.as_pointer())
            .with(|__a: &Option<Rc<dyn std::any::Any>>| {
                match __a.clone().map(|__r| __r.downcast::<RefCell<Point>>()) {
                    Some(Ok(__v)) => __v.as_pointer(),
                    _ => Ptr::null(),
                }
            })
            .with(|__s| __s.y)
            == 6)
    );
    let b: Value<Option<Rc<dyn std::any::Any>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(7)))));
    assert!(
        (((b.as_pointer())
            .with(|__a: &Option<Rc<dyn std::any::Any>>| {
                match __a.clone().map(|__r| __r.downcast::<RefCell<i32>>()) {
                    Some(Ok(__v)) => __v.as_pointer(),
                    _ => Ptr::null(),
                }
            })
            .read())
            == 7)
    );
    (*b.borrow_mut()) = None;
    assert!(!((*b.borrow()).is_some()));
    assert!(
        ((b.as_pointer()).with(|__a: &Option<Rc<dyn std::any::Any>>| {
            match __a.clone().map(|__r| __r.downcast::<RefCell<i32>>()) {
                Some(Ok(__v)) => __v.as_pointer(),
                _ => Ptr::null(),
            }
        }))
        .is_null()
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
