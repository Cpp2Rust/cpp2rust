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
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(48)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(24)]
    pub values: Value<Vec<i32>>,
    #[offset(24)]
    #[byte_size(24)]
    pub points: Value<Vec<Point>>,
}
impl Clone for Holder {
    fn clone(&self) -> Self {
        Self {
            values: Rc::new(RefCell::new((*self.values.borrow()).clone())),
            points: Rc::new(RefCell::new((*self.points.borrow()).clone())),
        }
    }
}
pub fn push_and_index_0(v: Ptr<Vec<i32>>) -> i32 {
    {
        let __a1 = 42;
        v.with_mut(|__v: &mut Vec<i32>| __v.push(__a1))
    };
    return (((*v.upgrade().deref()).len() as i32) - 1);
}
pub fn sum_ref_1(v: Ptr<Vec<i32>>) -> i32 {
    let s: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<usize> = Rc::new(RefCell::new(0_usize));
    'loop_: while {
        let _lhs = (*i.borrow());
        _lhs < (*v.upgrade().deref()).len()
    } {
        (*s.borrow_mut()) += ((Ptr::<Vec<i32>>::decay(&(v)) as Ptr<i32>)
            .offset((*i.borrow()))
            .read());
        (*i.borrow_mut()).prefix_inc();
    }
    return (*s.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(vec![1, 2, 3]));
    (v.as_pointer() as Ptr<i32>).offset(0_usize).write(10);
    assert!((((v.as_pointer() as Ptr<i32>).offset(0_usize).read()) == 10));
    {
        let _ptr = (v.as_pointer() as Ptr<i32>).offset(1_usize);
        _ptr.write(_ptr.read() + 5)
    };
    (v.as_pointer() as Ptr<i32>)
        .offset(2_usize)
        .with_mut(|__v| __v.postfix_inc());
    assert!(
        (((v.as_pointer() as Ptr<i32>).offset(1_usize).read()) == 7)
            && (((v.as_pointer() as Ptr<i32>).offset(2_usize).read()) == 4)
    );
    let __rhs = ((v.as_pointer() as Ptr<i32>).offset(1_usize).read());
    (v.as_pointer() as Ptr<i32>).offset(0_usize).write(__rhs);
    assert!((((v.as_pointer() as Ptr<i32>).offset(0_usize).read()) == 7));
    (v.as_pointer() as Ptr<i32>).offset(1_usize).write(0);
    assert!(
        (((v.as_pointer() as Ptr<i32>)
            .offset((((v.as_pointer() as Ptr<i32>).offset(1_usize).read()) as usize))
            .read())
            == 7)
    );
    let i: Value<i32> = Rc::new(RefCell::new(0));
    (v.as_pointer() as Ptr<i32>)
        .offset(((*i.borrow_mut()).postfix_inc() as usize))
        .write(3);
    assert!(((*i.borrow()) == 1) && (((v.as_pointer() as Ptr<i32>).offset(0_usize).read()) == 3));
    assert!(
        (((v.as_pointer() as Ptr<i32>)
            .offset((({ push_and_index_0(v.as_pointer(),) }) as usize))
            .read())
            == 42)
    );
    (v.as_pointer() as Ptr<i32>)
        .offset((({ push_and_index_0(v.as_pointer()) }) as usize))
        .write(5);
    assert!(
        ((*v.borrow()).len() == 5_usize)
            && (((v.as_pointer() as Ptr<i32>).offset(4_usize).read()) == 5)
    );
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(((v.as_pointer() as Ptr<i32>).offset(2_usize))));
    (*p.borrow()).write(9);
    assert!((((v.as_pointer() as Ptr<i32>).offset(2_usize).read()) == 9));
    assert!((({ sum_ref_1(v.as_pointer(),) }) == ((((3 + 0) + 9) + 42) + 5)));
    let h: Value<Holder> = Rc::new(RefCell::new(<Holder>::default()));
    {
        let __a0 = 2_usize as usize;
        (*{ (*h.borrow()).values.clone() }.borrow_mut()).resize_with(__a0, || <i32>::default())
    };
    ({ (*h.borrow()).values.clone() }.as_pointer() as Ptr<i32>)
        .offset(1_usize)
        .write(6);
    {
        let __a1 = Point { x: 1, y: 2 };
        (*{ (*h.borrow()).points.clone() }.borrow_mut()).push(__a1)
    };
    field!(
        ({ (*h.borrow()).points.clone() }.as_pointer() as Ptr<Point>).offset(0_usize),
        y
    )
    .write(5);
    assert!(
        ((({ (*h.borrow()).values.clone() }.as_pointer() as Ptr<i32>)
            .offset(1_usize)
            .read())
            == 6)
    );
    assert!(
        (({
            PointImpl::sum(
                &({ (*h.borrow()).points.clone() }.as_pointer() as Ptr<Point>).offset(0_usize),
            )
        }) == 6)
    );
    let hp: Value<Ptr<Holder>> = Rc::new(RefCell::new((h.as_pointer())));
    let __rhs = ((((*hp.borrow()).with(|__s| __s.values.clone()).as_pointer() as Ptr<i32>)
        .offset(1_usize)
        .read())
        + 1);
    ((*hp.borrow()).with(|__s| __s.values.clone()).as_pointer() as Ptr<i32>)
        .offset(0_usize)
        .write(__rhs);
    let __rhs = (((*hp.borrow()).with(|__s| __s.values.clone()).as_pointer() as Ptr<i32>)
        .offset(0_usize)
        .read());
    field!(
        ((*hp.borrow()).with(|__s| __s.points.clone()).as_pointer() as Ptr<Point>).offset(0_usize),
        x
    )
    .write(__rhs);
    assert!(
        ((({ (*h.borrow()).values.clone() }.as_pointer() as Ptr<i32>)
            .offset(0_usize)
            .read())
            == 7)
            && ({
                (*({ (*h.borrow()).points.clone() }.as_pointer() as Ptr<Point>)
                    .offset(0_usize)
                    .upgrade()
                    .deref())
                .x
            } == 7)
    );
    let q: Value<Point> = Rc::new(RefCell::new(
        (*({ (*h.borrow()).points.clone() }.as_pointer() as Ptr<Point>)
            .offset(0_usize)
            .upgrade()
            .deref())
        .clone(),
    ));
    (*q.borrow_mut()).x = 0;
    assert!(
        ({
            (*({ (*h.borrow()).points.clone() }.as_pointer() as Ptr<Point>)
                .offset(0_usize)
                .upgrade()
                .deref())
            .x
        } == 7)
    );
    let grid: Value<Vec<Value<Vec<i32>>>> = Rc::new(RefCell::new(Vec::new()));
    (grid.as_pointer() as Ptr<Vec<Value<Vec<i32>>>>).with_mut(|__v: &mut Vec<Value<Vec<i32>>>| {
        __v.push(Rc::new(RefCell::new(vec![0; 3_usize as usize])))
    });
    (grid.as_pointer() as Ptr<Vec<Value<Vec<i32>>>>).with_mut(|__v: &mut Vec<Value<Vec<i32>>>| {
        __v.push(Rc::new(RefCell::new(vec![0; 3_usize as usize])))
    });
    ((grid.as_pointer() as Ptr<Value<Vec<i32>>>)
        .offset(1_usize)
        .upgrade()
        .deref()
        .as_pointer() as Ptr<i32>)
        .offset(2_usize)
        .write(8);
    assert!(
        ((((grid.as_pointer() as Ptr<Value<Vec<i32>>>)
            .offset(1_usize)
            .upgrade()
            .deref()
            .as_pointer() as Ptr<i32>)
            .offset(2_usize)
            .read())
            == 8)
            && ((((grid.as_pointer() as Ptr<Value<Vec<i32>>>)
                .offset(0_usize)
                .upgrade()
                .deref()
                .as_pointer() as Ptr<i32>)
                .offset(2_usize)
                .read())
                == 0)
    );
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(vec![4, 5, 6]));
    let __rhs = (((a.as_pointer() as Ptr<i32>).offset(0_usize).read())
        + ((a.as_pointer() as Ptr<i32>).offset(2_usize).read()));
    (a.as_pointer() as Ptr<i32>).offset(1_usize).write(__rhs);
    assert!((((a.as_pointer() as Ptr<i32>).offset(1_usize).read()) == 10));
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
