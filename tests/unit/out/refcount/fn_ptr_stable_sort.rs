extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Item {
    pub key: Value<i32>,
    pub value: Value<i32>,
}
impl Clone for Item {
    fn clone(&self) -> Self {
        let mut this = Self {
            key: Rc::new(RefCell::new((*self.key.borrow()))),
            value: Rc::new(RefCell::new((*self.value.borrow()))),
        };
        this
    }
}
impl ByteRepr for Item {}
pub fn Compare_0(a: Ptr<Item>, b: Ptr<Item>) -> bool {
    return {
        let _lhs = (*(*a.upgrade().deref()).key.borrow());
        _lhs < (*(*b.upgrade().deref()).key.borrow())
    };
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<Vec<Item>> = Rc::new(RefCell::new(Vec::new()));
    (*v.borrow_mut()).push(Item {
        key: Rc::new(RefCell::new(3)),
        value: Rc::new(RefCell::new(30)),
    });
    (*v.borrow_mut()).push(Item {
        key: Rc::new(RefCell::new(1)),
        value: Rc::new(RefCell::new(10)),
    });
    (*v.borrow_mut()).push(Item {
        key: Rc::new(RefCell::new(2)),
        value: Rc::new(RefCell::new(20)),
    });
    (v.as_pointer() as Ptr<Item>).sort_with_cmp(
        (v.as_pointer() as Ptr<Item>).to_end().get_offset(),
        Compare_0,
    );
    assert!(
        ((*(*(v.as_pointer() as Ptr<Item>)
            .offset(0_u64 as isize)
            .upgrade()
            .deref())
        .key
        .borrow())
            == 1)
    );
    assert!(
        ((*(*(v.as_pointer() as Ptr<Item>)
            .offset(1_u64 as isize)
            .upgrade()
            .deref())
        .key
        .borrow())
            == 2)
    );
    assert!(
        ((*(*(v.as_pointer() as Ptr<Item>)
            .offset(2_u64 as isize)
            .upgrade()
            .deref())
        .key
        .borrow())
            == 3)
    );
    return 0;
}
