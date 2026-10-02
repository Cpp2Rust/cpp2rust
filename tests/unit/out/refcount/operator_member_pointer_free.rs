extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Inner {
    #[offset(0)]
    pub x: i32,
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct S {
    #[offset(0)]
    #[byte_size(12)]
    pub data: Value<Box<[i32]>>,
    #[offset(12)]
    #[byte_size(4)]
    pub inner: Inner,
}
impl Clone for S {
    fn clone(&self) -> Self {
        Self {
            data: Rc::new(RefCell::new((*self.data.borrow()).clone())),
            inner: self.inner.clone(),
        }
    }
}
impl Default for S {
    fn default() -> Self {
        S {
            data: Rc::new(RefCell::new((0..3).map(|_| 0_i32).collect::<Box<[i32]>>())),
            inner: <Inner>::default(),
        }
    }
}
pub fn operator_deref_0(s: Ptr<S>) -> Ptr<Inner> {
    return field_ptr!(s, inner);
}
pub fn operator_addr_1(s: Ptr<S>) -> Ptr<i32> {
    return ((array_field_ptr!(s, data) as Ptr<i32>).offset((0) as isize));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S {
        data: Rc::new(RefCell::new(Box::new([1, 2, 3]))),
        inner: Inner { x: 9 },
    }));
    assert!(
        (({
            let _s: Ptr<S> = s.as_pointer();
            operator_deref_0(_s)
        })
        .with(|__s| (__s).x)
            == 9)
    );
    field!(
        ({
            let _s: Ptr<S> = s.as_pointer();
            operator_deref_0(_s)
        }),
        x
    )
    .write(10);
    assert!(({ (*s.borrow()).inner.x } == 10));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(
        ({
            let _s: Ptr<S> = s.as_pointer();
            operator_addr_1(_s)
        }),
    ));
    assert!((((*p.borrow()).read()) == 1));
    (*p.borrow()).write(5);
    assert!(((elem!((array_field_ptr!(s.as_pointer(), data) as Ptr::<i32>), 0).read()) == 5));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
