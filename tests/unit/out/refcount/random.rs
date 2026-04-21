extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive()]
pub struct Pair {
    pub x: Value<i32>,
    pub y: Value<i32>,
    pub a: Value<Box<[i32]>>,
    pub r: Ptr<i32>,
    pub p: Value<Ptr<i32>>,
    pub pair: Value<Ptr<Pair>>,
    pub ap: Value<Box<[Ptr<i32>]>>,
}
impl Pair {
    pub fn method(&self) {
        (*self.x.borrow_mut()).postfix_inc();
        (*self.y.borrow_mut()).prefix_inc();
        (*self.a.borrow_mut())[(4) as usize] = 1;
        self.r.write(1);
        (*self.p.borrow_mut()) = Default::default();
        (*self.pair.borrow_mut()) = Default::default();
        (*self.ap.borrow_mut())[(0) as usize] = Default::default();
    }
    pub fn as_val(&self) -> i32 {
        return (*self.x.borrow());
    }
    pub fn as_ref(&self) -> Ptr<i32> {
        return self.x.as_pointer();
    }
    pub fn as_ptr(&self) -> Ptr<i32> {
        return (self.x.as_pointer());
    }
}
impl Clone for Pair {
    fn clone(&self) -> Self {
        let mut this = Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
            y: Rc::new(RefCell::new((*self.y.borrow()))),
            a: Rc::new(RefCell::new((*self.a.borrow()).clone())),
            r: (self.r).clone(),
            p: Rc::new(RefCell::new((*self.p.borrow()).clone())),
            pair: Rc::new(RefCell::new((*self.pair.borrow()).clone())),
            ap: Rc::new(RefCell::new((*self.ap.borrow()).clone())),
        };
        this
    }
}
impl Default for Pair {
    fn default() -> Self {
        Pair {
            x: <Value<i32>>::default(),
            y: <Value<i32>>::default(),
            a: Rc::new(RefCell::new(
                (0..5).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
            )),
            r: <Ptr<i32>>::default(),
            p: Rc::new(RefCell::new(Ptr::<i32>::null())),
            pair: Rc::new(RefCell::new(Ptr::<Pair>::null())),
            ap: Rc::new(RefCell::new(
                (0..2)
                    .map(|_| Ptr::<i32>::null())
                    .collect::<Box<[Ptr<i32>]>>(),
            )),
        }
    }
}
impl ByteRepr for Pair {}
pub fn zero_0() -> i32 {
    return 0;
}
#[derive(Default)]
pub struct X1 {}
impl Clone for X1 {
    fn clone(&self) -> Self {
        let mut this = Self {};
        this
    }
}
impl ByteRepr for X1 {}
pub fn foo_1(x1: i32, x2: Ptr<i32>, x3: Ptr<i32>, p2: Ptr<Pair>, p3: Ptr<Pair>) {
    let x1: Value<i32> = Rc::new(RefCell::new(x1));
    let x3: Value<Ptr<i32>> = Rc::new(RefCell::new(x3));
    let p3: Value<Ptr<Pair>> = Rc::new(RefCell::new(p3));
}
pub fn main() {
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
        x: Rc::new(RefCell::new(1)),
        y: Rc::new(RefCell::new(2)),
        a: Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5]))),
        r: x1.as_pointer(),
        p: Rc::new(RefCell::new(Default::default())),
        pair: Rc::new(RefCell::new(Default::default())),
        ap: Rc::new(RefCell::new(Box::new([
            Default::default(),
            Default::default(),
        ]))),
    }));
    let y4: Value<Pair> = Rc::new(RefCell::new(Pair {
        x: Rc::new(RefCell::new((*(*y1.borrow()).x.borrow()))),
        y: Rc::new(RefCell::new((*(*y1.borrow()).y.borrow()))),
        a: Rc::new(RefCell::new(Box::new([
            (*(*y1.borrow()).a.borrow())[(0) as usize],
            (*(*y1.borrow()).a.borrow())[(1) as usize],
            (*(*y1.borrow()).a.borrow())[(2) as usize],
            (*(*y1.borrow()).a.borrow())[(3) as usize],
            (*(*y1.borrow()).a.borrow())[(4) as usize],
        ]))),
        r: ((*y1.borrow()).r).clone(),
        p: Rc::new(RefCell::new((*(*y1.borrow()).p.borrow()).clone())),
        pair: Rc::new(RefCell::new((*(*y1.borrow()).pair.borrow()).clone())),
        ap: Rc::new(RefCell::new(Box::new([
            ((*(*y1.borrow()).ap.borrow())[(0) as usize]).clone(),
            ((*(*y1.borrow()).ap.borrow())[(1) as usize]).clone(),
        ]))),
    }));
    let ry1: Ptr<Pair> = y1.as_pointer();
    let py1: Value<Ptr<Pair>> = Rc::new(RefCell::new((y1.as_pointer())));
    let y2: Value<Pair> = Rc::new(RefCell::new(Pair {
        x: Rc::new(RefCell::new((*(*ry1.upgrade().deref()).x.borrow()))),
        y: Rc::new(RefCell::new((*(*ry1.upgrade().deref()).y.borrow()))),
        a: Rc::new(RefCell::new(Box::new([
            (*(*ry1.upgrade().deref()).a.borrow())[(0) as usize],
            (*(*ry1.upgrade().deref()).a.borrow())[(1) as usize],
            (*(*ry1.upgrade().deref()).a.borrow())[(2) as usize],
            (*(*ry1.upgrade().deref()).a.borrow())[(3) as usize],
            (*(*ry1.upgrade().deref()).a.borrow())[(4) as usize],
        ]))),
        r: ((*ry1.upgrade().deref()).r).clone(),
        p: Rc::new(RefCell::new((*(*ry1.upgrade().deref()).p.borrow()).clone())),
        pair: Rc::new(RefCell::new(
            (*(*ry1.upgrade().deref()).pair.borrow()).clone(),
        )),
        ap: Rc::new(RefCell::new(Box::new([
            ((*(*ry1.upgrade().deref()).ap.borrow())[(0) as usize]).clone(),
            ((*(*ry1.upgrade().deref()).ap.borrow())[(1) as usize]).clone(),
        ]))),
    }));
    let ry2: Ptr<Pair> = (ry1).clone();
    let py2: Value<Ptr<Pair>> = Rc::new(RefCell::new((ry1).clone()));
    let y3: Value<Pair> = Rc::new(RefCell::new(Pair {
        x: Rc::new(RefCell::new(
            (*(*(*py1.borrow()).upgrade().deref()).x.borrow()),
        )),
        y: Rc::new(RefCell::new(
            (*(*(*py1.borrow()).upgrade().deref()).y.borrow()),
        )),
        a: Rc::new(RefCell::new(Box::new([
            (*(*(*py1.borrow()).upgrade().deref()).a.borrow())[(0) as usize],
            (*(*(*py1.borrow()).upgrade().deref()).a.borrow())[(1) as usize],
            (*(*(*py1.borrow()).upgrade().deref()).a.borrow())[(2) as usize],
            (*(*(*py1.borrow()).upgrade().deref()).a.borrow())[(3) as usize],
            (*(*(*py1.borrow()).upgrade().deref()).a.borrow())[(4) as usize],
        ]))),
        r: ((*(*py1.borrow()).upgrade().deref()).r).clone(),
        p: Rc::new(RefCell::new(
            (*(*(*py1.borrow()).upgrade().deref()).p.borrow()).clone(),
        )),
        pair: Rc::new(RefCell::new(
            (*(*(*py1.borrow()).upgrade().deref()).pair.borrow()).clone(),
        )),
        ap: Rc::new(RefCell::new(Box::new([
            ((*(*(*py1.borrow()).upgrade().deref()).ap.borrow())[(0) as usize]).clone(),
            ((*(*(*py1.borrow()).upgrade().deref()).ap.borrow())[(1) as usize]).clone(),
        ]))),
    }));
    let ry3: Ptr<Pair> = (*py1.borrow()).clone();
    let py3: Value<Ptr<Pair>> = Rc::new(RefCell::new((*py1.borrow()).clone()));
    (*py3.borrow_mut()) = Default::default();
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
    let __rhs = (cr1.read());
    (*x1.borrow_mut()) = __rhs;
    let __rhs = ((*cp1.borrow()).read());
    (*x1.borrow_mut()) = __rhs;
    let __rhs = (*c1.borrow());
    rx1.write(__rhs);
    let __rhs = (cr1.read());
    rx2.write(__rhs);
    let __rhs = ((*cp1.borrow()).read());
    rx3.write(__rhs);
    let __rhs = (*c1.borrow());
    (*px1.borrow()).write(__rhs);
    let __rhs = (cr1.read());
    (*px2.borrow()).write(__rhs);
    let __rhs = ((*cp1.borrow()).read());
    (*px3.borrow()).write(__rhs);
    (*px1.borrow_mut()) = (c1.as_pointer());
    (*px2.borrow_mut()) = (cr1).clone();
    (*px3.borrow_mut()) = (*cp1.borrow()).clone();
    (*(*y1.borrow()).x.borrow_mut()) = 2;
    (*(*y1.borrow()).y.borrow_mut()) = 3;
    (*(*y1.borrow()).a.borrow_mut())[(0) as usize] = 100;
    (*y1.borrow()).r.write(10);
    (*(*y1.borrow()).p.borrow_mut()) = (*px3.borrow()).clone();
    (*px3.borrow_mut()) = (*px2.borrow()).clone();
    (*(*y1.borrow()).pair.borrow_mut()) = (y3.as_pointer());
    (*(*(*(*y1.borrow()).pair.borrow()).upgrade().deref())
        .x
        .borrow_mut()) = 100;
    (*(*(*(*y1.borrow()).pair.borrow()).upgrade().deref())
        .pair
        .borrow_mut()) = (y2.as_pointer());
    (*(*(*(*(*(*y1.borrow()).pair.borrow()).upgrade().deref())
        .pair
        .borrow())
    .upgrade()
    .deref())
    .x
    .borrow_mut()) = 100;
    (*(*y1.borrow()).ap.borrow_mut())[(0) as usize] = (x1.as_pointer());
    (*(*y1.borrow()).ap.borrow_mut())[(1) as usize] = (x2.as_pointer());
    (*(*y1.borrow()).ap.borrow())[(0) as usize].write(0);
    (*c1.borrow_mut()) = ((*x1.borrow()) + 1);
    let j: Value<i32> = Rc::new(RefCell::new(0));
    let new_y: Value<Pair> = Rc::new(RefCell::new(Pair {
        x: Rc::new(RefCell::new(1)),
        y: Rc::new(RefCell::new(2)),
        a: Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5]))),
        r: j.as_pointer(),
        p: Rc::new(RefCell::new(Default::default())),
        pair: Rc::new(RefCell::new(Default::default())),
        ap: Rc::new(RefCell::new(Box::new([
            Default::default(),
            Default::default(),
        ]))),
    }));
    let __rhs = (*(*new_y.borrow()).x.borrow());
    (*(*y1.borrow()).x.borrow_mut()) = __rhs;
    let i: Value<u32> = Rc::new(RefCell::new(1_u32));
    (*(*y1.borrow()).a.borrow_mut())[(*i.borrow()) as usize] = -1_i32;
    (*x1.borrow_mut()).postfix_inc();
    (*x1.borrow_mut()).prefix_inc();
    (*(*y1.borrow()).x.borrow_mut()).postfix_inc();
    (*(*(*(*y1.borrow()).pair.borrow()).upgrade().deref())
        .pair
        .borrow_mut()) = (y2.as_pointer());
    (*(*(*(*(*(*y1.borrow()).pair.borrow()).upgrade().deref())
        .pair
        .borrow())
    .upgrade()
    .deref())
    .x
    .borrow_mut()) = 10;
    ({ (*y1.borrow()).method() });
    (*(*y1.borrow()).pair.borrow_mut()) = (y2.as_pointer());
    (*(*y2.borrow()).pair.borrow_mut()) = (y3.as_pointer());
    ({
        (*(*(*(*(*y1.borrow()).pair.borrow()).upgrade().deref())
            .pair
            .borrow())
        .upgrade()
        .deref())
        .method()
    });
    let x: Value<X1> = Rc::new(RefCell::new(X1 {}));
    let y: Value<X1> = Rc::new(RefCell::new(X1 {}));
    (*x1.borrow_mut()) = (({ zero_0() }) + (*(*y1.borrow()).x.borrow()));
    (*(*y1.borrow()).x.borrow_mut()) = (({ zero_0() }) + 5);
    let ptr2ptr_1: Value<Ptr<Ptr<i32>>> = Rc::new(RefCell::new((px1.as_pointer())));
    let ptr2ptr_2: Value<Ptr<Ptr<Pair>>> = Rc::new(RefCell::new((py1.as_pointer())));
    return 0;
}
