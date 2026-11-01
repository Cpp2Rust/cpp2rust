extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Bar {
    #[offset(0)]
    pub w: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(40)]
pub struct Foo {
    #[offset(0)]
    pub x: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub y: Ptr<i32>,
    #[offset(16)]
    #[byte_size(8)]
    pub z: Ptr<i32>,
    #[offset(24)]
    #[byte_size(12)]
    pub a: Box<[i32]>,
    #[offset(36)]
    #[byte_size(4)]
    pub bar: Bar,
}
impl Default for Foo {
    fn default() -> Self {
        {
            Foo {
                x: 0_i32,
                y: <Ptr<i32>>::default(),
                z: Ptr::<i32>::null(),
                a: (0..3).map(|_| 0_i32).collect::<Box<[i32]>>(),
                bar: <Bar>::default(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Refs {
    #[offset(0)]
    #[byte_size(8)]
    pub a: Ptr<i32>,
    #[offset(8)]
    #[byte_size(8)]
    pub b: Ptr<i32>,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x1: Value<i32> = Rc::new(RefCell::new(1));
    let mut x2: i32 = (*x1.borrow());
    x2.prefix_inc();
    assert!(({ ((*x1.borrow()) == 1) }));
    assert!((x2 == 2));
    let mut x3: f64 = 3_f64;
    let mut x4: f64 = x3;
    x4.prefix_inc();
    assert!((x3 == 3_f64));
    assert!((x4 == 4_f64));
    let reference: Ptr<i32> = x1.as_pointer();
    let mut x5: i32 = (reference.read());
    x5.prefix_inc();
    assert!(((reference.read()) == 1));
    assert!((x5 == 2));
    let mut pointer: Ptr<i32> = (x1.as_pointer());
    let mut x6: i32 = (pointer.read());
    x6.prefix_inc();
    assert!(((pointer.read()) == 1));
    assert!((x6 == 2));
    let mut other_pointer: Ptr<i32> = (pointer).clone();
    assert!(({ (other_pointer).clone() } == { (pointer).clone() }));
    other_pointer.with_mut(|__v| __v.prefix_inc());
    assert!(({ (other_pointer.read()) } == { (pointer.read()) }));
    let mut f1: Foo = Foo {
        x: 1,
        y: x1.as_pointer(),
        z: (x1.as_pointer()),
        a: Box::new([0, 1, 2]),
        bar: Bar { w: 10 },
    };
    assert!((f1.x == 1));
    assert!(((f1.y.read()) == 2));
    assert!(({ (f1.z).clone() } == { (x1.as_pointer()) }));
    assert!(((f1.z.read()) == 2));
    let mut f2: Foo = (f1).clone();
    f2.x.prefix_inc();
    f2.y.with_mut(|__v| __v.prefix_inc());
    assert!((f2.x == 2));
    assert!(((f2.y.read()) == 3));
    assert!((f1.x == 1));
    assert!(((f1.y.read()) == 3));
    f2.z.with_mut(|__v| __v.prefix_inc());
    assert!(((f2.y.read()) == 4));
    assert!(({ (f2.z).clone() } == { (x1.as_pointer()) }));
    assert!(((f2.z.read()) == 4));
    assert!(((f1.y.read()) == 4));
    assert!(({ (f1.z).clone() } == { (x1.as_pointer()) }));
    assert!(((f1.z.read()) == 4));
    f2.a[(0) as usize].prefix_inc();
    f2.a[(1) as usize].prefix_inc();
    f2.a[(2) as usize].prefix_inc();
    assert!((f2.a[(0) as usize] == 1));
    assert!((f2.a[(1) as usize] == 2));
    assert!((f2.a[(2) as usize] == 3));
    assert!((f1.a[(0) as usize] == 0));
    assert!((f1.a[(1) as usize] == 1));
    assert!((f1.a[(2) as usize] == 2));
    f2.bar.w = 20;
    assert!((f2.bar.w == 20));
    assert!((f1.bar.w == 10));
    let mut N: i32 = 5;
    let mut v1: Vec<i32> = Vec::new();
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        {
            let a0_clone = i.clone();
            v1.push(a0_clone)
        };
        i.prefix_inc();
    }
    let mut v2: Vec<i32> = v1.clone();
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        assert!((v2[(i as usize)] == i));
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        v2[(i as usize)].prefix_inc();
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        assert!((v2[(i as usize)] == (i + 1)));
        assert!((v1[(i as usize)] == i));
        i.prefix_inc();
    }
    let m1: Value<Vec<Vec<i32>>> = Rc::new(RefCell::new(Vec::new()));
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        {
            let __a1 = (0..(10_usize) as usize).map(|_| 0_i32).collect::<Vec<_>>();
            (*m1.borrow_mut()).push(__a1)
        };
        i.prefix_inc();
    }
    let m2: Value<Vec<Vec<i32>>> = Rc::new(RefCell::new((*m1.borrow()).clone()));
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        assert!(({ ((*m1.decay().offset((i as usize)).upgrade().deref()).len() == 10_usize) }));
        assert!(({ ((*m2.decay().offset((i as usize)).upgrade().deref()).len() == 10_usize) }));
        let mut j: i32 = 0;
        'loop_: while (j < 10) {
            assert!(((index!(elem!(m1.decay(), (i as usize)), (j as usize)).read()) == 0));
            assert!(((index!(elem!(m2.decay(), (i as usize)), (j as usize)).read()) == 0));
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        let mut j: i32 = 0;
        'loop_: while (j < 10) {
            index!(elem!(m2.decay(), (i as usize)), (j as usize)).with_mut(|__v| __v.postfix_inc());
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        assert!(({ ((*m1.decay().offset((i as usize)).upgrade().deref()).len() == 10_usize) }));
        assert!(({ ((*m2.decay().offset((i as usize)).upgrade().deref()).len() == 10_usize) }));
        let mut j: i32 = 0;
        'loop_: while (j < 10) {
            assert!(((index!(elem!(m1.decay(), (i as usize)), (j as usize)).read()) == 0));
            assert!(((index!(elem!(m2.decay(), (i as usize)), (j as usize)).read()) == 1));
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let map1: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < N) {
        let __rhs = (*i.borrow());
        Map::subscript(map1.as_pointer(), i.as_pointer()).write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    let map2: Value<Map<i32, (Value<i32>, Value<i32>)>> =
        Rc::new(RefCell::new((*map1.borrow()).clone()));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < N) {
        assert!(
            ({ ((Map::subscript(map2.as_pointer(), i.as_pointer()).read()) == (*i.borrow())) })
        );
        Map::subscript(map2.as_pointer(), i.as_pointer()).with_mut(|__v| __v.prefix_inc());
        (*i.borrow_mut()).prefix_inc();
    }
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < N) {
        assert!(
            ({ ((Map::subscript(map1.as_pointer(), i.as_pointer()).read()) == (*i.borrow())) })
        );
        assert!(
            ({
                ((Map::subscript(map2.as_pointer(), i.as_pointer()).read()) == ((*i.borrow()) + 1))
            })
        );
        (*i.borrow_mut()).prefix_inc();
    }
    let pair1: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(1)),
        Rc::new(RefCell::new(2)),
    )));
    let pair2: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new((*pair1.borrow()).0.borrow().clone())),
        Rc::new(RefCell::new((*pair1.borrow()).1.borrow().clone())),
    )));
    (*(*pair2.borrow()).0.borrow_mut()) = { ((*(*pair2.borrow()).0.borrow()) * 10) };
    (*(*pair2.borrow()).1.borrow_mut()) = { ((*(*pair2.borrow()).1.borrow()) * 10) };
    assert!(({ ((*(*pair2.borrow()).0.borrow()) == 10) }));
    assert!(({ ((*(*pair2.borrow()).1.borrow()) == 20) }));
    assert!(({ ((*(*pair1.borrow()).0.borrow()) == 1) }));
    assert!(({ ((*(*pair1.borrow()).1.borrow()) == 2) }));
    let pair3: Value<(Value<Vec<i32>>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(
            (0..(0_usize) as usize).map(|_| 0_i32).collect::<Vec<_>>(),
        )),
        Rc::new(RefCell::new(0)),
    )));
    let pair4: Value<(Value<Vec<i32>>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new((*pair3.borrow()).0.borrow().clone())),
        Rc::new(RefCell::new((*pair3.borrow()).1.borrow().clone())),
    )));
    {
        let __a1 = 1;
        (*(*pair4.borrow()).0.borrow_mut()).push(__a1)
    };
    (*(*pair4.borrow()).1.borrow_mut()) = 1;
    assert!(({ ((*(*pair4.borrow()).0.borrow()).len() == 1_usize) }));
    assert!(({ ((*(*pair4.borrow()).1.borrow()) == 1) }));
    assert!(({ ((*(*pair3.borrow()).0.borrow()).len() == 0_usize) }));
    assert!(({ ((*(*pair3.borrow()).1.borrow()) == 0) }));
    let mut s1: Vec<i8> = vec![('a' as i8); (3_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0))
        .collect();
    let mut s2: Vec<i8> = (s1).clone();
    s2[0_usize] = ('b' as i8);
    s2[1_usize] = ('b' as i8);
    s2[2_usize] = ('b' as i8);
    assert!(((s2[0_usize] as i32) == (('b' as i8) as i32)));
    assert!(((s2[1_usize] as i32) == (('b' as i8) as i32)));
    assert!(((s2[2_usize] as i32) == (('b' as i8) as i32)));
    assert!(((s1[0_usize] as i32) == (('a' as i8) as i32)));
    assert!(((s1[1_usize] as i32) == (('a' as i8) as i32)));
    assert!(((s1[2_usize] as i32) == (('a' as i8) as i32)));
    let mut b1: Bar = Bar { w: 1 };
    let mut b2: Bar = Bar { w: 2 };
    b2 = (b1).clone();
    b2.w.postfix_inc();
    assert!((b1.w == 1));
    assert!((b2.w == 2));
    let v4: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    v4.as_pointer().write((v2).clone());
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        assert!(({ ({ (*v4.borrow())[(i as usize)] } == (i + 1)) }));
        (*v4.borrow_mut())[(i as usize)].prefix_inc();
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        assert!(({ ({ (*v4.borrow())[(i as usize)] } == (i + 2)) }));
        assert!((v2[(i as usize)] == (i + 1)));
        i.prefix_inc();
    }
    let ra: Value<i32> = Rc::new(RefCell::new(1));
    let rb: Value<i32> = Rc::new(RefCell::new(2));
    let mut r1: Refs = Refs {
        a: ra.as_pointer(),
        b: rb.as_pointer(),
    };
    let mut r2: Refs = (r1).clone();
    r2.a.write(10);
    r2.b.with_mut(|__v| __v.prefix_inc());
    assert!(({ ((*ra.borrow()) == 10) }));
    assert!(({ ((*rb.borrow()) == 3) }));
    assert!(((r1.a.read()) == 10));
    assert!(((r1.b.read()) == 3));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
