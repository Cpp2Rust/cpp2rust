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
    {
        let anon_0: Value<anon_0> = Rc::new(RefCell::new(<anon_0>::default()));
        anon_0::ai(anon_0.as_pointer()).write(11);
        assert!(((anon_0::ai(anon_0.as_pointer()).read()) == 11));
        anon_0::ai(anon_0.as_pointer()).write(1065353216);
        assert!(((anon_0::af(anon_0.as_pointer()).read()) == 1_f32));
    }
    return 0;
}
#[derive(VaArg, FnPtrArg, ByteRepr, DeepClone)]
#[byte_size(4)]
pub struct anon_0 {
    #[offset(0)]
    #[byte_size(4)]
    __bytes: Value<Box<[u8]>>,
}
impl anon_0 {
    pub fn ai(this: Ptr<Self>) -> Ptr<i32> {
        this.reinterpret_cast()
    }
    pub fn af(this: Ptr<Self>) -> Ptr<f32> {
        this.reinterpret_cast()
    }
}
impl Default for anon_0 {
    fn default() -> Self {
        anon_0 {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 4]))),
        }
    }
}
pub fn __cpp2rust_init_globals() {}
