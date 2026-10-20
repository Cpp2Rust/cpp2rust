extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut next_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub unsafe fn bump_1(mut v: *mut Vec<i32>, mut x: i32) -> i32 {
    (&mut (*v))[(0_usize)] += (x as i32);
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut next_0)).postfix_inc();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v: Vec<i32> = vec![1; 4_usize as usize];
    let mut items: Vec<u32> = (0..(3_usize) as usize)
        .map(|_| <u32>::default())
        .collect::<Vec<_>>();
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < (items.len())) {
        items[(i as usize)] = ((i).wrapping_mul(2_u32) as u32);
        i.prefix_inc();
    }
    items[(1_usize)] = (items[(1_usize)]).wrapping_add(5_u32);
    items[(2_usize)].postfix_inc();
    assert!(
        (((items[(0_usize)]) == (0_u32)) && ((items[(1_usize)]) == (7_u32)))
            && ((items[(2_usize)]) == (5_u32))
    );
    (*{
        let __idx = (v[(0_usize)] as usize);
        &mut v[__idx]
    }) = (3 as i32);
    (*{
        let __idx = (v[(1_usize)] as usize);
        &mut v[__idx]
    }) = (((v[(1_usize)] as i32) + (4)) as i32);
    assert!(((v[(1_usize)]) == (3)) && ((v[(3_usize)]) == (7)));
    let mut p: *mut i32 = ((&mut v[(0_usize)] as *mut i32) as *mut i32).cast::<i32>();
    v[((*p) as usize)] = (9 as i32);
    assert!(((v[(1_usize)]) == (9)));
    (*{
        let __idx = ((unsafe { bump_1((&mut v as *mut Vec<i32>), 10) }) as usize);
        &mut v[__idx]
    }) = (2 as i32);
    assert!(((v[(0_usize)]) == (2)));
    assert!(
        ((unsafe {
            let _v: *mut Vec<i32> = (&mut v as *mut Vec<i32>);
            let _x: i32 = (v[(2_usize)] as i32);
            bump_1(_v, _x)
        }) == (1))
            && ((v[(0_usize)]) == (3))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const next_0);
}
