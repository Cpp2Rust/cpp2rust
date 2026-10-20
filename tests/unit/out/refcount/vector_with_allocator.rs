extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestAllocator_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestAllocator_double_ {}
pub fn copy_0(mut copy_vector: Vec<i32>) {}
pub fn fn_1(v: Ptr<Vec<i32>>, v3: Vec<i32>) {
    let v3: Value<Vec<i32>> = Rc::new(RefCell::new(v3));
    {
        let __a1 = 20;
        v.with_mut(|__v: &mut Vec<i32>| __v.push(__a1))
    };
    let mut x: i32 = 0_i32;
    let mut v4: Ptr<Vec<i32>> = (v3.as_pointer());
    let mut v2: Vec<i32> = Vec::new();
    {
        let __a1 = 0;
        v2.push(__a1)
    };
    {
        let __a1 = 1;
        v2.push(__a1)
    };
    {
        let __a1 = 3;
        v2.push(__a1)
    };
    x = (index!(v, 2_usize).read());
    v2[0_usize] = 1;
    elem!(
        ((if true {
            (Ptr::<Vec<i32>>::decay(&(v3.as_pointer())))
        } else {
            (Ptr::<Vec<i32>>::decay(&(v)))
        }) as Ptr<i32>),
        0_usize
    )
    .write(7);
    index!(v4, 1_usize).write(13);
    assert!((x == 6));
    assert!(((((v).clone() as Ptr<Vec<i32>>).decay().read()) == 4));
    assert!(((index!(v, 1_usize).read()) == 5));
    assert!(((index!(v, 2_usize).read()) == 6));
    assert!(((((v).clone() as Ptr<Vec<i32>>).decay().to_last().read()) == 20));
    assert!(({ (*v3.borrow())[0_usize] } == 7));
    assert!(({ (*v3.borrow())[1_usize] } == 13));
    {
        let __a1 = 20;
        v.with_mut(|__v: &mut Vec<i32>| __v.push(__a1))
    };
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
        let __off =
            <RandomAccessIterator<i32>>::from((v2.as_pointer() as Ptr<Vec<i32>>).decay().begin())
                .index();
        (v2.as_pointer() as Ptr<Vec<i32>>).with_mut(|__v: &mut Vec<i32>| __v.remove(__off));
        RandomAccessIterator::Vec((v2.as_pointer() as Ptr<Vec<i32>>).decay() + __off)
    };
    assert!(((*v2.borrow()).len() == 2_usize));
    assert!(({ (*v2.borrow())[0_usize] } == 2));
    assert!(({ (*v2.borrow())[1_usize] } == 3));
    {
        let __off =
            <RandomAccessIterator<i32>>::from((v2.as_pointer() as Ptr<Vec<i32>>).decay().begin())
                .index();
        (*v2.borrow_mut()).insert(__off, 100);
        <RandomAccessIterator<i32>>::from((v2.as_pointer() as Ptr<Vec<i32>>).decay().begin())
    };
    ({ copy_0((*v2.borrow()).clone()) });
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
    let v6: Value<Vec<f64>> = Rc::new(RefCell::new(vec![2_f64; s2 as usize]));
    assert!(((*v6.borrow()).len() == s2));
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < s2) {
        assert!(({ (*v6.borrow())[(i as usize)] } == 2_f64));
        i.prefix_inc();
    }
    let mut p1: Ptr<f64> = (v6.as_pointer() as Ptr<Vec<f64>>).decay();
    assert!(((p1.read()) == 2_f64));
    let mut p2: Ptr<i32> = (v3.as_pointer() as Ptr<Vec<i32>>).decay();
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
    assert!((((v2.as_pointer() as Ptr<Vec<i32>>).decay().to_last().read()) == 3));
    assert!((((v3.as_pointer() as Ptr<Vec<i32>>).decay().to_last().read()) == 1));
    assert!((((v6.as_pointer() as Ptr<Vec<f64>>).decay().to_last().read()) == 2_f64));
    let ref0: Ptr<f64> = (v6.as_pointer() as Ptr<Vec<f64>>).decay().to_last();
    ref0.write(5_f64);
    assert!((((v6.as_pointer() as Ptr<Vec<f64>>).decay().to_last().read()) == 5_f64));
    let mut x0: f64 = ((v6.as_pointer() as Ptr<Vec<f64>>).decay().to_last().read());
    assert!((x0 == 5_f64));
    x0 = 6_f64;
    assert!((((v6.as_pointer() as Ptr<Vec<f64>>).decay().to_last().read()) == 5_f64));
    let mut idx: i32 = 0;
    assert!(
        (((v6.as_pointer() as Ptr<Vec<f64>>)
            .decay()
            .offset((idx as usize) as isize)
            .read())
            == 2_f64)
    );
    assert!(
        (((v6.as_pointer() as Ptr<Vec<f64>>)
            .decay()
            .offset((s2).wrapping_sub(1_usize) as isize)
            .read())
            == 5_f64)
    );
    let ref1: Ptr<f64> = (v6.as_pointer() as Ptr<Vec<f64>>)
        .decay()
        .offset((s2).wrapping_sub(1_usize) as isize);
    {
        ref1.with_mut(|__v| *__v = *__v + 1.5_f64)
    };
    assert!(
        (((v6.as_pointer() as Ptr<Vec<f64>>)
            .decay()
            .offset((s2).wrapping_sub(1_usize) as isize)
            .read())
            == 6.5_f64)
    );
    let mut x1: f64 = ((v6.as_pointer() as Ptr<Vec<f64>>)
        .decay()
        .offset((s2).wrapping_sub(1_usize) as isize)
        .read());
    assert!((x1 == 6.5_f64));
    x1 -= 1.5_f64;
    assert!(
        (((v6.as_pointer() as Ptr<Vec<f64>>)
            .decay()
            .offset((s2).wrapping_sub(1_usize) as isize)
            .read())
            == 6.5_f64)
    );
    let v7: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    let mut v8: Vec<i32> = Vec::new();
    {
        let __a1 = 4;
        (*v7.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 5;
        (*v7.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 6;
        (*v7.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 8;
        v8.push(__a1)
    };
    {
        let __a1 = 9;
        v8.push(__a1)
    };
    ({ fn_1(v7.as_pointer(), v8.clone()) });
    let src: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([1_u32, 2_u32, 3_u32])));
    let mut v9: Vec<u32> = {
        let mut __v = Vec::new();
        let mut __it = (src.as_pointer() as Ptr<u32>);
        while __it != (src.as_pointer() as Ptr<u32>).offset((3) as isize) {
            __v.push(<u32>::try_from(__it.iter_deref().read()).ok().unwrap());
            __it.iter_inc();
        }
        __v
    };
    assert!((v9.len() == 3_usize));
    assert!(((v9[0_usize] == 1_u32) && (v9[1_usize] == 2_u32)) && (v9[2_usize] == 3_u32));
    let mut v10: Vec<u64> = {
        let mut __v = Vec::new();
        let mut __it = (src.as_pointer() as Ptr<u32>);
        while __it != (src.as_pointer() as Ptr<u32>).offset((3) as isize) {
            __v.push(<u64>::try_from(__it.iter_deref().read()).ok().unwrap());
            __it.iter_inc();
        }
        __v
    };
    assert!((v10.len() == 3_usize));
    assert!(((v10[0_usize] == 1_u64) && (v10[1_usize] == 2_u64)) && (v10[2_usize] == 3_u64));
    let mut v11: Vec<i32> = {
        let mut __v = Vec::new();
        let mut __it = (src.as_pointer() as Ptr<u32>);
        while __it != (src.as_pointer() as Ptr<u32>).offset((3) as isize) {
            __v.push(<i32>::try_from(__it.iter_deref().read()).ok().unwrap());
            __it.iter_inc();
        }
        __v
    };
    assert!((v11.len() == 3_usize));
    assert!(((v11[0_usize] == 1) && (v11[1_usize] == 2)) && (v11[2_usize] == 3));
    let src1: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([1_u32, 2_u32, 3_u32])));
    let mut v12: Vec<u32> = {
        let mut __v = Vec::new();
        let mut __it = (src1.as_pointer() as Ptr<u32>);
        while __it != (src1.as_pointer() as Ptr<u32>).to_end() {
            __v.push(<u32>::try_from(__it.iter_deref().read()).ok().unwrap());
            __it.iter_inc();
        }
        __v
    };
    assert!((v12.len() == 3_usize));
    assert!(((v12[0_usize] == 1_u32) && (v12[1_usize] == 2_u32)) && (v12[2_usize] == 3_u32));
    let buf: Value<Box<[u8]>> =
        Rc::new(RefCell::new(Box::new([10_u8, 20_u8, 30_u8, 40_u8, 50_u8])));
    let mut start: Ptr<u8> = (buf.as_pointer() as Ptr<u8>);
    let mut len: usize = 5_usize;
    let mut v13: Vec<u8> = {
        let mut __v = Vec::new();
        let mut __it = (start).clone();
        while __it != start.offset((len) as isize) {
            __v.push(<u8>::try_from(__it.iter_deref().read()).ok().unwrap());
            __it.iter_inc();
        }
        __v
    };
    assert!((v13.len() == 5_usize));
    assert!(((v13[0_usize] as i32) == 10) && ((v13[4_usize] as i32) == 50));
    assert!(
        (((s1).wrapping_add(s2)).wrapping_add(
            (((v2.as_pointer() as Ptr<Vec<i32>>)
                .decay()
                .offset(0_usize as isize)
                .read()) as usize)
        ) == 103_usize)
    );
    return 0;
}
pub trait TestAllocator_double_Impl {
    fn allocate(&self, n: usize) -> Ptr<f64>;
    fn deallocate(&self, p: Ptr<f64>, _a1: usize);
}
impl TestAllocator_double_Impl for Ptr<TestAllocator_double_> {
    fn allocate(&self, mut n: usize) -> Ptr<f64> {
        return Ptr::alloc_array((0..n).map(|_| 0_f64).collect::<Box<[f64]>>());
    }
    fn deallocate(&self, mut p: Ptr<f64>, mut _a1: usize) {
        p.delete();
    }
}
pub trait TestAllocator_int_Impl {
    fn allocate(&self, n: usize) -> Ptr<i32>;
    fn deallocate(&self, p: Ptr<i32>, _a1: usize);
}
impl TestAllocator_int_Impl for Ptr<TestAllocator_int_> {
    fn allocate(&self, mut n: usize) -> Ptr<i32> {
        return Ptr::alloc_array((0..n).map(|_| 0_i32).collect::<Box<[i32]>>());
    }
    fn deallocate(&self, mut p: Ptr<i32>, mut _a1: usize) {
        p.delete();
    }
}
pub fn __cpp2rust_init_globals() {}
