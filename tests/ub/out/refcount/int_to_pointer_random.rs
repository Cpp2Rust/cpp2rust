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
    pub struct anon_0 {
        __bytes: Value<Box<[u8]>>,
    }
    impl anon_0 {
        pub fn p(&self) -> Ptr<Ptr<i32>> {
            (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
        }
        pub fn bits(&self) -> Ptr<u64> {
            (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
        }
    }
    impl Clone for anon_0 {
        fn clone(&self) -> Self {
            anon_0 {
                __bytes: Rc::new(RefCell::new(self.__bytes.borrow().clone())),
            }
        }
    }
    impl Default for anon_0 {
        fn default() -> Self {
            anon_0 {
                __bytes: Rc::new(RefCell::new(Box::from([0u8; 8]))),
            }
        }
    }
    impl ByteRepr for anon_0 {
        fn byte_size() -> usize {
            8
        }
        fn to_bytes(&self, buf: &mut [u8]) {
            buf.copy_from_slice(&self.__bytes.borrow());
        }
        fn from_bytes(buf: &[u8]) -> Self {
            anon_0 {
                __bytes: Rc::new(RefCell::new(Box::from(buf))),
            }
        }
    };
    let u: Value<anon_0> = <Value<anon_0>>::default();
    (*u.borrow_mut()).bits().write(3735928559_u64);
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(((*u.borrow()).p().read()).clone()));
    return if (((((*p.borrow()).read()) == 0) as i32) != 0) {
        0
    } else {
        1
    };
}
