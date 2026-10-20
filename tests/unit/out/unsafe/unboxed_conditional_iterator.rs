extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn median_0(mut w: i32, mut n: i32, mut nw: i32) -> i32 {
    let mx: i32 = if ((w) > (n)) { w } else { n };
    let mn: i32 = (((w) + (n)) - (mx));
    if ((nw) > (mx)) {
        return mn;
    } else if ((nw) < (mn)) {
        return mx;
    }
    return (((n) + (w)) - (nw));
}
pub unsafe fn assign_1(mut c: bool, mut v: i32) -> i32 {
    let mut a: i32 = 1;
    let mut b: i32 = 2;
    (*if c {
        (&mut a as *mut i32)
    } else {
        (&mut b as *mut i32)
    }) = v;
    (*if c {
        (&mut a as *mut i32)
    } else {
        (&mut b as *mut i32)
    })
    .prefix_inc();
    return (((a) * (10)) + (b));
}
pub unsafe fn lehmer_2(mut sigma: *const u32, mut len: u32, mut code: *mut u32) {
    let mut items: Vec<u32> = (0..(len as usize) as usize)
        .map(|_| <u32>::default())
        .collect::<Vec<_>>();
    let mut i: u32 = 0_u32;
    'loop_: while ((i) < (len)) {
        items[(i as usize)] = (i as u32);
        i.prefix_inc();
    }
    let mut i: u32 = 0_u32;
    'loop_: while ((i) < (len)) {
        let mut it: UnsafeRandomAccessIterator<u32> = {
            let mut __it = items.begin();
            while __it != items.end() && *__it.iter_deref() != (*sigma.offset((i) as isize)) {
                __it.iter_inc();
            }
            __it
        };
        assert!(it != items.end());
        (*code.offset((i) as isize)) = (it.iter_distance(&items.begin()) as u32);
        {
            let pos = <UnsafeRandomAccessIterator<u32>>::from((it).clone())
                .iter_distance(&UnsafeRandomAccessIterator::Vec(items.as_mut_ptr()))
                as usize;
            items.remove(pos);
            <UnsafeRandomAccessIterator<u32>>::from((it).clone())
        };
        i.prefix_inc();
    }
}
pub unsafe fn sum_3(v: *mut Vec<i32>) -> i32 {
    let mut s: i32 = 0;
    let mut it: UnsafeRandomAccessIterator<i32> = (*v).begin();
    'loop_: while it != (*v).end() {
        s += (*it.iter_deref());
        it.iter_inc();
    }
    return s;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((unsafe { median_0(1, 5, 7,) }) == (1)));
    assert!(((unsafe { median_0(1, 5, 0,) }) == (5)));
    assert!(((unsafe { median_0(1, 5, 3,) }) == (3)));
    assert!(((unsafe { assign_1(true, 5,) }) == (62)));
    assert!(((unsafe { assign_1(false, 5,) }) == (16)));
    let mut sigma: [u32; 4] = [2_u32, 0_u32, 3_u32, 1_u32];
    let mut code: [u32; 4] = [0_u32; 4];
    (unsafe { lehmer_2((sigma.as_mut_ptr()).cast_const(), 4_u32, code.as_mut_ptr()) });
    assert!(
        ((((code[(0) as usize]) == (2_u32)) && ((code[(1) as usize]) == (0_u32)))
            && ((code[(2) as usize]) == (1_u32)))
            && ((code[(3) as usize]) == (0_u32))
    );
    let mut v: Vec<i32> = Vec::from([1, 2, 3]);
    assert!(((unsafe { sum_3(&mut v,) }) == (6)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
