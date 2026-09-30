extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type widget_enum = u32;
pub const widget_enum_MODE_IDLE: widget_enum = 0;
pub const widget_enum_MODE_ACTIVE: widget_enum = 1;
pub const widget_enum_MODE_DONE: widget_enum = 2;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct widget {
    #[offset(0)]
    pub id: i32,
    #[offset(4)]
    pub mode: widget_enum,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct point_struct {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
#[derive(ByteRepr)]
#[byte_size(4)]
pub struct point {
    #[offset(0)]
    #[byte_size(4)]
    __bytes: Value<Box<[u8]>>,
}
impl point {
    pub fn whole(&self) -> Ptr<i32> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn half(&self) -> Ptr<i16> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
}
impl Clone for point {
    fn clone(&self) -> Self {
        point {
            __bytes: Rc::new(RefCell::new(self.__bytes.borrow().clone())),
        }
    }
}
impl Default for point {
    fn default() -> Self {
        point {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 4]))),
        }
    }
}
#[derive(ByteRepr)]
#[byte_size(4)]
pub struct slot_union {
    #[offset(0)]
    #[byte_size(4)]
    __bytes: Value<Box<[u8]>>,
}
impl slot_union {
    pub fn i(&self) -> Ptr<i32> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
    pub fn u(&self) -> Ptr<u32> {
        (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
    }
}
impl Clone for slot_union {
    fn clone(&self) -> Self {
        slot_union {
            __bytes: Rc::new(RefCell::new(self.__bytes.borrow().clone())),
        }
    }
}
impl Default for slot_union {
    fn default() -> Self {
        slot_union {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 4]))),
        }
    }
}
pub type slot = u32;
pub const slot_SLOT_A: slot = 0;
pub const slot_SLOT_B: slot = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Inner {
    #[offset(0)]
    pub tag_field: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(4)]
    pub field: Inner,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Inner_struct {
    #[offset(0)]
    pub typedef_field: i32,
}
pub fn is_active_0(w: Ptr<widget>) -> i32 {
    let w: Value<Ptr<widget>> = Rc::new(RefCell::new(w));
    return ((((*w.borrow()).with(|__s| __s.mode) as u32)
        == ((widget_enum_MODE_ACTIVE as i32) as u32)) as i32);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let w: Value<widget> = <Value<widget>>::default();
    (*w.borrow_mut()).id = 7;
    (*w.borrow_mut()).mode = widget_enum_MODE_ACTIVE;
    assert!((({ is_active_0((w.as_pointer()),) }) != 0));
    (*w.borrow_mut()).mode = widget_enum_MODE_DONE;
    assert!(
        (((({ (*w.borrow()).mode } as u32) == ((widget_enum_MODE_DONE as i32) as u32)) as i32)
            != 0)
    );
    let p: Value<point_struct> = <Value<point_struct>>::default();
    (*p.borrow_mut()).x = 3;
    (*p.borrow_mut()).y = 4;
    assert!((((({ (*p.borrow()).x } + { (*p.borrow()).y }) == 7) as i32) != 0));
    let up: Value<point> = <Value<point>>::default();
    (*up.borrow_mut()).whole().write(5);
    assert!((((((*up.borrow()).whole().read()) == 5) as i32) != 0));
    let b: Value<slot_union> = <Value<slot_union>>::default();
    (*b.borrow_mut()).i().write(9);
    assert!((((((*b.borrow()).i().read()) == 9) as i32) != 0));
    let e: Value<slot> = Rc::new(RefCell::new(slot_SLOT_B));
    assert!((((((*e.borrow()) as u32) == ((slot_SLOT_B as i32) as u32)) as i32) != 0));
    let inner_tag: Value<Inner> = <Value<Inner>>::default();
    (*inner_tag.borrow_mut()).tag_field = 11;
    assert!(((({ (*inner_tag.borrow()).tag_field } == 11) as i32) != 0));
    let inner_typedef: Value<Inner_struct> = <Value<Inner_struct>>::default();
    (*inner_typedef.borrow_mut()).typedef_field = 22;
    assert!(((({ (*inner_typedef.borrow()).typedef_field } == 22) as i32) != 0));
    let o: Value<Outer> = <Value<Outer>>::default();
    (*o.borrow_mut()).field.tag_field = 33;
    assert!(((({ (*o.borrow()).field.tag_field } == 33) as i32) != 0));
    assert!(((({ (*w.borrow()).id } == 7) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
