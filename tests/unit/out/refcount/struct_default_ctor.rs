extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct S {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: bool,
}
impl S {
    pub fn new() -> Self {
        Self { a: 11, b: true }
    }
}
impl Default for S {
    fn default() -> Self {
        { S::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Declared {
    #[offset(0)]
    pub v: i32,
}
impl Declared {}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(16)]
    pub items: Value<Box<[S]>>,
}
impl Default for Holder {
    fn default() -> Self {
        {
            Holder {
                items: Rc::new(RefCell::new(
                    (0..2).map(|_| <S>::default()).collect::<Box<[S]>>(),
                )),
            }
        }
    }
}
thread_local!(
    pub static kInit_0: Value<i32> = Rc::new(RefCell::new(3));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct FromStatic {
    #[offset(0)]
    pub v: i32,
}
impl Default for FromStatic {
    fn default() -> Self {
        {
            FromStatic {
                v: kInit_0.with(|rc| *rc.borrow()),
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut d: Ptr<Declared> = Ptr::<Declared>::null();
    assert!((d).is_null());
    let mut s: S = S::new();
    assert!((s.a == 11));
    assert!(((s.b as i32) == (true as i32)));
    let mut h: Ptr<Holder> = Ptr::alloc_array(
        (0..1_usize)
            .map(|_| <Holder>::default())
            .collect::<Box<[Holder]>>(),
    );
    assert!(
        ({
            (*elem!(
                (array_field_ptr!(h.offset((0) as isize), items) as Ptr<S>),
                1
            )
            .upgrade()
            .deref())
            .a
        } == 11)
    );
    h.delete();
    let mut fs: Ptr<FromStatic> = Ptr::alloc_array(
        (0..2_usize)
            .map(|_| <FromStatic>::default())
            .collect::<Box<[FromStatic]>>(),
    );
    assert!(({ (*elem!(fs, 1).upgrade().deref()).v } == 3));
    fs.delete();
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = kInit_0.with(|_| ());
}
