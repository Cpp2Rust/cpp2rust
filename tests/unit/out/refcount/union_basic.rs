extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, ByteRepr, DeepClone)]
#[byte_size(4)]
pub struct basic {
    #[offset(0)]
    #[byte_size(4)]
    __bytes: Value<Box<[u8]>>,
}
impl basic {
    pub fn i(this: Ptr<Self>) -> Ptr<i32> {
        this.reinterpret_cast()
    }
    pub fn f(this: Ptr<Self>) -> Ptr<f32> {
        this.reinterpret_cast()
    }
}
impl Default for basic {
    fn default() -> Self {
        basic {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 4]))),
        }
    }
}
#[derive(VaArg, FnPtrArg, ByteRepr, DeepClone)]
#[byte_size(1)]
pub struct empty {
    #[offset(0)]
    #[byte_size(1)]
    __bytes: Value<Box<[u8]>>,
}
impl empty {}
impl Default for empty {
    fn default() -> Self {
        empty {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 1]))),
        }
    }
}
#[derive(VaArg, FnPtrArg, ByteRepr, DeepClone)]
#[byte_size(8)]
pub struct underscores {
    #[offset(0)]
    #[byte_size(8)]
    __bytes: Value<Box<[u8]>>,
}
impl underscores {
    pub fn __(this: Ptr<Self>) -> Ptr<i32> {
        this.reinterpret_cast()
    }
    pub fn ___(this: Ptr<Self>) -> Ptr<f64> {
        this.reinterpret_cast()
    }
}
impl Default for underscores {
    fn default() -> Self {
        underscores {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 8]))),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let u: Value<basic> = Rc::new(RefCell::new(<basic>::default()));
    let e: Value<empty> = Rc::new(RefCell::new(<empty>::default()));
    &(*e.borrow_mut());
    basic::i(u.as_pointer()).write(42);
    assert!(((basic::i(u.as_pointer()).read()) == 42));
    basic::f(u.as_pointer()).write(3.14_f32);
    assert!(((basic::f(u.as_pointer()).read()) == 3.14_f32));
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new((0..4).map(|_| 0_u8).collect::<Box<[u8]>>()));
    {
        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>)
            .to_any()
            .memset((0) as u8, ::std::mem::size_of::<[u8; 4]>() as usize);
        ((buf.as_pointer() as Ptr<u8>) as Ptr<u8>).to_any()
    };
    let mut ru: Ptr<basic> = (buf.as_pointer() as Ptr<u8>).reinterpret_cast::<basic>();
    let mut pi: Ptr<i32> = (basic::i(ru.clone())).clone();
    let mut pf: Ptr<f32> = (basic::f(ru.clone())).clone();
    basic::i(ru.clone()).write(7);
    assert!(((pi.read()) == 7));
    pi.write(1065353216);
    assert!(((basic::i(ru.clone()).read()) == 1065353216));
    assert!(((pf.read()) == 1_f32));
    assert!((((*buf.borrow())[(3) as usize] as i32) == 63));
    let us: Value<underscores> = Rc::new(RefCell::new(<underscores>::default()));
    underscores::__(us.as_pointer()).write(5);
    assert!(((underscores::__(us.as_pointer()).read()) == 5));
    underscores::___(us.as_pointer()).write(2.5_f64);
    assert!(((underscores::___(us.as_pointer()).read()) == 2.5_f64));
    let mut __: i32 = 1;
    let mut ___: i32 = 2;
    assert!(((__ + ___) == 3));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
