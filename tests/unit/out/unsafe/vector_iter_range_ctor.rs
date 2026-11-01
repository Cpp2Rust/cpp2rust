extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct Degrees {
    pub value: i32,
}
impl Degrees {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { value: v };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_into(i64, to_i64)]
pub struct Reading {
    pub raw_: i32,
}
impl Reading {
    pub unsafe fn to_i64(&self) -> i64 {
        return ((self.raw_ as i64) * (10_i64));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut src: [u32; 3] = [1_u32, 2_u32, 3_u32];
    let mut v1: Vec<u32> = match ({ src.as_mut_ptr().offset((3) as isize) },) {
        (mut __tmp_0,) => {
            let mut __v = Vec::new();
            let mut __it = src.as_mut_ptr();
            while __it != __tmp_0 {
                __v.push((*__it.iter_deref()));
                __it.iter_inc();
            }
            __v
        }
    };
    assert!(((v1.len()) == (3_usize)));
    assert!(
        (((v1[(0_usize)]) == (1_u32)) && ((v1[(1_usize)]) == (2_u32)))
            && ((v1[(2_usize)]) == (3_u32))
    );
    let mut v2: Vec<u64> = match ({ src.as_mut_ptr().offset((3) as isize) },) {
        (mut __tmp_1,) => {
            let mut __v = Vec::new();
            let mut __it = src.as_mut_ptr();
            while __it != __tmp_1 {
                __v.push(((*__it.iter_deref()) as u64));
                __it.iter_inc();
            }
            __v
        }
    };
    assert!(((v2.len()) == (3_usize)));
    assert!(
        (((v2[(0_usize)]) == (1_u64)) && ((v2[(1_usize)]) == (2_u64)))
            && ((v2[(2_usize)]) == (3_u64))
    );
    let mut v3: Vec<i32> = match ({ src.as_mut_ptr().offset((3) as isize) },) {
        (mut __tmp_2,) => {
            let mut __v = Vec::new();
            let mut __it = src.as_mut_ptr();
            while __it != __tmp_2 {
                __v.push(((*__it.iter_deref()) as i32));
                __it.iter_inc();
            }
            __v
        }
    };
    assert!(((v3.len()) == (3_usize)));
    assert!((((v3[(0_usize)]) == (1)) && ((v3[(1_usize)]) == (2))) && ((v3[(2_usize)]) == (3)));
    let src1: [u32; 3] = [1_u32, 2_u32, 3_u32];
    let mut v4: Vec<u32> = match ({ src1.as_ptr().add(src1.len()) },) {
        (mut __tmp_3,) => {
            let mut __v = Vec::new();
            let mut __it = src1.as_ptr();
            while __it != __tmp_3 {
                __v.push((*__it.iter_deref()));
                __it.iter_inc();
            }
            __v
        }
    };
    assert!(((v4.len()) == (3_usize)));
    assert!(
        (((v4[(0_usize)]) == (1_u32)) && ((v4[(1_usize)]) == (2_u32)))
            && ((v4[(2_usize)]) == (3_u32))
    );
    let mut buf: [u8; 5] = [10_u8, 20_u8, 30_u8, 40_u8, 50_u8];
    let mut start: *const u8 = (buf.as_mut_ptr()).cast_const();
    let mut len: usize = 5_usize;
    let mut v5: Vec<u8> = match ({ start.offset((len) as isize) },) {
        (mut __tmp_4,) => {
            let mut __v = Vec::new();
            let mut __it = start;
            while __it != __tmp_4 {
                __v.push((*__it.iter_deref()));
                __it.iter_inc();
            }
            __v
        }
    };
    assert!(((v5.len()) == (5_usize)));
    assert!(((v5[(0_usize)] as i32) == (10)) && ((v5[(4_usize)] as i32) == (50)));
    let mut m: UnsafeMap<i32, (i32, i32)> = UnsafeMap::new();
    (*m.subscript_move(&mut 1, |__k| cpp_move_unsafe!(__k))) = 10;
    (*m.subscript_move(&mut 2, |__k| cpp_move_unsafe!(__k))) = 20;
    (*m.subscript_move(&mut 3, |__k| cpp_move_unsafe!(__k))) = 30;
    let mut v6: Vec<(i32, i32)> = match ({ m.begin() }, { m.end() }) {
        (mut __tmp_5, mut __tmp_6) => {
            let mut __v = Vec::new();
            let mut __it = (__tmp_5).clone();
            while __it != __tmp_6 {
                __v.push(<(i32, i32) as CppFromUnsafe<(i32, i32)>>::cpp_from(
                    __it.iter_deref(),
                ));
                __it.iter_inc();
            }
            __v
        }
    };
    assert!(((v6.len()) == (3_usize)));
    assert!(((v6[(0_usize)].0) == (1)) && ((v6[(0_usize)].1) == (10)));
    assert!(((v6[(2_usize)].0) == (3)) && ((v6[(2_usize)].1) == (30)));
    match (
        { <UnsafeRandomAccessIterator<(i32, i32)>>::from(v6.begin().iter_add(1_isize)) },
        { m.begin() },
        { m.end() },
    ) {
        (mut __tmp_7, mut __tmp_8, mut __tmp_9) => {
            let __off =
                __tmp_7.iter_distance(&UnsafeRandomAccessIterator::Vec(v6.as_mut_ptr())) as usize;
            let mut __tmp = Vec::new();
            let mut __it = (__tmp_8).clone();
            while __it != __tmp_9 {
                __tmp.push((*__it.iter_deref()).clone());
                __it.iter_inc();
            }
            v6.splice(__off..__off, __tmp);
            UnsafeRandomAccessIterator::Vec(v6.as_mut_ptr().add(__off))
        }
    };
    assert!(((v6.len()) == (6_usize)));
    assert!(
        (((v6[(1_usize)].0) == (1)) && ((v6[(3_usize)].0) == (3))) && ((v6[(4_usize)].0) == (2))
    );
    let mut ints: [i32; 3] = [4, 5, 6];
    let mut v7: Vec<Degrees> = match ({ ints.as_mut_ptr().offset((3) as isize) },) {
        (mut __tmp_10,) => {
            let mut __v = Vec::new();
            let mut __it = ints.as_mut_ptr();
            while __it != __tmp_10 {
                __v.push(<Degrees as CppFromUnsafe<i32>>::cpp_from(__it.iter_deref()));
                __it.iter_inc();
            }
            __v
        }
    };
    assert!(((v7[(1_usize)].value) == (5)));
    let mut readings: [Reading; 2] = [Reading { raw_: 4 }, Reading { raw_: 5 }];
    let mut v8: Vec<i64> = match ({ readings.as_mut_ptr().offset((2) as isize) },) {
        (mut __tmp_11,) => {
            let mut __v = Vec::new();
            let mut __it = readings.as_mut_ptr();
            while __it != __tmp_11 {
                __v.push(<i64 as CppFromUnsafe<Reading>>::cpp_from(__it.iter_deref()));
                __it.iter_inc();
            }
            __v
        }
    };
    assert!(((v8[(0_usize)]) == (40_i64)) && ((v8[(1_usize)]) == (50_i64)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
