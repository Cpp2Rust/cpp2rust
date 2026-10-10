extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Entry {
    #[offset(0)]
    #[byte_size(8)]
    pub name: Ptr<i8>,
    #[offset(8)]
    #[byte_size(8)]
    pub p: Ptr<i32>,
}
thread_local!(
    pub static single_entry_0: Value<Entry> = Rc::new(RefCell::new(Entry {
        name: Ptr::<i8>::from_string_literal(b"alone"),
        p: Ptr::<i32>::null(),
    }));
);
thread_local!(
    pub static entries_1: Value<Box<[Entry]>> = Rc::new(RefCell::new(Box::new([
        Entry {
            name: Ptr::<i8>::from_string_literal(b"first"),
            p: Ptr::<i32>::null(),
        },
        Entry {
            name: Ptr::<i8>::from_string_literal(b"second"),
            p: Ptr::<i32>::null(),
        },
    ])));
);
thread_local!(
    pub static arr_of_pointers_2: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::<i8>::null(),
        Ptr::<i8>::null(),
        Ptr::<i8>::null(),
    ])));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(({ (*single_entry_0.with(Value::clone).borrow()).p.clone() }).is_null());
    let mut i: i32 = 0;
    'loop_: while (i < 2) {
        assert!(
            ({
                (*entries_1.with(Value::clone).borrow())[(i) as usize]
                    .p
                    .clone()
            })
            .is_null()
        );
        assert!(
            ({
                let __idx = (i) as usize;
                arr_of_pointers_2.with(|rc| rc.borrow()[__idx].clone())
            })
            .is_null()
        );
        i.prefix_inc();
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = single_entry_0.with(|_| ());
    let _ = entries_1.with(|_| ());
    let _ = arr_of_pointers_2.with(|_| ());
}
