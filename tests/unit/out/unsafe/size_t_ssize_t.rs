extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn add_sizes_0(mut a: usize, mut b: usize) -> usize {
    return (a).wrapping_add(b);
}
pub unsafe fn take_ulong_1(mut x: u64) -> u64 {
    return x;
}
pub unsafe fn sub_signed_2(mut a: isize, mut b: isize) -> isize {
    return ((a) - (b));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut n: usize = (::std::mem::size_of::<i32>() as usize).wrapping_add(4_usize);
    assert!(((n) == ((::std::mem::size_of::<i32>() as usize).wrapping_add(4_usize))));
    let mut ul: u64 = 10_u64;
    let mut sz: usize = 20_usize;
    let mut mixed: usize = ((sz as u64).wrapping_add(ul) as usize);
    assert!(((mixed) == (30_usize)));
    assert!(((sz) > (ul as usize)));
    assert!(((ul as usize) < (sz)));
    assert!(!((sz) == (ul as usize)));
    let mut chain: usize = ((((sz as u64).wrapping_add(ul)).wrapping_add(5_u64))
        .wrapping_add((::std::mem::size_of::<i64>() as u64)) as usize);
    assert!(
        ((chain)
            == (((((20) + (10)) + (5)) as usize)
                .wrapping_add((::std::mem::size_of::<i64>() as usize))))
    );
    let mut acc: usize = 100_usize;
    acc = ((acc as u64).wrapping_add((::std::mem::size_of::<f64>() as u64))) as usize;
    acc = (acc).wrapping_mul(2_usize);
    acc = ((acc as u64).wrapping_sub(ul)) as usize;
    assert!(
        ((acc)
            == (((((100_usize).wrapping_add((::std::mem::size_of::<f64>() as usize))) as usize)
                .wrapping_mul(2_usize) as usize)
                .wrapping_sub(10_usize)))
    );
    sz = (sz).wrapping_add(1_usize);
    assert!(((sz) == (21_usize)));
    let mut fr: usize = (unsafe {
        let _a: usize = ((::std::mem::size_of::<i32>() as u64).wrapping_add((sz as u64)) as usize);
        let _b: usize = (ul as usize);
        add_sizes_0(_a, _b)
    });
    assert!(
        ((fr)
            == (((::std::mem::size_of::<i32>() as usize).wrapping_add(21_usize) as usize)
                .wrapping_add(10_usize)))
    );
    let mut fr2: u64 = (unsafe {
        let _x: u64 = (sz as u64);
        take_ulong_1(_x)
    });
    assert!(((fr2) == (21_u64)));
    let mut lo: usize = ({
        let mut __tmp_0 = (sz as u64);
        let mut __tmp_1 = (::std::mem::size_of::<i64>() as u64).wrapping_add(ul);
        (*if *&mut __tmp_0 <= *&mut __tmp_1 {
            (&mut __tmp_0) as *const _
        } else {
            (&mut __tmp_1) as *const _
        })
    } as usize);
    let mut hi: usize = ({
        let mut __tmp_0 = (::std::mem::size_of::<i32>() as u64).wrapping_add((sz as u64));
        (*if *&mut __tmp_0 >= *&mut ul {
            (&mut __tmp_0) as *const _
        } else {
            (&mut ul) as *const _
        })
    } as usize);
    assert!(((lo) == ((::std::mem::size_of::<i64>() as usize).wrapping_add(10_usize))));
    assert!(((hi) == ((::std::mem::size_of::<i32>() as usize).wrapping_add(21_usize))));
    let mut bound: usize = ({
        let mut __tmp_0 = (sz as u64);
        let mut __tmp_1 = (4_usize as u64);
        (*if *&mut __tmp_0 <= *&mut __tmp_1 {
            (&mut __tmp_0) as *const _
        } else {
            (&mut __tmp_1) as *const _
        })
    } as usize);
    assert!(((bound) == (4_usize)));
    let mut data: [i32; 8] = [0_i32; 8];
    let mut count: usize = (::std::mem::size_of::<[i32; 8]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize));
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (count)) {
        data[(i)] = (((i).wrapping_mul(2_usize)) as i32);
        i.postfix_inc();
    }
    let mut total: usize = 0_usize;
    let mut i: usize = 0_usize;
    'loop_: while ((i) < (count)) {
        total = (total).wrapping_add((data[(i)] as usize));
        i.postfix_inc();
    }
    assert!(((total) == (56_usize)));
    let mut cond: usize = (if ((sz) > (ul as usize)) {
        (sz as u64).wrapping_add((::std::mem::size_of::<i32>() as u64))
    } else {
        ul
    } as usize);
    assert!(((cond) == ((21_usize).wrapping_add((::std::mem::size_of::<i32>() as usize)))));
    let mut arr: [usize; 4] = [0_usize, 1_usize, 2_usize, 3_usize];
    let mut idx: usize = (if ((::std::mem::size_of::<i32>()) > (2_usize)) {
        2
    } else {
        0
    } as usize);
    assert!(((arr[(idx)]) == (2_usize)));
    let mut s1: isize = 5_isize;
    let mut s2: isize = 12_isize;
    let mut sd: isize = (unsafe {
        let _a: isize = s1;
        let _b: isize = s2;
        sub_signed_2(_a, _b)
    });
    assert!(((sd) == (-7_i32 as isize)));
    assert!(((sd) < (0_isize)));
    let mut l: i64 = 3_i64;
    let mut sm: isize = (((s2 as i64) + (l)) as isize);
    assert!(((sm) == (15_isize)));
    assert!(((sm) > (l as isize)));
    let mut smin: isize = ({
        let mut __tmp_0 = (sd as i64);
        let mut __tmp_1 = (sm as i64);
        (*if *&mut __tmp_0 <= *&mut __tmp_1 {
            (&mut __tmp_0) as *const _
        } else {
            (&mut __tmp_1) as *const _
        })
    } as isize);
    let mut smax: isize = ({
        let mut __tmp_0 = (sd as i64);
        let mut __tmp_1 = (sm as i64);
        (*if *&mut __tmp_0 >= *&mut __tmp_1 {
            (&mut __tmp_0) as *const _
        } else {
            (&mut __tmp_1) as *const _
        })
    } as isize);
    assert!(((smin) == (-7_i32 as isize)));
    assert!(((smax) == (15_isize)));
    let mut delta: isize = ((sz as isize) - (ul as isize));
    assert!(((delta) == (11_isize)));
    let mut a64: i64 = 100_i64;
    let mut b: isize = 30_isize;
    a64 -= (b as i64);
    assert!(((a64) == (70_i64)));
    a64 += (b as i64);
    assert!(((a64) == (100_i64)));
    let mut c: isize = (-20_i32 as isize);
    a64 -= (c as i64);
    assert!(((a64) == (120_i64)));
    return (((n).wrapping_rem(7_usize)) as i32);
}
