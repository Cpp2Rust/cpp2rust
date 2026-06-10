extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn add_sizes_0(a: usize, b: usize) -> usize {
    let a: Value<usize> = Rc::new(RefCell::new(a));
    let b: Value<usize> = Rc::new(RefCell::new(b));
    return (*a.borrow()).wrapping_add((*b.borrow()));
}
pub fn take_ulong_1(x: u64) -> u64 {
    let x: Value<u64> = Rc::new(RefCell::new(x));
    return (*x.borrow());
}
pub fn sub_signed_2(a: isize, b: isize) -> isize {
    let a: Value<isize> = Rc::new(RefCell::new(a));
    let b: Value<isize> = Rc::new(RefCell::new(b));
    return ((*a.borrow()) - (*b.borrow()));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let n: Value<usize> = Rc::new(RefCell::new(
        (::std::mem::size_of::<i32>() as usize).wrapping_add(4_usize),
    ));
    assert!(((*n.borrow()) == (::std::mem::size_of::<i32>() as usize).wrapping_add(4_usize)));
    let ul: Value<u64> = Rc::new(RefCell::new(10_u64));
    let sz: Value<usize> = Rc::new(RefCell::new(20_usize));
    let mixed: Value<usize> = Rc::new(RefCell::new(
        ((((*sz.borrow()) as u64).wrapping_add((*ul.borrow()))) as usize),
    ));
    assert!(((*mixed.borrow()) == 30_usize));
    assert!(((*sz.borrow()) > ((*ul.borrow()) as usize)));
    assert!((((*ul.borrow()) as usize) < (*sz.borrow())));
    assert!(!((*sz.borrow()) == ((*ul.borrow()) as usize)));
    let chain: Value<usize> = Rc::new(RefCell::new(
        ((((((*sz.borrow()) as u64).wrapping_add((*ul.borrow()))).wrapping_add(5_u64))
            .wrapping_add(::std::mem::size_of::<i64>() as u64)) as usize),
    ));
    assert!(
        ((*chain.borrow())
            == (((20 + 10) + 5) as usize).wrapping_add(::std::mem::size_of::<i64>() as usize))
    );
    let acc: Value<usize> = Rc::new(RefCell::new(100_usize));
    let rhs_0 =
        (((*acc.borrow()) as u64).wrapping_add(::std::mem::size_of::<f64>() as u64)) as usize;
    (*acc.borrow_mut()) = rhs_0;
    let rhs_0 = (*acc.borrow()).wrapping_mul(2_usize);
    (*acc.borrow_mut()) = rhs_0;
    let rhs_0 = (((*acc.borrow()) as u64).wrapping_sub((*ul.borrow()))) as usize;
    (*acc.borrow_mut()) = rhs_0;
    assert!(
        ((*acc.borrow())
            == ((((100_usize).wrapping_add(::std::mem::size_of::<f64>() as usize)) as usize)
                .wrapping_mul(2_usize) as usize)
                .wrapping_sub(10_usize))
    );
    let __rhs = (*sz.borrow()).wrapping_add(1_usize);
    (*sz.borrow_mut()) = __rhs;
    assert!(((*sz.borrow()) == 21_usize));
    let fr: Value<usize> = Rc::new(RefCell::new(
        ({
            let _a: usize = (((::std::mem::size_of::<i32>() as u64)
                .wrapping_add((*sz.borrow()) as u64)) as usize);
            let _b: usize = ((*ul.borrow()) as usize);
            add_sizes_0(_a, _b)
        }),
    ));
    assert!(
        ((*fr.borrow())
            == ((::std::mem::size_of::<i32>() as usize).wrapping_add(21_usize) as usize)
                .wrapping_add(10_usize))
    );
    let fr2: Value<u64> = Rc::new(RefCell::new(
        ({
            let _x: u64 = ((*sz.borrow()) as u64);
            take_ulong_1(_x)
        }),
    ));
    assert!(((*fr2.borrow()) == 21_u64));
    let lo: Value<usize> = Rc::new(RefCell::new(
        (({
            let __tmp_0: Value<u64> = Rc::new(RefCell::new(((*sz.borrow()) as u64)));
            let __tmp_1: Value<u64> = Rc::new(RefCell::new(
                (::std::mem::size_of::<i64>() as u64).wrapping_add((*ul.borrow())),
            ));
            (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                __tmp_0.as_pointer()
            } else {
                __tmp_1.as_pointer()
            }
            .read())
        }) as usize),
    ));
    let hi: Value<usize> = Rc::new(RefCell::new(
        (({
            let __tmp_0: Value<u64> = Rc::new(RefCell::new(
                (::std::mem::size_of::<i32>() as u64).wrapping_add((*sz.borrow()) as u64),
            ));
            (if __tmp_0.as_pointer().read() >= ul.as_pointer().read() {
                __tmp_0.as_pointer()
            } else {
                ul.as_pointer()
            }
            .read())
        }) as usize),
    ));
    assert!(((*lo.borrow()) == (::std::mem::size_of::<i64>() as usize).wrapping_add(10_usize)));
    assert!(((*hi.borrow()) == (::std::mem::size_of::<i32>() as usize).wrapping_add(21_usize)));
    let bound: Value<usize> = Rc::new(RefCell::new(
        (({
            let __tmp_0: Value<u64> = Rc::new(RefCell::new(((*sz.borrow()) as u64)));
            let __tmp_1: Value<u64> = Rc::new(RefCell::new(((4_usize) as u64)));
            (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                __tmp_0.as_pointer()
            } else {
                __tmp_1.as_pointer()
            }
            .read())
        }) as usize),
    ));
    assert!(((*bound.borrow()) == 4_usize));
    let data: Value<Box<[i32]>> = Rc::new(RefCell::new(
        (0..8).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
    ));
    let count: Value<usize> = Rc::new(RefCell::new(
        (::std::mem::size_of::<[i32; 8]>() as usize)
            .wrapping_div(::std::mem::size_of::<i32>() as usize),
    ));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*count.borrow())) {
        let __rhs = (((*i.borrow()).wrapping_mul(2_usize)) as i32);
        (*data.borrow_mut())[(*i.borrow()) as usize] = __rhs;
        (*i.borrow_mut()).postfix_inc();
    }
    let total: Value<usize> = Rc::new(RefCell::new(0_usize));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while ((*i.borrow()) < (*count.borrow())) {
        let rhs_0 =
            (*total.borrow()).wrapping_add(((*data.borrow())[(*i.borrow()) as usize] as usize));
        (*total.borrow_mut()) = rhs_0;
        (*i.borrow_mut()).postfix_inc();
    }
    assert!(((*total.borrow()) == 56_usize));
    let cond: Value<usize> = Rc::new(RefCell::new(
        ((if ((*sz.borrow()) > ((*ul.borrow()) as usize)) {
            ((*sz.borrow()) as u64).wrapping_add(::std::mem::size_of::<i32>() as u64)
        } else {
            (*ul.borrow())
        }) as usize),
    ));
    assert!(((*cond.borrow()) == (21_usize).wrapping_add(::std::mem::size_of::<i32>() as usize)));
    let arr: Value<Box<[usize]>> =
        Rc::new(RefCell::new(Box::new([0_usize, 1_usize, 2_usize, 3_usize])));
    let idx: Value<usize> = Rc::new(RefCell::new(
        (if (::std::mem::size_of::<i32>() > 2_usize) {
            2
        } else {
            0
        } as usize),
    ));
    assert!(((*arr.borrow())[(*idx.borrow()) as usize] == 2_usize));
    let s1: Value<isize> = Rc::new(RefCell::new(5_isize));
    let s2: Value<isize> = Rc::new(RefCell::new(12_isize));
    let sd: Value<isize> = Rc::new(RefCell::new(
        ({
            let _a: isize = (*s1.borrow());
            let _b: isize = (*s2.borrow());
            sub_signed_2(_a, _b)
        }),
    ));
    assert!(((*sd.borrow()) == (-7_i32 as isize)));
    assert!(((*sd.borrow()) < 0_isize));
    let l: Value<i64> = Rc::new(RefCell::new(3_i64));
    let sm: Value<isize> = Rc::new(RefCell::new(
        (((*s2.borrow()) + ((*l.borrow()) as isize)) as isize),
    ));
    assert!(((*sm.borrow()) == 15_isize));
    assert!(((*sm.borrow()) > ((*l.borrow()) as isize)));
    let smin: Value<isize> = Rc::new(RefCell::new(
        (({
            let __tmp_0: Value<i64> = Rc::new(RefCell::new(((*sd.borrow()) as i64)));
            let __tmp_1: Value<i64> = Rc::new(RefCell::new(((*sm.borrow()) as i64)));
            (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                __tmp_0.as_pointer()
            } else {
                __tmp_1.as_pointer()
            }
            .read())
        }) as isize),
    ));
    let smax: Value<isize> = Rc::new(RefCell::new(
        (({
            let __tmp_0: Value<i64> = Rc::new(RefCell::new(((*sd.borrow()) as i64)));
            let __tmp_1: Value<i64> = Rc::new(RefCell::new(((*sm.borrow()) as i64)));
            (if __tmp_0.as_pointer().read() >= __tmp_1.as_pointer().read() {
                __tmp_0.as_pointer()
            } else {
                __tmp_1.as_pointer()
            }
            .read())
        }) as isize),
    ));
    assert!(((*smin.borrow()) == (-7_i32 as isize)));
    assert!(((*smax.borrow()) == 15_isize));
    let delta: Value<isize> = Rc::new(RefCell::new(
        (((*sz.borrow()) as isize) - ((*ul.borrow()) as isize)),
    ));
    assert!(((*delta.borrow()) == 11_isize));
    return (((*n.borrow()).wrapping_rem(7_usize)) as i32);
}
