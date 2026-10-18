extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(48)]
pub struct S {
    #[offset(0)]
    #[byte_size(48)]
    pub m: Map<i32, (Value<i32>, Value<i32>)>,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<S> = Rc::new(RefCell::new(<S>::default()));
    (field_ptr!(a.as_pointer(), m) as Ptr<Map<i32, (Value<i32>, Value<i32>)>>)
        .with_mut(|__m: &mut Map<i32, (Value<i32>, Value<i32>)>| {
            __m.get_or_insert_with(1.clone(), || {
                (
                    Rc::new(RefCell::new(1)),
                    Rc::new(RefCell::new(<i32>::default())),
                )
            })
            .borrow()
            .1
            .as_pointer()
        })
        .write(10);
    let b: Value<S> = Rc::new(RefCell::new((*a.borrow()).clone()));
    (field_ptr!(b.as_pointer(), m) as Ptr<Map<i32, (Value<i32>, Value<i32>)>>)
        .with_mut(|__m: &mut Map<i32, (Value<i32>, Value<i32>)>| {
            __m.get_or_insert_with(1.clone(), || {
                (
                    Rc::new(RefCell::new(1)),
                    Rc::new(RefCell::new(<i32>::default())),
                )
            })
            .borrow()
            .1
            .as_pointer()
        })
        .write(20);
    assert!(
        (((field_ptr!(a.as_pointer(), m) as Ptr<Map<i32, (Value<i32>, Value<i32>)>>)
            .with_mut(|__m: &mut Map<i32, (Value<i32>, Value<i32>)>| {
                __m.get_or_insert_with(1.clone(), || {
                    (
                        Rc::new(RefCell::new(1)),
                        Rc::new(RefCell::new(<i32>::default())),
                    )
                })
                .borrow()
                .1
                .as_pointer()
            })
            .read())
            == 10)
    );
    let x: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    (x.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<i32>)>>)
        .with_mut(|__m: &mut Map<i32, (Value<i32>, Value<i32>)>| {
            __m.get_or_insert_with(2.clone(), || {
                (
                    Rc::new(RefCell::new(2)),
                    Rc::new(RefCell::new(<i32>::default())),
                )
            })
            .borrow()
            .1
            .as_pointer()
        })
        .write(3);
    let y: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new((*x.borrow()).clone()));
    (y.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<i32>)>>)
        .with_mut(|__m: &mut Map<i32, (Value<i32>, Value<i32>)>| {
            __m.get_or_insert_with(2.clone(), || {
                (
                    Rc::new(RefCell::new(2)),
                    Rc::new(RefCell::new(<i32>::default())),
                )
            })
            .borrow()
            .1
            .as_pointer()
        })
        .write(4);
    assert!(
        (((x.as_pointer() as Ptr<Map<i32, (Value<i32>, Value<i32>)>>)
            .with_mut(|__m: &mut Map<i32, (Value<i32>, Value<i32>)>| {
                __m.get_or_insert_with(2.clone(), || {
                    (
                        Rc::new(RefCell::new(2)),
                        Rc::new(RefCell::new(<i32>::default())),
                    )
                })
                .borrow()
                .1
                .as_pointer()
            })
            .read())
            == 3)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
