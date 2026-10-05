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
pub fn copy_0(copy_vector: Vec<i32>) {
    let copy_vector: Value<Vec<i32>> = Rc::new(RefCell::new(copy_vector));
}
pub fn fn_1(v: Ptr<Vec<i32>>, v3: Vec<i32>) {
    let v3: Value<Vec<i32>> = Rc::new(RefCell::new(v3));
    v.with_mut(|__v: &mut Vec<i32>| __v.push(20));
    let mut x: i32 = 0_i32;
    let mut v4: Ptr<Vec<i32>> = (v3.as_pointer());
    let v2: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    (*v2.borrow_mut()).push(0);
    (*v2.borrow_mut()).push(1);
    (*v2.borrow_mut()).push(3);
    x = (elem!((Ptr::<Vec<i32>>::decay(&(v)) as Ptr<i32>), 2_usize).read());
    elem!((v2.as_pointer() as Ptr<i32>), 0_usize).write(1);
    elem!(
        ((if true {
            v3.as_pointer()
        } else {
            Ptr::<Vec<i32>>::decay(&(v))
        }) as Ptr<i32>),
        0_usize
    )
    .write(7);
    elem!(((Ptr::<Vec<i32>>::decay(&(v4))) as Ptr<i32>), 1_usize).write(13);
    assert!((x == 6));
    assert!((((Ptr::<Vec<i32>>::decay(&(v)) as Ptr<i32>).read()) == 4));
    assert!(((elem!((Ptr::<Vec<i32>>::decay(&(v)) as Ptr<i32>), 1_usize).read()) == 5));
    assert!(((elem!((Ptr::<Vec<i32>>::decay(&(v)) as Ptr<i32>), 2_usize).read()) == 6));
    assert!((((Ptr::<Vec<i32>>::decay(&(v)) as Ptr<i32>).to_last().read()) == 20));
    assert!(((elem!((v3.as_pointer() as Ptr<i32>), 0_usize).read()) == 7));
    assert!(((elem!((v3.as_pointer() as Ptr<i32>), 1_usize).read()) == 13));
    v.with_mut(|__v: &mut Vec<i32>| __v.push(20));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v1: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    assert!(((*v1.borrow()).len() == 0_usize));
    assert!((*v1.borrow()).is_empty());
    (*v1.borrow_mut()).push(1);
    assert!(!((*v1.borrow()).is_empty()));
    (*v1.borrow_mut()).pop();
    assert!((*v1.borrow()).is_empty());
    let mut s1: usize = (*v1.borrow()).len();
    {
        let __a0 = 100_usize as usize;
        (*v1.borrow_mut()).resize_with(__a0, || <i32>::default())
    };
    assert!(((*v1.borrow()).len() == 100_usize));
    assert!(((elem!((v1.as_pointer() as Ptr<i32>), 99_usize).read()) == 0));
    elem!((v1.as_pointer() as Ptr<i32>), 0_usize).write(40);
    elem!((v1.as_pointer() as Ptr<i32>), 99_usize).write(50);
    assert!(((elem!((v1.as_pointer() as Ptr<i32>), 0_usize).read()) == 40));
    assert!(((elem!((v1.as_pointer() as Ptr<i32>), 99_usize).read()) == 50));
    let v2: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    assert!(((*v2.borrow()).len() == 0_usize));
    (*v2.borrow_mut()).push(1);
    (*v2.borrow_mut()).push(2);
    (*v2.borrow_mut()).push(3);
    assert!(((*v2.borrow()).len() == 3_usize));
    {
        let idx = (v2.as_pointer() as Ptr<i32>).get_offset();
        (v2.as_pointer() as Ptr<Vec<i32>>).with_mut(|__v: &mut Vec<i32>| __v.remove(idx));
        (v2.as_pointer() as Ptr<Vec<i32>>).decay()
    };
    assert!(((*v2.borrow()).len() == 2_usize));
    assert!(((elem!((v2.as_pointer() as Ptr<i32>), 0_usize).read()) == 2));
    assert!(((elem!((v2.as_pointer() as Ptr<i32>), 1_usize).read()) == 3));
    {
        let __off = (v2.as_pointer() as Ptr<i32>).get_offset();
        (*v2.borrow_mut()).insert(__off, 100);
        (v2.as_pointer() as Ptr<i32>)
    };
    ({ copy_0((*v2.borrow()).clone()) });
    assert!(((*v2.borrow()).len() == 3_usize));
    assert!(((elem!((v2.as_pointer() as Ptr<i32>), 0_usize).read()) == 100));
    assert!(((elem!((v2.as_pointer() as Ptr<i32>), 1_usize).read()) == 2));
    assert!(((elem!((v2.as_pointer() as Ptr<i32>), 2_usize).read()) == 3));
    let mut s2: usize = (*v2.borrow()).len();
    let v3: Value<Vec<i32>> = Rc::new(RefCell::new(vec![1; 100_usize as usize]));
    assert!(((*v3.borrow()).len() == 100_usize));
    let mut i: i32 = 0;
    'loop_: while (i < 100) {
        assert!(((elem!((v3.as_pointer() as Ptr<i32>), (i as usize)).read()) == 1));
        i.prefix_inc();
    }
    let v6: Value<Vec<f64>> = Rc::new(RefCell::new(vec![2.0E+0; s2 as usize]));
    assert!(((*v6.borrow()).len() == s2));
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < s2) {
        assert!(((elem!((v6.as_pointer() as Ptr<f64>), (i as usize)).read()) == 2.0E+0));
        i.prefix_inc();
    }
    let mut p1: Ptr<f64> = (v6.as_pointer() as Ptr<f64>);
    assert!(((p1.read()) == 2.0E+0));
    let mut p2: Ptr<i32> = (v3.as_pointer() as Ptr<i32>);
    assert!(((p2.read()) == 1));
    assert!(((elem!((v3.as_pointer() as Ptr<i32>), 0_usize).read()) == 1));
    assert!(((elem!((v3.as_pointer() as Ptr<i32>), 1_usize).read()) == 1));
    p2.write((9.9E+1 as i32));
    assert!(((p2.read()) == 99));
    assert!(((elem!((v3.as_pointer() as Ptr<i32>), 0_usize).read()) == 99));
    assert!(((elem!((v3.as_pointer() as Ptr<i32>), 1_usize).read()) == 1));
    p2.prefix_inc();
    p2.write(98);
    assert!(((elem!((v3.as_pointer() as Ptr<i32>), 0_usize).read()) == 99));
    assert!(((elem!((v3.as_pointer() as Ptr<i32>), 1_usize).read()) == 98));
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
    let mut x0: f64 = ((v6.as_pointer() as Ptr<f64>).to_last().read());
    assert!((x0 == 5.0E+0));
    x0 = 6.0E+0;
    assert!((((v6.as_pointer() as Ptr<f64>).to_last().read()) == 5.0E+0));
    let mut idx: i32 = 0;
    assert!(
        (((v6.as_pointer() as Ptr<f64>)
            .offset((idx as usize) as isize)
            .read())
            == 2.0E+0)
    );
    assert!(
        (((v6.as_pointer() as Ptr<f64>)
            .offset((s2).wrapping_sub(1_usize) as isize)
            .read())
            == 5.0E+0)
    );
    let ref1: Ptr<f64> = (v6.as_pointer() as Ptr<f64>).offset((s2).wrapping_sub(1_usize) as isize);
    {
        let _ptr = ref1.clone();
        _ptr.write(_ptr.read() + 1.5E+0)
    };
    assert!(
        (((v6.as_pointer() as Ptr<f64>)
            .offset((s2).wrapping_sub(1_usize) as isize)
            .read())
            == 6.5E+0)
    );
    let mut x1: f64 = ((v6.as_pointer() as Ptr<f64>)
        .offset((s2).wrapping_sub(1_usize) as isize)
        .read());
    assert!((x1 == 6.5E+0));
    x1 -= 1.5E+0;
    assert!(
        (((v6.as_pointer() as Ptr<f64>)
            .offset((s2).wrapping_sub(1_usize) as isize)
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
        (((elem!((v9.as_pointer() as Ptr<u32>), 0_usize).read()) == 1_u32)
            && ((elem!((v9.as_pointer() as Ptr<u32>), 1_usize).read()) == 2_u32))
            && ((elem!((v9.as_pointer() as Ptr<u32>), 2_usize).read()) == 3_u32)
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
        (((elem!((v10.as_pointer() as Ptr<u64>), 0_usize).read()) == 1_u64)
            && ((elem!((v10.as_pointer() as Ptr<u64>), 1_usize).read()) == 2_u64))
            && ((elem!((v10.as_pointer() as Ptr<u64>), 2_usize).read()) == 3_u64)
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
        (((elem!((v11.as_pointer() as Ptr<i32>), 0_usize).read()) == 1)
            && ((elem!((v11.as_pointer() as Ptr<i32>), 1_usize).read()) == 2))
            && ((elem!((v11.as_pointer() as Ptr<i32>), 2_usize).read()) == 3)
    );
    let src1: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([1_u32, 2_u32, 3_u32])));
    let v12: Value<Vec<u32>> = Rc::new(RefCell::new({
        let __count = (src1.as_pointer() as Ptr<u32>).to_end().get_offset()
            - (src1.as_pointer() as Ptr<u32>).get_offset();
        PtrValueIter::new(&(src1.as_pointer() as Ptr<u32>), __count).collect::<Vec<_>>()
    }));
    assert!(((*v12.borrow()).len() == 3_usize));
    assert!(
        (((elem!((v12.as_pointer() as Ptr<u32>), 0_usize).read()) == 1_u32)
            && ((elem!((v12.as_pointer() as Ptr<u32>), 1_usize).read()) == 2_u32))
            && ((elem!((v12.as_pointer() as Ptr<u32>), 2_usize).read()) == 3_u32)
    );
    let buf: Value<Box<[u8]>> =
        Rc::new(RefCell::new(Box::new([10_u8, 20_u8, 30_u8, 40_u8, 50_u8])));
    let mut start: Ptr<u8> = (buf.as_pointer() as Ptr<u8>);
    let mut len: usize = 5_usize;
    let v13: Value<Vec<u8>> = Rc::new(RefCell::new({
        let __count = start.offset((len) as isize).get_offset() - start.get_offset();
        PtrValueIter::new(&start, __count).collect::<Vec<_>>()
    }));
    assert!(((*v13.borrow()).len() == 5_usize));
    assert!(
        (((elem!((v13.as_pointer() as Ptr<u8>), 0_usize).read()) as i32) == 10)
            && (((elem!((v13.as_pointer() as Ptr<u8>), 4_usize).read()) as i32) == 50)
    );
    assert!(
        (((s1).wrapping_add(s2)).wrapping_add(
            (((v2.as_pointer() as Ptr<i32>)
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
