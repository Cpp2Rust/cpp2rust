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
    let a: Value<Option<Rc<dyn std::any::Any>>> = Rc::new(RefCell::new(
        ({
            let ___args_0: Value<i32> = Rc::new(RefCell::new(3));
            let ___args_1: Value<i32> = Rc::new(RefCell::new(4));
            make_any_0(___args_0.as_pointer(), ___args_1.as_pointer())
        }),
    ));
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
    let q: Ptr<Point> = ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(5));
        let ___args_1: Value<i32> = Rc::new(RefCell::new(6));
        (*a.borrow()).emplace(___args_0.as_pointer(), ___args_1.as_pointer())
    });
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
    let b: Value<Option<Rc<dyn std::any::Any>>> = Rc::new(RefCell::new(
        ({
            let ___args: Value<i32> = Rc::new(RefCell::new(7));
            make_any_1(___args.as_pointer())
        }),
    ));
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
    b.as_pointer()
        .with_mut(|__a: &mut Option<Rc<dyn std::any::Any>>| *__a = None);
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
