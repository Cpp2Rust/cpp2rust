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
pub struct Lt {
    #[offset(0)]
    pub v: i32,
}
impl std::cmp::Ord for Lt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if LtImpl::operator_lt(
                &Rc::new(RefCell::new(Lt { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Lt { v: other.v.clone() })).as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if LtImpl::operator_lt(
                &Rc::new(RefCell::new(Lt { v: other.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Lt { v: self.v.clone() })).as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Lt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Lt {
    fn eq(&self, other: &Self) -> bool {
        {
            !(LtImpl::operator_lt(
                &Rc::new(RefCell::new(Lt { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Lt { v: other.v.clone() })).as_pointer(),
            )) && !(LtImpl::operator_lt(
                &Rc::new(RefCell::new(Lt { v: other.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Lt { v: self.v.clone() })).as_pointer(),
            ))
        }
    }
}
impl std::cmp::Eq for Lt {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Eq {
    #[offset(0)]
    pub v: i32,
}
impl std::cmp::PartialEq for Eq {
    fn eq(&self, other: &Self) -> bool {
        {
            EqImpl::operator_eq(
                &Rc::new(RefCell::new(Eq { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Eq { v: other.v.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Eq {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Cmp {
    #[offset(0)]
    pub v: i32,
}
impl std::cmp::Ord for Cmp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            CmpImpl::operator_cmp(
                &Rc::new(RefCell::new(Cmp { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Cmp { v: other.v.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for Cmp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Cmp {
    fn eq(&self, other: &Self) -> bool {
        {
            CmpImpl::operator_eq(
                &Rc::new(RefCell::new(Cmp { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Cmp { v: other.v.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Cmp {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Free {
    #[offset(0)]
    pub v: i32,
}
impl std::cmp::Ord for Free {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_0(
                Rc::new(RefCell::new(Free { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Free { v: other.v.clone() })).as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_0(
                Rc::new(RefCell::new(Free { v: other.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Free { v: self.v.clone() })).as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Free {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Free {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_1(
                Rc::new(RefCell::new(Free { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Free { v: other.v.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Free {}
pub fn operator_lt_0(a: Ptr<Free>, b: Ptr<Free>) -> bool {
    return ({ a.with(|__s| __s.v) } < { b.with(|__s| __s.v) });
}
pub fn operator_eq_1(a: Ptr<Free>, b: Ptr<Free>) -> bool {
    return ({ a.with(|__s| __s.v) } == { b.with(|__s| __s.v) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Wrapped_int_ {
    #[offset(0)]
    pub v: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut w: Wrapped_int_ = Wrapped_int_ { v: 2 };
    assert!((w.v == 2));
    let lts: Value<Vec<Lt>> = Rc::new(RefCell::new(Vec::from([
        Lt { v: 3 },
        Lt { v: 1 },
        Lt { v: 2 },
    ])));
    {
        let __n = lts
            .as_pointer()
            .decay()
            .end()
            .iter_distance(&lts.as_pointer().decay().begin());
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| {
            lts.as_pointer().decay().begin().iter_at(__x).with(|__a| {
                lts.as_pointer()
                    .decay()
                    .begin()
                    .iter_at(__y)
                    .with(|__b| __a.cmp(__b))
            })
        });
        reorder(__order, |__i| lts.as_pointer().decay().begin().iter_at(__i));
    };
    assert!(
        (({ (*lts.borrow())[0_usize].v } == 1) && ({ (*lts.borrow())[1_usize].v } == 2))
            && ({ (*lts.borrow())[2_usize].v } == 3)
    );
    let eqs: Value<Vec<Eq>> = Rc::new(RefCell::new(Vec::from([
        Eq { v: 1 },
        Eq { v: 2 },
        Eq { v: 3 },
    ])));
    let mut two: Eq = Eq { v: 2 };
    let mut nine: Eq = Eq { v: 9 };
    assert!(
        ({
            let mut __it = eqs.as_pointer().decay().begin();
            while __it != eqs.as_pointer().decay().end()
                && __it.iter_deref().with(|__v| *__v != two)
            {
                __it.iter_inc();
            }
            __it
        }
        .iter_deref()
        .with(|__s| __s.v)
            == 2)
    );
    assert!(
        {
            let mut __it = eqs.as_pointer().decay().begin();
            while __it != eqs.as_pointer().decay().end()
                && __it.iter_deref().with(|__v| *__v != nine)
            {
                __it.iter_inc();
            }
            __it
        } == eqs.as_pointer().decay().end()
    );
    let cmps: Value<Vec<Cmp>> = Rc::new(RefCell::new(Vec::from([
        Cmp { v: 3 },
        Cmp { v: 1 },
        Cmp { v: 2 },
    ])));
    {
        let __n = cmps
            .as_pointer()
            .decay()
            .end()
            .iter_distance(&cmps.as_pointer().decay().begin());
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| {
            cmps.as_pointer().decay().begin().iter_at(__x).with(|__a| {
                cmps.as_pointer()
                    .decay()
                    .begin()
                    .iter_at(__y)
                    .with(|__b| __a.cmp(__b))
            })
        });
        reorder(__order, |__i| {
            cmps.as_pointer().decay().begin().iter_at(__i)
        });
    };
    assert!(({ (*cmps.borrow())[0_usize].v } == 1) && ({ (*cmps.borrow())[2_usize].v } == 3));
    let mut three: Cmp = Cmp { v: 3 };
    assert!(
        ({
            let mut __it = cmps.as_pointer().decay().begin();
            while __it != cmps.as_pointer().decay().end()
                && __it.iter_deref().with(|__v| *__v != three)
            {
                __it.iter_inc();
            }
            __it
        }
        .iter_deref()
        .with(|__s| __s.v)
            == 3)
    );
    let frees: Value<Vec<Free>> = Rc::new(RefCell::new(Vec::from([Free { v: 2 }, Free { v: 1 }])));
    {
        let __n = frees
            .as_pointer()
            .decay()
            .end()
            .iter_distance(&frees.as_pointer().decay().begin());
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| {
            frees.as_pointer().decay().begin().iter_at(__x).with(|__a| {
                frees
                    .as_pointer()
                    .decay()
                    .begin()
                    .iter_at(__y)
                    .with(|__b| __a.cmp(__b))
            })
        });
        reorder(__order, |__i| {
            frees.as_pointer().decay().begin().iter_at(__i)
        });
    };
    assert!(({ (*frees.borrow())[0_usize].v } == 1));
    let mut ftwo: Free = Free { v: 2 };
    assert!(
        ({
            let mut __it = frees.as_pointer().decay().begin();
            while __it != frees.as_pointer().decay().end()
                && __it.iter_deref().with(|__v| *__v != ftwo)
            {
                __it.iter_inc();
            }
            __it
        }
        .iter_deref()
        .with(|__s| __s.v)
            == 2)
    );
    let m: Value<Map<Lt, (Value<Lt>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    m.as_pointer()
        .with_mut(|__m: &mut Map<Lt, (Value<Lt>, Value<i32>)>| {
            __m.get_or_insert_with(Lt { v: 2 }.clone(), || {
                (
                    Rc::new(RefCell::new(Lt { v: 2 })),
                    Rc::new(RefCell::new(<i32>::default())),
                )
            })
            .borrow()
            .1
            .as_pointer()
        })
        .write(20);
    m.as_pointer()
        .with_mut(|__m: &mut Map<Lt, (Value<Lt>, Value<i32>)>| {
            __m.get_or_insert_with(Lt { v: 1 }.clone(), || {
                (
                    Rc::new(RefCell::new(Lt { v: 1 })),
                    Rc::new(RefCell::new(<i32>::default())),
                )
            })
            .borrow()
            .1
            .as_pointer()
        })
        .write(10);
    assert!(
        ((*(*m
            .as_pointer()
            .with(|__m: &Map<Lt, (Value<Lt>, Value<i32>)>| __m.begin())
            .iter_deref()
            .upgrade()
            .deref())
        .1
        .borrow())
            == 10)
    );
    assert!(
        ((m.as_pointer()
            .with_mut(|__m: &mut Map<Lt, (Value<Lt>, Value<i32>)>| {
                __m.get_or_insert_with(Lt { v: 2 }.clone(), || {
                    (
                        Rc::new(RefCell::new(Lt { v: 2 })),
                        Rc::new(RefCell::new(<i32>::default())),
                    )
                })
                .borrow()
                .1
                .as_pointer()
            })
            .read())
            == 20)
    );
    return 0;
}
pub trait CmpImpl {
    fn operator_cmp(&self, o: Ptr<Cmp>) -> std::cmp::Ordering;
    fn operator_eq(&self, o: Ptr<Cmp>) -> bool;
}
impl CmpImpl for Ptr<Cmp> {
    fn operator_cmp(&self, o: Ptr<Cmp>) -> std::cmp::Ordering {
        return std::cmp::Ord::cmp(&((*self).with(|__s| __s.v)), &(o.with(|__s| __s.v)));
    }
    fn operator_eq(&self, o: Ptr<Cmp>) -> bool {
        return ({ (*self).with(|__s| __s.v) } == { o.with(|__s| __s.v) });
    }
}
pub trait EqImpl {
    fn operator_eq(&self, o: Ptr<Eq>) -> bool;
}
impl EqImpl for Ptr<Eq> {
    fn operator_eq(&self, o: Ptr<Eq>) -> bool {
        return ({ (*self).with(|__s| __s.v) } == { o.with(|__s| __s.v) });
    }
}
pub trait LtImpl {
    fn operator_lt(&self, o: Ptr<Lt>) -> bool;
}
impl LtImpl for Ptr<Lt> {
    fn operator_lt(&self, o: Ptr<Lt>) -> bool {
        return ({ (*self).with(|__s| __s.v) } < { o.with(|__s| __s.v) });
    }
}
pub fn __cpp2rust_init_globals() {}
