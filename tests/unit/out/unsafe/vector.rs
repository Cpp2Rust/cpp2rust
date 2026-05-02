extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn copy_0(mut copy_vector: Vec<i32>) {}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v1: Vec<i32> = Vec::new();
    assert!(((v1.len() as u64) == (0_u64)));
    assert!(v1.is_empty());
    v1.push(1);
    assert!(!v1.is_empty());
    v1.pop();
    assert!(v1.is_empty());
    let mut s1: u64 = v1.len() as u64;
    {
        let __a0 = 100_u64 as usize;
        v1.resize_with(__a0, || <i32>::default())
    };
    assert!(((v1.len() as u64) == (100_u64)));
    assert!(((v1[(99_u64) as usize]) == (0)));
    v1[(0_u64) as usize] = 40;
    v1[(99_u64) as usize] = 50;
    assert!(((v1[(0_u64) as usize]) == (40)));
    assert!(((v1[(99_u64) as usize]) == (50)));
    let mut v2: Vec<i32> = Vec::new();
    assert!(((v2.len() as u64) == (0_u64)));
    v2.push(1);
    v2.push(2);
    v2.push(3);
    assert!(((v2.len() as u64) == (3_u64)));
    {
        let pos = v2.as_mut_ptr().offset_from(v2.as_ptr()) as usize;
        v2.remove(pos);
        v2.as_mut_ptr()
    };
    assert!(((v2.len() as u64) == (2_u64)));
    assert!(((v2[(0_u64) as usize]) == (2)));
    assert!(((v2[(1_u64) as usize]) == (3)));
    {
        let pos = v2.as_mut_ptr().offset_from(v2.as_ptr()) as usize;
        v2.insert(pos, 100);
    };
    (unsafe {
        let _copy_vector: Vec<i32> = v2.clone();
        copy_0(_copy_vector)
    });
    assert!(((v2.len() as u64) == (3_u64)));
    assert!(((v2[(0_u64) as usize]) == (100)));
    assert!(((v2[(1_u64) as usize]) == (2)));
    assert!(((v2[(2_u64) as usize]) == (3)));
    let mut s2: u64 = v2.len() as u64;
    let mut v3: Vec<i32> = vec![1; 100_u64 as usize];
    assert!(((v3.len() as u64) == (100_u64)));
    let mut i: i32 = 0;
    'loop_: while ((i) < (100)) {
        assert!(((v3[(i as u64) as usize]) == (1)));
        i.prefix_inc();
    }
    let mut v4: Vec<*mut i32> = (0..(100_u64) as usize)
        .map(|_| <*mut i32>::default())
        .collect::<Vec<_>>();
    assert!(((v4.len() as u64) == (100_u64)));
    let mut i: u32 = 0_u32;
    'loop_: while ((i as u64) < (v4.len() as u64)) {
        assert!((v4[(i as u64) as usize]).is_null());
        i.prefix_inc();
    }
    let mut v5: Vec<*const i32> = (0..(100_u64) as usize)
        .map(|_| <*const i32>::default())
        .collect::<Vec<_>>();
    assert!(((v5.len() as u64) == (100_u64)));
    let mut i: u32 = 0_u32;
    'loop_: while ((i as u64) < (v5.len() as u64)) {
        assert!((v5[(i as u64) as usize]).is_null());
        i.prefix_inc();
    }
    let mut v6: Vec<f64> = vec![2.0E+0; s2 as usize];
    assert!(((v6.len() as u64) == (s2)));
    let mut i: u32 = 0_u32;
    'loop_: while ((i as u64) < (s2)) {
        assert!(((v6[(i as u64) as usize]) == (2.0E+0)));
        i.prefix_inc();
    }
    let mut v7: Vec<(*const i32, i32)> = (0..(200_u64) as usize)
        .map(|_| <(*const i32, i32)>::default())
        .collect::<Vec<_>>();
    assert!(((v7.len() as u64) == (200_u64)));
    let mut i: u32 = 0_u32;
    'loop_: while ((i) < (200_u32)) {
        assert!((((v7[(i as u64) as usize].0).is_null()) && ((v7[(i as u64) as usize].1) == (0))));
        i.prefix_inc();
    }
    let mut p1: *const f64 = v6.as_mut_ptr().cast_const();
    assert!(((*p1) == (2.0E+0)));
    let mut p2: *mut i32 = v3.as_mut_ptr();
    assert!(((*p2) == (1)));
    assert!(((v3[(0_u64) as usize]) == (1)));
    assert!(((v3[(1_u64) as usize]) == (1)));
    (*p2) = (9.9E+1 as i32);
    assert!(((*p2) == (99)));
    assert!(((v3[(0_u64) as usize]) == (99)));
    assert!(((v3[(1_u64) as usize]) == (1)));
    p2.prefix_inc();
    (*p2) = 98;
    assert!(((v3[(0_u64) as usize]) == (99)));
    assert!(((v3[(1_u64) as usize]) == (98)));
    assert!(((v3.capacity() as u64) == (100_u64)));
    assert!(((v3.len() as u64) == (100_u64)));
    if 200_u64 as usize > v3.capacity() as usize {
        let len_0 = v3.len();
        v3.reserve_exact(200_u64 as usize - len_0 as usize);
    };
    assert!(((v3.capacity() as u64) == (200_u64)));
    assert!(((v3.len() as u64) == (100_u64)));
    if 50_u64 as usize > v3.capacity() as usize {
        let len_0 = v3.len();
        v3.reserve_exact(50_u64 as usize - len_0 as usize);
    };
    assert!(((v3.capacity() as u64) == (200_u64)));
    assert!(((v3.len() as u64) == (100_u64)));
    if 200_u64 as usize > v3.capacity() as usize {
        let len_0 = v3.len();
        v3.reserve_exact(200_u64 as usize - len_0 as usize);
    };
    assert!(((v3.capacity() as u64) == (200_u64)));
    assert!(((v3.len() as u64) == (100_u64)));
    if 201_u64 as usize > v3.capacity() as usize {
        let len_0 = v3.len();
        v3.reserve_exact(201_u64 as usize - len_0 as usize);
    };
    assert!(((v3.capacity() as u64) == (201_u64)));
    assert!(((v3.len() as u64) == (100_u64)));
    assert!(((*((v2).last_mut().unwrap())) == (3)));
    assert!(((*((v3).last_mut().unwrap())) == (1)));
    assert!((*((v4).last_mut().unwrap())).is_null());
    assert!((*((v5).last_mut().unwrap())).is_null());
    assert!(((*((v6).last_mut().unwrap())) == (2.0E+0)));
    let ref0: *mut f64 = ((v6).last_mut().unwrap());
    (*ref0) = 5.0E+0;
    assert!(((*((v6).last_mut().unwrap())) == (5.0E+0)));
    let mut x0: f64 = (*((v6).last_mut().unwrap()));
    assert!(((x0) == (5.0E+0)));
    x0 = 6.0E+0;
    assert!(((*((v6).last_mut().unwrap())) == (5.0E+0)));
    let mut idx: i32 = 0;
    assert!(((*&mut (v6)[(idx as u64) as usize]) == (2.0E+0)));
    assert!(((*&mut (v6)[(s2).wrapping_sub(1_u64) as usize]) == (5.0E+0)));
    let ref1: *mut f64 = &mut (v6)[(s2).wrapping_sub(1_u64) as usize];
    (*ref1) += 1.5E+0;
    assert!(((*&mut (v6)[(s2).wrapping_sub(1_u64) as usize]) == (6.5E+0)));
    let mut x1: f64 = (*&mut (v6)[(s2).wrapping_sub(1_u64) as usize]);
    assert!(((x1) == (6.5E+0)));
    x1 -= 1.5E+0;
    assert!(((*&mut (v6)[(s2).wrapping_sub(1_u64) as usize]) == (6.5E+0)));
    return ((((s1).wrapping_add(s2)).wrapping_add(((*&mut (v2)[0_u64 as usize]) as u64))) as i32);
}
