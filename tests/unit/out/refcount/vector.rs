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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(48)]
pub struct Foo {
    #[offset(0)]
    #[byte_size(24)]
    pub a: Vec<i32>,
    #[offset(24)]
    #[byte_size(24)]
    pub b: Vec<i32>,
}
impl Foo {
    pub fn move_from(_a0: Ptr<Foo>) -> Self {
        Self {
            a: field!(_a0, a).with_mut(std::mem::take),
            b: field!(_a0, b).with_mut(std::mem::take),
        }
    }
}
pub fn exchange_1(x: Ptr<Vec<i32>>, y: Ptr<Vec<i32>>) {
    x.swap_with(&(y).clone(), |__v| __v.with_mut(std::mem::take));
}
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
        let __a0 = 100_usize;
        v1.resize_with(__a0, || 0_i32)
    };
    assert!((v1.len() == 100_usize));
    assert!((v1[99_usize] == 0));
    v1[0_usize] = 40;
    v1[99_usize] = 50;
    assert!((v1[0_usize] == 40));
    assert!((v1[99_usize] == 50));
    let v2: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    assert!(({ ((*v2.borrow()).len() == 0_usize) }));
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
    assert!(({ ((*v2.borrow()).len() == 3_usize) }));
    match ({ <RandomAccessIterator<i32>>::from(v2.decay().begin()) },) {
        (mut __tmp_0,) => {
            let __off = __tmp_0.index();
            v2.as_pointer()
                .with_mut(|__v: &mut Vec<i32>| __v.remove(__off));
            (__tmp_0).clone()
        }
    };
    assert!(({ ((*v2.borrow()).len() == 2_usize) }));
    assert!(({ ({ (*v2.borrow())[0_usize] } == 2) }));
    assert!(({ ({ (*v2.borrow())[1_usize] } == 3) }));
    match ({ <RandomAccessIterator<i32>>::from(v2.decay().begin()) },) {
        (mut __tmp_1,) => {
            let __off = __tmp_1.index();
            (*v2.borrow_mut()).insert(__off, 100);
            (__tmp_1).clone()
        }
    };
    ({ copy_0((*v2.borrow()).clone()) });
    assert!(({ ((*v2.borrow()).len() == 3_usize) }));
    assert!(({ ({ (*v2.borrow())[0_usize] } == 100) }));
    assert!(({ ({ (*v2.borrow())[1_usize] } == 2) }));
    assert!(({ ({ (*v2.borrow())[2_usize] } == 3) }));
    let mut x: i32 = 7;
    match ({ <RandomAccessIterator<i32>>::from(v2.decay().begin().iter_add(1_isize)) },) {
        (mut __tmp_2,) => {
            let __off = __tmp_2.index();
            (*v2.borrow_mut()).insert(__off, x);
            (__tmp_2).clone()
        }
    };
    assert!(({ ((*v2.borrow()).len() == 4_usize) }));
    assert!(({ ({ (*v2.borrow())[0_usize] } == 100) }));
    assert!(({ ({ (*v2.borrow())[1_usize] } == 7) }));
    assert!(({ ({ (*v2.borrow())[2_usize] } == 2) }));
    match ({ <RandomAccessIterator<i32>>::from(v2.decay().end()) },) {
        (mut __tmp_3,) => {
            let __off = __tmp_3.index();
            (*v2.borrow_mut()).splice(__off..__off, std::iter::repeat_n(9, 2_usize));
            (__tmp_3).clone()
        }
    };
    assert!(({ ((*v2.borrow()).len() == 6_usize) }));
    assert!(({ ({ (*v2.borrow())[4_usize] } == 9) }));
    assert!(({ ({ (*v2.borrow())[5_usize] } == 9) }));
    match ({ <RandomAccessIterator<i32>>::from(v2.decay().begin().iter_add(2_isize)) },) {
        (mut __tmp_4,) => {
            let __off = __tmp_4.index();
            (*v2.borrow_mut()).splice(__off..__off, Vec::from([11, 12, 13]));
            (__tmp_4).clone()
        }
    };
    assert!(({ ((*v2.borrow()).len() == 9_usize) }));
    assert!(({ ({ (*v2.borrow())[1_usize] } == 7) }));
    assert!(({ ({ (*v2.borrow())[2_usize] } == 11) }));
    assert!(({ ({ (*v2.borrow())[3_usize] } == 12) }));
    assert!(({ ({ (*v2.borrow())[4_usize] } == 13) }));
    assert!(({ ({ (*v2.borrow())[5_usize] } == 2) }));
    assert!(({ ({ (*v2.borrow())[8_usize] } == 9) }));
    let src: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([21, 22])));
    match (
        { <RandomAccessIterator<i32>>::from(v2.decay().begin()) },
        { src.decay().begin() },
        { src.decay().end() },
    ) {
        (mut __tmp_5, mut __tmp_6, mut __tmp_7) => {
            let start_idx = __tmp_5.index();
            let mut temp_vec: Vec<i32> = Vec::new();
            let mut __it = (__tmp_6).clone();
            while __it != __tmp_7 {
                temp_vec.push(__it.iter_deref().read());
                __it.iter_inc();
            }
            v2.as_pointer().with_mut(|v: &mut Vec<i32>| {
                v.splice(start_idx..start_idx, temp_vec);
            });
            (__tmp_5).clone()
        }
    };
    assert!(({ ((*v2.borrow()).len() == 11_usize) }));
    assert!(({ ({ (*v2.borrow())[0_usize] } == 21) }));
    assert!(({ ({ (*v2.borrow())[1_usize] } == 22) }));
    assert!(({ ({ (*v2.borrow())[2_usize] } == 100) }));
    match ({ <RandomAccessIterator<i32>>::from(v2.decay().begin()) },) {
        (mut __tmp_8,) => {
            let __off = __tmp_8.index();
            v2.as_pointer()
                .with_mut(|__v: &mut Vec<i32>| __v.remove(__off));
            (__tmp_8).clone()
        }
    };
    match ({ <RandomAccessIterator<i32>>::from(v2.decay().begin()) },) {
        (mut __tmp_9,) => {
            let __off = __tmp_9.index();
            v2.as_pointer()
                .with_mut(|__v: &mut Vec<i32>| __v.remove(__off));
            (__tmp_9).clone()
        }
    };
    let mut i: i32 = 0;
    'loop_: while (i < 4) {
        match ({ <RandomAccessIterator<i32>>::from(v2.decay().begin().iter_add(1_isize)) },) {
            (mut __tmp_10,) => {
                let __off = __tmp_10.index();
                v2.as_pointer()
                    .with_mut(|__v: &mut Vec<i32>| __v.remove(__off));
                (__tmp_10).clone()
            }
        };
        i.prefix_inc();
    }
    (*v2.borrow_mut()).pop();
    (*v2.borrow_mut()).pop();
    assert!(({ ((*v2.borrow()).len() == 3_usize) }));
    assert!(({ ({ (*v2.borrow())[0_usize] } == 100) }));
    assert!(({ ({ (*v2.borrow())[1_usize] } == 2) }));
    assert!(({ ({ (*v2.borrow())[2_usize] } == 3) }));
    let mut s2: usize = (*v2.borrow()).len();
    let v3: Value<Vec<i32>> = Rc::new(RefCell::new(vec![1; 100_usize]));
    assert!(({ ((*v3.borrow()).len() == 100_usize) }));
    let mut i: i32 = 0;
    'loop_: while (i < 100) {
        assert!(({ ({ (*v3.borrow())[(i as usize)] } == 1) }));
        i.prefix_inc();
    }
    let v4: Value<Vec<Ptr<i32>>> = Rc::new(RefCell::new(
        (0..(100_usize) as usize)
            .map(|_| Ptr::<i32>::null())
            .collect::<Vec<_>>(),
    ));
    assert!(({ ((*v4.borrow()).len() == 100_usize) }));
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < (*v4.borrow()).len()) {
        assert!(({ ((*v4.borrow())[(i as usize)]).is_null() }));
        i.prefix_inc();
    }
    let v5: Value<Vec<Ptr<i32>>> = Rc::new(RefCell::new(
        (0..(100_usize) as usize)
            .map(|_| Ptr::<i32>::null())
            .collect::<Vec<_>>(),
    ));
    assert!(({ ((*v5.borrow()).len() == 100_usize) }));
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < (*v5.borrow()).len()) {
        assert!(({ ((*v5.borrow())[(i as usize)]).is_null() }));
        i.prefix_inc();
    }
    let v6: Value<Vec<f64>> = Rc::new(RefCell::new(vec![2_f64; s2]));
    assert!(({ ((*v6.borrow()).len() == s2) }));
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < s2) {
        assert!(({ ({ (*v6.borrow())[(i as usize)] } == 2_f64) }));
        i.prefix_inc();
    }
    let v7: Value<Vec<(Value<Ptr<i32>>, Value<i32>)>> = Rc::new(RefCell::new(
        (0..(200_usize) as usize)
            .map(|_| {
                (
                    Rc::new(RefCell::new(Ptr::<i32>::null())),
                    Rc::new(RefCell::new(0_i32)),
                )
            })
            .collect::<Vec<_>>(),
    ));
    assert!(({ ((*v7.borrow()).len() == 200_usize) }));
    let mut i: u32 = 0_u32;
    'loop_: while (i < 200_u32) {
        assert!(
            ({
                ((*(*v7.borrow())[(i as usize)].0.borrow()).is_null())
                    && ((*(*v7.borrow())[(i as usize)].1.borrow()) == 0)
            })
        );
        i.prefix_inc();
    }
    let mut p1: Ptr<f64> = v6.decay();
    assert!(((p1.read()) == 2_f64));
    let mut p2: Ptr<i32> = v3.decay();
    assert!(((p2.read()) == 1));
    assert!(({ ({ (*v3.borrow())[0_usize] } == 1) }));
    assert!(({ ({ (*v3.borrow())[1_usize] } == 1) }));
    p2.write((99_f64 as i32));
    assert!(((p2.read()) == 99));
    assert!(({ ({ (*v3.borrow())[0_usize] } == 99) }));
    assert!(({ ({ (*v3.borrow())[1_usize] } == 1) }));
    p2.prefix_inc();
    p2.write(98);
    assert!(({ ({ (*v3.borrow())[0_usize] } == 99) }));
    assert!(({ ({ (*v3.borrow())[1_usize] } == 98) }));
    assert!(({ ((*v3.borrow()).capacity() == 100_usize) }));
    assert!(({ ((*v3.borrow()).len() == 100_usize) }));
    if 200_usize > (*v3.borrow()).capacity() as usize {
        let len_0 = (*v3.borrow()).len();
        (*v3.borrow_mut()).reserve_exact(200_usize - len_0 as usize);
    };
    assert!(({ ((*v3.borrow()).capacity() == 200_usize) }));
    assert!(({ ((*v3.borrow()).len() == 100_usize) }));
    if 50_usize > (*v3.borrow()).capacity() as usize {
        let len_0 = (*v3.borrow()).len();
        (*v3.borrow_mut()).reserve_exact(50_usize - len_0 as usize);
    };
    assert!(({ ((*v3.borrow()).capacity() == 200_usize) }));
    assert!(({ ((*v3.borrow()).len() == 100_usize) }));
    if 200_usize > (*v3.borrow()).capacity() as usize {
        let len_0 = (*v3.borrow()).len();
        (*v3.borrow_mut()).reserve_exact(200_usize - len_0 as usize);
    };
    assert!(({ ((*v3.borrow()).capacity() == 200_usize) }));
    assert!(({ ((*v3.borrow()).len() == 100_usize) }));
    if 201_usize > (*v3.borrow()).capacity() as usize {
        let len_0 = (*v3.borrow()).len();
        (*v3.borrow_mut()).reserve_exact(201_usize - len_0 as usize);
    };
    assert!(({ ((*v3.borrow()).capacity() == 201_usize) }));
    assert!(({ ((*v3.borrow()).len() == 100_usize) }));
    assert!(((v2.decay().to_last().read()) == 3));
    assert!(((v3.decay().to_last().read()) == 1));
    assert!((v4.decay().to_last().read()).is_null());
    assert!((v5.decay().to_last().read()).is_null());
    assert!(((v6.decay().to_last().read()) == 2_f64));
    let ref0: Ptr<f64> = v6.decay().to_last();
    ref0.write(5_f64);
    assert!(((v6.decay().to_last().read()) == 5_f64));
    let mut x0: f64 = (v6.decay().to_last().read());
    assert!((x0 == 5_f64));
    x0 = 6_f64;
    assert!(((v6.decay().to_last().read()) == 5_f64));
    let mut idx: i32 = 0;
    assert!(
        ((match ({ (idx as usize) },) {
            (mut __tmp_11,) => {
                if __tmp_11 >= v6.decay().len() {
                    panic!("out of bounds access")
                } else {
                    v6.decay().offset(__tmp_11 as isize)
                }
            }
        }
        .read())
            == 2_f64)
    );
    assert!(
        ((match ({ (s2).wrapping_sub(1_usize) },) {
            (mut __tmp_12,) => {
                if __tmp_12 >= v6.decay().len() {
                    panic!("out of bounds access")
                } else {
                    v6.decay().offset(__tmp_12 as isize)
                }
            }
        }
        .read())
            == 5_f64)
    );
    let ref1: Ptr<f64> = match ({ (s2).wrapping_sub(1_usize) },) {
        (mut __tmp_13,) => {
            if __tmp_13 >= v6.decay().len() {
                panic!("out of bounds access")
            } else {
                v6.decay().offset(__tmp_13 as isize)
            }
        }
    };
    {
        ref1.with_mut(|__v| *__v = *__v + 1.5_f64)
    };
    assert!(
        ((match ({ (s2).wrapping_sub(1_usize) },) {
            (mut __tmp_14,) => {
                if __tmp_14 >= v6.decay().len() {
                    panic!("out of bounds access")
                } else {
                    v6.decay().offset(__tmp_14 as isize)
                }
            }
        }
        .read())
            == 6.5_f64)
    );
    let mut x1: f64 = (match ({ (s2).wrapping_sub(1_usize) },) {
        (mut __tmp_15,) => {
            if __tmp_15 >= v6.decay().len() {
                panic!("out of bounds access")
            } else {
                v6.decay().offset(__tmp_15 as isize)
            }
        }
    }
    .read());
    assert!((x1 == 6.5_f64));
    x1 -= 1.5_f64;
    assert!(
        ((match ({ (s2).wrapping_sub(1_usize) },) {
            (mut __tmp_16,) => {
                if __tmp_16 >= v6.decay().len() {
                    panic!("out of bounds access")
                } else {
                    v6.decay().offset(__tmp_16 as isize)
                }
            }
        }
        .read())
            == 6.5_f64)
    );
    assert!(
        (((s1).wrapping_add(s2)).wrapping_add(
            ((if 0_usize >= v2.decay().len() {
                panic!("out of bounds access")
            } else {
                v2.decay().offset(0_usize as isize)
            }
            .read()) as usize)
        ) == 103_usize)
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
    assert!(({ ({ (*a.borrow()).x } == 2) }));
    let v10: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1, 2, 3])));
    let v11: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([4])));
    v10.as_pointer()
        .swap_with(&v11.as_pointer(), |__v| __v.with_mut(std::mem::take));
    assert!(
        ({
            ((((*v10.borrow()).len() == 1_usize) && ({ (*v10.borrow())[0_usize] } == 4))
                && ((*v11.borrow()).len() == 3_usize))
                && ({ (*v11.borrow())[2_usize] } == 3)
        })
    );
    let foo: Value<Foo> = Rc::new(RefCell::new(<Foo>::default()));
    {
        let __a1 = 5;
        (*foo.borrow_mut()).a.push(__a1)
    };
    {
        let __a1 = 6;
        (*foo.borrow_mut()).b.push(__a1)
    };
    {
        let __a1 = 7;
        (*foo.borrow_mut()).b.push(__a1)
    };
    field_ref!(foo.as_pointer(), a)
        .as_pointer()
        .swap_with(&field_ref!(foo.as_pointer(), b).as_pointer(), |__v| {
            __v.with_mut(std::mem::take)
        });
    assert!(
        ({
            (((*foo.borrow()).a.len() == 2_usize)
                && ((vec_elem!(foo.as_pointer(), a, 1_usize).read()) == 7))
                && ((vec_elem!(foo.as_pointer(), b, 0_usize).read()) == 5)
        })
    );
    ({
        exchange_1(
            field_ref!(foo.as_pointer(), a).as_pointer(),
            v10.as_pointer(),
        )
    });
    assert!(
        ({
            (((*foo.borrow()).a.len() == 1_usize)
                && ((vec_elem!(foo.as_pointer(), a, 0_usize).read()) == 4))
                && ({ (*v10.borrow())[1_usize] } == 7)
        })
    );
    return 0;
}
pub trait FooImpl {
    fn move_assign(&self, _a0: Ptr<Foo>) -> Ptr<Foo>;
}
impl FooImpl for Ptr<Foo> {
    fn move_assign(&self, _a0: Ptr<Foo>) -> Ptr<Foo> {
        field_ref!((*self), a)
            .as_pointer()
            .write(field!(_a0, a).with_mut(std::mem::take));
        field_ref!((*self), b)
            .as_pointer()
            .write(field!(_a0, b).with_mut(std::mem::take));
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
