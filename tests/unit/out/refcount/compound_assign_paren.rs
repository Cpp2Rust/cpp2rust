extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Item {
    #[offset(0)]
    pub flags: u8,
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let item: Value<Item> = Rc::new(RefCell::new(Item { flags: 0_u8 }));
    let mut ptr: Ptr<Item> = (item.as_pointer());
    match ({ (1 << 0) }, &(field!((ptr), flags))) {
        (__tmp_0, __tmp_1) => __tmp_1.with_mut(|__tmp_1| {
            (*__tmp_1) = (((*__tmp_1) as i32) | (__tmp_0 as i32)) as u8;
        }),
    };
    match ({ (1 << 1) }, &(field!((ptr), flags))) {
        (__tmp_2, __tmp_3) => __tmp_3.with_mut(|__tmp_3| {
            (*__tmp_3) = (((*__tmp_3) as i32) | (__tmp_2 as i32)) as u8;
        }),
    };
    assert!(((ptr.with(|__s| __s.flags) as i32) == 3));
    match ({ ((!(1 << 0) as u8) as i32) }, &(field!((ptr), flags))) {
        (__tmp_4, __tmp_5) => __tmp_5.with_mut(|__tmp_5| {
            (*__tmp_5) = (((*__tmp_5) as i32) & (__tmp_4 as i32)) as u8;
        }),
    };
    assert!(((ptr.with(|__s| __s.flags) as i32) == 2));
    let mut bits: [u8; 4] = [0_u8; 4];
    match (
        { (((1 << ((5) & 7)) as u8) as i32) },
        &mut ((bits)[((5) / 8) as usize]),
    ) {
        (__tmp_6, __tmp_7) => {
            (*__tmp_7) = (((*__tmp_7) as i32) | (__tmp_6 as i32)) as u8;
        }
    };
    match (
        { (((1 << ((13) & 7)) as u8) as i32) },
        &mut ((bits)[((13) / 8) as usize]),
    ) {
        (__tmp_8, __tmp_9) => {
            (*__tmp_9) = (((*__tmp_9) as i32) | (__tmp_8 as i32)) as u8;
        }
    };
    assert!(((bits[(0) as usize] as i32) == 32));
    assert!(((bits[(1) as usize] as i32) == 32));
    assert!(((bits[(2) as usize] as i32) == 0));
    if ((ptr.with(|__s| __s.flags) as i32) != 0) {
        match ({ ((!(1 << 1) as u8) as i32) }, &(field!((ptr), flags))) {
            (__tmp_10, __tmp_11) => __tmp_11.with_mut(|__tmp_11| {
                (*__tmp_11) = (((*__tmp_11) as i32) & (__tmp_10 as i32)) as u8;
            }),
        };
    }
    assert!(((ptr.with(|__s| __s.flags) as i32) == 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
