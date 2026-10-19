extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = 4;
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 7;
        (*v.borrow_mut()).push(__a1)
    };
    let it: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new({
        let mut __it = (v.as_pointer() as Ptr<i32>).begin();
        while __it != (v.as_pointer() as Ptr<i32>).end() && __it.iter_deref().with(|__v| *__v != 7)
        {
            __it.iter_inc();
        }
        __it
    }));
    assert!((*it.borrow()) != (v.as_pointer() as Ptr<i32>).end());
    assert!((((*it.borrow()).iter_deref().read()) == 7));
    {
        let __n = (v.as_pointer() as Ptr<i32>)
            .end()
            .iter_distance(&(v.as_pointer() as Ptr<i32>).begin());
        let __random = std::hash::RandomState::new();
        for __i in (1..__n).rev() {
            let __j =
                (std::hash::BuildHasher::hash_one(&__random, __i) % (__i + 1) as u64) as isize;
            let __x = (v.as_pointer() as Ptr<i32>).begin().iter_at(__i);
            let __y = (v.as_pointer() as Ptr<i32>).begin().iter_at(__j);
            let __tmp = __x.read();
            __x.write(__y.read());
            __y.write(__tmp);
        }
    };
    {
        let __n = (v.as_pointer() as Ptr<i32>)
            .end()
            .iter_distance(&(v.as_pointer() as Ptr<i32>).begin());
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| {
            (v.as_pointer() as Ptr<i32>)
                .begin()
                .iter_at(__x)
                .with(|__a| {
                    (v.as_pointer() as Ptr<i32>)
                        .begin()
                        .iter_at(__y)
                        .with(|__b| __a.cmp(__b))
                })
        });
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| (v.as_pointer() as Ptr<i32>).begin().iter_at(__i).read())
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            (v.as_pointer() as Ptr<i32>).begin().iter_at(__i).write(__v);
        }
    };
    assert!(({ (*v.borrow())[0_usize] } == 4) && ({ (*v.borrow())[1_usize] } == 7));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
