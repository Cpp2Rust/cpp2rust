extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Point {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(48)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(24)]
    pub values: Vec<i32>,
    #[offset(24)]
    #[byte_size(24)]
    pub points: Vec<Point>,
}
pub fn push_and_index_0(v: Ptr<Vec<i32>>) -> i32 {
    {
        let __a1 = 42;
        v.with_mut(|__v: &mut Vec<i32>| __v.push(__a1))
    };
    return (((*v.upgrade().deref()).len() as i32) - 1);
}
pub fn sum_ref_1(v: Ptr<Vec<i32>>) -> i32 {
    let mut s: i32 = 0;
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*v.upgrade().deref()).len() }) {
        s += ((index!(v, i).read()) as i32);
        i.prefix_inc();
    }
    return s;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1, 2, 3])));
    (*v.borrow_mut())[0_usize] = (10 as i32);
    assert!(({ (*v.borrow())[0_usize] } == 10));
    (*v.borrow_mut())[1_usize] += (5 as i32);
    (*v.borrow_mut())[2_usize].postfix_inc();
    assert!(({ (*v.borrow())[1_usize] } == 7) && ({ (*v.borrow())[2_usize] } == 4));
    let __rhs = { (*v.borrow())[1_usize] };
    (*v.borrow_mut())[0_usize] = __rhs;
    assert!(({ (*v.borrow())[0_usize] } == 7));
    (*v.borrow_mut())[1_usize] = (0 as i32);
    assert!(
        ((elem!(
            v.as_pointer().decay(),
            ({ (*v.borrow())[1_usize] } as usize)
        )
        .read())
            == 7)
    );
    let mut i: i32 = 0;
    elem!(v.as_pointer().decay(), (i.postfix_inc() as usize)).write((3 as i32));
    assert!((i == 1) && ({ (*v.borrow())[0_usize] } == 3));
    assert!(
        ((elem!(
            v.as_pointer().decay(),
            (({ push_and_index_0(v.as_pointer(),) }) as usize)
        )
        .read())
            == 42)
    );
    elem!(
        v.as_pointer().decay(),
        (({ push_and_index_0(v.as_pointer(),) }) as usize)
    )
    .write((5 as i32));
    assert!(((*v.borrow()).len() == 5_usize) && ({ (*v.borrow())[4_usize] } == 5));
    let mut p: Ptr<i32> = (v.as_pointer().decay().offset(2_usize)).reinterpret_cast::<i32>();
    p.write(9);
    assert!(({ (*v.borrow())[2_usize] } == 9));
    assert!((({ sum_ref_1(v.as_pointer(),) }) == ((((3 + 0) + 9) + 42) + 5)));
    let h: Value<Holder> = Rc::new(RefCell::new(<Holder>::default()));
    {
        let __a0 = 2_usize as usize;
        (*h.borrow_mut())
            .values
            .resize_with(__a0, || <i32>::default())
    };
    vec_elem!(h.as_pointer(), values, 1_usize).write((6 as i32));
    {
        let __a1 = Point { x: 1, y: 2 };
        (*h.borrow_mut()).points.push(__a1)
    };
    field!(vec_elem!(h.as_pointer(), points, 0_usize), y).write(5);
    assert!(((vec_elem!(h.as_pointer(), values, 1_usize).read()) == 6));
    assert!(
        (({
            PointImpl::sum(
                &field_ref!(h.as_pointer(), points)
                    .as_pointer()
                    .decay()
                    .offset(0_usize),
            )
        }) == 6)
    );
    let mut hp: Ptr<Holder> = (h.as_pointer());
    let __rhs = ((((vec_elem!(hp, values, 1_usize).read()) as i32) + 1) as i32);
    vec_elem!(hp, values, 0_usize).write(__rhs);
    let __rhs = ((vec_elem!(hp, values, 0_usize).read()) as i32);
    field!(vec_elem!(hp, points, 0_usize), x).write(__rhs);
    assert!(
        ((vec_elem!(h.as_pointer(), values, 0_usize).read()) == 7)
            && (vec_elem!(h.as_pointer(), points, 0_usize).with(|__s| __s.x) == 7)
    );
    let mut q: Point = vec_elem!(h.as_pointer(), points, 0_usize).read();
    q.x = 0;
    assert!((vec_elem!(h.as_pointer(), points, 0_usize).with(|__s| __s.x) == 7));
    let grid: Value<Vec<Vec<i32>>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = vec![0; 3_usize as usize];
        (*grid.borrow_mut()).push(__a1)
    };
    {
        let __a1 = vec![0; 3_usize as usize];
        (*grid.borrow_mut()).push(__a1)
    };
    elem!(grid.as_pointer().decay().offset(1_usize), 2_usize).write((8 as i32));
    assert!(
        ((elem!(grid.as_pointer().decay().offset(1_usize), 2_usize).read()) == 8)
            && ((elem!(grid.as_pointer().decay().offset(0_usize), 2_usize).read()) == 0)
    );
    let __rhs = (elem!(grid.as_pointer().decay().offset(1_usize), 2_usize).read());
    elem!(grid.as_pointer().decay().offset(0_usize), 1_usize).write(__rhs);
    let __rhs = (elem!(grid.as_pointer().decay().offset(0_usize), 1_usize).read());
    {
        elem!(grid.as_pointer().decay().offset(1_usize), 0_usize)
            .with_mut(|__v| *__v = *__v + __rhs)
    };
    assert!(
        ((elem!(grid.as_pointer().decay().offset(0_usize), 1_usize).read()) == 8)
            && ((elem!(grid.as_pointer().decay().offset(1_usize), 0_usize).read()) == 8)
    );
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([4, 5, 6])));
    let __rhs = ({ (*a.borrow())[0_usize] } + { (*a.borrow())[2_usize] });
    (*a.borrow_mut())[1_usize] = __rhs;
    assert!(({ (*a.borrow())[1_usize] } == 10));
    return 0;
}
pub trait PointImpl {
    fn sum(&self) -> i32;
}
impl PointImpl for Ptr<Point> {
    fn sum(&self) -> i32 {
        return ((*self).with(|__s| __s.x) + (*self).with(|__s| __s.y));
    }
}
pub fn __cpp2rust_init_globals() {}
