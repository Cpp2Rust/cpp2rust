extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Item {
    #[offset(0)]
    pub key: i32,
    #[offset(4)]
    pub value: i32,
}
pub fn Compare_0(a: Ptr<Item>, b: Ptr<Item>) -> bool {
    return ({ a.with(|__s| __s.key) } < { b.with(|__s| __s.key) });
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
    match ({ v.decay().begin() },) {
        (mut __tmp_0,) => {
            let __n = v.decay().end().iter_distance(&__tmp_0);
            let mut __order: Vec<isize> = (0..__n).collect();
            __order.sort_by(|&__x, &__y| {
                let __a = __tmp_0.iter_at(__x);
                let __b = __tmp_0.iter_at(__y);
                if Compare_0.call(__a.clone(), __b.clone()) {
                    std::cmp::Ordering::Less
                } else if Compare_0.call(__b, __a) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });
            reorder(__order, |__i| __tmp_0.iter_at(__i));
        }
    };
    assert!(({ ({ (*v.borrow())[0_usize].key } == 1) }));
    assert!(({ ({ (*v.borrow())[1_usize].key } == 2) }));
    assert!(({ ({ (*v.borrow())[2_usize].key } == 3) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
