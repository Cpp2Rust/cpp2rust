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
        let mut __it = RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>));
        while __it != RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>).to_end())
            && __it.iter_deref().with(|__v| *__v != 7)
        {
            __it.iter_inc();
        }
        __it
    }));
    assert!((*it.borrow()) != RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>).to_end()));
    assert!((((*it.borrow()).iter_deref().read()) == 7));
    {
        let count = Rc::new(RefCell::new(RandomAccessIterator::Vec(
            (v.as_pointer() as Ptr<i32>).to_end(),
        )))
        .as_pointer()
        .get_offset()
            - Rc::new(RefCell::new(RandomAccessIterator::Vec(
                (v.as_pointer() as Ptr<i32>),
            )))
            .as_pointer()
            .get_offset();
        let random = std::hash::RandomState::new();
        for i in (1..count).rev() {
            let j = std::hash::BuildHasher::hash_one(&random, i) as usize % (i + 1);
            let x = Rc::new(RefCell::new(RandomAccessIterator::Vec(
                (v.as_pointer() as Ptr<i32>),
            )))
            .as_pointer()
            .offset(i);
            let y = Rc::new(RefCell::new(RandomAccessIterator::Vec(
                (v.as_pointer() as Ptr<i32>),
            )))
            .as_pointer()
            .offset(j);
            let tmp = x.read();
            x.write(y.read());
            y.write(tmp);
        }
    };
    {
        let __n = RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>).to_end())
            .iter_distance(&RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>)));
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| {
            RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>))
                .iter_at(__x)
                .with(|__a| {
                    RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>))
                        .iter_at(__y)
                        .with(|__b| __a.cmp(__b))
                })
        });
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| {
                RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>))
                    .iter_at(__i)
                    .read()
            })
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            RandomAccessIterator::Vec((v.as_pointer() as Ptr<i32>))
                .iter_at(__i)
                .write(__v);
        }
    };
    assert!(({ (*v.borrow())[0_usize] } == 4) && ({ (*v.borrow())[1_usize] } == 7));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
