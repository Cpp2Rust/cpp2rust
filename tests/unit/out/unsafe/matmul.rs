extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn matalloc_0(mut n: i32, mut p: i32, mut e: i32) -> Option<Box<[Option<Box<[i32]>>]>> {
    let mut m: Option<Box<[Option<Box<[i32]>>]>> = Some(
        (0..(n as usize))
            .map(|_| <Option<Box<[i32]>>>::default())
            .collect::<Box<[_]>>(),
    );
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        m.as_mut().unwrap()[(i as usize)] = Some(
            (0..(p as usize))
                .map(|_| <i32>::default())
                .collect::<Box<[_]>>(),
        );
        let mut j: i32 = 0;
        'loop_: while ((j) < (p)) {
            m.as_mut().unwrap()[(i as usize)].as_mut().unwrap()[(j as usize)] = e;
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    return m;
}
pub unsafe fn matmul_1(
    mut m1: Option<Box<[Option<Box<[i32]>>]>>,
    mut n1: i32,
    mut p1: i32,
    mut m2: Option<Box<[Option<Box<[i32]>>]>>,
    mut n2: i32,
    mut p2: i32,
) -> Option<Box<[Option<Box<[i32]>>]>> {
    let mut m3: Option<Box<[Option<Box<[i32]>>]>> = (unsafe { matalloc_0(n1, p2, 0) });
    let mut i: i32 = 0;
    'loop_: while ((i) < (n1)) {
        let mut j: i32 = 0;
        let mut sum: i32 = 0;
        'loop_: while ((j) < (p2)) {
            let mut k: i32 = 0;
            'loop_: while ((k) < (p1)) {
                sum += ((m1.as_mut().unwrap()[(i as usize)].as_mut().unwrap()[(k as usize)])
                    * (m2.as_mut().unwrap()[(k as usize)].as_mut().unwrap()[(j as usize)]));
                k.prefix_inc();
            }
            m3.as_mut().unwrap()[(i as usize)].as_mut().unwrap()[(j as usize)] = sum;
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    return m3;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut n: i32 = 1;
    let mut p: i32 = 10;
    let mut m1: Option<Box<[Option<Box<[i32]>>]>> = (unsafe { matalloc_0(n, p, 1) });
    let mut m2: Option<Box<[Option<Box<[i32]>>]>> = (unsafe { matalloc_0(p, n, 2) });
    let mut m3: Option<Box<[Option<Box<[i32]>>]>> = (unsafe {
        let _m1: Option<Box<[Option<Box<[i32]>>]>> = m1;
        let _n1: i32 = n;
        let _p1: i32 = p;
        let _m2: Option<Box<[Option<Box<[i32]>>]>> = m2;
        let _n2: i32 = p;
        let _p2: i32 = n;
        matmul_1(_m1, _n1, _p1, _m2, _n2, _p2)
    });
    return m3.as_mut().unwrap()[(0_usize)].as_mut().unwrap()[(0_usize)];
}
