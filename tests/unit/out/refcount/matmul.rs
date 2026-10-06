extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn matalloc_0(
    mut n: i32,
    mut p: i32,
    mut e: i32,
) -> Option<Value<Box<[Option<Value<Box<[i32]>>>]>>> {
    let m: Value<Option<Value<Box<[Option<Value<Box<[i32]>>>]>>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
            (0..(n as usize))
                .map(|_| <Option<Value<Box<[i32]>>>>::default())
                .collect::<Box<[_]>>(),
        )))));
    let mut i: i32 = 0;
    'loop_: while (i < n) {
        (((*m.borrow())
            .as_ref()
            .unwrap()
            .as_pointer()
            .offset((i as usize)))
        .clone() as Ptr<Option<Value<Box<[i32]>>>>)
            .write(
                Some(Rc::new(RefCell::new(
                    (0..(p as usize))
                        .map(|_| <i32>::default())
                        .collect::<Box<[_]>>(),
                )))
                .take(),
            );
        let mut j: i32 = 0;
        'loop_: while (j < p) {
            (*m.borrow()).as_ref().unwrap().borrow()[(i as usize) as usize]
                .as_ref()
                .unwrap()
                .borrow_mut()[(j as usize) as usize] = e;
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    return (*m.borrow_mut()).take();
}
pub fn matmul_1(
    mut m1: Option<Value<Box<[Option<Value<Box<[i32]>>>]>>>,
    mut n1: i32,
    mut p1: i32,
    mut m2: Option<Value<Box<[Option<Value<Box<[i32]>>>]>>>,
    mut n2: i32,
    mut p2: i32,
) -> Option<Value<Box<[Option<Value<Box<[i32]>>>]>>> {
    let m3: Value<Option<Value<Box<[Option<Value<Box<[i32]>>>]>>>> =
        Rc::new(RefCell::new(({ matalloc_0(n1, p2, 0) })));
    let mut i: i32 = 0;
    'loop_: while (i < n1) {
        let mut j: i32 = 0;
        let mut sum: i32 = 0;
        'loop_: while (j < p2) {
            let mut k: i32 = 0;
            'loop_: while (k < p1) {
                sum += (m1.as_ref().unwrap().borrow()[(i as usize) as usize]
                    .as_ref()
                    .unwrap()
                    .borrow()[(k as usize) as usize]
                    * m2.as_ref().unwrap().borrow()[(k as usize) as usize]
                        .as_ref()
                        .unwrap()
                        .borrow()[(j as usize) as usize]);
                k.prefix_inc();
            }
            (*m3.borrow()).as_ref().unwrap().borrow()[(i as usize) as usize]
                .as_ref()
                .unwrap()
                .borrow_mut()[(j as usize) as usize] = sum;
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    return (*m3.borrow_mut()).take();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut n: i32 = 1;
    let mut p: i32 = 10;
    let m1: Value<Option<Value<Box<[Option<Value<Box<[i32]>>>]>>>> =
        Rc::new(RefCell::new(({ matalloc_0(n, p, 1) })));
    let m2: Value<Option<Value<Box<[Option<Value<Box<[i32]>>>]>>>> =
        Rc::new(RefCell::new(({ matalloc_0(p, n, 2) })));
    let mut m3: Option<Value<Box<[Option<Value<Box<[i32]>>>]>>> = ({
        let _m1: Option<Value<Box<[Option<Value<Box<[i32]>>>]>>> = (*m1.borrow_mut()).take();
        let _n1: i32 = n;
        let _p1: i32 = p;
        let _m2: Option<Value<Box<[Option<Value<Box<[i32]>>>]>>> = (*m2.borrow_mut()).take();
        let _n2: i32 = p;
        let _p2: i32 = n;
        matmul_1(_m1, _n1, _p1, _m2, _n2, _p2)
    });
    assert!(
        (m3.as_ref().unwrap().borrow()[(0_usize) as usize]
            .as_ref()
            .unwrap()
            .borrow()[(0_usize) as usize]
            == 20)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
