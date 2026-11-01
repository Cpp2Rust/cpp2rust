extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn median_0(mut w: i32, mut n: i32, mut nw: i32) -> i32 {
    let mut mx: i32 = if (w > n) { w } else { n };
    let mut mn: i32 = ((w + n) - mx);
    if (nw > mx) {
        return mn;
    } else if (nw < mn) {
        return mx;
    }
    return ((n + w) - nw);
}
pub fn assign_1(mut c: bool, mut v: i32) -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(1));
    let b: Value<i32> = Rc::new(RefCell::new(2));
    if c {
        (a.as_pointer())
    } else {
        (b.as_pointer())
    }
    .write(v);
    if c {
        (a.as_pointer())
    } else {
        (b.as_pointer())
    }
    .with_mut(|__v| __v.prefix_inc());
    return (((*a.borrow()) * 10) + (*b.borrow()));
}
pub fn lehmer_2(mut sigma: Ptr<u32>, mut len: u32, mut code: Ptr<u32>) {
    let items: Value<Vec<u32>> = Rc::new(RefCell::new(
        (0..(len as usize) as usize)
            .map(|_| 0_u32)
            .collect::<Vec<_>>(),
    ));
    let mut i: u32 = 0_u32;
    'loop_: while (i < len) {
        let __rhs = i;
        (*items.borrow_mut())[(i as usize)] = __rhs;
        i.prefix_inc();
    }
    let mut i: u32 = 0_u32;
    'loop_: while (i < len) {
        let it: Value<RandomAccessIterator<u32>> = Rc::new(RefCell::new(
            match ({ items.decay().begin() }, { items.decay().end() }) {
                (mut __tmp_0, mut __tmp_1) => {
                    let mut __it = (__tmp_0).clone();
                    while __it != __tmp_1
                        && !CppEq::equal(&__it.iter_deref(), &sigma.offset((i) as isize))
                    {
                        __it.iter_inc();
                    }
                    __it
                }
            },
        ));
        assert!((*it.borrow()) != items.decay().end());
        let __rhs = ((*it.borrow()).iter_distance(&items.decay().begin()) as u32);
        elem!(code, i).write(__rhs);
        match ({ <RandomAccessIterator<u32>>::from((*it.borrow()).clone()) },) {
            (mut __tmp_2,) => {
                let __off = __tmp_2.index();
                items
                    .as_pointer()
                    .with_mut(|__v: &mut Vec<u32>| __v.remove(__off));
                (__tmp_2).clone()
            }
        };
        i.prefix_inc();
    }
}
pub fn sum_3(v: Ptr<Vec<i32>>) -> i32 {
    let mut s: i32 = 0;
    let it: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(v.decay().begin()));
    'loop_: while (*it.borrow()) != v.decay().end() {
        s += ((*it.borrow()).iter_deref().read());
        (*it.borrow_mut()).iter_inc();
    }
    return s;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ median_0(1, 5, 7,) }) == 1));
    assert!((({ median_0(1, 5, 0,) }) == 5));
    assert!((({ median_0(1, 5, 3,) }) == 3));
    assert!((({ assign_1(true, 5,) }) == 62));
    assert!((({ assign_1(false, 5,) }) == 16));
    let sigma: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([2_u32, 0_u32, 3_u32, 1_u32])));
    let code: Value<Box<[u32]>> =
        Rc::new(RefCell::new((0..4).map(|_| 0_u32).collect::<Box<[u32]>>()));
    ({
        lehmer_2(
            (sigma.as_pointer() as Ptr<u32>),
            4_u32,
            (code.as_pointer() as Ptr<u32>),
        )
    });
    assert!(
        ((((*code.borrow())[(0) as usize] == 2_u32) && ((*code.borrow())[(1) as usize] == 0_u32))
            && ((*code.borrow())[(2) as usize] == 1_u32))
            && ((*code.borrow())[(3) as usize] == 0_u32)
    );
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1, 2, 3])));
    assert!((({ sum_3(v.as_pointer(),) }) == 6));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
