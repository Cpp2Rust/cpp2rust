extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub x: i32,
}
pub unsafe fn copy_0(mut copy_vector: Vec<i32>) {}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Foo {
    pub a: Vec<i32>,
    pub b: Vec<i32>,
}
impl Foo {
    pub unsafe fn move_from(_a0: *mut Foo) -> Self {
        let mut this = Self {
            a: std::mem::take(&mut (*_a0).a),
            b: std::mem::take(&mut (*_a0).b),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Foo) -> *mut Foo {
        {
            self.a = std::mem::take(&mut (*_a0).a);
        };
        {
            self.b = std::mem::take(&mut (*_a0).b);
        };
        return &mut (*(self as *mut Foo));
    }
}
pub unsafe fn exchange_1(x: *mut Vec<i32>, y: *mut Vec<i32>) {
    std::mem::swap(&mut (*x), &mut (*y));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v1: Vec<i32> = Vec::new();
    assert!(((v1.len()) == (0_usize)));
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
    assert!(((v1.len()) == (100_usize)));
    assert!(((v1[(99_usize)]) == (0)));
    v1[(0_usize)] = 40;
    v1[(99_usize)] = 50;
    assert!(((v1[(0_usize)]) == (40)));
    assert!(((v1[(99_usize)]) == (50)));
    let mut v2: Vec<i32> = Vec::new();
    assert!(((v2.len()) == (0_usize)));
    {
        let __a1 = 1;
        v2.push(__a1)
    };
    {
        let __a1 = 2;
        v2.push(__a1)
    };
    {
        let __a1 = 3;
        v2.push(__a1)
    };
    assert!(((v2.len()) == (3_usize)));
    {
        let mut __tmp_0 = { <UnsafeRandomAccessIterator<i32>>::from(v2.begin()) };
        let pos = __tmp_0.iter_distance(&UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr())) as usize;
        v2.remove(pos);
        (__tmp_0).clone()
    };
    assert!(((v2.len()) == (2_usize)));
    assert!(((v2[(0_usize)]) == (2)));
    assert!(((v2[(1_usize)]) == (3)));
    {
        let pos = <UnsafeRandomAccessIterator<i32>>::from(v2.begin())
            .iter_distance(&UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr()))
            as usize;
        UnsafeRandomAccessIterator::Vec(v2.insert_mut(pos, 100))
    };
    (unsafe { copy_0(v2.clone()) });
    assert!(((v2.len()) == (3_usize)));
    assert!(((v2[(0_usize)]) == (100)));
    assert!(((v2[(1_usize)]) == (2)));
    assert!(((v2[(2_usize)]) == (3)));
    let mut x: i32 = 7;
    {
        let pos = <UnsafeRandomAccessIterator<i32>>::from(v2.begin().iter_add(1_isize))
            .iter_distance(&UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr()))
            as usize;
        UnsafeRandomAccessIterator::Vec(v2.insert_mut(pos, x))
    };
    assert!(((v2.len()) == (4_usize)));
    assert!(((v2[(0_usize)]) == (100)));
    assert!(((v2[(1_usize)]) == (7)));
    assert!(((v2[(2_usize)]) == (2)));
    {
        let __off = <UnsafeRandomAccessIterator<i32>>::from(v2.end())
            .iter_distance(&UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr()))
            as usize;
        v2.splice(__off..__off, std::iter::repeat_n(9, 2_usize));
        UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr().add(__off))
    };
    assert!(((v2.len()) == (6_usize)));
    assert!(((v2[(4_usize)]) == (9)));
    assert!(((v2[(5_usize)]) == (9)));
    {
        let __off = <UnsafeRandomAccessIterator<i32>>::from(v2.begin().iter_add(2_isize))
            .iter_distance(&UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr()))
            as usize;
        v2.splice(__off..__off, Vec::from([11, 12, 13]));
        UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr().add(__off))
    };
    assert!(((v2.len()) == (9_usize)));
    assert!(((v2[(1_usize)]) == (7)));
    assert!(((v2[(2_usize)]) == (11)));
    assert!(((v2[(3_usize)]) == (12)));
    assert!(((v2[(4_usize)]) == (13)));
    assert!(((v2[(5_usize)]) == (2)));
    assert!(((v2[(8_usize)]) == (9)));
    let mut src: Vec<i32> = Vec::from([21, 22]);
    {
        let mut __tmp_1 = { <UnsafeRandomAccessIterator<i32>>::from(v2.begin()) };
        let mut __tmp_2 = { src.begin() };
        let mut __tmp_3 = { src.end() };
        let __off =
            __tmp_1.iter_distance(&UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr())) as usize;
        let mut __tmp = Vec::new();
        let mut __it = (__tmp_2).clone();
        while __it != __tmp_3 {
            __tmp.push((*__it.iter_deref()).clone());
            __it.iter_inc();
        }
        v2.splice(__off..__off, __tmp);
        UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr().add(__off))
    };
    assert!(((v2.len()) == (11_usize)));
    assert!(((v2[(0_usize)]) == (21)));
    assert!(((v2[(1_usize)]) == (22)));
    assert!(((v2[(2_usize)]) == (100)));
    {
        let mut __tmp_4 = { <UnsafeRandomAccessIterator<i32>>::from(v2.begin()) };
        let pos = __tmp_4.iter_distance(&UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr())) as usize;
        v2.remove(pos);
        (__tmp_4).clone()
    };
    {
        let mut __tmp_5 = { <UnsafeRandomAccessIterator<i32>>::from(v2.begin()) };
        let pos = __tmp_5.iter_distance(&UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr())) as usize;
        v2.remove(pos);
        (__tmp_5).clone()
    };
    {
        let mut i: i32 = 0;
        'loop_: while ((i) < (4)) {
            'for_body: {
                {
                    let mut __tmp_6 =
                        { <UnsafeRandomAccessIterator<i32>>::from(v2.begin().iter_add(1_isize)) };
                    let pos = __tmp_6
                        .iter_distance(&UnsafeRandomAccessIterator::Vec(v2.as_mut_ptr()))
                        as usize;
                    v2.remove(pos);
                    (__tmp_6).clone()
                };
            }
            i.prefix_inc();
        }
    }
    v2.pop();
    v2.pop();
    assert!(((v2.len()) == (3_usize)));
    assert!(((v2[(0_usize)]) == (100)));
    assert!(((v2[(1_usize)]) == (2)));
    assert!(((v2[(2_usize)]) == (3)));
    let mut s2: usize = v2.len();
    let mut v3: Vec<i32> = vec![1; 100_usize];
    assert!(((v3.len()) == (100_usize)));
    {
        let mut i: i32 = 0;
        'loop_: while ((i) < (100)) {
            'for_body: {
                assert!(((v3[(i as usize)]) == (1)));
            }
            i.prefix_inc();
        }
    }
    let mut v4: Vec<*mut i32> = Vec::from_fn(100_usize, |_| std::ptr::null_mut());
    assert!(((v4.len()) == (100_usize)));
    {
        let mut i: u32 = 0_u32;
        'loop_: while ((i as usize) < (v4.len())) {
            'for_body: {
                assert!((v4[(i as usize)]).is_null());
            }
            i.prefix_inc();
        }
    }
    let mut v5: Vec<*const i32> = Vec::from_fn(100_usize, |_| std::ptr::null());
    assert!(((v5.len()) == (100_usize)));
    {
        let mut i: u32 = 0_u32;
        'loop_: while ((i as usize) < (v5.len())) {
            'for_body: {
                assert!((v5[(i as usize)]).is_null());
            }
            i.prefix_inc();
        }
    }
    let mut v6: Vec<f64> = vec![2_f64; s2];
    assert!(((v6.len()) == (s2)));
    {
        let mut i: u32 = 0_u32;
        'loop_: while ((i as usize) < (s2)) {
            'for_body: {
                assert!(((v6[(i as usize)]) == (2_f64)));
            }
            i.prefix_inc();
        }
    }
    let mut v7: Vec<(*const i32, i32)> =
        Vec::from_fn(200_usize, |_| <(*const i32, i32)>::default());
    assert!(((v7.len()) == (200_usize)));
    {
        let mut i: u32 = 0_u32;
        'loop_: while ((i) < (200_u32)) {
            'for_body: {
                assert!(((v7[(i as usize)].0).is_null()) && ((v7[(i as usize)].1) == (0)));
            }
            i.prefix_inc();
        }
    }
    let mut p1: *const f64 = (v6.as_mut_ptr()).cast_const();
    assert!(((*p1) == (2_f64)));
    let mut p2: *mut i32 = v3.as_mut_ptr();
    assert!(((*p2) == (1)));
    assert!(((v3[(0_usize)]) == (1)));
    assert!(((v3[(1_usize)]) == (1)));
    (*p2) = (99_f64 as i32);
    assert!(((*p2) == (99)));
    assert!(((v3[(0_usize)]) == (99)));
    assert!(((v3[(1_usize)]) == (1)));
    p2.prefix_inc();
    (*p2) = 98;
    assert!(((v3[(0_usize)]) == (99)));
    assert!(((v3[(1_usize)]) == (98)));
    assert!(((v3.capacity()) == (100_usize)));
    assert!(((v3.len()) == (100_usize)));
    if 200_usize > v3.capacity() as usize {
        let len_0 = v3.len();
        v3.reserve_exact(200_usize - len_0 as usize);
    };
    assert!(((v3.capacity()) == (200_usize)));
    assert!(((v3.len()) == (100_usize)));
    if 50_usize > v3.capacity() as usize {
        let len_0 = v3.len();
        v3.reserve_exact(50_usize - len_0 as usize);
    };
    assert!(((v3.capacity()) == (200_usize)));
    assert!(((v3.len()) == (100_usize)));
    if 200_usize > v3.capacity() as usize {
        let len_0 = v3.len();
        v3.reserve_exact(200_usize - len_0 as usize);
    };
    assert!(((v3.capacity()) == (200_usize)));
    assert!(((v3.len()) == (100_usize)));
    if 201_usize > v3.capacity() as usize {
        let len_0 = v3.len();
        v3.reserve_exact(201_usize - len_0 as usize);
    };
    assert!(((v3.capacity()) == (201_usize)));
    assert!(((v3.len()) == (100_usize)));
    assert!(
        ((*{
            let __n = v2.len() - 1;
            &mut v2[__n]
        }) == (3))
    );
    assert!(
        ((*{
            let __n = v3.len() - 1;
            &mut v3[__n]
        }) == (1))
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
        ((*{
            let __n = v6.len() - 1;
            &mut v6[__n]
        }) == (2_f64))
    );
    let ref0: *mut f64 = {
        let __n = v6.len() - 1;
        &raw mut v6[__n]
    };
    (*ref0) = 5_f64;
    assert!(
        ((*{
            let __n = v6.len() - 1;
            &mut v6[__n]
        }) == (5_f64))
    );
    let mut x0: f64 = (*{
        let __n = v6.len() - 1;
        &mut v6[__n]
    });
    assert!(((x0) == (5_f64)));
    x0 = 6_f64;
    assert!(
        ((*{
            let __n = v6.len() - 1;
            &mut v6[__n]
        }) == (5_f64))
    );
    let mut idx: i32 = 0;
    assert!(((v6[(idx as usize)]) == (2_f64)));
    assert!(((v6[(s2).wrapping_sub(1_usize)]) == (5_f64)));
    let ref1: *mut f64 = &raw mut v6[(s2).wrapping_sub(1_usize)];
    (*ref1) += 1.5_f64;
    assert!(((v6[(s2).wrapping_sub(1_usize)]) == (6.5_f64)));
    let mut x1: f64 = v6[(s2).wrapping_sub(1_usize)];
    assert!(((x1) == (6.5_f64)));
    x1 -= 1.5_f64;
    assert!(((v6[(s2).wrapping_sub(1_usize)]) == (6.5_f64)));
    assert!(((((s1).wrapping_add(s2)).wrapping_add(((v2[0_usize]) as usize))) == (103_usize)));
    let mut a: S = S { x: 1 };
    let mut pa: *mut S = (&mut a as *mut S);
    let mut v8: Vec<*const *mut S> = Vec::new();
    {
        let __a1 = (&mut pa as *mut *mut S).cast_const();
        v8.push(__a1)
    };
    assert!((((*(*v8[(0_usize)])).x) == (1)));
    let mut ppa: *mut *mut S = (&mut pa as *mut *mut S);
    let mut v9: Vec<*mut *mut *mut S> = Vec::new();
    {
        let __a1 = (&mut ppa as *mut *mut *mut S);
        v9.push(__a1)
    };
    (*(*(*v9[(0_usize)]))).x = 2;
    assert!(((a.x) == (2)));
    let mut v10: Vec<i32> = Vec::from([1, 2, 3]);
    let mut v11: Vec<i32> = Vec::from([4]);
    std::mem::swap(&mut v10, &mut v11);
    assert!(
        ((((v10.len()) == (1_usize)) && ((v10[(0_usize)]) == (4))) && ((v11.len()) == (3_usize)))
            && ((v11[(2_usize)]) == (3))
    );
    let mut foo: Foo = <Foo>::default();
    {
        let __a1 = 5;
        foo.a.push(__a1)
    };
    {
        let __a1 = 6;
        foo.b.push(__a1)
    };
    {
        let __a1 = 7;
        foo.b.push(__a1)
    };
    std::mem::swap(&mut foo.a, &mut foo.b);
    assert!(
        (((foo.a.len()) == (2_usize)) && ((foo.a[(1_usize)]) == (7)))
            && ((foo.b[(0_usize)]) == (5))
    );
    (unsafe { exchange_1(&mut foo.a, &mut v10) });
    assert!(
        (((foo.a.len()) == (1_usize)) && ((foo.a[(0_usize)]) == (4))) && ((v10[(1_usize)]) == (7))
    );
    let mut v12: Vec<i32> = Vec::new();
    let mut moved: i32 = 5;
    {
        let __a1 = moved;
        v12.push(__a1)
    };
    assert!(((v12[(0_usize)]) == (5)) && ((moved) == (5)));
    let mut v13: Vec<*mut i32> = Vec::new();
    let mut moved_ptr: *mut i32 = (&mut moved as *mut i32);
    {
        let __a1 = moved_ptr;
        v13.push(__a1)
    };
    assert!(
        ((v13[(0_usize)]) == (&mut moved as *mut i32)) && ((moved_ptr) == (&mut moved as *mut i32))
    );
    let mut v14: Vec<i32> = Vec::from([1, 2, 3]);
    v14[0_usize] = ((v14[1_usize])
        + (*{
            let __n = v14.len() - 1;
            &mut v14[__n]
        }));
    v14[0] += 1;
    assert!(((v14[(0_usize)]) == (6)) && ((v14[2_usize]) == (3)));
    let mut v15: Vec<i32> = Vec::from([1, 2, 3]);
    let front: *mut i32 = &raw mut v15[0];
    (*front) += 1;
    let mut vp: *mut Vec<i32> = (&mut v15 as *mut Vec<i32>);
    (*{
        let __n = (*vp).len() - 1;
        &mut (&mut (*vp))[__n]
    }) = (&mut (*vp))[0_usize];
    let mut last: *mut i32 = ({
        let __n = (*vp).len() - 1;
        &raw mut (&mut (*vp))[__n]
    });
    (*last) += 1;
    assert!(((v15[(0_usize)]) == (2)) && ((v15[(2_usize)]) == (3)));
    let mut v16: Vec<i32> = Vec::from([1, 2, 3, 4, 5, 6, 7, 8]);
    let mut after_v16: Vec<i32> = Vec::from([0]);
    let mut inserted: UnsafeRandomAccessIterator<i32> = {
        let pos = <UnsafeRandomAccessIterator<i32>>::from(v16.begin().iter_add(1_isize))
            .iter_distance(&UnsafeRandomAccessIterator::Vec(v16.as_mut_ptr()))
            as usize;
        UnsafeRandomAccessIterator::Vec(v16.insert_mut(pos, 7))
    };
    (*inserted.iter_deref()) += 1;
    let mut nine: i32 = 9;
    let mut inserted_copy: UnsafeRandomAccessIterator<i32> = {
        let pos = <UnsafeRandomAccessIterator<i32>>::from(v16.begin())
            .iter_distance(&UnsafeRandomAccessIterator::Vec(v16.as_mut_ptr()))
            as usize;
        UnsafeRandomAccessIterator::Vec(v16.insert_mut(pos, nine))
    };
    (*inserted_copy.iter_deref()) += 1;
    assert!(
        (((v16[(0_usize)]) == (10)) && ((v16[(2_usize)]) == (8)))
            && ((after_v16[(0_usize)]) == (0))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
