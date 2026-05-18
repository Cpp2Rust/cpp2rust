extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Bar {
    pub w: Value<i32>,
}
impl Clone for Bar {
    fn clone(&self) -> Self {
        let mut this = Self {
            w: Rc::new(RefCell::new((*self.w.borrow()))),
        };
        this
    }
}
impl ByteRepr for Bar {}
#[derive()]
pub struct Foo {
    pub x: Value<i32>,
    pub y: Ptr<i32>,
    pub z: Value<Ptr<i32>>,
    pub a: Value<Box<[i32]>>,
    pub bar: Value<Bar>,
}
impl Clone for Foo {
    fn clone(&self) -> Self {
        let mut this = Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
            y: (self.y).clone(),
            z: Rc::new(RefCell::new((*self.z.borrow()).clone())),
            a: Rc::new(RefCell::new((*self.a.borrow()).clone())),
            bar: Rc::new(RefCell::new((*self.bar.borrow()).clone())),
        };
        this
    }
}
impl Default for Foo {
    fn default() -> Self {
        Foo {
            x: <Value<i32>>::default(),
            y: <Ptr<i32>>::default(),
            z: Rc::new(RefCell::new(Ptr::<i32>::null())),
            a: Rc::new(RefCell::new(
                (0..3).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
            )),
            bar: <Value<Bar>>::default(),
        }
    }
}
impl ByteRepr for Foo {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x1: Value<i32> = Rc::new(RefCell::new(1));
    let x2: Value<i32> = Rc::new(RefCell::new((*x1.borrow())));
    (*x2.borrow_mut()).prefix_inc();
    assert!(((*x1.borrow()) == 1));
    assert!(((*x2.borrow()) == 2));
    let x3: Value<f64> = Rc::new(RefCell::new(3.0E+0));
    let x4: Value<f64> = Rc::new(RefCell::new((*x3.borrow())));
    (*x4.borrow_mut()).prefix_inc();
    assert!(((*x3.borrow()) == 3.0E+0));
    assert!(((*x4.borrow()) == 4.0E+0));
    let reference: Ptr<i32> = x1.as_pointer();
    let x5: Value<i32> = Rc::new(RefCell::new((reference.read())));
    (*x5.borrow_mut()).prefix_inc();
    assert!(((reference.read()) == 1));
    assert!(((*x5.borrow()) == 2));
    let pointer: Value<Ptr<i32>> = Rc::new(RefCell::new((x1.as_pointer())));
    let x6: Value<i32> = Rc::new(RefCell::new(((*pointer.borrow()).read())));
    (*x6.borrow_mut()).prefix_inc();
    assert!((((*pointer.borrow()).read()) == 1));
    assert!(((*x6.borrow()) == 2));
    let other_pointer: Value<Ptr<i32>> = Rc::new(RefCell::new((*pointer.borrow()).clone()));
    assert!({
        let _lhs = (*other_pointer.borrow()).clone();
        _lhs == (*pointer.borrow()).clone()
    });
    (*other_pointer.borrow()).with_mut(|__v| __v.prefix_inc());
    assert!({
        let _lhs = ((*other_pointer.borrow()).read());
        _lhs == ((*pointer.borrow()).read())
    });
    let f1: Value<Foo> = Rc::new(RefCell::new(Foo {
        x: Rc::new(RefCell::new(1)),
        y: x1.as_pointer(),
        z: Rc::new(RefCell::new((x1.as_pointer()))),
        a: Rc::new(RefCell::new(Box::new([0, 1, 2]))),
        bar: Rc::new(RefCell::new(Bar {
            w: Rc::new(RefCell::new(10)),
        })),
    }));
    assert!(((*(*f1.borrow()).x.borrow()) == 1));
    assert!((((*f1.borrow()).y.read()) == 2));
    assert!({
        let _lhs = (*(*f1.borrow()).z.borrow()).clone();
        _lhs == (x1.as_pointer())
    });
    assert!((((*(*f1.borrow()).z.borrow()).read()) == 2));
    let f2: Value<Foo> = Rc::new(RefCell::new((*f1.borrow()).clone()));
    (*(*f2.borrow()).x.borrow_mut()).prefix_inc();
    (*f2.borrow()).y.with_mut(|__v| __v.prefix_inc());
    assert!(((*(*f2.borrow()).x.borrow()) == 2));
    assert!((((*f2.borrow()).y.read()) == 3));
    assert!(((*(*f1.borrow()).x.borrow()) == 1));
    assert!((((*f1.borrow()).y.read()) == 3));
    (*(*f2.borrow()).z.borrow()).with_mut(|__v| __v.prefix_inc());
    assert!((((*f2.borrow()).y.read()) == 4));
    assert!({
        let _lhs = (*(*f2.borrow()).z.borrow()).clone();
        _lhs == (x1.as_pointer())
    });
    assert!((((*(*f2.borrow()).z.borrow()).read()) == 4));
    assert!((((*f1.borrow()).y.read()) == 4));
    assert!({
        let _lhs = (*(*f1.borrow()).z.borrow()).clone();
        _lhs == (x1.as_pointer())
    });
    assert!((((*(*f1.borrow()).z.borrow()).read()) == 4));
    (*(*f2.borrow()).a.borrow_mut())[(0) as usize].prefix_inc();
    (*(*f2.borrow()).a.borrow_mut())[(1) as usize].prefix_inc();
    (*(*f2.borrow()).a.borrow_mut())[(2) as usize].prefix_inc();
    assert!(((*(*f2.borrow()).a.borrow())[(0) as usize] == 1));
    assert!(((*(*f2.borrow()).a.borrow())[(1) as usize] == 2));
    assert!(((*(*f2.borrow()).a.borrow())[(2) as usize] == 3));
    assert!(((*(*f1.borrow()).a.borrow())[(0) as usize] == 0));
    assert!(((*(*f1.borrow()).a.borrow())[(1) as usize] == 1));
    assert!(((*(*f1.borrow()).a.borrow())[(2) as usize] == 2));
    (*(*(*f2.borrow()).bar.borrow()).w.borrow_mut()) = 20;
    assert!(((*(*(*f2.borrow()).bar.borrow()).w.borrow()) == 20));
    assert!(((*(*(*f1.borrow()).bar.borrow()).w.borrow()) == 10));
    let N: Value<i32> = Rc::new(RefCell::new(5));
    let v1: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*N.borrow())) {
        {
            let a0_clone = (*i.borrow()).clone();
            (*v1.borrow_mut()).push(a0_clone)
        };
        (*i.borrow_mut()).prefix_inc();
    }
    let v2: Value<Vec<i32>> = Rc::new(RefCell::new((*v1.borrow()).clone()));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*N.borrow())) {
        assert!(
            (((v2.as_pointer() as Ptr<i32>)
                .offset(((*i.borrow()) as u64) as isize)
                .read())
                == (*i.borrow()))
        );
        (*i.borrow_mut()).prefix_inc();
    }
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*N.borrow())) {
        (v2.as_pointer() as Ptr<i32>)
            .offset(((*i.borrow()) as u64) as isize)
            .with_mut(|__v| __v.prefix_inc());
        (*i.borrow_mut()).prefix_inc();
    }
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*N.borrow())) {
        assert!(
            (((v2.as_pointer() as Ptr<i32>)
                .offset(((*i.borrow()) as u64) as isize)
                .read())
                == ((*i.borrow()) + 1))
        );
        assert!(
            (((v1.as_pointer() as Ptr<i32>)
                .offset(((*i.borrow()) as u64) as isize)
                .read())
                == (*i.borrow()))
        );
        (*i.borrow_mut()).prefix_inc();
    }
    let m1: Value<Vec<Value<Vec<i32>>>> = Rc::new(RefCell::new(Vec::new()));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*N.borrow())) {
        (m1.as_pointer() as Ptr<Vec<Value<Vec<i32>>>>).with_mut(
            |__v: &mut Vec<Value<Vec<i32>>>| {
                __v.push(Rc::new(RefCell::new(
                    (0..(10_u64) as usize)
                        .map(|_| <i32>::default())
                        .collect::<Vec<_>>()
                        .clone(),
                )))
            },
        );
        (*i.borrow_mut()).prefix_inc();
    }
    let m2: Value<Vec<Value<Vec<i32>>>> = Rc::new(RefCell::new(
        (*m1.borrow())
            .iter()
            .map(|inner_vec| Rc::new(RefCell::new(inner_vec.borrow().clone())))
            .collect(),
    ));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*N.borrow())) {
        assert!(
            ((*((m1.as_pointer() as Ptr<Value<Vec<i32>>>)
                .offset(((*i.borrow()) as u64) as isize)
                .upgrade()
                .deref()
                .as_pointer() as Ptr<Vec<i32>>)
                .upgrade()
                .deref())
            .len() as u64
                == 10_u64)
        );
        assert!(
            ((*((m2.as_pointer() as Ptr<Value<Vec<i32>>>)
                .offset(((*i.borrow()) as u64) as isize)
                .upgrade()
                .deref()
                .as_pointer() as Ptr<Vec<i32>>)
                .upgrade()
                .deref())
            .len() as u64
                == 10_u64)
        );
        let j: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*j.borrow()) < 10) {
            assert!(
                ((((m1.as_pointer() as Ptr<Value<Vec<i32>>>)
                    .offset(((*i.borrow()) as u64) as isize)
                    .upgrade()
                    .deref()
                    .as_pointer() as Ptr<i32>)
                    .offset(((*j.borrow()) as u64) as isize)
                    .read())
                    == 0)
            );
            assert!(
                ((((m2.as_pointer() as Ptr<Value<Vec<i32>>>)
                    .offset(((*i.borrow()) as u64) as isize)
                    .upgrade()
                    .deref()
                    .as_pointer() as Ptr<i32>)
                    .offset(((*j.borrow()) as u64) as isize)
                    .read())
                    == 0)
            );
            (*j.borrow_mut()).prefix_inc();
        }
        (*i.borrow_mut()).prefix_inc();
    }
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*N.borrow())) {
        let j: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*j.borrow()) < 10) {
            ((m2.as_pointer() as Ptr<Value<Vec<i32>>>)
                .offset(((*i.borrow()) as u64) as isize)
                .upgrade()
                .deref()
                .as_pointer() as Ptr<i32>)
                .offset(((*j.borrow()) as u64) as isize)
                .with_mut(|__v| __v.postfix_inc());
            (*j.borrow_mut()).prefix_inc();
        }
        (*i.borrow_mut()).prefix_inc();
    }
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*N.borrow())) {
        assert!(
            ((*((m1.as_pointer() as Ptr<Value<Vec<i32>>>)
                .offset(((*i.borrow()) as u64) as isize)
                .upgrade()
                .deref()
                .as_pointer() as Ptr<Vec<i32>>)
                .upgrade()
                .deref())
            .len() as u64
                == 10_u64)
        );
        assert!(
            ((*((m2.as_pointer() as Ptr<Value<Vec<i32>>>)
                .offset(((*i.borrow()) as u64) as isize)
                .upgrade()
                .deref()
                .as_pointer() as Ptr<Vec<i32>>)
                .upgrade()
                .deref())
            .len() as u64
                == 10_u64)
        );
        let j: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*j.borrow()) < 10) {
            assert!(
                ((((m1.as_pointer() as Ptr<Value<Vec<i32>>>)
                    .offset(((*i.borrow()) as u64) as isize)
                    .upgrade()
                    .deref()
                    .as_pointer() as Ptr<i32>)
                    .offset(((*j.borrow()) as u64) as isize)
                    .read())
                    == 0)
            );
            assert!(
                ((((m2.as_pointer() as Ptr<Value<Vec<i32>>>)
                    .offset(((*i.borrow()) as u64) as isize)
                    .upgrade()
                    .deref()
                    .as_pointer() as Ptr<i32>)
                    .offset(((*j.borrow()) as u64) as isize)
                    .read())
                    == 1)
            );
            (*j.borrow_mut()).prefix_inc();
        }
        (*i.borrow_mut()).prefix_inc();
    }
    let map1: Value<BTreeMap<i32, Value<i32>>> = Rc::new(RefCell::new(BTreeMap::new()));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*N.borrow())) {
        let __rhs = (*i.borrow());
        (map1.as_pointer() as Ptr<BTreeMap<i32, Value<i32>>>)
            .with_mut(|__v: &mut BTreeMap<i32, Value<i32>>| {
                __v.entry((*i.borrow()).clone())
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
    'loop_: while ((*i.borrow()) < (*N.borrow())) {
        assert!(
            (((map2.as_pointer() as Ptr<BTreeMap<i32, Value<i32>>>)
                .with_mut(|__v: &mut BTreeMap<i32, Value<i32>>| {
                    __v.entry((*i.borrow()).clone())
                        .or_insert_with(|| Rc::new(RefCell::new(<i32>::default())))
                        .as_pointer()
                })
                .read())
                == (*i.borrow()))
        );
        (map2.as_pointer() as Ptr<BTreeMap<i32, Value<i32>>>)
            .with_mut(|__v: &mut BTreeMap<i32, Value<i32>>| {
                __v.entry((*i.borrow()).clone())
                    .or_insert_with(|| Rc::new(RefCell::new(<i32>::default())))
                    .as_pointer()
            })
            .with_mut(|__v| __v.prefix_inc());
        (*i.borrow_mut()).prefix_inc();
    }
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*N.borrow())) {
        assert!(
            (((map1.as_pointer() as Ptr<BTreeMap<i32, Value<i32>>>)
                .with_mut(|__v: &mut BTreeMap<i32, Value<i32>>| {
                    __v.entry((*i.borrow()).clone())
                        .or_insert_with(|| Rc::new(RefCell::new(<i32>::default())))
                        .as_pointer()
                })
                .read())
                == (*i.borrow()))
        );
        assert!(
            (((map2.as_pointer() as Ptr<BTreeMap<i32, Value<i32>>>)
                .with_mut(|__v: &mut BTreeMap<i32, Value<i32>>| {
                    __v.entry((*i.borrow()).clone())
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
    let __rhs = ((*(*pair2.borrow()).0.borrow()) * 10);
    (*(*pair2.borrow()).0.borrow_mut()) = __rhs;
    let __rhs = ((*(*pair2.borrow()).1.borrow()) * 10);
    (*(*pair2.borrow()).1.borrow_mut()) = __rhs;
    assert!(((*(*pair2.borrow()).0.borrow()) == 10));
    assert!(((*(*pair2.borrow()).1.borrow()) == 20));
    assert!(((*(*pair1.borrow()).0.borrow()) == 1));
    assert!(((*(*pair1.borrow()).1.borrow()) == 2));
    let pair3: Value<(Value<Vec<i32>>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(
            (0..(0_u64) as usize)
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
    (*(*pair4.borrow()).0.borrow_mut()).push(1);
    (*(*pair4.borrow()).1.borrow_mut()) = 1;
    assert!(((*(*pair4.borrow()).0.borrow()).len() as u64 == 1_u64));
    assert!(((*(*pair4.borrow()).1.borrow()) == 1));
    assert!(((*(*pair3.borrow()).0.borrow()).len() as u64 == 0_u64));
    assert!(((*(*pair3.borrow()).1.borrow()) == 0));
    let s1: Value<Vec<u8>> = Rc::new(RefCell::new(
        vec![('a' as u8); (3_u64) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once(0))
            .collect(),
    ));
    let s2: Value<Vec<u8>> = Rc::new(RefCell::new((*s1.borrow()).clone()));
    (s2.as_pointer() as Ptr<u8>)
        .offset(0_u64 as isize)
        .write(('b' as u8));
    (s2.as_pointer() as Ptr<u8>)
        .offset(1_u64 as isize)
        .write(('b' as u8));
    (s2.as_pointer() as Ptr<u8>)
        .offset(2_u64 as isize)
        .write(('b' as u8));
    assert!(
        ((((s2.as_pointer() as Ptr<u8>).offset(0_u64 as isize).read()) as i32)
            == (('b' as u8) as i32))
    );
    assert!(
        ((((s2.as_pointer() as Ptr<u8>).offset(1_u64 as isize).read()) as i32)
            == (('b' as u8) as i32))
    );
    assert!(
        ((((s2.as_pointer() as Ptr<u8>).offset(2_u64 as isize).read()) as i32)
            == (('b' as u8) as i32))
    );
    assert!(
        ((((s1.as_pointer() as Ptr<u8>).offset(0_u64 as isize).read()) as i32)
            == (('a' as u8) as i32))
    );
    assert!(
        ((((s1.as_pointer() as Ptr<u8>).offset(1_u64 as isize).read()) as i32)
            == (('a' as u8) as i32))
    );
    assert!(
        ((((s1.as_pointer() as Ptr<u8>).offset(2_u64 as isize).read()) as i32)
            == (('a' as u8) as i32))
    );
    let b1: Value<Bar> = Rc::new(RefCell::new(Bar {
        w: Rc::new(RefCell::new(1)),
    }));
    let b2: Value<Bar> = Rc::new(RefCell::new(Bar {
        w: Rc::new(RefCell::new(2)),
    }));
    (*b2.borrow_mut()) = (*b1.borrow()).clone();
    (*(*b2.borrow()).w.borrow_mut()).postfix_inc();
    assert!(((*(*b1.borrow()).w.borrow()) == 1));
    assert!(((*(*b2.borrow()).w.borrow()) == 2));
    let v4: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    (v4.as_pointer() as Ptr<Vec<i32>>).write((*v2.borrow()).clone());
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*N.borrow())) {
        assert!(
            (((v4.as_pointer() as Ptr<i32>)
                .offset(((*i.borrow()) as u64) as isize)
                .read())
                == ((*i.borrow()) + 1))
        );
        (v4.as_pointer() as Ptr<i32>)
            .offset(((*i.borrow()) as u64) as isize)
            .with_mut(|__v| __v.prefix_inc());
        (*i.borrow_mut()).prefix_inc();
    }
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*N.borrow())) {
        assert!(
            (((v4.as_pointer() as Ptr<i32>)
                .offset(((*i.borrow()) as u64) as isize)
                .read())
                == ((*i.borrow()) + 2))
        );
        assert!(
            (((v2.as_pointer() as Ptr<i32>)
                .offset(((*i.borrow()) as u64) as isize)
                .read())
                == ((*i.borrow()) + 1))
        );
        (*i.borrow_mut()).prefix_inc();
    }
    return 0;
}
