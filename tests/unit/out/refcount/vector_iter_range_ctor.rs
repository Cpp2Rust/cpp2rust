extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let src: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([1_u32, 2_u32, 3_u32])));
    let mut v1: Vec<u32> = {
        let __count = (src.as_pointer() as Ptr<u32>)
            .offset((3) as isize)
            .get_offset()
            - (src.as_pointer() as Ptr<u32>).get_offset();
        PtrValueIter::new(&(src.as_pointer() as Ptr<u32>), __count)
            .map(|item| u32::try_from(item).ok().unwrap())
            .collect::<Vec<_>>()
    };
    assert!((v1.len() == 3_usize));
    assert!(((v1[0_usize] == 1_u32) && (v1[1_usize] == 2_u32)) && (v1[2_usize] == 3_u32));
    let mut v2: Vec<u64> = {
        let __count = (src.as_pointer() as Ptr<u32>)
            .offset((3) as isize)
            .get_offset()
            - (src.as_pointer() as Ptr<u32>).get_offset();
        PtrValueIter::new(&(src.as_pointer() as Ptr<u32>), __count)
            .map(|item| u64::try_from(item).ok().unwrap())
            .collect::<Vec<_>>()
    };
    assert!((v2.len() == 3_usize));
    assert!(((v2[0_usize] == 1_u64) && (v2[1_usize] == 2_u64)) && (v2[2_usize] == 3_u64));
    let mut v3: Vec<i32> = {
        let __count = (src.as_pointer() as Ptr<u32>)
            .offset((3) as isize)
            .get_offset()
            - (src.as_pointer() as Ptr<u32>).get_offset();
        PtrValueIter::new(&(src.as_pointer() as Ptr<u32>), __count)
            .map(|item| i32::try_from(item).ok().unwrap())
            .collect::<Vec<_>>()
    };
    assert!((v3.len() == 3_usize));
    assert!(((v3[0_usize] == 1) && (v3[1_usize] == 2)) && (v3[2_usize] == 3));
    let src1: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([1_u32, 2_u32, 3_u32])));
    let mut v4: Vec<u32> = {
        let __count = (src1.as_pointer() as Ptr<u32>).to_end().get_offset()
            - (src1.as_pointer() as Ptr<u32>).get_offset();
        PtrValueIter::new(&(src1.as_pointer() as Ptr<u32>), __count).collect::<Vec<_>>()
    };
    assert!((v4.len() == 3_usize));
    assert!(((v4[0_usize] == 1_u32) && (v4[1_usize] == 2_u32)) && (v4[2_usize] == 3_u32));
    let buf: Value<Box<[u8]>> =
        Rc::new(RefCell::new(Box::new([10_u8, 20_u8, 30_u8, 40_u8, 50_u8])));
    let mut start: Ptr<u8> = (buf.as_pointer() as Ptr<u8>);
    let mut len: usize = 5_usize;
    let mut v5: Vec<u8> = {
        let __count = start.offset((len) as isize).get_offset() - start.get_offset();
        PtrValueIter::new(&start, __count).collect::<Vec<_>>()
    };
    assert!((v5.len() == 5_usize));
    assert!(((v5[0_usize] as i32) == 10) && ((v5[4_usize] as i32) == 50));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
