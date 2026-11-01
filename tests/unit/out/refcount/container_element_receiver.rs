extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(32)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(8)]
    pub ref_: Ptr<Vec<i32>>,
    #[offset(8)]
    #[byte_size(24)]
    pub vec_: Vec<i32>,
}
impl Holder {
    pub fn move_from(_a0: Ptr<Holder>) -> Self {
        Self {
            ref_: ({ (*_a0.upgrade().deref()).ref_.clone() }).clone(),
            vec_: field!(_a0, vec_).with_mut(std::mem::take),
        }
    }
}
pub fn pick_0(a: Ptr<Vec<i32>>, b: Ptr<Vec<i32>>, mut first: bool) -> Ptr<Vec<i32>> {
    return if first { (a).clone() } else { (b).clone() };
}
pub fn sum_1(v: Ptr<Vec<i32>>) -> i32 {
    let mut total: i32 = 0;
    let it: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(v.decay().begin()));
    'loop_: while (*it.borrow()) != v.decay().end() {
        total += ((*it.borrow()).iter_deref().read());
        (*it.borrow_mut()).iter_inc();
    }
    return ({ total } + {
        (if 0_usize as usize >= (*v.upgrade().deref()).len() {
            panic!("out of bounds access")
        } else {
            v.decay().offset(0_usize as isize)
        }
        .read())
    });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1, 2, 3])));
    let h: Value<Holder> = Rc::new(RefCell::new(Holder {
        ref_: v.as_pointer(),
        vec_: Vec::from([4, 5]),
    }));
    assert!((({ (*h.borrow()).ref_.clone() }.decay().read()) == 1));
    { (*h.borrow()).ref_.clone() }.decay().write(9);
    assert!(({ (*v.borrow())[0_usize] } == 9));
    { (*h.borrow()).ref_.clone() }.decay().write(1);
    assert!(((field_ref!(h.as_pointer(), vec_).decay().to_last().read()) == 5));
    let mut p: Ptr<Vec<i32>> = (field_ref!(h.as_pointer(), vec_).as_pointer());
    assert!(((p.decay().read()) == 4));
    ({
        pick_0(
            v.as_pointer(),
            field_ref!(h.as_pointer(), vec_).as_pointer(),
            false,
        )
    })
    .decay()
    .write(7);
    assert!(((vec_elem!(h.as_pointer(), vec_, 0_usize).read()) == 7));
    let m: Value<Map<i32, (Value<i32>, Value<Vec<i32>>)>> = Rc::new(RefCell::new(Map::new()));
    {
        let __a1 = 8;
        Map::subscript_move(
            m.as_pointer(),
            Rc::new(RefCell::new(1)).as_pointer(),
            |__k| cpp_move!(__k),
        )
        .with_mut(|__v: &mut Vec<i32>| __v.push(__a1))
    };
    assert!(
        ((Map::subscript_move(
            m.as_pointer(),
            Rc::new(RefCell::new(1)).as_pointer(),
            |__k| cpp_move!(__k)
        )
        .decay()
        .begin()
        .iter_deref()
        .read())
            == 8)
    );
    let c: Ptr<Vec<i32>> = v.as_pointer();
    assert!(
        ((if 2_usize as usize >= (*c.upgrade().deref()).len() {
            panic!("out of bounds access")
        } else {
            c.decay().offset(2_usize as isize)
        }
        .read())
            == 3)
    );
    assert!((({ sum_1(v.as_pointer(),) }) == 7));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
