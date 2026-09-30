extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Item {
    #[offset(0)]
    pub key: i32,
    #[offset(4)]
    pub value: i32,
}
impl ByteRepr for Item {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.key.to_bytes(&mut buf[0..4]);
        self.value.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            key: <i32>::from_bytes(&buf[0..4]),
            value: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
pub fn Compare_0(a: Ptr<Item>, b: Ptr<Item>) -> bool {
    return {
        let _lhs = a.with(|__s| __s.key);
        _lhs < b.with(|__s| __s.key)
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<Vec<Item>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = Item { key: 3, value: 30 };
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = Item { key: 1, value: 10 };
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = Item { key: 2, value: 20 };
        (*v.borrow_mut()).push(__a1)
    };
    (v.as_pointer() as Ptr<Item>).sort_with_cmp(
        (v.as_pointer() as Ptr<Item>).to_end().get_offset(),
        |x, y| Compare_0.call(x, y),
    );
    assert!(
        ({
            (*(v.as_pointer() as Ptr<Item>)
                .offset(0_usize)
                .upgrade()
                .deref())
            .key
        } == 1)
    );
    assert!(
        ({
            (*(v.as_pointer() as Ptr<Item>)
                .offset(1_usize)
                .upgrade()
                .deref())
            .key
        } == 2)
    );
    assert!(
        ({
            (*(v.as_pointer() as Ptr<Item>)
                .offset(2_usize)
                .upgrade()
                .deref())
            .key
        } == 3)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
