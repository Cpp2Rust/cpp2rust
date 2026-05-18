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
        (0..(n as u64))
            .map(|_| <Option<Box<[i32]>>>::default())
            .collect::<Box<[_]>>(),
    );
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        m.as_mut().unwrap()[(i as u64) as usize] = Some(
            (0..(p as u64))
                .map(|_| <i32>::default())
                .collect::<Box<[_]>>(),
        );
        let mut j: i32 = 0;
        'loop_: while ((j) < (p)) {
            m.as_mut().unwrap()[(i as u64) as usize].as_mut().unwrap()[(j as u64) as usize] = e;
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
    let mut m3: Option<Box<[Option<Box<[i32]>>]>> = (unsafe {
        let _n: i32 = n1;
        let _p: i32 = p2;
        let _e: i32 = 0;
        matalloc_0(_n, _p, _e)
    });
    let mut i: i32 = 0;
    'loop_: while ((i) < (n1)) {
        let mut j: i32 = 0;
        let mut sum: i32 = 0;
        'loop_: while ((j) < (p2)) {
            let mut k: i32 = 0;
            'loop_: while ((k) < (p1)) {
                sum += ((m1.as_mut().unwrap()[(i as u64) as usize].as_mut().unwrap()
                    [(k as u64) as usize])
                    * (m2.as_mut().unwrap()[(k as u64) as usize].as_mut().unwrap()
                        [(j as u64) as usize]));
                k.prefix_inc();
            }
            m3.as_mut().unwrap()[(i as u64) as usize].as_mut().unwrap()[(j as u64) as usize] = sum;
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
    let mut m1: Option<Box<[Option<Box<[i32]>>]>> = (unsafe {
        let _n: i32 = n;
        let _p: i32 = p;
        let _e: i32 = 1;
        matalloc_0(_n, _p, _e)
    });
    let mut m2: Option<Box<[Option<Box<[i32]>>]>> = (unsafe {
        let _n: i32 = p;
        let _p: i32 = n;
        let _e: i32 = 2;
        matalloc_0(_n, _p, _e)
    });
    let mut m3: Option<Box<[Option<Box<[i32]>>]>> = (unsafe {
        let _m1: Option<Box<[Option<Box<[i32]>>]>> = m1;
        let _n1: i32 = n;
        let _p1: i32 = p;
        let _m2: Option<Box<[Option<Box<[i32]>>]>> = m2;
        let _n2: i32 = p;
        let _p2: i32 = n;
        matmul_1(_m1, _n1, _p1, _m2, _n2, _p2)
    });
    return m3.as_mut().unwrap()[(0_u64) as usize].as_mut().unwrap()[(0_u64) as usize];
}
