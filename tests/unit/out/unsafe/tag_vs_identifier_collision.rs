extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
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
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct widget {
    pub id: i32,
    pub mode: widget_enum,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct point_struct {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union point {
    pub whole: i32,
    pub half: i16,
}
impl Default for point {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union slot_union {
    pub i: i32,
    pub u: u32,
}
impl Default for slot_union {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
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
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Inner {
    pub tag_field: i32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Outer {
    pub field: Inner,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Inner_struct {
    pub typedef_field: i32,
}
pub unsafe fn is_active_0(mut w: *mut widget) -> i32 {
    return ((((*w).mode as u32) == ((widget_enum::MODE_ACTIVE as i32) as u32)) as i32);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut w: widget = <widget>::default();
    w.id = 7;
    w.mode = widget_enum::MODE_ACTIVE;
    assert!(((unsafe { is_active_0((&mut w as *mut widget),) }) != 0));
    w.mode = widget_enum::MODE_DONE;
    assert!(((((w.mode as u32) == ((widget_enum::MODE_DONE as i32) as u32)) as i32) != 0));
    let mut p: point_struct = <point_struct>::default();
    p.x = 3;
    p.y = 4;
    assert!((((((p.x) + (p.y)) == (7)) as i32) != 0));
    let mut up: point = <point>::default();
    up.whole = 5;
    assert!(((((up.whole) == (5)) as i32) != 0));
    let mut b: slot_union = <slot_union>::default();
    b.i = 9;
    assert!(((((b.i) == (9)) as i32) != 0));
    let mut e: slot = slot::SLOT_B;
    assert!(((((e as u32) == ((slot::SLOT_B as i32) as u32)) as i32) != 0));
    let mut inner_tag: Inner = <Inner>::default();
    inner_tag.tag_field = 11;
    assert!(((((inner_tag.tag_field) == (11)) as i32) != 0));
    let mut inner_typedef: Inner_struct = <Inner_struct>::default();
    inner_typedef.typedef_field = 22;
    assert!(((((inner_typedef.typedef_field) == (22)) as i32) != 0));
    let mut o: Outer = <Outer>::default();
    o.field.tag_field = 33;
    assert!(((((o.field.tag_field) == (33)) as i32) != 0));
    return w.id;
}
