extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn copy_0(copy_vector: Vec<i32>) {
    let copy_vector: Value<Vec<i32>> = Rc::new(RefCell::new(copy_vector));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v1: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    assert!(((*v1.borrow()).len() as u64 == 0_u64));
    assert!((*v1.borrow()).is_empty());
    (*v1.borrow_mut()).push(1);
    assert!(!((*v1.borrow()).is_empty()));
    (*v1.borrow_mut()).pop();
    assert!((*v1.borrow()).is_empty());
    let s1: Value<u64> = Rc::new(RefCell::new((*v1.borrow()).len() as u64));
    {
        let __a0 = 100_u64 as usize;
        (*v1.borrow_mut()).resize_with(__a0, || <i32>::default())
    };
    assert!(((*v1.borrow()).len() as u64 == 100_u64));
    assert!((((v1.as_pointer() as Ptr<i32>).offset(99_u64 as isize).read()) == 0));
    (v1.as_pointer() as Ptr<i32>)
        .offset(0_u64 as isize)
        .write(40);
    (v1.as_pointer() as Ptr<i32>)
        .offset(99_u64 as isize)
        .write(50);
    assert!((((v1.as_pointer() as Ptr<i32>).offset(0_u64 as isize).read()) == 40));
    assert!((((v1.as_pointer() as Ptr<i32>).offset(99_u64 as isize).read()) == 50));
    let v2: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    assert!(((*v2.borrow()).len() as u64 == 0_u64));
    (*v2.borrow_mut()).push(1);
    (*v2.borrow_mut()).push(2);
    (*v2.borrow_mut()).push(3);
    assert!(((*v2.borrow()).len() as u64 == 3_u64));
    {
        let idx = (v2.as_pointer() as Ptr<i32>).clone().get_offset();
        (v2.as_pointer() as Ptr<Vec<i32>>).with_mut(|__v: &mut Vec<i32>| __v.remove(idx));
        (v2.as_pointer() as Ptr<Vec<i32>>).to_strong().as_pointer() as Ptr<i32>
    };
    assert!(((*v2.borrow()).len() as u64 == 2_u64));
    assert!((((v2.as_pointer() as Ptr<i32>).offset(0_u64 as isize).read()) == 2));
    assert!((((v2.as_pointer() as Ptr<i32>).offset(1_u64 as isize).read()) == 3));
    {
        let __off = (v2.as_pointer() as Ptr<i32>).clone().get_offset();
        (*v2.borrow_mut()).insert(__off, 100);
        (v2.as_pointer() as Ptr<i32>).clone()
    };
    ({
        let _copy_vector: Vec<i32> = (*v2.borrow()).clone();
        copy_0(_copy_vector)
    });
    assert!(((*v2.borrow()).len() as u64 == 3_u64));
    assert!((((v2.as_pointer() as Ptr<i32>).offset(0_u64 as isize).read()) == 100));
    assert!((((v2.as_pointer() as Ptr<i32>).offset(1_u64 as isize).read()) == 2));
    assert!((((v2.as_pointer() as Ptr<i32>).offset(2_u64 as isize).read()) == 3));
    let s2: Value<u64> = Rc::new(RefCell::new((*v2.borrow()).len() as u64));
    let v3: Value<Vec<i32>> = Rc::new(RefCell::new(vec![1; 100_u64 as usize]));
    assert!(((*v3.borrow()).len() as u64 == 100_u64));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 100) {
        assert!(
            (((v3.as_pointer() as Ptr<i32>)
                .offset(((*i.borrow()) as u64) as isize)
                .read())
                == 1)
        );
        (*i.borrow_mut()).prefix_inc();
    }
    let v4: Value<Vec<Ptr<i32>>> = Rc::new(RefCell::new(
        (0..(100_u64) as usize)
            .map(|_| <Ptr<i32>>::default())
            .collect::<Vec<_>>(),
    ));
    assert!(((*v4.borrow()).len() as u64 == 100_u64));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while (((*i.borrow()) as u64) < (*v4.borrow()).len() as u64) {
        assert!(((v4.as_pointer() as Ptr<Ptr::<i32>>)
            .offset(((*i.borrow()) as u64) as isize)
            .read())
        .is_null());
        (*i.borrow_mut()).prefix_inc();
    }
    let v5: Value<Vec<Ptr<i32>>> = Rc::new(RefCell::new(
        (0..(100_u64) as usize)
            .map(|_| <Ptr<i32>>::default())
            .collect::<Vec<_>>(),
    ));
    assert!(((*v5.borrow()).len() as u64 == 100_u64));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while (((*i.borrow()) as u64) < (*v5.borrow()).len() as u64) {
        assert!(((v5.as_pointer() as Ptr<Ptr::<i32>>)
            .offset(((*i.borrow()) as u64) as isize)
            .read())
        .is_null());
        (*i.borrow_mut()).prefix_inc();
    }
    let v6: Value<Vec<f64>> = Rc::new(RefCell::new(vec![2.0E+0; (*s2.borrow()) as usize]));
    assert!(((*v6.borrow()).len() as u64 == (*s2.borrow())));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while (((*i.borrow()) as u64) < (*s2.borrow())) {
        assert!(
            (((v6.as_pointer() as Ptr<f64>)
                .offset(((*i.borrow()) as u64) as isize)
                .read())
                == 2.0E+0)
        );
        (*i.borrow_mut()).prefix_inc();
    }
    let v7: Value<Vec<(Value<Ptr<i32>>, Value<i32>)>> = Rc::new(RefCell::new(
        (0..(200_u64) as usize)
            .map(|_| <(Value<Ptr<i32>>, Value<i32>)>::default())
            .collect::<Vec<_>>(),
    ));
    assert!(((*v7.borrow()).len() as u64 == 200_u64));
    let i: Value<u32> = Rc::new(RefCell::new(0_u32));
    'loop_: while ((*i.borrow()) < 200_u32) {
        assert!({
            let _lhs = ((*(*(v7.as_pointer() as Ptr<(Value<Ptr<i32>>, Value<i32>)>)
                .offset(((*i.borrow()) as u64) as isize)
                .upgrade()
                .deref())
            .0
            .borrow())
            .is_null())
            .clone();
            _lhs && ((*(*(v7.as_pointer() as Ptr<(Value<Ptr<i32>>, Value<i32>)>)
                .offset(((*i.borrow()) as u64) as isize)
                .upgrade()
                .deref())
            .1
            .borrow())
                == 0)
        });
        (*i.borrow_mut()).prefix_inc();
    }
    let p1: Value<Ptr<f64>> = Rc::new(RefCell::new((v6.as_pointer() as Ptr<f64>)));
    assert!((((*p1.borrow()).read()) == 2.0E+0));
    let p2: Value<Ptr<i32>> = Rc::new(RefCell::new((v3.as_pointer() as Ptr<i32>)));
    assert!((((*p2.borrow()).read()) == 1));
    assert!((((v3.as_pointer() as Ptr<i32>).offset(0_u64 as isize).read()) == 1));
    assert!((((v3.as_pointer() as Ptr<i32>).offset(1_u64 as isize).read()) == 1));
    (*p2.borrow()).write((9.9E+1 as i32));
    assert!((((*p2.borrow()).read()) == 99));
    assert!((((v3.as_pointer() as Ptr<i32>).offset(0_u64 as isize).read()) == 99));
    assert!((((v3.as_pointer() as Ptr<i32>).offset(1_u64 as isize).read()) == 1));
    (*p2.borrow_mut()).prefix_inc();
    (*p2.borrow()).write(98);
    assert!((((v3.as_pointer() as Ptr<i32>).offset(0_u64 as isize).read()) == 99));
    assert!((((v3.as_pointer() as Ptr<i32>).offset(1_u64 as isize).read()) == 98));
    assert!(((*v3.borrow()).capacity() as u64 == 100_u64));
    assert!(((*v3.borrow()).len() as u64 == 100_u64));
    if 200_u64 as usize > (*v3.borrow()).capacity() as usize {
        let len_0 = (*v3.borrow()).len();
        (*v3.borrow_mut()).reserve_exact(200_u64 as usize - len_0 as usize);
    };
    assert!(((*v3.borrow()).capacity() as u64 == 200_u64));
    assert!(((*v3.borrow()).len() as u64 == 100_u64));
    if 50_u64 as usize > (*v3.borrow()).capacity() as usize {
        let len_0 = (*v3.borrow()).len();
        (*v3.borrow_mut()).reserve_exact(50_u64 as usize - len_0 as usize);
    };
    assert!(((*v3.borrow()).capacity() as u64 == 200_u64));
    assert!(((*v3.borrow()).len() as u64 == 100_u64));
    if 200_u64 as usize > (*v3.borrow()).capacity() as usize {
        let len_0 = (*v3.borrow()).len();
        (*v3.borrow_mut()).reserve_exact(200_u64 as usize - len_0 as usize);
    };
    assert!(((*v3.borrow()).capacity() as u64 == 200_u64));
    assert!(((*v3.borrow()).len() as u64 == 100_u64));
    if 201_u64 as usize > (*v3.borrow()).capacity() as usize {
        let len_0 = (*v3.borrow()).len();
        (*v3.borrow_mut()).reserve_exact(201_u64 as usize - len_0 as usize);
    };
    assert!(((*v3.borrow()).capacity() as u64 == 201_u64));
    assert!(((*v3.borrow()).len() as u64 == 100_u64));
    assert!((((v2.as_pointer() as Ptr<i32>).to_last().read()) == 3));
    assert!((((v3.as_pointer() as Ptr<i32>).to_last().read()) == 1));
    assert!(((v4.as_pointer() as Ptr<Ptr::<i32>>).to_last().read()).is_null());
    assert!(((v5.as_pointer() as Ptr<Ptr::<i32>>).to_last().read()).is_null());
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
            .offset(((*idx.borrow()) as u64) as isize)
            .read())
            == 2.0E+0)
    );
    assert!(
        (((v6.as_pointer() as Ptr<f64>)
            .offset((*s2.borrow()).wrapping_sub(1_u64) as isize)
            .read())
            == 5.0E+0)
    );
    let ref1: Ptr<f64> =
        (v6.as_pointer() as Ptr<f64>).offset((*s2.borrow()).wrapping_sub(1_u64) as isize);
    {
        let __ptr = ref1.clone();
        let __tmp = __ptr.read() + 1.5E+0;
        __ptr.write(__tmp)
    };
    assert!(
        (((v6.as_pointer() as Ptr<f64>)
            .offset((*s2.borrow()).wrapping_sub(1_u64) as isize)
            .read())
            == 6.5E+0)
    );
    let x1: Value<f64> = Rc::new(RefCell::new(
        ((v6.as_pointer() as Ptr<f64>)
            .offset((*s2.borrow()).wrapping_sub(1_u64) as isize)
            .read()),
    ));
    assert!(((*x1.borrow()) == 6.5E+0));
    (*x1.borrow_mut()) -= 1.5E+0;
    assert!(
        (((v6.as_pointer() as Ptr<f64>)
            .offset((*s2.borrow()).wrapping_sub(1_u64) as isize)
            .read())
            == 6.5E+0)
    );
    return ((((*s1.borrow()).wrapping_add((*s2.borrow())))
        .wrapping_add((((v2.as_pointer() as Ptr<i32>).offset(0_u64 as isize).read()) as u64)))
        as i32);
}
