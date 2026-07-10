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
pub struct TestAllocator_int_ {}
impl TestAllocator_int_ {
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut i32 {
        return Box::leak((0..n).map(|_| 0_i32).collect::<Box<[i32]>>()).as_mut_ptr();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut i32, _: usize) {
        ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
            p,
            libcc2rs::malloc_usable_size(p as *mut ::libc::c_void) / ::std::mem::size_of::<i32>(),
        )));
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct TestAllocator_double_ {}
impl TestAllocator_double_ {
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut f64 {
        return Box::leak((0..n).map(|_| 0.0_f64).collect::<Box<[f64]>>()).as_mut_ptr();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut f64, _: usize) {
        ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
            p,
            libcc2rs::malloc_usable_size(p as *mut ::libc::c_void) / ::std::mem::size_of::<f64>(),
        )));
    }
}
pub unsafe fn copy_0(mut copy_vector: Vec<i32>) {}
pub unsafe fn fn_1(v: *mut Vec<i32>, mut v3: Vec<i32>) {
    (*v).push(20);
    let mut x: i32 = 0_i32;
    let mut v4: *mut Vec<i32> = (&mut v3 as *mut Vec<i32>);
    let mut v2: Vec<i32> = Vec::new();
    v2.push(0);
    v2.push(1);
    v2.push(3);
    x = (&mut (*v))[(2_usize)];
    v2[(0_usize)] = 1;
    (if true { &mut v3 } else { &mut (*v) })[(0_usize)] = 7;
    (&mut (*v4))[(1_usize)] = 13;
    assert!(((x) == (6)));
    assert!(((*((*v).first_mut().unwrap())) == (4)));
    assert!((((&mut (*v))[(1_usize)]) == (5)));
    assert!((((&mut (*v))[(2_usize)]) == (6)));
    assert!(((*((*v).last_mut().unwrap())) == (20)));
    assert!(((v3[(0_usize)]) == (7)));
    assert!(((v3[(1_usize)]) == (13)));
    (*v).push(20);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v1: Vec<i32> = Vec::new();
    assert!(((v1.len()) == (0_usize)));
    assert!(v1.is_empty());
    v1.push(1);
    assert!(!v1.is_empty());
    v1.pop();
    assert!(v1.is_empty());
    let mut s1: usize = v1.len();
    {
        let __a0 = 100_usize as usize;
        v1.resize_with(__a0, || <i32>::default())
    };
    assert!(((v1.len()) == (100_usize)));
    assert!(((v1[(99_usize)]) == (0)));
    v1[(0_usize)] = 40;
    v1[(99_usize)] = 50;
    assert!(((v1[(0_usize)]) == (40)));
    assert!(((v1[(99_usize)]) == (50)));
    let mut v2: Vec<i32> = Vec::new();
    assert!(((v2.len()) == (0_usize)));
    v2.push(1);
    v2.push(2);
    v2.push(3);
    assert!(((v2.len()) == (3_usize)));
    {
        let pos = v2.as_mut_ptr().offset_from(v2.as_ptr()) as usize;
        v2.remove(pos);
        v2.as_mut_ptr()
    };
    assert!(((v2.len()) == (2_usize)));
    assert!(((v2[(0_usize)]) == (2)));
    assert!(((v2[(1_usize)]) == (3)));
    {
        let pos = v2.as_mut_ptr().offset_from(v2.as_ptr()) as usize;
        v2.insert(pos, 100);
    };
    (unsafe { copy_0(v2.clone()) });
    assert!(((v2.len()) == (3_usize)));
    assert!(((v2[(0_usize)]) == (100)));
    assert!(((v2[(1_usize)]) == (2)));
    assert!(((v2[(2_usize)]) == (3)));
    let mut s2: usize = v2.len();
    let mut v3: Vec<i32> = vec![1; 100_usize as usize];
    assert!(((v3.len()) == (100_usize)));
    let mut i: i32 = 0;
    'loop_: while ((i) < (100)) {
        assert!(((v3[(i as usize)]) == (1)));
        i.prefix_inc();
    }
    let mut v6: Vec<f64> = vec![2.0E+0; s2 as usize];
    assert!(((v6.len()) == (s2)));
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < (s2)) {
        assert!(((v6[(i as usize)]) == (2.0E+0)));
        i.prefix_inc();
    }
    let mut p1: *const f64 = (v6.as_mut_ptr()).cast_const();
    assert!(((*p1) == (2.0E+0)));
    let mut p2: *mut i32 = v3.as_mut_ptr();
    assert!(((*p2) == (1)));
    assert!(((v3[(0_usize)]) == (1)));
    assert!(((v3[(1_usize)]) == (1)));
    (*p2) = (9.9E+1 as i32);
    assert!(((*p2) == (99)));
    assert!(((v3[(0_usize)]) == (99)));
    assert!(((v3[(1_usize)]) == (1)));
    p2.prefix_inc();
    (*p2) = 98;
    assert!(((v3[(0_usize)]) == (99)));
    assert!(((v3[(1_usize)]) == (98)));
    assert!(((v3.capacity()) == (100_usize)));
    assert!(((v3.len()) == (100_usize)));
    if 200_usize as usize > v3.capacity() as usize {
        let len_0 = v3.len();
        v3.reserve_exact(200_usize as usize - len_0 as usize);
    };
    assert!(((v3.capacity()) == (200_usize)));
    assert!(((v3.len()) == (100_usize)));
    if 50_usize as usize > v3.capacity() as usize {
        let len_0 = v3.len();
        v3.reserve_exact(50_usize as usize - len_0 as usize);
    };
    assert!(((v3.capacity()) == (200_usize)));
    assert!(((v3.len()) == (100_usize)));
    if 200_usize as usize > v3.capacity() as usize {
        let len_0 = v3.len();
        v3.reserve_exact(200_usize as usize - len_0 as usize);
    };
    assert!(((v3.capacity()) == (200_usize)));
    assert!(((v3.len()) == (100_usize)));
    if 201_usize as usize > v3.capacity() as usize {
        let len_0 = v3.len();
        v3.reserve_exact(201_usize as usize - len_0 as usize);
    };
    assert!(((v3.capacity()) == (201_usize)));
    assert!(((v3.len()) == (100_usize)));
    assert!(((*((v2).last_mut().unwrap())) == (3)));
    assert!(((*((v3).last_mut().unwrap())) == (1)));
    assert!(((*((v6).last_mut().unwrap())) == (2.0E+0)));
    let ref0: *mut f64 = ((v6).last_mut().unwrap());
    (*ref0) = 5.0E+0;
    assert!(((*((v6).last_mut().unwrap())) == (5.0E+0)));
    let mut x0: f64 = (*((v6).last_mut().unwrap()));
    assert!(((x0) == (5.0E+0)));
    x0 = 6.0E+0;
    assert!(((*((v6).last_mut().unwrap())) == (5.0E+0)));
    let mut idx: i32 = 0;
    assert!(((*&mut (v6)[(idx as usize) as usize]) == (2.0E+0)));
    assert!(((*&mut (v6)[(s2).wrapping_sub(1_usize) as usize]) == (5.0E+0)));
    let ref1: *mut f64 = &mut (v6)[(s2).wrapping_sub(1_usize) as usize];
    (*ref1) += 1.5E+0;
    assert!(((*&mut (v6)[(s2).wrapping_sub(1_usize) as usize]) == (6.5E+0)));
    let mut x1: f64 = (*&mut (v6)[(s2).wrapping_sub(1_usize) as usize]);
    assert!(((x1) == (6.5E+0)));
    x1 -= 1.5E+0;
    assert!(((*&mut (v6)[(s2).wrapping_sub(1_usize) as usize]) == (6.5E+0)));
    let mut v7: Vec<i32> = Vec::new();
    let mut v8: Vec<i32> = Vec::new();
    v7.push(4);
    v7.push(5);
    v7.push(6);
    v8.push(8);
    v8.push(9);
    (unsafe { fn_1(&mut v7 as *mut Vec<i32>, v8.clone()) });
    let mut src: [u32; 3] = [1_u32, 2_u32, 3_u32];
    let mut v9: Vec<u32> = core::slice::from_raw_parts(
        src.as_mut_ptr(),
        (src.as_mut_ptr().offset((3) as isize)).offset_from(src.as_mut_ptr()) as usize,
    )
    .iter()
    .map(|x| u32::try_from(x.clone()).ok().unwrap())
    .collect();
    assert!(((v9.len()) == (3_usize)));
    assert!(
        (((v9[(0_usize)]) == (1_u32)) && ((v9[(1_usize)]) == (2_u32)))
            && ((v9[(2_usize)]) == (3_u32))
    );
    let mut v10: Vec<u64> = core::slice::from_raw_parts(
        src.as_mut_ptr(),
        (src.as_mut_ptr().offset((3) as isize)).offset_from(src.as_mut_ptr()) as usize,
    )
    .iter()
    .map(|x| u64::try_from(x.clone()).ok().unwrap())
    .collect();
    assert!(((v10.len()) == (3_usize)));
    assert!(
        (((v10[(0_usize)]) == (1_u64)) && ((v10[(1_usize)]) == (2_u64)))
            && ((v10[(2_usize)]) == (3_u64))
    );
    let mut v11: Vec<i32> = core::slice::from_raw_parts(
        src.as_mut_ptr(),
        (src.as_mut_ptr().offset((3) as isize)).offset_from(src.as_mut_ptr()) as usize,
    )
    .iter()
    .map(|x| i32::try_from(x.clone()).ok().unwrap())
    .collect();
    assert!(((v11.len()) == (3_usize)));
    assert!((((v11[(0_usize)]) == (1)) && ((v11[(1_usize)]) == (2))) && ((v11[(2_usize)]) == (3)));
    let src1: [u32; 3] = [1_u32, 2_u32, 3_u32];
    let mut v12: Vec<u32> = core::slice::from_raw_parts(
        src1.as_ptr(),
        (src1.as_ptr().add(src1.len())).offset_from(src1.as_ptr()) as usize,
    )
    .to_vec();
    assert!(((v12.len()) == (3_usize)));
    assert!(
        (((v12[(0_usize)]) == (1_u32)) && ((v12[(1_usize)]) == (2_u32)))
            && ((v12[(2_usize)]) == (3_u32))
    );
    let mut buf: [u8; 5] = [10_u8, 20_u8, 30_u8, 40_u8, 50_u8];
    let mut start: *const u8 = (buf.as_mut_ptr()).cast_const();
    let mut len: usize = 5_usize;
    let mut v13: Vec<u8> = core::slice::from_raw_parts(
        start,
        (start.offset((len) as isize)).offset_from(start) as usize,
    )
    .to_vec();
    assert!(((v13.len()) == (5_usize)));
    assert!(((v13[(0_usize)] as i32) == (10)) && ((v13[(4_usize)] as i32) == (50)));
    return ((((s1).wrapping_add(s2)).wrapping_add(((*&mut (v2)[0_usize as usize]) as usize)))
        as i32);
}
