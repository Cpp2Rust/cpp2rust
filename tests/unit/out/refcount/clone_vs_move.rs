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
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
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
    pub a: Value<Box<[i32]>>,
    #[offset(36)]
    #[byte_size(4)]
    pub bar: Bar,
}
impl Default for Foo {
    fn default() -> Self {
        Foo {
            x: 0_i32,
            y: <Ptr<i32>>::default(),
            z: Ptr::<i32>::null(),
            a: Rc::new(RefCell::new((0..3).map(|_| 0_i32).collect::<Box<[i32]>>())),
            bar: <Bar>::default(),
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
    assert!(((*x1.borrow()) == 1));
    assert!((x2 == 2));
    let mut x3: f64 = 3.0E+0;
    let mut x4: f64 = x3;
    x4.prefix_inc();
    assert!((x3 == 3.0E+0));
    assert!((x4 == 4.0E+0));
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
    let f1: Value<Foo> = Rc::new(RefCell::new(Foo {
        x: 1,
        y: x1.as_pointer(),
        z: (x1.as_pointer()),
        a: Rc::new(RefCell::new(Box::new([0, 1, 2]))),
        bar: Bar { w: 10 },
    }));
    assert!(({ (*f1.borrow()).x } == 1));
    assert!((({ (*f1.borrow()).y.clone() }.read()) == 2));
    assert!(({ { (*f1.borrow()).z.clone() } } == { (x1.as_pointer()) }));
    assert!((({ (*f1.borrow()).z.clone() }.read()) == 2));
    let f2: Value<Foo> = Rc::new(RefCell::new((*f1.borrow()).clone()));
    (*f2.borrow_mut()).x.prefix_inc();
    { (*f2.borrow()).y.clone() }.with_mut(|__v| __v.prefix_inc());
    assert!(({ (*f2.borrow()).x } == 2));
    assert!((({ (*f2.borrow()).y.clone() }.read()) == 3));
    assert!(({ (*f1.borrow()).x } == 1));
    assert!((({ (*f1.borrow()).y.clone() }.read()) == 3));
    { (*f2.borrow()).z.clone() }.with_mut(|__v| __v.prefix_inc());
    assert!((({ (*f2.borrow()).y.clone() }.read()) == 4));
    assert!(({ { (*f2.borrow()).z.clone() } } == { (x1.as_pointer()) }));
    assert!((({ (*f2.borrow()).z.clone() }.read()) == 4));
    assert!((({ (*f1.borrow()).y.clone() }.read()) == 4));
    assert!(({ { (*f1.borrow()).z.clone() } } == { (x1.as_pointer()) }));
    assert!((({ (*f1.borrow()).z.clone() }.read()) == 4));
    elem!((array_field_ptr!(f2.as_pointer(), a) as Ptr::<i32>), 0).with_mut(|__v| __v.prefix_inc());
    elem!((array_field_ptr!(f2.as_pointer(), a) as Ptr::<i32>), 1).with_mut(|__v| __v.prefix_inc());
    elem!((array_field_ptr!(f2.as_pointer(), a) as Ptr::<i32>), 2).with_mut(|__v| __v.prefix_inc());
    assert!(((elem!((array_field_ptr!(f2.as_pointer(), a) as Ptr::<i32>), 0).read()) == 1));
    assert!(((elem!((array_field_ptr!(f2.as_pointer(), a) as Ptr::<i32>), 1).read()) == 2));
    assert!(((elem!((array_field_ptr!(f2.as_pointer(), a) as Ptr::<i32>), 2).read()) == 3));
    assert!(((elem!((array_field_ptr!(f1.as_pointer(), a) as Ptr::<i32>), 0).read()) == 0));
    assert!(((elem!((array_field_ptr!(f1.as_pointer(), a) as Ptr::<i32>), 1).read()) == 1));
    assert!(((elem!((array_field_ptr!(f1.as_pointer(), a) as Ptr::<i32>), 2).read()) == 2));
    (*f2.borrow_mut()).bar.w = 20;
    assert!(({ (*f2.borrow()).bar.w } == 20));
    assert!(({ (*f1.borrow()).bar.w } == 10));
    let mut N: i32 = 5;
    let v1: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < N) {
        {
            let a0_clone = (*i.borrow()).clone();
            (*v1.borrow_mut()).push(a0_clone)
        };
        (*i.borrow_mut()).prefix_inc();
    }
    let v2: Value<Vec<i32>> = Rc::new(RefCell::new((*v1.borrow()).clone()));
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        assert!(((elem!((v2.as_pointer() as Ptr<i32>), (i as usize)).read()) == i));
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        elem!((v2.as_pointer() as Ptr<i32>), (i as usize)).with_mut(|__v| __v.prefix_inc());
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        assert!(((elem!((v2.as_pointer() as Ptr<i32>), (i as usize)).read()) == (i + 1)));
        assert!(((elem!((v1.as_pointer() as Ptr<i32>), (i as usize)).read()) == i));
        i.prefix_inc();
    }
    let m1: Value<Vec<Value<Vec<i32>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        (m1.as_pointer() as Ptr<Vec<Value<Vec<i32>>>>).with_mut(
            |__v: &mut Vec<Value<Vec<i32>>>| {
                __v.push(Rc::new(RefCell::new(
                    (0..(10_usize) as usize)
                        .map(|_| <i32>::default())
                        .collect::<Vec<_>>(),
                )))
            },
        );
        i.prefix_inc();
    }
    let m2: Value<Vec<Value<Vec<i32>>>> = Rc::new(RefCell::new(
        (*m1.borrow())
            .iter()
            .map(|inner_vec| Rc::new(RefCell::new(inner_vec.borrow().clone())))
            .collect(),
    ));
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        assert!(
            ((*((m1.as_pointer() as Ptr<Value<Vec<i32>>>)
                .offset((i as usize))
                .upgrade()
                .deref()
                .as_pointer() as Ptr<Vec<i32>>)
                .upgrade()
                .deref())
            .len()
                == 10_usize)
        );
        assert!(
            ((*((m2.as_pointer() as Ptr<Value<Vec<i32>>>)
                .offset((i as usize))
                .upgrade()
                .deref()
                .as_pointer() as Ptr<Vec<i32>>)
                .upgrade()
                .deref())
            .len()
                == 10_usize)
        );
        let mut j: i32 = 0;
        'loop_: while (j < 10) {
            assert!(
                ((elem!(
                    ((m1.as_pointer() as Ptr<Value<Vec<i32>>>)
                        .offset((i as usize))
                        .upgrade()
                        .deref()
                        .as_pointer() as Ptr<i32>),
                    (j as usize)
                )
                .read())
                    == 0)
            );
            assert!(
                ((elem!(
                    ((m2.as_pointer() as Ptr<Value<Vec<i32>>>)
                        .offset((i as usize))
                        .upgrade()
                        .deref()
                        .as_pointer() as Ptr<i32>),
                    (j as usize)
                )
                .read())
                    == 0)
            );
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        let mut j: i32 = 0;
        'loop_: while (j < 10) {
            elem!(
                ((m2.as_pointer() as Ptr<Value<Vec<i32>>>)
                    .offset((i as usize))
                    .upgrade()
                    .deref()
                    .as_pointer() as Ptr<i32>),
                (j as usize)
            )
            .with_mut(|__v| __v.postfix_inc());
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        assert!(
            ((*((m1.as_pointer() as Ptr<Value<Vec<i32>>>)
                .offset((i as usize))
                .upgrade()
                .deref()
                .as_pointer() as Ptr<Vec<i32>>)
                .upgrade()
                .deref())
            .len()
                == 10_usize)
        );
        assert!(
            ((*((m2.as_pointer() as Ptr<Value<Vec<i32>>>)
                .offset((i as usize))
                .upgrade()
                .deref()
                .as_pointer() as Ptr<Vec<i32>>)
                .upgrade()
                .deref())
            .len()
                == 10_usize)
        );
        let mut j: i32 = 0;
        'loop_: while (j < 10) {
            assert!(
                ((elem!(
                    ((m1.as_pointer() as Ptr<Value<Vec<i32>>>)
                        .offset((i as usize))
                        .upgrade()
                        .deref()
                        .as_pointer() as Ptr<i32>),
                    (j as usize)
                )
                .read())
                    == 0)
            );
            assert!(
                ((elem!(
                    ((m2.as_pointer() as Ptr<Value<Vec<i32>>>)
                        .offset((i as usize))
                        .upgrade()
                        .deref()
                        .as_pointer() as Ptr<i32>),
                    (j as usize)
                )
                .read())
                    == 1)
            );
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let map1: Value<BTreeMap<i32, Value<i32>>> = Rc::new(RefCell::new(BTreeMap::new()));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < N) {
        let __rhs = (*i.borrow());
        (map1.as_pointer() as Ptr<BTreeMap<i32, Value<i32>>>)
            .with_mut(|__v: &mut BTreeMap<i32, Value<i32>>| {
                __v.entry((*i.borrow()))
                    .or_insert_with(|| Rc::new(RefCell::new(<i32>::default())))
                    .as_pointer()
            })
            .write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    let map2: Value<BTreeMap<i32, Value<i32>>> = Rc::new(RefCell::new(
        (*map1.borrow())
            .iter()
            .map(|(k, v)| (k.clone(), Rc::new(RefCell::new(v.borrow().clone()))))
            .collect(),
    ));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < N) {
        assert!(
            (((map2.as_pointer() as Ptr<BTreeMap<i32, Value<i32>>>)
                .with_mut(|__v: &mut BTreeMap<i32, Value<i32>>| {
                    __v.entry((*i.borrow()))
                        .or_insert_with(|| Rc::new(RefCell::new(<i32>::default())))
                        .as_pointer()
                })
                .read())
                == (*i.borrow()))
        );
        (map2.as_pointer() as Ptr<BTreeMap<i32, Value<i32>>>)
            .with_mut(|__v: &mut BTreeMap<i32, Value<i32>>| {
                __v.entry((*i.borrow()))
                    .or_insert_with(|| Rc::new(RefCell::new(<i32>::default())))
                    .as_pointer()
            })
            .with_mut(|__v| __v.prefix_inc());
        (*i.borrow_mut()).prefix_inc();
    }
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < N) {
        assert!(
            (((map1.as_pointer() as Ptr<BTreeMap<i32, Value<i32>>>)
                .with_mut(|__v: &mut BTreeMap<i32, Value<i32>>| {
                    __v.entry((*i.borrow()))
                        .or_insert_with(|| Rc::new(RefCell::new(<i32>::default())))
                        .as_pointer()
                })
                .read())
                == (*i.borrow()))
        );
        assert!(
            (((map2.as_pointer() as Ptr<BTreeMap<i32, Value<i32>>>)
                .with_mut(|__v: &mut BTreeMap<i32, Value<i32>>| {
                    __v.entry((*i.borrow()))
                        .or_insert_with(|| Rc::new(RefCell::new(<i32>::default())))
                        .as_pointer()
                })
                .read())
                == ((*i.borrow()) + 1))
        );
        (*i.borrow_mut()).prefix_inc();
    }
    let pair1: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(1.try_into().expect("failed conversion"))),
        Rc::new(RefCell::new(2.try_into().expect("failed conversion"))),
    )));
    let pair2: Value<(Value<i32>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new((*pair1.borrow()).0.borrow().clone())),
        Rc::new(RefCell::new((*pair1.borrow()).1.borrow().clone())),
    )));
    (*(*pair2.borrow()).0.borrow_mut()) = { ((*(*pair2.borrow()).0.borrow()) * 10) };
    (*(*pair2.borrow()).1.borrow_mut()) = { ((*(*pair2.borrow()).1.borrow()) * 10) };
    assert!(((*(*pair2.borrow()).0.borrow()) == 10));
    assert!(((*(*pair2.borrow()).1.borrow()) == 20));
    assert!(((*(*pair1.borrow()).0.borrow()) == 1));
    assert!(((*(*pair1.borrow()).1.borrow()) == 2));
    let pair3: Value<(Value<Vec<i32>>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(
            (0..(0_usize) as usize)
                .map(|_| <i32>::default())
                .collect::<Vec<_>>()
                .try_into()
                .expect("failed conversion"),
        )),
        Rc::new(RefCell::new(0.try_into().expect("failed conversion"))),
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
    assert!(((*(*pair4.borrow()).0.borrow()).len() == 1_usize));
    assert!(((*(*pair4.borrow()).1.borrow()) == 1));
    assert!(((*(*pair3.borrow()).0.borrow()).len() == 0_usize));
    assert!(((*(*pair3.borrow()).1.borrow()) == 0));
    let s1: Value<Vec<i8>> = Rc::new(RefCell::new(
        vec![('a' as i8); (3_usize) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once(0))
            .collect(),
    ));
    let s2: Value<Vec<i8>> = Rc::new(RefCell::new((*s1.borrow()).clone()));
    elem!((s2.as_pointer() as Ptr<i8>), 0_usize).write(('b' as i8));
    elem!((s2.as_pointer() as Ptr<i8>), 1_usize).write(('b' as i8));
    elem!((s2.as_pointer() as Ptr<i8>), 2_usize).write(('b' as i8));
    assert!(
        (((elem!((s2.as_pointer() as Ptr<i8>), 0_usize).read()) as i32) == (('b' as i8) as i32))
    );
    assert!(
        (((elem!((s2.as_pointer() as Ptr<i8>), 1_usize).read()) as i32) == (('b' as i8) as i32))
    );
    assert!(
        (((elem!((s2.as_pointer() as Ptr<i8>), 2_usize).read()) as i32) == (('b' as i8) as i32))
    );
    assert!(
        (((elem!((s1.as_pointer() as Ptr<i8>), 0_usize).read()) as i32) == (('a' as i8) as i32))
    );
    assert!(
        (((elem!((s1.as_pointer() as Ptr<i8>), 1_usize).read()) as i32) == (('a' as i8) as i32))
    );
    assert!(
        (((elem!((s1.as_pointer() as Ptr<i8>), 2_usize).read()) as i32) == (('a' as i8) as i32))
    );
    let b1: Value<Bar> = Rc::new(RefCell::new(Bar { w: 1 }));
    let b2: Value<Bar> = Rc::new(RefCell::new(Bar { w: 2 }));
    (*b2.borrow_mut()) = (*b1.borrow()).clone();
    (*b2.borrow_mut()).w.postfix_inc();
    assert!(({ (*b1.borrow()).w } == 1));
    assert!(({ (*b2.borrow()).w } == 2));
    let v4: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    (v4.as_pointer() as Ptr<Vec<i32>>).write((*v2.borrow()).clone());
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        assert!(((elem!((v4.as_pointer() as Ptr<i32>), (i as usize)).read()) == (i + 1)));
        elem!((v4.as_pointer() as Ptr<i32>), (i as usize)).with_mut(|__v| __v.prefix_inc());
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        assert!(((elem!((v4.as_pointer() as Ptr<i32>), (i as usize)).read()) == (i + 2)));
        assert!(((elem!((v2.as_pointer() as Ptr<i32>), (i as usize)).read()) == (i + 1)));
        i.prefix_inc();
    }
    let ra: Value<i32> = Rc::new(RefCell::new(1));
    let rb: Value<i32> = Rc::new(RefCell::new(2));
    let r1: Value<Refs> = Rc::new(RefCell::new(Refs {
        a: ra.as_pointer(),
        b: rb.as_pointer(),
    }));
    let r2: Value<Refs> = Rc::new(RefCell::new((*r1.borrow()).clone()));
    { (*r2.borrow()).a.clone() }.write(10);
    { (*r2.borrow()).b.clone() }.with_mut(|__v| __v.prefix_inc());
    assert!(((*ra.borrow()) == 10));
    assert!(((*rb.borrow()) == 3));
    assert!((({ (*r1.borrow()).a.clone() }.read()) == 10));
    assert!((({ (*r1.borrow()).b.clone() }.read()) == 3));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
