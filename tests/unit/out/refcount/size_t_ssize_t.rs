extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn add_sizes_0(mut a: usize, mut b: usize) -> usize {
    return (a).wrapping_add(b);
}
pub fn take_ulong_1(mut x: u64) -> u64 {
    return x;
}
pub fn sub_signed_2(mut a: isize, mut b: isize) -> isize {
    return (a - b);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut n: usize = (::std::mem::size_of::<i32>() as usize).wrapping_add(4_usize);
    assert!((n == (::std::mem::size_of::<i32>() as usize).wrapping_add(4_usize)));
    let ul: Value<u64> = Rc::new(RefCell::new(10_u64));
    let sz: Value<usize> = Rc::new(RefCell::new(20_usize));
    let mut mixed: usize = (((*sz.borrow()) as u64).wrapping_add((*ul.borrow())) as usize);
    assert!((mixed == 30_usize));
    assert!(((*sz.borrow()) > ((*ul.borrow()) as usize)));
    assert!((((*ul.borrow()) as usize) < (*sz.borrow())));
    assert!(!((*sz.borrow()) == ((*ul.borrow()) as usize)));
    let mut chain: usize = (((((*sz.borrow()) as u64).wrapping_add((*ul.borrow())))
        .wrapping_add(5_u64))
    .wrapping_add((::std::mem::size_of::<i64>() as u64)) as usize);
    assert!(
        (chain == (((20 + 10) + 5) as usize).wrapping_add((::std::mem::size_of::<i64>() as usize)))
    );
    let mut acc: usize = 100_usize;
    acc = { ((acc as u64).wrapping_add((::std::mem::size_of::<f64>() as u64))) as usize };
    acc = { (acc).wrapping_mul(2_usize) };
    acc = { ((acc as u64).wrapping_sub((*ul.borrow()))) as usize };
    assert!(
        (acc == ((((100_usize).wrapping_add((::std::mem::size_of::<f64>() as usize))) as usize)
            .wrapping_mul(2_usize) as usize)
            .wrapping_sub(10_usize))
    );
    (*sz.borrow_mut()) = { (*sz.borrow()).wrapping_add(1_usize) };
    assert!(((*sz.borrow()) == 21_usize));
    let mut fr: usize = ({
        add_sizes_0(
            ((::std::mem::size_of::<i32>() as u64).wrapping_add(((*sz.borrow()) as u64)) as usize),
            ((*ul.borrow()) as usize),
        )
    });
    assert!(
        (fr == ((::std::mem::size_of::<i32>() as usize).wrapping_add(21_usize) as usize)
            .wrapping_add(10_usize))
    );
    let mut fr2: u64 = ({ take_ulong_1(((*sz.borrow()) as u64)) });
    assert!((fr2 == 21_u64));
    let mut lo: usize = ({
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
    } as usize);
    let mut hi: usize = ({
        let __tmp_0: Value<u64> = Rc::new(RefCell::new(
            (::std::mem::size_of::<i32>() as u64).wrapping_add(((*sz.borrow()) as u64)),
        ));
        (if __tmp_0.as_pointer().read() >= ul.as_pointer().read() {
            __tmp_0.as_pointer()
        } else {
            ul.as_pointer()
        }
        .read())
    } as usize);
    assert!((lo == (::std::mem::size_of::<i64>() as usize).wrapping_add(10_usize)));
    assert!((hi == (::std::mem::size_of::<i32>() as usize).wrapping_add(21_usize)));
    let mut bound: usize = ({
        let __tmp_0: Value<u64> = Rc::new(RefCell::new(((*sz.borrow()) as u64)));
        let __tmp_1: Value<u64> = Rc::new(RefCell::new((4_usize as u64)));
        (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
            __tmp_0.as_pointer()
        } else {
            __tmp_1.as_pointer()
        }
        .read())
    } as usize);
    assert!((bound == 4_usize));
    let mut data: [i32; 8] = [0_i32; 8];
    let mut count: usize = (::std::mem::size_of::<[i32; 8]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize));
    let mut i: usize = 0_usize;
    'loop_: while (i < count) {
        data[(i) as usize] = { (((i).wrapping_mul(2_usize)) as i32) };
        i.postfix_inc();
    }
    let mut total: usize = 0_usize;
    let mut i: usize = 0_usize;
    'loop_: while (i < count) {
        total = { (total).wrapping_add((data[(i) as usize] as usize)) };
        i.postfix_inc();
    }
    assert!((total == 56_usize));
    let mut cond: usize = (if ((*sz.borrow()) > ((*ul.borrow()) as usize)) {
        ((*sz.borrow()) as u64).wrapping_add((::std::mem::size_of::<i32>() as u64))
    } else {
        (*ul.borrow())
    } as usize);
    assert!((cond == (21_usize).wrapping_add((::std::mem::size_of::<i32>() as usize))));
    let mut arr: [usize; 4] = [0_usize, 1_usize, 2_usize, 3_usize];
    let mut idx: usize = (if (::std::mem::size_of::<i32>() > 2_usize) {
        2
    } else {
        0
    } as usize);
    assert!((arr[(idx) as usize] == 2_usize));
    let mut s1: isize = 5_isize;
    let mut s2: isize = 12_isize;
    let sd: Value<isize> = Rc::new(RefCell::new(({ sub_signed_2(s1, s2) })));
    assert!(((*sd.borrow()) == (-7_i32 as isize)));
    assert!(((*sd.borrow()) < 0_isize));
    let mut l: i64 = 3_i64;
    let sm: Value<isize> = Rc::new(RefCell::new((((s2 as i64) + l) as isize)));
    assert!(((*sm.borrow()) == 15_isize));
    assert!(((*sm.borrow()) > (l as isize)));
    let mut smin: isize = ({
        let __tmp_0: Value<i64> = Rc::new(RefCell::new(((*sd.borrow()) as i64)));
        let __tmp_1: Value<i64> = Rc::new(RefCell::new(((*sm.borrow()) as i64)));
        (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
            __tmp_0.as_pointer()
        } else {
            __tmp_1.as_pointer()
        }
        .read())
    } as isize);
    let mut smax: isize = ({
        let __tmp_0: Value<i64> = Rc::new(RefCell::new(((*sd.borrow()) as i64)));
        let __tmp_1: Value<i64> = Rc::new(RefCell::new(((*sm.borrow()) as i64)));
        (if __tmp_0.as_pointer().read() >= __tmp_1.as_pointer().read() {
            __tmp_0.as_pointer()
        } else {
            __tmp_1.as_pointer()
        }
        .read())
    } as isize);
    assert!((smin == (-7_i32 as isize)));
    assert!((smax == 15_isize));
    let mut delta: isize = (((*sz.borrow()) as isize) - ((*ul.borrow()) as isize));
    assert!((delta == 11_isize));
    let mut a64: i64 = 100_i64;
    let mut b: isize = 30_isize;
    a64 -= (b as i64);
    assert!((a64 == 70_i64));
    a64 += (b as i64);
    assert!((a64 == 100_i64));
    let mut c: isize = (-20_i32 as isize);
    a64 -= (c as i64);
    assert!((a64 == 120_i64));
    assert!(((((n).wrapping_rem(7_usize)) as i32) == 1));
    let mx: Value<usize> = Rc::new(RefCell::new(5_usize));
    let mut mins: [usize; 4] = [
        0_usize,
        ({
            let __tmp_0: Value<u64> = Rc::new(RefCell::new(1_u64));
            let __tmp_1: Value<u64> = Rc::new(RefCell::new(((*mx.borrow()) as u64)));
            (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                __tmp_0.as_pointer()
            } else {
                __tmp_1.as_pointer()
            }
            .read())
        } as usize),
        ({
            let __tmp_0: Value<u64> = Rc::new(RefCell::new(((*mx.borrow()) as u64)));
            let __tmp_1: Value<u64> =
                Rc::new(RefCell::new(((*mx.borrow()).wrapping_sub(3_usize) as u64)));
            (if __tmp_0.as_pointer().read() <= __tmp_1.as_pointer().read() {
                __tmp_0.as_pointer()
            } else {
                __tmp_1.as_pointer()
            }
            .read())
        } as usize),
        (*mx.borrow()),
    ];
    assert!((mins[(1) as usize] == 1_usize));
    assert!((mins[(2) as usize] == 2_usize));
    let pr: Value<(Value<u64>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(
            ((*sz.borrow()) as u64)
                .try_into()
                .expect("failed conversion"),
        )),
        Rc::new(RefCell::new(1.try_into().expect("failed conversion"))),
    )));
    assert!(((*(*pr.borrow()).0.borrow()) == 21_u64));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
