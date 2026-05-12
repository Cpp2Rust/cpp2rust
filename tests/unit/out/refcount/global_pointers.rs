extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Entry {
    pub name: Value<Ptr<u8>>,
    pub p: Value<Ptr<i32>>,
}
impl Clone for Entry {
    fn clone(&self) -> Self {
        let mut this = Self {
            name: Rc::new(RefCell::new((*self.name.borrow()).clone())),
            p: Rc::new(RefCell::new((*self.p.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for Entry {}
thread_local!(
    pub static single_entry: Value<Entry> = Rc::new(RefCell::new(Entry {
        name: Rc::new(RefCell::new(Ptr::from_string_literal("alone"))),
        p: Rc::new(RefCell::new(Ptr::<i32>::null())),
    }));
);
thread_local!(
    pub static entries: Value<Box<[Entry]>> = Rc::new(RefCell::new(Box::new([
        Entry {
            name: Rc::new(RefCell::new(Ptr::from_string_literal("first"))),
            p: Rc::new(RefCell::new(Ptr::<i32>::null())),
        },
        Entry {
            name: Rc::new(RefCell::new(Ptr::from_string_literal("second"))),
            p: Rc::new(RefCell::new(Ptr::<i32>::null())),
        },
    ])));
);
thread_local!(
    pub static arr_of_pointers: Value<Box<[Ptr<u8>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::<u8>::null(),
        Ptr::<u8>::null(),
        Ptr::<u8>::null(),
    ])));
);
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((*(*single_entry.with(Value::clone).borrow()).p.borrow()).is_null());
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 2) {
        assert!(
            (*(*entries.with(Value::clone).borrow())[(*i.borrow()) as usize]
                .p
                .borrow())
            .is_null()
        );
        assert!(((*arr_of_pointers.with(Value::clone).borrow())[(*i.borrow()) as usize]).is_null());
        (*i.borrow_mut()).prefix_inc();
    }
    return 0;
}
