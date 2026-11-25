extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
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
    libcc2rs::cpp2rust_exit(main_0());
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
    v1.resize_with(100_usize, || 0_i32);
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
    {
        let mut __tmp_0 = { <RandomAccessIterator<i32>>::from(v2.decay().begin()) };
        v2.as_pointer()
            .with_mut(|__v: &mut Vec<i32>| __v.remove(__tmp_0.index()));
        (__tmp_0).clone()
    };
    assert!(({ ((*v2.borrow()).len() == 2_usize) }));
    assert!(({ ({ (*v2.borrow())[0_usize] } == 2) }));
    assert!(({ ({ (*v2.borrow())[1_usize] } == 3) }));
    {
        let mut __tmp_1 = { <RandomAccessIterator<i32>>::from(v2.decay().begin()) };
        (*v2.borrow_mut()).insert(__tmp_1.index(), 100);
        (__tmp_1).clone()
    };
    ({ copy_0((*v2.borrow()).clone()) });
    assert!(({ ((*v2.borrow()).len() == 3_usize) }));
    assert!(({ ({ (*v2.borrow())[0_usize] } == 100) }));
    assert!(({ ({ (*v2.borrow())[1_usize] } == 2) }));
    assert!(({ ({ (*v2.borrow())[2_usize] } == 3) }));
    let mut x: i32 = 7;
    {
        let mut __tmp_2 =
            { <RandomAccessIterator<i32>>::from(v2.decay().begin().iter_add(1_isize)) };
        (*v2.borrow_mut()).insert(__tmp_2.index(), x);
        (__tmp_2).clone()
    };
    assert!(({ ((*v2.borrow()).len() == 4_usize) }));
    assert!(({ ({ (*v2.borrow())[0_usize] } == 100) }));
    assert!(({ ({ (*v2.borrow())[1_usize] } == 7) }));
    assert!(({ ({ (*v2.borrow())[2_usize] } == 2) }));
    {
        let mut __tmp_3 = { <RandomAccessIterator<i32>>::from(v2.decay().end()) };
        let __off = __tmp_3.index();
        (*v2.borrow_mut()).splice(__off..__off, std::iter::repeat_n(9, 2_usize));
        (__tmp_3).clone()
    };
    assert!(({ ((*v2.borrow()).len() == 6_usize) }));
    assert!(({ ({ (*v2.borrow())[4_usize] } == 9) }));
    assert!(({ ({ (*v2.borrow())[5_usize] } == 9) }));
    {
        let mut __tmp_4 =
            { <RandomAccessIterator<i32>>::from(v2.decay().begin().iter_add(2_isize)) };
        let __off = __tmp_4.index();
        (*v2.borrow_mut()).splice(__off..__off, Vec::from([11, 12, 13]));
        (__tmp_4).clone()
    };
    assert!(({ ((*v2.borrow()).len() == 9_usize) }));
    assert!(({ ({ (*v2.borrow())[1_usize] } == 7) }));
    assert!(({ ({ (*v2.borrow())[2_usize] } == 11) }));
    assert!(({ ({ (*v2.borrow())[3_usize] } == 12) }));
    assert!(({ ({ (*v2.borrow())[4_usize] } == 13) }));
    assert!(({ ({ (*v2.borrow())[5_usize] } == 2) }));
    assert!(({ ({ (*v2.borrow())[8_usize] } == 9) }));
    let src: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([21, 22])));
    {
        let mut __tmp_5 = { <RandomAccessIterator<i32>>::from(v2.decay().begin()) };
        let mut __tmp_6 = { src.decay().begin() };
        let mut __tmp_7 = { src.decay().end() };
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
    };
    assert!(({ ((*v2.borrow()).len() == 11_usize) }));
    assert!(({ ({ (*v2.borrow())[0_usize] } == 21) }));
    assert!(({ ({ (*v2.borrow())[1_usize] } == 22) }));
    assert!(({ ({ (*v2.borrow())[2_usize] } == 100) }));
    {
        let mut __tmp_8 = { <RandomAccessIterator<i32>>::from(v2.decay().begin()) };
        v2.as_pointer()
            .with_mut(|__v: &mut Vec<i32>| __v.remove(__tmp_8.index()));
        (__tmp_8).clone()
    };
    {
        let mut __tmp_9 = { <RandomAccessIterator<i32>>::from(v2.decay().begin()) };
        v2.as_pointer()
            .with_mut(|__v: &mut Vec<i32>| __v.remove(__tmp_9.index()));
        (__tmp_9).clone()
    };
    {
        let mut i: i32 = 0;
        'loop_: while (i < 4) {
            'for_body: {
                {
                    let mut __tmp_10 =
                        { <RandomAccessIterator<i32>>::from(v2.decay().begin().iter_add(1_isize)) };
                    v2.as_pointer()
                        .with_mut(|__v: &mut Vec<i32>| __v.remove(__tmp_10.index()));
                    (__tmp_10).clone()
                };
            }
            i.prefix_inc();
        }
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
    {
        let mut i: i32 = 0;
        'loop_: while (i < 100) {
            'for_body: {
                assert!(({ ({ (*v3.borrow())[(i as usize)] } == 1) }));
            }
            i.prefix_inc();
        }
    }
    let mut v4: Vec<Ptr<i32>> = Vec::from_fn(100_usize, |_| Ptr::<i32>::null());
    assert!((v4.len() == 100_usize));
    {
        let mut i: u32 = 0_u32;
        'loop_: while ((i as usize) < v4.len()) {
            'for_body: {
                assert!((v4[(i as usize)]).is_null());
            }
            i.prefix_inc();
        }
    }
    let mut v5: Vec<Ptr<i32>> = Vec::from_fn(100_usize, |_| Ptr::<i32>::null());
    assert!((v5.len() == 100_usize));
    {
        let mut i: u32 = 0_u32;
        'loop_: while ((i as usize) < v5.len()) {
            'for_body: {
                assert!((v5[(i as usize)]).is_null());
            }
            i.prefix_inc();
        }
    }
    let v6: Value<Vec<f64>> = Rc::new(RefCell::new(vec![2_f64; s2]));
    assert!(({ ((*v6.borrow()).len() == s2) }));
    {
        let mut i: u32 = 0_u32;
        'loop_: while ((i as usize) < s2) {
            'for_body: {
                assert!(({ ({ (*v6.borrow())[(i as usize)] } == 2_f64) }));
            }
            i.prefix_inc();
        }
    }
    let v7: Value<Vec<(Value<Ptr<i32>>, Value<i32>)>> =
        Rc::new(RefCell::new(Vec::from_fn(200_usize, |_| {
            (
                Rc::new(RefCell::new(Ptr::<i32>::null())),
                Rc::new(RefCell::new(0_i32)),
            )
        })));
    assert!(({ ((*v7.borrow()).len() == 200_usize) }));
    {
        let mut i: u32 = 0_u32;
        'loop_: while (i < 200_u32) {
            'for_body: {
                assert!(
                    ({
                        ((*(*v7.borrow())[(i as usize)].0.borrow()).is_null())
                            && ((*(*v7.borrow())[(i as usize)].1.borrow()) == 0)
                    })
                );
            }
            i.prefix_inc();
        }
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
    v3.with_mut(|__b0| {
        if 200_usize > (*__b0).capacity() as usize {
            let len_0 = (*__b0).len();
            (*__b0).reserve_exact(200_usize - len_0 as usize);
        }
    });
    assert!(({ ((*v3.borrow()).capacity() == 200_usize) }));
    assert!(({ ((*v3.borrow()).len() == 100_usize) }));
    v3.with_mut(|__b0| {
        if 50_usize > (*__b0).capacity() as usize {
            let len_0 = (*__b0).len();
            (*__b0).reserve_exact(50_usize - len_0 as usize);
        }
    });
    assert!(({ ((*v3.borrow()).capacity() == 200_usize) }));
    assert!(({ ((*v3.borrow()).len() == 100_usize) }));
    v3.with_mut(|__b0| {
        if 200_usize > (*__b0).capacity() as usize {
            let len_0 = (*__b0).len();
            (*__b0).reserve_exact(200_usize - len_0 as usize);
        }
    });
    assert!(({ ((*v3.borrow()).capacity() == 200_usize) }));
    assert!(({ ((*v3.borrow()).len() == 100_usize) }));
    v3.with_mut(|__b0| {
        if 201_usize > (*__b0).capacity() as usize {
            let len_0 = (*__b0).len();
            (*__b0).reserve_exact(201_usize - len_0 as usize);
        }
    });
    assert!(({ ((*v3.borrow()).capacity() == 201_usize) }));
    assert!(({ ((*v3.borrow()).len() == 100_usize) }));
    assert!(
        ({
            (({
                let __n = (*v2.borrow()).len() - 1;
                v2.decay().offset((__n) as isize)
            }
            .read())
                == 3)
        })
    );
    assert!(
        ({
            (({
                let __n = (*v3.borrow()).len() - 1;
                v3.decay().offset((__n) as isize)
            }
            .read())
                == 1)
        })
    );
    assert!(
        (*{
            let __n = v4.len() - 1;
            &mut v4[__n]
        })
        .is_null()
    );
    assert!(
        (*{
            let __n = v5.len() - 1;
            &mut v5[__n]
        })
        .is_null()
    );
    assert!(
        ({
            (({
                let __n = (*v6.borrow()).len() - 1;
                v6.decay().offset((__n) as isize)
            }
            .read())
                == 2_f64)
        })
    );
    let ref0: Ptr<f64> = {
        let __n = (*v6.borrow()).len() - 1;
        v6.decay().offset((__n) as isize)
    };
    ref0.write(5_f64);
    assert!(
        ({
            (({
                let __n = (*v6.borrow()).len() - 1;
                v6.decay().offset((__n) as isize)
            }
            .read())
                == 5_f64)
        })
    );
    let mut x0: f64 = ({
        let __n = (*v6.borrow()).len() - 1;
        v6.decay().offset((__n) as isize)
    }
    .read());
    assert!((x0 == 5_f64));
    x0 = 6_f64;
    assert!(
        ({
            (({
                let __n = (*v6.borrow()).len() - 1;
                v6.decay().offset((__n) as isize)
            }
            .read())
                == 5_f64)
        })
    );
    let mut idx: i32 = 0;
    assert!(
        ({
            (({
                let mut __tmp_11 = { (idx as usize) };
                if __tmp_11 >= (*v6.borrow()).len() {
                    panic!("out of bounds access")
                }
                v6.decay().offset((__tmp_11) as isize)
            }
            .read())
                == 2_f64)
        })
    );
    assert!(
        ({
            (({
                let mut __tmp_12 = { (s2).wrapping_sub(1_usize) };
                if __tmp_12 >= (*v6.borrow()).len() {
                    panic!("out of bounds access")
                }
                v6.decay().offset((__tmp_12) as isize)
            }
            .read())
                == 5_f64)
        })
    );
    let ref1: Ptr<f64> = {
        let mut __tmp_13 = { (s2).wrapping_sub(1_usize) };
        if __tmp_13 >= (*v6.borrow()).len() {
            panic!("out of bounds access")
        }
        v6.decay().offset((__tmp_13) as isize)
    };
    {
        ref1.with_mut(|__v| *__v = *__v + 1.5_f64)
    };
    assert!(
        ({
            (({
                let mut __tmp_14 = { (s2).wrapping_sub(1_usize) };
                if __tmp_14 >= (*v6.borrow()).len() {
                    panic!("out of bounds access")
                }
                v6.decay().offset((__tmp_14) as isize)
            }
            .read())
                == 6.5_f64)
        })
    );
    let mut x1: f64 = ({
        let mut __tmp_15 = { (s2).wrapping_sub(1_usize) };
        if __tmp_15 >= (*v6.borrow()).len() {
            panic!("out of bounds access")
        }
        v6.decay().offset((__tmp_15) as isize)
    }
    .read());
    assert!((x1 == 6.5_f64));
    x1 -= 1.5_f64;
    assert!(
        ({
            (({
                let mut __tmp_16 = { (s2).wrapping_sub(1_usize) };
                if __tmp_16 >= (*v6.borrow()).len() {
                    panic!("out of bounds access")
                }
                v6.decay().offset((__tmp_16) as isize)
            }
            .read())
                == 6.5_f64)
        })
    );
    assert!(
        ({
            (((s1).wrapping_add(s2)).wrapping_add(
                (({
                    if 0_usize >= (*v2.borrow()).len() {
                        panic!("out of bounds access")
                    }
                    v2.decay().offset((0_usize) as isize)
                }
                .read()) as usize),
            ) == 103_usize)
        })
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
    let mut v12: Vec<i32> = Vec::new();
    let moved: Value<i32> = Rc::new(RefCell::new(5));
    {
        let __a1 = ({ (*moved.borrow()) });
        v12.push(__a1)
    };
    assert!(({ (v12[0_usize] == 5) && ((*moved.borrow()) == 5) }));
    let mut v13: Vec<Ptr<i32>> = Vec::new();
    let mut moved_ptr: Ptr<i32> = (moved.as_pointer());
    {
        let __a1 = (moved_ptr).clone();
        v13.push(__a1)
    };
    assert!(
        (v13[0_usize] == (moved.as_pointer()))
            && ({ (moved_ptr).clone() } == { (moved.as_pointer()) })
    );
    let mut v14: Vec<i32> = Vec::from([1, 2, 3]);
    let __rhs = ((*{
        if 1_usize >= v14.len() {
            panic!("out of bounds access")
        }
        &mut v14[1_usize]
    }) + (*{
        let __n = v14.len() - 1;
        &mut v14[__n]
    }));
    (*{
        if 0_usize >= v14.len() {
            panic!("out of bounds access")
        }
        &mut v14[0_usize]
    }) = __rhs;
    v14[0] += 1;
    assert!(
        (v14[0_usize] == 6)
            && ((*{
                if 2_usize >= v14.len() {
                    panic!("out of bounds access")
                }
                &mut v14[2_usize]
            }) == 3)
    );
    let v15: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1, 2, 3])));
    let front: Ptr<i32> = v15.decay();
    {
        front.with_mut(|__v| *__v = *__v + 1)
    };
    let mut vp: Ptr<Vec<i32>> = (v15.as_pointer());
    let __rhs = ({
        if 0_usize >= (*vp.upgrade().deref()).len() {
            panic!("out of bounds access")
        }
        vp.decay().offset((0_usize) as isize)
    }
    .read());
    {
        let __n = (*vp.upgrade().deref()).len() - 1;
        vp.decay().offset((__n) as isize)
    }
    .write(__rhs);
    let mut last: Ptr<i32> = ({
        let __n = (*vp.upgrade().deref()).len() - 1;
        vp.decay().offset((__n) as isize)
    });
    {
        last.with_mut(|__v| *__v = *__v + 1)
    };
    assert!(({ ({ (*v15.borrow())[0_usize] } == 2) && ({ (*v15.borrow())[2_usize] } == 3) }));
    let v16: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1, 2, 3, 4, 5, 6, 7, 8])));
    let mut after_v16: Vec<i32> = Vec::from([0]);
    let inserted: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new({
        let mut __tmp_17 =
            { <RandomAccessIterator<i32>>::from(v16.decay().begin().iter_add(1_isize)) };
        (*v16.borrow_mut()).insert(__tmp_17.index(), 7);
        (__tmp_17).clone()
    }));
    {
        ({ (*inserted.borrow()).iter_deref() }).with_mut(|__v| *__v = *__v + 1)
    };
    let mut nine: i32 = 9;
    let inserted_copy: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new({
        let mut __tmp_18 = { <RandomAccessIterator<i32>>::from(v16.decay().begin()) };
        (*v16.borrow_mut()).insert(__tmp_18.index(), nine);
        (__tmp_18).clone()
    }));
    {
        ({ (*inserted_copy.borrow()).iter_deref() }).with_mut(|__v| *__v = *__v + 1)
    };
    assert!(
        ({
            (({ (*v16.borrow())[0_usize] } == 10) && ({ (*v16.borrow())[2_usize] } == 8))
                && (after_v16[0_usize] == 0)
        })
    );
    return 0;
}
pub trait FooImpl {
    fn move_assign(&self, _a0: Ptr<Foo>) -> Ptr<Foo>;
}
impl FooImpl for Ptr<Foo> {
    fn move_assign(&self, _a0: Ptr<Foo>) -> Ptr<Foo> {
        field!((*self), a).with_mut(|__b0| {
            (*__b0) = field!(_a0, a).with_mut(std::mem::take);
        });
        field!((*self), b).with_mut(|__b0| {
            (*__b0) = field!(_a0, b).with_mut(std::mem::take);
        });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
