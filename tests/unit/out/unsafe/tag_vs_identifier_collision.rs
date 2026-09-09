extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type widget_enum = u32;
pub const widget_enum_MODE_IDLE: widget_enum = 0;
pub const widget_enum_MODE_ACTIVE: widget_enum = 1;
pub const widget_enum_MODE_DONE: widget_enum = 2;
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
pub type slot = u32;
pub const slot_SLOT_A: slot = 0;
pub const slot_SLOT_B: slot = 1;
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
    return ((((*w).mode as u32) == ((widget_enum_MODE_ACTIVE as i32) as u32)) as i32);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut w: widget = <widget>::default();
    w.id = 7;
    w.mode = widget_enum_MODE_ACTIVE;
    assert!(((unsafe { is_active_0((&mut w as *mut widget),) }) != 0));
    w.mode = widget_enum_MODE_DONE;
    assert!(((((w.mode as u32) == ((widget_enum_MODE_DONE as i32) as u32)) as i32) != 0));
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
    let mut e: slot = slot_SLOT_B;
    assert!(((((e as u32) == ((slot_SLOT_B as i32) as u32)) as i32) != 0));
    let mut inner_tag: Inner = <Inner>::default();
    inner_tag.tag_field = 11;
    assert!(((((inner_tag.tag_field) == (11)) as i32) != 0));
    let mut inner_typedef: Inner_struct = <Inner_struct>::default();
    inner_typedef.typedef_field = 22;
    assert!(((((inner_typedef.typedef_field) == (22)) as i32) != 0));
    let mut o: Outer = <Outer>::default();
    o.field.tag_field = 33;
    assert!(((((o.field.tag_field) == (33)) as i32) != 0));
    assert!(((((w.id) == (7)) as i32) != 0));
    return 0;
}
