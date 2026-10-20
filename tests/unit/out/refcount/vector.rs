extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub x: i32,
}
pub fn copy_0(mut copy_vector: Vec<i32>) {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut v1: Vec<i32> = Vec::new();
    assert!((v1.len() == 0_usize));
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
    assert!((v1.len() == 100_usize));
    assert!((v1[99_usize] == 0));
    v1[0_usize] = 40;
    v1[99_usize] = 50;
    assert!((v1[0_usize] == 40));
    assert!((v1[99_usize] == 50));
    let v2: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    assert!(((*v2.borrow()).len() == 0_usize));
    {
        let __a1 = 1;
        (*v2.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 2;
        (*v2.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 3;
        (*v2.borrow_mut()).push(__a1)
    };
    assert!(((*v2.borrow()).len() == 3_usize));
    {
        let __off = <RandomAccessIterator<i32>>::from(v2.as_pointer().decay().begin()).index();
        v2.as_pointer()
            .with_mut(|__v: &mut Vec<i32>| __v.remove(__off));
        RandomAccessIterator::Vec(v2.as_pointer().decay() + __off)
    };
    assert!(((*v2.borrow()).len() == 2_usize));
    assert!(({ (*v2.borrow())[0_usize] } == 2));
    assert!(({ (*v2.borrow())[1_usize] } == 3));
    {
        let __off = <RandomAccessIterator<i32>>::from(v2.as_pointer().decay().begin()).index();
        (*v2.borrow_mut()).insert(__off, 100);
        <RandomAccessIterator<i32>>::from(v2.as_pointer().decay().begin())
    };
    ({ copy_0((*v2.borrow()).clone()) });
    assert!(((*v2.borrow()).len() == 3_usize));
    assert!(({ (*v2.borrow())[0_usize] } == 100));
    assert!(({ (*v2.borrow())[1_usize] } == 2));
    assert!(({ (*v2.borrow())[2_usize] } == 3));
    let mut x: i32 = 7;
    {
        let __off =
            <RandomAccessIterator<i32>>::from(v2.as_pointer().decay().begin().iter_add(1_isize))
                .index();
        (*v2.borrow_mut()).insert(__off, x);
        <RandomAccessIterator<i32>>::from(v2.as_pointer().decay().begin().iter_add(1_isize))
    };
    assert!(((*v2.borrow()).len() == 4_usize));
    assert!(({ (*v2.borrow())[0_usize] } == 100));
    assert!(({ (*v2.borrow())[1_usize] } == 7));
    assert!(({ (*v2.borrow())[2_usize] } == 2));
    {
        let __off = <RandomAccessIterator<i32>>::from(v2.as_pointer().decay().end()).index();
        (*v2.borrow_mut()).splice(__off..__off, std::iter::repeat_n(9, 2_usize));
        <RandomAccessIterator<i32>>::from(v2.as_pointer().decay().end())
    };
    assert!(((*v2.borrow()).len() == 6_usize));
    assert!(({ (*v2.borrow())[4_usize] } == 9));
    assert!(({ (*v2.borrow())[5_usize] } == 9));
    {
        let __off =
            <RandomAccessIterator<i32>>::from(v2.as_pointer().decay().begin().iter_add(2_isize))
                .index();
        (*v2.borrow_mut()).splice(__off..__off, Vec::from([11, 12, 13]));
        <RandomAccessIterator<i32>>::from(v2.as_pointer().decay().begin().iter_add(2_isize))
    };
    assert!(((*v2.borrow()).len() == 9_usize));
    assert!(({ (*v2.borrow())[1_usize] } == 7));
    assert!(({ (*v2.borrow())[2_usize] } == 11));
    assert!(({ (*v2.borrow())[3_usize] } == 12));
    assert!(({ (*v2.borrow())[4_usize] } == 13));
    assert!(({ (*v2.borrow())[5_usize] } == 2));
    assert!(({ (*v2.borrow())[8_usize] } == 9));
    let src: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([21, 22])));
    {
        let start_idx = <RandomAccessIterator<i32>>::from(v2.as_pointer().decay().begin()).index();
        let mut temp_vec: Vec<i32> = Vec::new();
        let mut __it = src.as_pointer().decay().begin();
        while __it != src.as_pointer().decay().end() {
            temp_vec.push(__it.iter_deref().read());
            __it.iter_inc();
        }
        v2.as_pointer().with_mut(|v: &mut Vec<i32>| {
            v.splice(start_idx..start_idx, temp_vec);
        });
        RandomAccessIterator::Vec(v2.as_pointer().decay() + start_idx)
    };
    assert!(((*v2.borrow()).len() == 11_usize));
    assert!(({ (*v2.borrow())[0_usize] } == 21));
    assert!(({ (*v2.borrow())[1_usize] } == 22));
    assert!(({ (*v2.borrow())[2_usize] } == 100));
    {
        let __off = <RandomAccessIterator<i32>>::from(v2.as_pointer().decay().begin()).index();
        v2.as_pointer()
            .with_mut(|__v: &mut Vec<i32>| __v.remove(__off));
        RandomAccessIterator::Vec(v2.as_pointer().decay() + __off)
    };
    {
        let __off = <RandomAccessIterator<i32>>::from(v2.as_pointer().decay().begin()).index();
        v2.as_pointer()
            .with_mut(|__v: &mut Vec<i32>| __v.remove(__off));
        RandomAccessIterator::Vec(v2.as_pointer().decay() + __off)
    };
    let mut i: i32 = 0;
    'loop_: while (i < 4) {
        {
            let __off = <RandomAccessIterator<i32>>::from(
                v2.as_pointer().decay().begin().iter_add(1_isize),
            )
            .index();
            v2.as_pointer()
                .with_mut(|__v: &mut Vec<i32>| __v.remove(__off));
            RandomAccessIterator::Vec(v2.as_pointer().decay() + __off)
        };
        i.prefix_inc();
    }
    (*v2.borrow_mut()).pop();
    (*v2.borrow_mut()).pop();
    assert!(((*v2.borrow()).len() == 3_usize));
    assert!(({ (*v2.borrow())[0_usize] } == 100));
    assert!(({ (*v2.borrow())[1_usize] } == 2));
    assert!(({ (*v2.borrow())[2_usize] } == 3));
    let mut s2: usize = (*v2.borrow()).len();
    let v3: Value<Vec<i32>> = Rc::new(RefCell::new(vec![1; 100_usize as usize]));
    assert!(((*v3.borrow()).len() == 100_usize));
    let mut i: i32 = 0;
    'loop_: while (i < 100) {
        assert!(({ (*v3.borrow())[(i as usize)] } == 1));
        i.prefix_inc();
    }
    let v4: Value<Vec<Ptr<i32>>> = Rc::new(RefCell::new(
        (0..(100_usize) as usize)
            .map(|_| <Ptr<i32>>::default())
            .collect::<Vec<_>>(),
    ));
    assert!(((*v4.borrow()).len() == 100_usize));
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < (*v4.borrow()).len()) {
        assert!(((*v4.borrow())[(i as usize)]).is_null());
        i.prefix_inc();
    }
    let v5: Value<Vec<Ptr<i32>>> = Rc::new(RefCell::new(
        (0..(100_usize) as usize)
            .map(|_| <Ptr<i32>>::default())
            .collect::<Vec<_>>(),
    ));
    assert!(((*v5.borrow()).len() == 100_usize));
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < (*v5.borrow()).len()) {
        assert!(((*v5.borrow())[(i as usize)]).is_null());
        i.prefix_inc();
    }
    let v6: Value<Vec<f64>> = Rc::new(RefCell::new(vec![2_f64; s2 as usize]));
    assert!(((*v6.borrow()).len() == s2));
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < s2) {
        assert!(({ (*v6.borrow())[(i as usize)] } == 2_f64));
        i.prefix_inc();
    }
    let v7: Value<Vec<(Value<Ptr<i32>>, Value<i32>)>> = Rc::new(RefCell::new(
        (0..(200_usize) as usize)
            .map(|_| <(Value<Ptr<i32>>, Value<i32>)>::default())
            .collect::<Vec<_>>(),
    ));
    assert!(((*v7.borrow()).len() == 200_usize));
    let mut i: u32 = 0_u32;
    'loop_: while (i < 200_u32) {
        assert!(
            ((*(*v7.borrow())[(i as usize)].0.borrow()).is_null())
                && ((*(*v7.borrow())[(i as usize)].1.borrow()) == 0)
        );
        i.prefix_inc();
    }
    let mut p1: Ptr<f64> = v6.as_pointer().decay();
    assert!(((p1.read()) == 2_f64));
    let mut p2: Ptr<i32> = v3.as_pointer().decay();
    assert!(((p2.read()) == 1));
    assert!(({ (*v3.borrow())[0_usize] } == 1));
    assert!(({ (*v3.borrow())[1_usize] } == 1));
    p2.write((99_f64 as i32));
    assert!(((p2.read()) == 99));
    assert!(({ (*v3.borrow())[0_usize] } == 99));
    assert!(({ (*v3.borrow())[1_usize] } == 1));
    p2.prefix_inc();
    p2.write(98);
    assert!(({ (*v3.borrow())[0_usize] } == 99));
    assert!(({ (*v3.borrow())[1_usize] } == 98));
    assert!(((*v3.borrow()).capacity() == 100_usize));
    assert!(((*v3.borrow()).len() == 100_usize));
    if 200_usize as usize > (*v3.borrow()).capacity() as usize {
        let len_0 = (*v3.borrow()).len();
        (*v3.borrow_mut()).reserve_exact(200_usize as usize - len_0 as usize);
    };
    assert!(((*v3.borrow()).capacity() == 200_usize));
    assert!(((*v3.borrow()).len() == 100_usize));
    if 50_usize as usize > (*v3.borrow()).capacity() as usize {
        let len_0 = (*v3.borrow()).len();
        (*v3.borrow_mut()).reserve_exact(50_usize as usize - len_0 as usize);
    };
    assert!(((*v3.borrow()).capacity() == 200_usize));
    assert!(((*v3.borrow()).len() == 100_usize));
    if 200_usize as usize > (*v3.borrow()).capacity() as usize {
        let len_0 = (*v3.borrow()).len();
        (*v3.borrow_mut()).reserve_exact(200_usize as usize - len_0 as usize);
    };
    assert!(((*v3.borrow()).capacity() == 200_usize));
    assert!(((*v3.borrow()).len() == 100_usize));
    if 201_usize as usize > (*v3.borrow()).capacity() as usize {
        let len_0 = (*v3.borrow()).len();
        (*v3.borrow_mut()).reserve_exact(201_usize as usize - len_0 as usize);
    };
    assert!(((*v3.borrow()).capacity() == 201_usize));
    assert!(((*v3.borrow()).len() == 100_usize));
    assert!(((v2.as_pointer().decay().to_last().read()) == 3));
    assert!(((v3.as_pointer().decay().to_last().read()) == 1));
    assert!((v4.as_pointer().decay().to_last().read()).is_null());
    assert!((v5.as_pointer().decay().to_last().read()).is_null());
    assert!(((v6.as_pointer().decay().to_last().read()) == 2_f64));
    let ref0: Ptr<f64> = v6.as_pointer().decay().to_last();
    ref0.write(5_f64);
    assert!(((v6.as_pointer().decay().to_last().read()) == 5_f64));
    let mut x0: f64 = (v6.as_pointer().decay().to_last().read());
    assert!((x0 == 5_f64));
    x0 = 6_f64;
    assert!(((v6.as_pointer().decay().to_last().read()) == 5_f64));
    let mut idx: i32 = 0;
    assert!(
        ((v6.as_pointer()
            .decay()
            .offset((idx as usize) as isize)
            .read())
            == 2_f64)
    );
    assert!(
        ((v6.as_pointer()
            .decay()
            .offset((s2).wrapping_sub(1_usize) as isize)
            .read())
            == 5_f64)
    );
    let ref1: Ptr<f64> = v6
        .as_pointer()
        .decay()
        .offset((s2).wrapping_sub(1_usize) as isize);
    {
        ref1.with_mut(|__v| *__v = *__v + 1.5_f64)
    };
    assert!(
        ((v6.as_pointer()
            .decay()
            .offset((s2).wrapping_sub(1_usize) as isize)
            .read())
            == 6.5_f64)
    );
    let mut x1: f64 = (v6
        .as_pointer()
        .decay()
        .offset((s2).wrapping_sub(1_usize) as isize)
        .read());
    assert!((x1 == 6.5_f64));
    x1 -= 1.5_f64;
    assert!(
        ((v6.as_pointer()
            .decay()
            .offset((s2).wrapping_sub(1_usize) as isize)
            .read())
            == 6.5_f64)
    );
    assert!(
        (((s1).wrapping_add(s2))
            .wrapping_add(((v2.as_pointer().decay().offset(0_usize as isize).read()) as usize))
            == 103_usize)
    );
    let a: Value<S> = Rc::new(RefCell::new(S { x: 1 }));
    let pa: Value<Ptr<S>> = Rc::new(RefCell::new((a.as_pointer())));
    let mut v8: Vec<Ptr<Ptr<S>>> = Vec::new();
    {
        let __a1 = (pa.as_pointer());
        v8.push(__a1)
    };
    assert!(((v8[0_usize].read()).with(|__s| __s.x) == 1));
    let ppa: Value<Ptr<Ptr<S>>> = Rc::new(RefCell::new((pa.as_pointer())));
    let mut v9: Vec<Ptr<Ptr<Ptr<S>>>> = Vec::new();
    {
        let __a1 = (ppa.as_pointer());
        v9.push(__a1)
    };
    field!(((v9[0_usize].read()).read()), x).write(2);
    assert!(({ (*a.borrow()).x } == 2));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
