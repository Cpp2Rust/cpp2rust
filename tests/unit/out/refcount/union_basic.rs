extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub struct basic {
    __bytes: Value<Box<[u8]>>,
}
impl basic {
    pub fn i(&self) -> Ptr<i32> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn f(&self) -> Ptr<f32> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
}
impl Clone for basic {
    fn clone(&self) -> Self {
        basic {
            __bytes: Rc::new(RefCell::new(self.__bytes.borrow().clone())),
        }
    }
}
impl Default for basic {
    fn default() -> Self {
        basic {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 4]))),
        }
    }
}
impl ByteRepr for basic {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        buf.copy_from_slice(&self.__bytes.borrow());
    }
    fn from_bytes(buf: &[u8]) -> Self {
        basic {
            __bytes: Rc::new(RefCell::new(Box::from(buf))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let u: Value<basic> = <Value<basic>>::default();
    (*u.borrow_mut()).i().write(42);
    assert!((((((*u.borrow()).i().read()) == 42) as i32) != 0));
    (*u.borrow_mut()).f().write(3.140000105E+0);
    assert!((((((*u.borrow()).f().read()) == 3.140000105E+0) as i32) != 0));
    return 0;
}
