extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(16)]
pub struct Element {
    #[offset(0)]
    pub id: i64,
    #[offset(8)]
    pub value: i64,
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let mut a: Ptr<Element> = Ptr::alloc_array(Box::new([
        Element {
            id: 1_i64,
            value: 10_i64,
        },
        Element {
            id: 2_i64,
            value: 20_i64,
        },
        Element {
            id: 3_i64,
            value: 30_i64,
        },
    ]));
    {
        let __p = a;
        for __i in (0..__p.len()).rev() {
            __p.offset(__i as isize).destructor();
        }
        __p.delete();
    };
    printf!(b"\n");
    return 0;
}
pub trait ElementImpl {
    fn destructor(&self);
}
impl ElementImpl for Ptr<Element> {
    fn destructor(&self) {
        printf!(
            b"%ld:%ld ",
            (*self).with(|__s| __s.id),
            (*self).with(|__s| __s.value)
        );
    }
}
pub fn __cpp2rust_init_globals() {}
