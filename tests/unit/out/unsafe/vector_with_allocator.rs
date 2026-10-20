extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestAllocator_int_ {}
impl TestAllocator_int_ {
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut i32 {
        return Box::leak((0..n).map(|_| 0_i32).collect::<Box<[i32]>>()).as_mut_ptr();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut i32, mut _a1: usize) {
        {
            let __p = p;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<i32>(),
                )))
            }
        };
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestAllocator_double_ {}
impl TestAllocator_double_ {
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut f64 {
        return Box::leak((0..n).map(|_| 0_f64).collect::<Box<[f64]>>()).as_mut_ptr();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut f64, mut _a1: usize) {
        {
            let __p = p;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<f64>(),
                )))
            }
        };
    }
}
pub unsafe fn copy_0(mut copy_vector: Vec<i32>) {}
pub unsafe fn fn_1(v: *mut Vec<i32>, mut v3: Vec<i32>) {
    {
        let __a1 = 20;
        (*v).push(__a1)
    };
    let mut x: i32 = 0_i32;
    let mut v4: *mut Vec<i32> = (&mut v3 as *mut Vec<i32>);
    let mut v2: Vec<i32> = Vec::new();
    {
        let __a1 = 0;
        v2.push(__a1)
    };
    {
        let __a1 = 1;
        v2.push(__a1)
    };
    {
        let __a1 = 3;
        v2.push(__a1)
    };
    x = ((&mut (*v))[(2_usize)] as i32);
    v2[(0_usize)] = (1 as i32);
    (&mut (*if true {
        (&mut v3 as *mut Vec<i32>)
    } else {
        (v)
    }))[(0_usize)] = (7 as i32);
    (&mut (*v4))[(1_usize)] = (13 as i32);
    assert!(((x) == (6)));
    assert!(((*((*v).first_mut().unwrap())) == (4)));
    assert!((((&mut (*v))[(1_usize)]) == (5)));
    assert!((((&mut (*v))[(2_usize)]) == (6)));
    assert!(((*((*v).last_mut().unwrap())) == (20)));
    assert!(((v3[(0_usize)]) == (7)));
    assert!(((v3[(1_usize)]) == (13)));
    {
        let __a1 = 20;
        (*v).push(__a1)
    };
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v1: Vec<i32> = Vec::new();
    assert!(((v1.len()) == (0_usize)));
    assert!(v1.is_empty());
    {
        let __a1 = 1;
        v1.push(__a1)
    };
    assert!(!(v1.is_empty()));
    v1.pop();
    assert!(v1.is_empty());
    let mut s1: usize = v1.len();
    {
        let __a0 = 100_usize as usize;
        v1.resize_with(__a0, || <i32>::default())
    };
    assert!(((v1.len()) == (100_usize)));
    assert!(((v1[(99_usize)]) == (0)));
    v1[(0_usize)] = (40 as i32);
    v1[(99_usize)] = (50 as i32);
    assert!(((v1[(0_usize)]) == (40)));
    assert!(((v1[(99_usize)]) == (50)));
    let mut v2: Vec<i32> = Vec::new();
    assert!(((v2.len()) == (0_usize)));
    {
        let __a1 = 1;
        v2.push(__a1)
    };
    {
        let __a1 = 2;
        v2.push(__a1)
    };
    {
        let __a1 = 3;
        v2.push(__a1)
    };
    assert!(((v2.len()) == (3_usize)));
    {
        let pos = <UnsafeRandomAccessIterator<i32>>::from(v2.begin())
            .iter_distance(&UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr()))
            as usize;
        v2.remove(pos);
        <UnsafeRandomAccessIterator<i32>>::from(v2.begin())
    };
    assert!(((v2.len()) == (2_usize)));
    assert!(((v2[(0_usize)]) == (2)));
    assert!(((v2[(1_usize)]) == (3)));
    {
        let pos = <UnsafeRandomAccessIterator<i32>>::from(v2.begin())
            .iter_distance(&UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr()))
            as usize;
        v2.insert(pos, 100);
        <UnsafeRandomAccessIterator<i32>>::from(v2.begin())
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
    let mut v6: Vec<f64> = vec![2_f64; s2 as usize];
    assert!(((v6.len()) == (s2)));
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < (s2)) {
        assert!(((v6[(i as usize)]) == (2_f64)));
        i.prefix_inc();
    }
    let mut p1: *const f64 = (v6.as_mut_ptr()).cast_const();
    assert!(((*p1) == (2_f64)));
    let mut p2: *mut i32 = v3.as_mut_ptr();
    assert!(((*p2) == (1)));
    assert!(((v3[(0_usize)]) == (1)));
    assert!(((v3[(1_usize)]) == (1)));
    (*p2) = (99_f64 as i32);
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
    assert!(((*((v6).last_mut().unwrap())) == (2_f64)));
    let ref0: *mut f64 = (((v6).last_mut().unwrap()) as *mut f64).cast::<f64>();
    (*ref0) = 5_f64;
    assert!(((*((v6).last_mut().unwrap())) == (5_f64)));
    let mut x0: f64 = (*((v6).last_mut().unwrap()));
    assert!(((x0) == (5_f64)));
    x0 = 6_f64;
    assert!(((*((v6).last_mut().unwrap())) == (5_f64)));
    let mut idx: i32 = 0;
    assert!(((*(&mut (v6)[(idx as usize) as usize] as *mut f64)) == (2_f64)));
    assert!(((*(&mut (v6)[(s2).wrapping_sub(1_usize) as usize] as *mut f64)) == (5_f64)));
    let ref1: *mut f64 =
        ((&mut (v6)[(s2).wrapping_sub(1_usize) as usize] as *mut f64) as *mut f64).cast::<f64>();
    (*ref1) += 1.5_f64;
    assert!(((*(&mut (v6)[(s2).wrapping_sub(1_usize) as usize] as *mut f64)) == (6.5_f64)));
    let mut x1: f64 = (*(&mut (v6)[(s2).wrapping_sub(1_usize) as usize] as *mut f64));
    assert!(((x1) == (6.5_f64)));
    x1 -= 1.5_f64;
    assert!(((*(&mut (v6)[(s2).wrapping_sub(1_usize) as usize] as *mut f64)) == (6.5_f64)));
    let mut v7: Vec<i32> = Vec::new();
    let mut v8: Vec<i32> = Vec::new();
    {
        let __a1 = 4;
        v7.push(__a1)
    };
    {
        let __a1 = 5;
        v7.push(__a1)
    };
    {
        let __a1 = 6;
        v7.push(__a1)
    };
    {
        let __a1 = 8;
        v8.push(__a1)
    };
    {
        let __a1 = 9;
        v8.push(__a1)
    };
    (unsafe { fn_1(&mut v7, v8.clone()) });
    let mut src: [u32; 3] = [1_u32, 2_u32, 3_u32];
    let mut v9: Vec<u32> = {
        let mut __v = Vec::new();
        let mut __it = src.as_mut_ptr();
        while __it != src.as_mut_ptr().offset((3) as isize) {
            __v.push(<u32>::try_from((*__it.iter_deref()).clone()).ok().unwrap());
            __it.iter_inc();
        }
        __v
    };
    assert!(((v9.len()) == (3_usize)));
    assert!(
        (((v9[(0_usize)]) == (1_u32)) && ((v9[(1_usize)]) == (2_u32)))
            && ((v9[(2_usize)]) == (3_u32))
    );
    let mut v10: Vec<u64> = {
        let mut __v = Vec::new();
        let mut __it = src.as_mut_ptr();
        while __it != src.as_mut_ptr().offset((3) as isize) {
            __v.push(<u64>::try_from((*__it.iter_deref()).clone()).ok().unwrap());
            __it.iter_inc();
        }
        __v
    };
    assert!(((v10.len()) == (3_usize)));
    assert!(
        (((v10[(0_usize)]) == (1_u64)) && ((v10[(1_usize)]) == (2_u64)))
            && ((v10[(2_usize)]) == (3_u64))
    );
    let mut v11: Vec<i32> = {
        let mut __v = Vec::new();
        let mut __it = src.as_mut_ptr();
        while __it != src.as_mut_ptr().offset((3) as isize) {
            __v.push(<i32>::try_from((*__it.iter_deref()).clone()).ok().unwrap());
            __it.iter_inc();
        }
        __v
    };
    assert!(((v11.len()) == (3_usize)));
    assert!((((v11[(0_usize)]) == (1)) && ((v11[(1_usize)]) == (2))) && ((v11[(2_usize)]) == (3)));
    let src1: [u32; 3] = [1_u32, 2_u32, 3_u32];
    let mut v12: Vec<u32> = {
        let mut __v = Vec::new();
        let mut __it = src1.as_ptr();
        while __it != src1.as_ptr().add(src1.len()) {
            __v.push(<u32>::try_from((*__it.iter_deref()).clone()).ok().unwrap());
            __it.iter_inc();
        }
        __v
    };
    assert!(((v12.len()) == (3_usize)));
    assert!(
        (((v12[(0_usize)]) == (1_u32)) && ((v12[(1_usize)]) == (2_u32)))
            && ((v12[(2_usize)]) == (3_u32))
    );
    let mut buf: [u8; 5] = [10_u8, 20_u8, 30_u8, 40_u8, 50_u8];
    let mut start: *const u8 = (buf.as_mut_ptr()).cast_const();
    let mut len: usize = 5_usize;
    let mut v13: Vec<u8> = {
        let mut __v = Vec::new();
        let mut __it = start;
        while __it != start.offset((len) as isize) {
            __v.push(<u8>::try_from((*__it.iter_deref()).clone()).ok().unwrap());
            __it.iter_inc();
        }
        __v
    };
    assert!(((v13.len()) == (5_usize)));
    assert!(((v13[(0_usize)] as i32) == (10)) && ((v13[(4_usize)] as i32) == (50)));
    assert!(
        ((((s1).wrapping_add(s2))
            .wrapping_add(((*(&mut (v2)[0_usize as usize] as *mut i32)) as usize)))
            == (103_usize))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
