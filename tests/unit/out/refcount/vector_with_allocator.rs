extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct TestAllocator_int_ {}
impl TestAllocator_int_ {
    pub fn allocate(&self, n: usize) -> Ptr<i32> {
        let n: Value<usize> = Rc::new(RefCell::new(n));
        return Ptr::alloc_array(
            (0..(*n.borrow()))
                .map(|_| <i32>::default())
                .collect::<Box<[i32]>>(),
        );
    }
    pub fn deallocate(&self, p: Ptr<i32>, _: usize) {
        let p: Value<Ptr<i32>> = Rc::new(RefCell::new(p));
        (*p.borrow()).delete_array();
    }
}
impl Clone for TestAllocator_int_ {
    fn clone(&self) -> Self {
        let mut this = Self {};
        this
    }
}
impl ByteRepr for TestAllocator_int_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct TestAllocator_double_ {}
impl TestAllocator_double_ {
    pub fn allocate(&self, n: usize) -> Ptr<f64> {
        let n: Value<usize> = Rc::new(RefCell::new(n));
        return Ptr::alloc_array(
            (0..(*n.borrow()))
                .map(|_| <f64>::default())
                .collect::<Box<[f64]>>(),
        );
    }
    pub fn deallocate(&self, p: Ptr<f64>, _: usize) {
        let p: Value<Ptr<f64>> = Rc::new(RefCell::new(p));
        (*p.borrow()).delete_array();
    }
}
impl Clone for TestAllocator_double_ {
    fn clone(&self) -> Self {
        let mut this = Self {};
        this
    }
}
impl ByteRepr for TestAllocator_double_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
pub fn copy_0(copy_vector: Vec<i32>) {
    let copy_vector: Value<Vec<i32>> = Rc::new(RefCell::new(copy_vector));
}
pub fn fn_1(v: Ptr<Vec<i32>>, v3: Vec<i32>) {
    let v3: Value<Vec<i32>> = Rc::new(RefCell::new(v3));
    v.with_mut(|__v: &mut Vec<i32>| __v.push(20));
    let x: Value<i32> = <Value<i32>>::default();
    let v4: Value<Ptr<Vec<i32>>> = Rc::new(RefCell::new((v3.as_pointer())));
    let v2: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    (*v2.borrow_mut()).push(0);
    (*v2.borrow_mut()).push(1);
    (*v2.borrow_mut()).push(3);
    (*x.borrow_mut()) = ((v.to_strong().as_pointer() as Ptr<i32>)
        .offset(2_usize)
        .read());
    (v2.as_pointer() as Ptr<i32>).offset(0_usize).write(1);
    ((if true {
        v3.as_pointer()
    } else {
        v.to_strong().as_pointer()
    }) as Ptr<i32>)
        .offset(0_usize)
        .write(7);
    (((*v4.borrow()).to_strong().as_pointer()) as Ptr<i32>)
        .offset(1_usize)
        .write(13);
    assert!(((*x.borrow()) == 6));
    assert!((((v.to_strong().as_pointer() as Ptr<i32>).read()) == 4));
    assert!(
        (((v.to_strong().as_pointer() as Ptr<i32>)
            .offset(1_usize)
            .read())
            == 5)
    );
    assert!(
        (((v.to_strong().as_pointer() as Ptr<i32>)
            .offset(2_usize)
            .read())
            == 6)
    );
    assert!((((v.to_strong().as_pointer() as Ptr<i32>).to_last().read()) == 20));
    assert!((((v3.as_pointer() as Ptr<i32>).offset(0_usize).read()) == 7));
    assert!((((v3.as_pointer() as Ptr<i32>).offset(1_usize).read()) == 13));
    v.with_mut(|__v: &mut Vec<i32>| __v.push(20));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v1: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    assert!(((*v1.borrow()).len() == 0_usize));
    assert!((*v1.borrow()).is_empty());
    (*v1.borrow_mut()).push(1);
    assert!(!(*v1.borrow()).is_empty());
    (*v1.borrow_mut()).pop();
    assert!((*v1.borrow()).is_empty());
    let s1: Value<usize> = Rc::new(RefCell::new((*v1.borrow()).len()));
    {
        let __a0 = 100_usize as usize;
        (*v1.borrow_mut()).resize_with(__a0, || <i32>::default())
    };
    assert!(((*v1.borrow()).len() == 100_usize));
    assert!((((v1.as_pointer() as Ptr<i32>).offset(99_usize).read()) == 0));
    (v1.as_pointer() as Ptr<i32>).offset(0_usize).write(40);
    (v1.as_pointer() as Ptr<i32>).offset(99_usize).write(50);
    assert!((((v1.as_pointer() as Ptr<i32>).offset(0_usize).read()) == 40));
    assert!((((v1.as_pointer() as Ptr<i32>).offset(99_usize).read()) == 50));
    let v2: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    assert!(((*v2.borrow()).len() == 0_usize));
    (*v2.borrow_mut()).push(1);
    (*v2.borrow_mut()).push(2);
    (*v2.borrow_mut()).push(3);
    assert!(((*v2.borrow()).len() == 3_usize));
    {
        let idx = (v2.as_pointer() as Ptr<i32>).clone().get_offset();
        (v2.as_pointer() as Ptr<Vec<i32>>).with_mut(|__v: &mut Vec<i32>| __v.remove(idx));
        (v2.as_pointer() as Ptr<Vec<i32>>).to_strong().as_pointer() as Ptr<i32>
    };
    assert!(((*v2.borrow()).len() == 2_usize));
    assert!((((v2.as_pointer() as Ptr<i32>).offset(0_usize).read()) == 2));
    assert!((((v2.as_pointer() as Ptr<i32>).offset(1_usize).read()) == 3));
    {
        let __off = (v2.as_pointer() as Ptr<i32>).clone().get_offset();
        (*v2.borrow_mut()).insert(__off, 100);
        (v2.as_pointer() as Ptr<i32>).clone()
    };
    ({ copy_0((*v2.borrow()).clone()) });
    assert!(((*v2.borrow()).len() == 3_usize));
    assert!((((v2.as_pointer() as Ptr<i32>).offset(0_usize).read()) == 100));
    assert!((((v2.as_pointer() as Ptr<i32>).offset(1_usize).read()) == 2));
    assert!((((v2.as_pointer() as Ptr<i32>).offset(2_usize).read()) == 3));
    let s2: Value<usize> = Rc::new(RefCell::new((*v2.borrow()).len()));
    let v3: Value<Vec<i32>> = Rc::new(RefCell::new(vec![1; 100_usize as usize]));
    assert!(((*v3.borrow()).len() == 100_usize));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 100) {
        assert!(
            (((v3.as_pointer() as Ptr<i32>)
                .offset(((*i.borrow()) as usize))
                .read())
                == 1)
        );
        (*i.borrow_mut()).prefix_inc();
    }
    let v6: Value<Vec<f64>> = Rc::new(RefCell::new(vec![2.0E+0; (*s2.borrow()) as usize]));
    assert!(((*v6.borrow()).len() == (*s2.borrow())));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while (((*i.borrow()) as usize) < (*s2.borrow())) {
        assert!(
            (((v6.as_pointer() as Ptr<f64>)
                .offset(((*i.borrow()) as usize))
                .read())
                == 2.0E+0)
        );
        (*i.borrow_mut()).prefix_inc();
    }
    let p1: Value<Ptr<f64>> = Rc::new(RefCell::new((v6.as_pointer() as Ptr<f64>)));
    assert!((((*p1.borrow()).read()) == 2.0E+0));
    let p2: Value<Ptr<i32>> = Rc::new(RefCell::new((v3.as_pointer() as Ptr<i32>)));
    assert!((((*p2.borrow()).read()) == 1));
    assert!((((v3.as_pointer() as Ptr<i32>).offset(0_usize).read()) == 1));
    assert!((((v3.as_pointer() as Ptr<i32>).offset(1_usize).read()) == 1));
    (*p2.borrow()).write((9.9E+1 as i32));
    assert!((((*p2.borrow()).read()) == 99));
    assert!((((v3.as_pointer() as Ptr<i32>).offset(0_usize).read()) == 99));
    assert!((((v3.as_pointer() as Ptr<i32>).offset(1_usize).read()) == 1));
    (*p2.borrow_mut()).prefix_inc();
    (*p2.borrow()).write(98);
    assert!((((v3.as_pointer() as Ptr<i32>).offset(0_usize).read()) == 99));
    assert!((((v3.as_pointer() as Ptr<i32>).offset(1_usize).read()) == 98));
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
    assert!((((v2.as_pointer() as Ptr<i32>).to_last().read()) == 3));
    assert!((((v3.as_pointer() as Ptr<i32>).to_last().read()) == 1));
    assert!((((v6.as_pointer() as Ptr<f64>).to_last().read()) == 2.0E+0));
    let ref0: Ptr<f64> = (v6.as_pointer() as Ptr<f64>).to_last();
    ref0.write(5.0E+0);
    assert!((((v6.as_pointer() as Ptr<f64>).to_last().read()) == 5.0E+0));
    let x0: Value<f64> = Rc::new(RefCell::new(
        ((v6.as_pointer() as Ptr<f64>).to_last().read()),
    ));
    assert!(((*x0.borrow()) == 5.0E+0));
    (*x0.borrow_mut()) = 6.0E+0;
    assert!((((v6.as_pointer() as Ptr<f64>).to_last().read()) == 5.0E+0));
    let idx: Value<i32> = Rc::new(RefCell::new(0));
    assert!(
        (((v6.as_pointer() as Ptr<f64>)
            .offset(((*idx.borrow()) as usize) as isize)
            .read())
            == 2.0E+0)
    );
    assert!(
        (((v6.as_pointer() as Ptr<f64>)
            .offset((*s2.borrow()).wrapping_sub(1_usize) as isize)
            .read())
            == 5.0E+0)
    );
    let ref1: Ptr<f64> =
        (v6.as_pointer() as Ptr<f64>).offset((*s2.borrow()).wrapping_sub(1_usize) as isize);
    {
        let _ptr = ref1.clone();
        _ptr.write(_ptr.read() + 1.5E+0)
    };
    assert!(
        (((v6.as_pointer() as Ptr<f64>)
            .offset((*s2.borrow()).wrapping_sub(1_usize) as isize)
            .read())
            == 6.5E+0)
    );
    let x1: Value<f64> = Rc::new(RefCell::new(
        ((v6.as_pointer() as Ptr<f64>)
            .offset((*s2.borrow()).wrapping_sub(1_usize) as isize)
            .read()),
    ));
    assert!(((*x1.borrow()) == 6.5E+0));
    (*x1.borrow_mut()) -= 1.5E+0;
    assert!(
        (((v6.as_pointer() as Ptr<f64>)
            .offset((*s2.borrow()).wrapping_sub(1_usize) as isize)
            .read())
            == 6.5E+0)
    );
    let v7: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    let v8: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    (*v7.borrow_mut()).push(4);
    (*v7.borrow_mut()).push(5);
    (*v7.borrow_mut()).push(6);
    (*v8.borrow_mut()).push(8);
    (*v8.borrow_mut()).push(9);
    ({ fn_1(v7.as_pointer(), (*v8.borrow()).clone()) });
    let src: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([1_u32, 2_u32, 3_u32])));
    let v9: Value<Vec<u32>> = Rc::new(RefCell::new({
        let __count = (src.as_pointer() as Ptr<u32>)
            .offset((3) as isize)
            .get_offset()
            - (src.as_pointer() as Ptr<u32>).get_offset();
        PtrValueIter::new(&(src.as_pointer() as Ptr<u32>), __count)
            .map(|item| u32::try_from(item).ok().unwrap())
            .collect::<Vec<_>>()
    }));
    assert!(((*v9.borrow()).len() == 3_usize));
    assert!(
        ((((v9.as_pointer() as Ptr<u32>).offset(0_usize).read()) == 1_u32)
            && (((v9.as_pointer() as Ptr<u32>).offset(1_usize).read()) == 2_u32))
            && (((v9.as_pointer() as Ptr<u32>).offset(2_usize).read()) == 3_u32)
    );
    let v10: Value<Vec<u64>> = Rc::new(RefCell::new({
        let __count = (src.as_pointer() as Ptr<u32>)
            .offset((3) as isize)
            .get_offset()
            - (src.as_pointer() as Ptr<u32>).get_offset();
        PtrValueIter::new(&(src.as_pointer() as Ptr<u32>), __count)
            .map(|item| u64::try_from(item).ok().unwrap())
            .collect::<Vec<_>>()
    }));
    assert!(((*v10.borrow()).len() == 3_usize));
    assert!(
        ((((v10.as_pointer() as Ptr<u64>).offset(0_usize).read()) == 1_u64)
            && (((v10.as_pointer() as Ptr<u64>).offset(1_usize).read()) == 2_u64))
            && (((v10.as_pointer() as Ptr<u64>).offset(2_usize).read()) == 3_u64)
    );
    let v11: Value<Vec<i32>> = Rc::new(RefCell::new({
        let __count = (src.as_pointer() as Ptr<u32>)
            .offset((3) as isize)
            .get_offset()
            - (src.as_pointer() as Ptr<u32>).get_offset();
        PtrValueIter::new(&(src.as_pointer() as Ptr<u32>), __count)
            .map(|item| i32::try_from(item).ok().unwrap())
            .collect::<Vec<_>>()
    }));
    assert!(((*v11.borrow()).len() == 3_usize));
    assert!(
        ((((v11.as_pointer() as Ptr<i32>).offset(0_usize).read()) == 1)
            && (((v11.as_pointer() as Ptr<i32>).offset(1_usize).read()) == 2))
            && (((v11.as_pointer() as Ptr<i32>).offset(2_usize).read()) == 3)
    );
    let src1: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([1_u32, 2_u32, 3_u32])));
    let v12: Value<Vec<u32>> = Rc::new(RefCell::new({
        let __count = (src1.as_pointer() as Ptr<u32>).to_end().get_offset()
            - (src1.as_pointer() as Ptr<u32>).get_offset();
        PtrValueIter::new(&(src1.as_pointer() as Ptr<u32>), __count).collect::<Vec<_>>()
    }));
    assert!(((*v12.borrow()).len() == 3_usize));
    assert!(
        ((((v12.as_pointer() as Ptr<u32>).offset(0_usize).read()) == 1_u32)
            && (((v12.as_pointer() as Ptr<u32>).offset(1_usize).read()) == 2_u32))
            && (((v12.as_pointer() as Ptr<u32>).offset(2_usize).read()) == 3_u32)
    );
    let buf: Value<Box<[u8]>> =
        Rc::new(RefCell::new(Box::new([10_u8, 20_u8, 30_u8, 40_u8, 50_u8])));
    let start: Value<Ptr<u8>> = Rc::new(RefCell::new((buf.as_pointer() as Ptr<u8>)));
    let len: Value<usize> = Rc::new(RefCell::new(5_usize));
    let v13: Value<Vec<u8>> = Rc::new(RefCell::new({
        let __count = (*start.borrow())
            .offset((*len.borrow()) as isize)
            .get_offset()
            - (*start.borrow()).get_offset();
        PtrValueIter::new(&(*start.borrow()), __count).collect::<Vec<_>>()
    }));
    assert!(((*v13.borrow()).len() == 5_usize));
    assert!(
        ((((v13.as_pointer() as Ptr<u8>).offset(0_usize).read()) as i32) == 10)
            && ((((v13.as_pointer() as Ptr<u8>).offset(4_usize).read()) as i32) == 50)
    );
    return ((((*s1.borrow()).wrapping_add((*s2.borrow()))).wrapping_add(
        (((v2.as_pointer() as Ptr<i32>)
            .offset(0_usize as isize)
            .read()) as usize),
    )) as i32);
}
