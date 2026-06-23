extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<i64> = Rc::new(RefCell::new((-1_i32 as i64)));
    #[derive(Clone)]
    pub struct anon_0 {
        __store: libcc2rs::UnionStorage,
    }
    impl anon_0 {
        pub fn as_unsigned(&self) -> Ptr<Ptr<u64>> {
            self.__store.reinterpret(0)
        }
        pub fn as_signed(&self) -> Ptr<Ptr<i64>> {
            self.__store.reinterpret(0)
        }
    }
    impl Default for anon_0 {
        fn default() -> Self {
            anon_0 {
                __store: libcc2rs::UnionStorage::new(8),
            }
        }
    }
    impl ByteRepr for anon_0 {};
    let pp: Value<anon_0> = <Value<anon_0>>::default();
    (*pp.borrow_mut()).as_signed().write((x.as_pointer()));
    ((*pp.borrow()).as_unsigned().read()).write(42_u64);
    assert!(((((*x.borrow()) == 42_i64) as i32) != 0));
    return 0;
}
