extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn matalloc_0(n: i32, p: i32, e: i32) -> Option<Value<Box<[Option<Value<Box<[i32]>>>]>>> {
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let p: Value<i32> = Rc::new(RefCell::new(p));
    let e: Value<i32> = Rc::new(RefCell::new(e));
    let m: Value<Option<Value<Box<[Option<Value<Box<[i32]>>>]>>>> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(
            (0..((*n.borrow()) as u64))
                .map(|_| <Option<Value<Box<[i32]>>>>::default())
                .collect::<Box<[_]>>(),
        )))));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*n.borrow())) {
        (*m.borrow()).as_ref().unwrap().borrow_mut()[((*i.borrow()) as u64) as usize] =
            Some(Rc::new(RefCell::new(
                (0..((*p.borrow()) as u64))
                    .map(|_| <i32>::default())
                    .collect::<Box<[_]>>(),
            )));
        let j: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*j.borrow()) < (*p.borrow())) {
            (*m.borrow()).as_ref().unwrap().borrow()[((*i.borrow()) as u64) as usize]
                .as_ref()
                .unwrap()
                .borrow_mut()[((*j.borrow()) as u64) as usize] = (*e.borrow());
            (*j.borrow_mut()).prefix_inc();
        }
        (*i.borrow_mut()).prefix_inc();
    }
    return (*m.borrow_mut()).take();
}
pub fn matmul_1(
    m1: Option<Value<Box<[Option<Value<Box<[i32]>>>]>>>,
    n1: i32,
    p1: i32,
    m2: Option<Value<Box<[Option<Value<Box<[i32]>>>]>>>,
    n2: i32,
    p2: i32,
) -> Option<Value<Box<[Option<Value<Box<[i32]>>>]>>> {
    let m1: Value<Option<Value<Box<[Option<Value<Box<[i32]>>>]>>>> = Rc::new(RefCell::new(m1));
    let n1: Value<i32> = Rc::new(RefCell::new(n1));
    let p1: Value<i32> = Rc::new(RefCell::new(p1));
    let m2: Value<Option<Value<Box<[Option<Value<Box<[i32]>>>]>>>> = Rc::new(RefCell::new(m2));
    let n2: Value<i32> = Rc::new(RefCell::new(n2));
    let p2: Value<i32> = Rc::new(RefCell::new(p2));
    let m3: Value<Option<Value<Box<[Option<Value<Box<[i32]>>>]>>>> = Rc::new(RefCell::new(
        ({
            let _n: i32 = (*n1.borrow());
            let _p: i32 = (*p2.borrow());
            let _e: i32 = 0;
            matalloc_0(_n, _p, _e)
        }),
    ));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*n1.borrow())) {
        let j: Value<i32> = Rc::new(RefCell::new(0));
        let sum: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*j.borrow()) < (*p2.borrow())) {
            let k: Value<i32> = Rc::new(RefCell::new(0));
            'loop_: while ((*k.borrow()) < (*p1.borrow())) {
                (*sum.borrow_mut()) += ((*m1.borrow()).as_ref().unwrap().borrow()
                    [((*i.borrow()) as u64) as usize]
                    .as_ref()
                    .unwrap()
                    .borrow()[((*k.borrow()) as u64) as usize]
                    * (*m2.borrow()).as_ref().unwrap().borrow()[((*k.borrow()) as u64) as usize]
                        .as_ref()
                        .unwrap()
                        .borrow()[((*j.borrow()) as u64) as usize]);
                (*k.borrow_mut()).prefix_inc();
            }
            (*m3.borrow()).as_ref().unwrap().borrow()[((*i.borrow()) as u64) as usize]
                .as_ref()
                .unwrap()
                .borrow_mut()[((*j.borrow()) as u64) as usize] = (*sum.borrow());
            (*j.borrow_mut()).prefix_inc();
        }
        (*i.borrow_mut()).prefix_inc();
    }
    return (*m3.borrow_mut()).take();
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(1));
    let p: Value<i32> = Rc::new(RefCell::new(10));
    let m1: Value<Option<Value<Box<[Option<Value<Box<[i32]>>>]>>>> = Rc::new(RefCell::new(
        ({
            let _n: i32 = (*n.borrow());
            let _p: i32 = (*p.borrow());
            let _e: i32 = 1;
            matalloc_0(_n, _p, _e)
        }),
    ));
    let m2: Value<Option<Value<Box<[Option<Value<Box<[i32]>>>]>>>> = Rc::new(RefCell::new(
        ({
            let _n: i32 = (*p.borrow());
            let _p: i32 = (*n.borrow());
            let _e: i32 = 2;
            matalloc_0(_n, _p, _e)
        }),
    ));
    let m3: Value<Option<Value<Box<[Option<Value<Box<[i32]>>>]>>>> = Rc::new(RefCell::new(
        ({
            let _m1: Option<Value<Box<[Option<Value<Box<[i32]>>>]>>> = (*m1.borrow_mut()).take();
            let _n1: i32 = (*n.borrow());
            let _p1: i32 = (*p.borrow());
            let _m2: Option<Value<Box<[Option<Value<Box<[i32]>>>]>>> = (*m2.borrow_mut()).take();
            let _n2: i32 = (*p.borrow());
            let _p2: i32 = (*n.borrow());
            matmul_1(_m1, _n1, _p1, _m2, _n2, _p2)
        }),
    ));
    return (*m3.borrow()).as_ref().unwrap().borrow()[(0_u64) as usize]
        .as_ref()
        .unwrap()
        .borrow()[(0_u64) as usize];
}
