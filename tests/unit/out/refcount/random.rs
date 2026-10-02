extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(72)]
pub struct Pair {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
    #[offset(8)]
    #[byte_size(20)]
    pub a: Value<Box<[i32]>>,
    #[offset(32)]
    #[byte_size(8)]
    pub r: Ptr<i32>,
    #[offset(40)]
    #[byte_size(8)]
    pub p: Ptr<i32>,
    #[offset(48)]
    #[byte_size(8)]
    pub pair: Ptr<Pair>,
    #[offset(56)]
    #[byte_size(16)]
    pub ap: Value<Box<[Ptr<i32>]>>,
}
impl Clone for Pair {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
            a: Rc::new(RefCell::new((*self.a.borrow()).clone())),
            r: self.r.clone(),
            p: self.p.clone(),
            pair: self.pair.clone(),
            ap: Rc::new(RefCell::new((*self.ap.borrow()).clone())),
        }
    }
}
impl Default for Pair {
    fn default() -> Self {
        Pair {
            x: 0_i32,
            y: 0_i32,
            a: Rc::new(RefCell::new((0..5).map(|_| 0_i32).collect::<Box<[i32]>>())),
            r: <Ptr<i32>>::default(),
            p: Ptr::<i32>::null(),
            pair: Ptr::<Pair>::null(),
            ap: Rc::new(RefCell::new(
                (0..2)
                    .map(|_| Ptr::<i32>::null())
                    .collect::<Box<[Ptr<i32>]>>(),
            )),
        }
    }
}
pub fn zero_0() -> i32 {
    return 0;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct X1 {}
pub fn foo_1(x1: i32, x2: Ptr<i32>, x3: Ptr<i32>, p2: Ptr<Pair>, p3: Ptr<Pair>) {
    let x1: Value<i32> = Rc::new(RefCell::new(x1));
    let x3: Value<Ptr<i32>> = Rc::new(RefCell::new(x3));
    let p3: Value<Ptr<Pair>> = Rc::new(RefCell::new(p3));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x1: Value<i32> = Rc::new(RefCell::new(1));
    let c1: Value<i32> = Rc::new(RefCell::new((*x1.borrow())));
    let rx1: Ptr<i32> = x1.as_pointer();
    let px1: Value<Ptr<i32>> = Rc::new(RefCell::new((x1.as_pointer())));
    let x2: Value<i32> = Rc::new(RefCell::new((rx1.read())));
    let rx2: Ptr<i32> = (rx1).clone();
    let px2: Value<Ptr<i32>> = Rc::new(RefCell::new((rx1).clone()));
    let x3: Value<i32> = Rc::new(RefCell::new(((*px1.borrow()).read())));
    let rx3: Ptr<i32> = (*px1.borrow()).clone();
    let px3: Value<Ptr<i32>> = Rc::new(RefCell::new((*px1.borrow()).clone()));
    let res: Value<i32> = Rc::new(RefCell::new(((*x1.borrow()) + (*x2.borrow()))));
    (*res.borrow_mut()) = ((*x1.borrow()) + (*x2.borrow()));
    let y1: Value<Pair> = Rc::new(RefCell::new(Pair {
        x: 1,
        y: 2,
        a: Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5]))),
        r: x1.as_pointer(),
        p: Ptr::<i32>::null(),
        pair: Ptr::<Pair>::null(),
        ap: Rc::new(RefCell::new(Box::new([
            Ptr::<i32>::null(),
            Ptr::<i32>::null(),
        ]))),
    }));
    let y4: Value<Pair> = Rc::new(RefCell::new(Pair {
        x: { (*y1.borrow()).x },
        y: { (*y1.borrow()).y },
        a: Rc::new(RefCell::new(Box::new([
            (elem!((array_field_ptr!(y1.as_pointer(), a) as Ptr::<i32>), 0).read()),
            (elem!((array_field_ptr!(y1.as_pointer(), a) as Ptr::<i32>), 1).read()),
            (elem!((array_field_ptr!(y1.as_pointer(), a) as Ptr::<i32>), 2).read()),
            (elem!((array_field_ptr!(y1.as_pointer(), a) as Ptr::<i32>), 3).read()),
            (elem!((array_field_ptr!(y1.as_pointer(), a) as Ptr::<i32>), 4).read()),
        ]))),
        r: ({ (*y1.borrow()).r.clone() }).clone(),
        p: { (*y1.borrow()).p.clone() },
        pair: { (*y1.borrow()).pair.clone() },
        ap: Rc::new(RefCell::new(Box::new([
            (elem!(
                (array_field_ptr!(y1.as_pointer(), ap) as Ptr<Ptr::<i32>>),
                0
            )
            .read())
            .clone(),
            (elem!(
                (array_field_ptr!(y1.as_pointer(), ap) as Ptr<Ptr::<i32>>),
                1
            )
            .read())
            .clone(),
        ]))),
    }));
    let ry1: Ptr<Pair> = y1.as_pointer();
    let py1: Value<Ptr<Pair>> = Rc::new(RefCell::new((y1.as_pointer())));
    let y2: Value<Pair> = Rc::new(RefCell::new(Pair {
        x: ry1.with(|__s| __s.x),
        y: ry1.with(|__s| __s.y),
        a: Rc::new(RefCell::new(Box::new([
            (elem!((array_field_ptr!(ry1, a) as Ptr::<i32>), 0).read()),
            (elem!((array_field_ptr!(ry1, a) as Ptr::<i32>), 1).read()),
            (elem!((array_field_ptr!(ry1, a) as Ptr::<i32>), 2).read()),
            (elem!((array_field_ptr!(ry1, a) as Ptr::<i32>), 3).read()),
            (elem!((array_field_ptr!(ry1, a) as Ptr::<i32>), 4).read()),
        ]))),
        r: (ry1.with(|__s| __s.r.clone())).clone(),
        p: ry1.with(|__s| __s.p.clone()),
        pair: ry1.with(|__s| __s.pair.clone()),
        ap: Rc::new(RefCell::new(Box::new([
            (elem!((array_field_ptr!(ry1, ap) as Ptr<Ptr::<i32>>), 0).read()).clone(),
            (elem!((array_field_ptr!(ry1, ap) as Ptr<Ptr::<i32>>), 1).read()).clone(),
        ]))),
    }));
    let ry2: Ptr<Pair> = (ry1).clone();
    let py2: Value<Ptr<Pair>> = Rc::new(RefCell::new((ry1).clone()));
    let y3: Value<Pair> = Rc::new(RefCell::new(Pair {
        x: (*py1.borrow()).with(|__s| __s.x),
        y: (*py1.borrow()).with(|__s| __s.y),
        a: Rc::new(RefCell::new(Box::new([
            (elem!((array_field_ptr!((*py1.borrow()), a) as Ptr::<i32>), 0).read()),
            (elem!((array_field_ptr!((*py1.borrow()), a) as Ptr::<i32>), 1).read()),
            (elem!((array_field_ptr!((*py1.borrow()), a) as Ptr::<i32>), 2).read()),
            (elem!((array_field_ptr!((*py1.borrow()), a) as Ptr::<i32>), 3).read()),
            (elem!((array_field_ptr!((*py1.borrow()), a) as Ptr::<i32>), 4).read()),
        ]))),
        r: ((*py1.borrow()).with(|__s| __s.r.clone())).clone(),
        p: (*py1.borrow()).with(|__s| __s.p.clone()),
        pair: (*py1.borrow()).with(|__s| __s.pair.clone()),
        ap: Rc::new(RefCell::new(Box::new([
            (elem!(
                (array_field_ptr!((*py1.borrow()), ap) as Ptr<Ptr::<i32>>),
                0
            )
            .read())
            .clone(),
            (elem!(
                (array_field_ptr!((*py1.borrow()), ap) as Ptr<Ptr::<i32>>),
                1
            )
            .read())
            .clone(),
        ]))),
    }));
    let ry3: Ptr<Pair> = (*py1.borrow()).clone();
    let py3: Value<Ptr<Pair>> = Rc::new(RefCell::new((*py1.borrow()).clone()));
    (*py3.borrow_mut()) = Ptr::<Pair>::null();
    let ptr2pair: Value<Ptr<Pair>> = Rc::new(RefCell::new((*py3.borrow()).clone()));
    ({
        let _x1: i32 = (*x1.borrow());
        let _x2: Ptr<i32> = x1.as_pointer();
        let _x3: Ptr<i32> = (x1.as_pointer());
        let _p2: Ptr<Pair> = y1.as_pointer();
        let _p3: Ptr<Pair> = (y1.as_pointer());
        foo_1(_x1, _x2, _x3, _p2, _p3)
    });
    ({
        let _x1: i32 = (rx1.read());
        let _x2: Ptr<i32> = (rx1).clone();
        let _x3: Ptr<i32> = (rx1).clone();
        let _p2: Ptr<Pair> = (ry1).clone();
        let _p3: Ptr<Pair> = (ry1).clone();
        foo_1(_x1, _x2, _x3, _p2, _p3)
    });
    ({
        let _x1: i32 = ((*px1.borrow()).read());
        let _x2: Ptr<i32> = (*px1.borrow()).clone();
        let _x3: Ptr<i32> = (*px1.borrow()).clone();
        let _p2: Ptr<Pair> = (*py1.borrow()).clone();
        let _p3: Ptr<Pair> = (*py1.borrow()).clone();
        foo_1(_x1, _x2, _x3, _p2, _p3)
    });
    let cr1: Ptr<i32> = c1.as_pointer();
    let cp1: Value<Ptr<i32>> = Rc::new(RefCell::new((c1.as_pointer())));
    (*x1.borrow_mut()) = (*c1.borrow());
    (*x1.borrow_mut()) = 1;
    (*x1.borrow_mut()) = { (cr1.read()) };
    (*x1.borrow_mut()) = { ((*cp1.borrow()).read()) };
    rx1.write({ (*c1.borrow()) });
    rx2.write({ (cr1.read()) });
    rx3.write({ ((*cp1.borrow()).read()) });
    (*px1.borrow()).write({ (*c1.borrow()) });
    (*px2.borrow()).write({ (cr1.read()) });
    (*px3.borrow()).write({ ((*cp1.borrow()).read()) });
    (*px1.borrow_mut()) = (c1.as_pointer());
    (*px2.borrow_mut()) = (cr1).clone();
    (*px3.borrow_mut()) = (*cp1.borrow()).clone();
    (*y1.borrow_mut()).x = 2;
    (*y1.borrow_mut()).y = 3;
    elem!((array_field_ptr!(y1.as_pointer(), a) as Ptr::<i32>), 0).write(100);
    { (*y1.borrow()).r.clone() }.write(10);
    (*y1.borrow_mut()).p = (*px3.borrow()).clone();
    (*px3.borrow_mut()) = (*px2.borrow()).clone();
    (*y1.borrow_mut()).pair = (y3.as_pointer());
    field!({ (*y1.borrow()).pair.clone() }, x).write(100);
    field!({ (*y1.borrow()).pair.clone() }, pair).write((y2.as_pointer()));
    field!(
        { (*y1.borrow()).pair.clone() }.with(|__s| (__s).pair.clone()),
        x
    )
    .write(100);
    elem!(
        (array_field_ptr!(y1.as_pointer(), ap) as Ptr<Ptr::<i32>>),
        0
    )
    .write((x1.as_pointer()));
    elem!(
        (array_field_ptr!(y1.as_pointer(), ap) as Ptr<Ptr::<i32>>),
        1
    )
    .write((x2.as_pointer()));
    (elem!(
        (array_field_ptr!(y1.as_pointer(), ap) as Ptr<Ptr::<i32>>),
        0
    )
    .read())
    .write(0);
    (*c1.borrow_mut()) = ((*x1.borrow()) + 1);
    let j: Value<i32> = Rc::new(RefCell::new(0));
    let new_y: Value<Pair> = Rc::new(RefCell::new(Pair {
        x: 1,
        y: 2,
        a: Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5]))),
        r: j.as_pointer(),
        p: Ptr::<i32>::null(),
        pair: Ptr::<Pair>::null(),
        ap: Rc::new(RefCell::new(Box::new([
            Ptr::<i32>::null(),
            Ptr::<i32>::null(),
        ]))),
    }));
    (*y1.borrow_mut()).x = { { (*new_y.borrow()).x } };
    let i: Value<u32> = Rc::new(RefCell::new(1_u32));
    elem!(
        (array_field_ptr!(y1.as_pointer(), a) as Ptr::<i32>),
        (*i.borrow())
    )
    .write(-1_i32);
    (*x1.borrow_mut()).postfix_inc();
    (*x1.borrow_mut()).prefix_inc();
    (*y1.borrow_mut()).x.postfix_inc();
    field!({ (*y1.borrow()).pair.clone() }, pair).write((y2.as_pointer()));
    field!(
        { (*y1.borrow()).pair.clone() }.with(|__s| __s.pair.clone()),
        x
    )
    .write(10);
    ({ PairImpl::method(&y1.as_pointer()) });
    (*y1.borrow_mut()).pair = (y2.as_pointer());
    (*y2.borrow_mut()).pair = (y3.as_pointer());
    ({ PairImpl::method(&{ (*y1.borrow()).pair.clone() }.with(|__s| __s.pair.clone())) });
    let x: Value<X1> = Rc::new(RefCell::new(<X1>::default()));
    let y: Value<X1> = Rc::new(RefCell::new(<X1>::default()));
    (*x1.borrow_mut()) = (({ zero_0() }) + { (*y1.borrow()).x });
    (*y1.borrow_mut()).x = (({ zero_0() }) + 5);
    let ptr2ptr_1: Value<Ptr<Ptr<i32>>> = Rc::new(RefCell::new((px1.as_pointer())));
    let ptr2ptr_2: Value<Ptr<Ptr<Pair>>> = Rc::new(RefCell::new((py1.as_pointer())));
    return 0;
}
pub trait PairImpl {
    fn method(&self);
    fn as_val(&self) -> i32;
    fn as_ref(&self) -> Ptr<i32>;
    fn as_ptr(&self) -> Ptr<i32>;
}
impl PairImpl for Ptr<Pair> {
    fn method(&self) {
        field!((*self), x).with_mut(|__v| __v.postfix_inc());
        field!((*self), y).with_mut(|__v| __v.prefix_inc());
        elem!((array_field_ptr!((*self), a) as Ptr::<i32>), 4).write(1);
        (*self).with(|__s| __s.r.clone()).write(1);
        field!((*self), p).write(Ptr::<i32>::null());
        field!((*self), pair).write(Ptr::<Pair>::null());
        elem!((array_field_ptr!((*self), ap) as Ptr<Ptr::<i32>>), 0).write(Ptr::<i32>::null());
    }
    fn as_val(&self) -> i32 {
        return (*self).with(|__s| __s.x);
    }
    fn as_ref(&self) -> Ptr<i32> {
        return field_ptr!((*self), x);
    }
    fn as_ptr(&self) -> Ptr<i32> {
        return (field_ptr!((*self), x));
    }
}
pub fn __cpp2rust_init_globals() {}
