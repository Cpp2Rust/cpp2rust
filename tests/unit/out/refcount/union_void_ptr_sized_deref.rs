extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type Width_enum = u32;
pub const Width_enum_W_64: Width_enum = 0;
pub const Width_enum_W_32: Width_enum = 1;
pub const Width_enum_W_16: Width_enum = 2;
#[derive(VaArg, FnPtrArg, ByteRepr, DeepClone)]
#[byte_size(8)]
pub struct anon_0 {
    #[offset(0)]
    #[byte_size(8)]
    __bytes: Value<Box<[u8]>>,
}
impl anon_0 {
    pub fn text(&self) -> Ptr<Ptr<i8>> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn handle(&self) -> Ptr<AnyPtr> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn signed_n(&self) -> Ptr<i64> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn f(&self) -> Ptr<f64> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
}
impl Default for anon_0 {
    fn default() -> Self {
        anon_0 {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 8]))),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Sink {
    #[offset(0)]
    pub width: Width_enum,
    #[offset(8)]
    #[byte_size(8)]
    pub out: anon_0,
}
pub fn write_count_1(mut s: Ptr<Sink>, mut count: i64) {
    'switch: {
        match { (s.with(|__s| __s.width) as u32) } {
            __v if __v == ((Width_enum_W_64 as i32) as u32) => {
                ((*s.upgrade().deref()).out.handle().read())
                    .reinterpret_cast::<i64>()
                    .write(count);
                break 'switch;
            }
            __v if __v == ((Width_enum_W_32 as i32) as u32) => {
                ((*s.upgrade().deref()).out.handle().read())
                    .reinterpret_cast::<i32>()
                    .write((count as i32));
                break 'switch;
            }
            __v if __v == ((Width_enum_W_16 as i32) as u32) => {
                ((*s.upgrade().deref()).out.handle().read())
                    .reinterpret_cast::<i16>()
                    .write((count as i16));
                break 'switch;
            }
            _ => {}
        }
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let buf64: Value<i64> = Rc::new(RefCell::new(0_i64));
    let buf32: Value<i32> = Rc::new(RefCell::new(0));
    let buf16: Value<i16> = Rc::new(RefCell::new(0_i16));
    let s: Value<Sink> = <Value<Sink>>::default();
    (*s.borrow_mut()).width = Width_enum_W_64;
    (*s.borrow_mut())
        .out
        .handle()
        .write(((buf64.as_pointer()) as Ptr<i64>).to_any());
    ({ write_count_1((s.as_pointer()), 1234605616436508552_i64) });
    assert!(((((*buf64.borrow()) == 1234605616436508552_i64) as i32) != 0));
    (*s.borrow_mut()).width = Width_enum_W_32;
    (*s.borrow_mut())
        .out
        .handle()
        .write(((buf32.as_pointer()) as Ptr<i32>).to_any());
    ({ write_count_1((s.as_pointer()), 305419896_i64) });
    assert!(((((*buf32.borrow()) == 305419896) as i32) != 0));
    (*s.borrow_mut()).width = Width_enum_W_16;
    (*s.borrow_mut())
        .out
        .handle()
        .write(((buf16.as_pointer()) as Ptr<i16>).to_any());
    ({ write_count_1((s.as_pointer()), 4660_i64) });
    assert!((((((*buf16.borrow()) as i32) == 4660) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
