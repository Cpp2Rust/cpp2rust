extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum widget_enum {
    #[default]
    MODE_IDLE = 0,
    MODE_ACTIVE = 1,
    MODE_DONE = 2,
}
impl From<i32> for widget_enum {
    fn from(n: i32) -> widget_enum {
        match n {
            0 => widget_enum::MODE_IDLE,
            1 => widget_enum::MODE_ACTIVE,
            2 => widget_enum::MODE_DONE,
            _ => panic!("invalid widget_enum value: {}", n),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(widget_enum);
impl ByteRepr for widget_enum {
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self as i32).to_bytes(buf);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        <widget_enum>::from(i32::from_bytes(buf))
    }
}
#[derive(Default)]
pub struct widget {
    pub id: Value<i32>,
    pub mode: Value<widget_enum>,
}
impl Clone for widget {
    fn clone(&self) -> Self {
        Self {
            id: Rc::new(RefCell::new((*self.id.borrow()).clone())),
            mode: Rc::new(RefCell::new((*self.mode.borrow()).clone())),
        }
    }
}
impl ByteRepr for widget {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.id.borrow()).to_bytes(&mut buf[0..4]);
        (*self.mode.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            id: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            mode: Rc::new(RefCell::new(<widget_enum>::from_bytes(&buf[4..8]))),
        }
    }
}
#[derive(Default)]
pub struct point_struct {
    pub x: Value<i32>,
    pub y: Value<i32>,
}
impl Clone for point_struct {
    fn clone(&self) -> Self {
        Self {
            x: Rc::new(RefCell::new((*self.x.borrow()).clone())),
            y: Rc::new(RefCell::new((*self.y.borrow()).clone())),
        }
    }
}
impl ByteRepr for point_struct {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x.borrow()).to_bytes(&mut buf[0..4]);
        (*self.y.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            y: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
pub struct point {
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
impl ByteRepr for point {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        buf.copy_from_slice(&self.__bytes.borrow());
    }
    fn from_bytes(buf: &[u8]) -> Self {
        point {
            __bytes: Rc::new(RefCell::new(Box::from(buf))),
        }
    }
}
pub struct slot_union {
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
impl ByteRepr for slot_union {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        buf.copy_from_slice(&self.__bytes.borrow());
    }
    fn from_bytes(buf: &[u8]) -> Self {
        slot_union {
            __bytes: Rc::new(RefCell::new(Box::from(buf))),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum slot {
    #[default]
    SLOT_A = 0,
    SLOT_B = 1,
}
impl From<i32> for slot {
    fn from(n: i32) -> slot {
        match n {
            0 => slot::SLOT_A,
            1 => slot::SLOT_B,
            _ => panic!("invalid slot value: {}", n),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(slot);
impl ByteRepr for slot {
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self as i32).to_bytes(buf);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        <slot>::from(i32::from_bytes(buf))
    }
}
#[derive(Default)]
pub struct Inner {
    pub tag_field: Value<i32>,
}
impl Clone for Inner {
    fn clone(&self) -> Self {
        Self {
            tag_field: Rc::new(RefCell::new((*self.tag_field.borrow()).clone())),
        }
    }
}
impl ByteRepr for Inner {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.tag_field.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            tag_field: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct Outer {
    pub field: Value<Inner>,
}
impl Clone for Outer {
    fn clone(&self) -> Self {
        Self {
            field: Rc::new(RefCell::new((*self.field.borrow()).clone())),
        }
    }
}
impl ByteRepr for Outer {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.field.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            field: Rc::new(RefCell::new(<Inner>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct Inner_struct {
    pub typedef_field: Value<i32>,
}
impl Clone for Inner_struct {
    fn clone(&self) -> Self {
        Self {
            typedef_field: Rc::new(RefCell::new((*self.typedef_field.borrow()).clone())),
        }
    }
}
impl ByteRepr for Inner_struct {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.typedef_field.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            typedef_field: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn is_active_0(w: Ptr<widget>) -> i32 {
    let w: Value<Ptr<widget>> = Rc::new(RefCell::new(w));
    return (({
        let _lhs = ((*(*(*w.borrow()).upgrade().deref()).mode.borrow()) as u32).clone();
        _lhs == ((widget_enum::MODE_ACTIVE as i32) as u32)
    }) as i32);
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let w: Value<widget> = <Value<widget>>::default();
    (*(*w.borrow()).id.borrow_mut()) = 7;
    (*(*w.borrow()).mode.borrow_mut()) = widget_enum::MODE_ACTIVE;
    assert!((({ is_active_0((w.as_pointer()),) }) != 0));
    (*(*w.borrow()).mode.borrow_mut()) = widget_enum::MODE_DONE;
    assert!(
        (((((*(*w.borrow()).mode.borrow()) as u32) == ((widget_enum::MODE_DONE as i32) as u32))
            as i32)
            != 0)
    );
    let p: Value<point_struct> = <Value<point_struct>>::default();
    (*(*p.borrow()).x.borrow_mut()) = 3;
    (*(*p.borrow()).y.borrow_mut()) = 4;
    assert!((((((*(*p.borrow()).x.borrow()) + (*(*p.borrow()).y.borrow())) == 7) as i32) != 0));
    let up: Value<point> = <Value<point>>::default();
    (*up.borrow_mut()).whole().write(5);
    assert!((((((*up.borrow()).whole().read()) == 5) as i32) != 0));
    let b: Value<slot_union> = <Value<slot_union>>::default();
    (*b.borrow_mut()).i().write(9);
    assert!((((((*b.borrow()).i().read()) == 9) as i32) != 0));
    let e: Value<slot> = Rc::new(RefCell::new(slot::SLOT_B));
    assert!((((((*e.borrow()) as u32) == ((slot::SLOT_B as i32) as u32)) as i32) != 0));
    let inner_tag: Value<Inner> = <Value<Inner>>::default();
    (*(*inner_tag.borrow()).tag_field.borrow_mut()) = 11;
    assert!(((((*(*inner_tag.borrow()).tag_field.borrow()) == 11) as i32) != 0));
    let inner_typedef: Value<Inner_struct> = <Value<Inner_struct>>::default();
    (*(*inner_typedef.borrow()).typedef_field.borrow_mut()) = 22;
    assert!(((((*(*inner_typedef.borrow()).typedef_field.borrow()) == 22) as i32) != 0));
    let o: Value<Outer> = <Value<Outer>>::default();
    (*(*(*o.borrow()).field.borrow()).tag_field.borrow_mut()) = 33;
    assert!(((((*(*(*o.borrow()).field.borrow()).tag_field.borrow()) == 33) as i32) != 0));
    return (*(*w.borrow()).id.borrow());
}
