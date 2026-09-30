extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Entry {
    #[offset(0)]
    pub name: Ptr<u8>,
    #[offset(8)]
    pub p: Ptr<i32>,
}
impl ByteRepr for Entry {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.name.to_bytes(&mut buf[0..8]);
        self.p.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            name: <Ptr<u8>>::from_bytes(&buf[0..8]),
            p: <Ptr<i32>>::from_bytes(&buf[8..16]),
        }
    }
}
thread_local!(
    pub static single_entry_0: Value<Entry> = Rc::new(RefCell::new(Entry {
        name: Ptr::<u8>::from_string_literal(b"alone"),
        p: Ptr::<i32>::null(),
    }));
);
thread_local!(
    pub static entries_1: Value<Box<[Entry]>> = Rc::new(RefCell::new(Box::new([
        Entry {
            name: Ptr::<u8>::from_string_literal(b"first"),
            p: Ptr::<i32>::null(),
        },
        Entry {
            name: Ptr::<u8>::from_string_literal(b"second"),
            p: Ptr::<i32>::null(),
        },
    ])));
);
thread_local!(
    pub static arr_of_pointers_2: Value<Box<[Ptr<u8>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::<u8>::null(),
        Ptr::<u8>::null(),
        Ptr::<u8>::null(),
    ])));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(({ (*single_entry_0.with(Value::clone).borrow()).p.clone() }).is_null());
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 2) {
        assert!(
            ({
                (*entries_1.with(Value::clone).borrow())[(*i.borrow()) as usize]
                    .p
                    .clone()
            })
            .is_null()
        );
        assert!(
            ({
                let __idx = (*i.borrow()) as usize;
                arr_of_pointers_2.with(|rc| rc.borrow()[__idx].clone())
            })
            .is_null()
        );
        (*i.borrow_mut()).prefix_inc();
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = single_entry_0.with(|_| ());
    let _ = entries_1.with(|_| ());
    let _ = arr_of_pointers_2.with(|_| ());
}
