extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Item {
    pub key: i32,
    pub value: i32,
}
pub unsafe fn Compare_0(a: *const Item, b: *const Item) -> bool {
    return (((*a).key) < ((*b).key));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v: Vec<Item> = Vec::new();
    v.push(Item { key: 3, value: 30 });
    v.push(Item { key: 1, value: 10 });
    v.push(Item { key: 2, value: 20 });
    {
        let len = v.as_mut_ptr().add(v.len()).offset_from(v.as_mut_ptr()) as usize;
        ::std::slice::from_raw_parts_mut(v.as_mut_ptr(), len).sort_by(|x, y| {
            if (Compare_0)(x, y) {
                std::cmp::Ordering::Less
            } else if (Compare_0)(y, x) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        })
    };
    assert!(((v[(0_u64) as usize].key) == (1)));
    assert!(((v[(1_u64) as usize].key) == (2)));
    assert!(((v[(2_u64) as usize].key) == (3)));
    return 0;
}
