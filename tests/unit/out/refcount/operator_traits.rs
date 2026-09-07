extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Lt {
    pub v: Value<i32>,
}
impl std::cmp::Ord for Lt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if LtImpl::operator_lt(
                &Rc::new(RefCell::new(self.clone())).as_pointer(),
                Rc::new(RefCell::new(other.clone())).as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if LtImpl::operator_lt(
                &Rc::new(RefCell::new(other.clone())).as_pointer(),
                Rc::new(RefCell::new(self.clone())).as_pointer(),
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
                &Rc::new(RefCell::new(self.clone())).as_pointer(),
                Rc::new(RefCell::new(other.clone())).as_pointer(),
            )) && !(LtImpl::operator_lt(
                &Rc::new(RefCell::new(other.clone())).as_pointer(),
                Rc::new(RefCell::new(self.clone())).as_pointer(),
            ))
        }
    }
}
impl std::cmp::Eq for Lt {}
impl Clone for Lt {
    fn clone(&self) -> Self {
        let __this: Value<Lt> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        }));
        let this: Ptr<Lt> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Lt {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct Eq {
    pub v: Value<i32>,
}
impl std::cmp::PartialEq for Eq {
    fn eq(&self, other: &Self) -> bool {
        {
            EqImpl::operator_eq(
                &Rc::new(RefCell::new(self.clone())).as_pointer(),
                Rc::new(RefCell::new(other.clone())).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Eq {}
impl Clone for Eq {
    fn clone(&self) -> Self {
        let __this: Value<Eq> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        }));
        let this: Ptr<Eq> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Eq {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct Cmp {
    pub v: Value<i32>,
}
impl std::cmp::Ord for Cmp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            CmpImpl::operator_cmp(
                &Rc::new(RefCell::new(self.clone())).as_pointer(),
                Rc::new(RefCell::new(other.clone())).as_pointer(),
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
                &Rc::new(RefCell::new(self.clone())).as_pointer(),
                Rc::new(RefCell::new(other.clone())).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Cmp {}
impl Clone for Cmp {
    fn clone(&self) -> Self {
        let __this: Value<Cmp> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        }));
        let this: Ptr<Cmp> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Cmp {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct Free {
    pub v: Value<i32>,
}
impl std::cmp::Ord for Free {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_0(
                Rc::new(RefCell::new(self.clone())).as_pointer(),
                Rc::new(RefCell::new(other.clone())).as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_0(
                Rc::new(RefCell::new(other.clone())).as_pointer(),
                Rc::new(RefCell::new(self.clone())).as_pointer(),
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
                Rc::new(RefCell::new(self.clone())).as_pointer(),
                Rc::new(RefCell::new(other.clone())).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Free {}
impl Clone for Free {
    fn clone(&self) -> Self {
        let __this: Value<Free> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        }));
        let this: Ptr<Free> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Free {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn operator_lt_0(a: Ptr<Free>, b: Ptr<Free>) -> bool {
    return {
        let _lhs = (*(*a.upgrade().deref()).v.borrow());
        _lhs < (*(*b.upgrade().deref()).v.borrow())
    };
}
pub fn operator_eq_1(a: Ptr<Free>, b: Ptr<Free>) -> bool {
    return {
        let _lhs = (*(*a.upgrade().deref()).v.borrow());
        _lhs == (*(*b.upgrade().deref()).v.borrow())
    };
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let lts: Value<Vec<Lt>> = Rc::new(RefCell::new(vec![
        Lt {
            v: Rc::new(RefCell::new(3)),
        },
        Lt {
            v: Rc::new(RefCell::new(1)),
        },
        Lt {
            v: Rc::new(RefCell::new(2)),
        },
    ]));
    (lts.as_pointer() as Ptr<Lt>).sort((lts.as_pointer() as Ptr<Lt>).to_end().get_offset());
    assert!(
        (((*(*(lts.as_pointer() as Ptr<Lt>)
            .offset(0_usize)
            .upgrade()
            .deref())
        .v
        .borrow())
            == 1)
            && ((*(*(lts.as_pointer() as Ptr<Lt>)
                .offset(1_usize)
                .upgrade()
                .deref())
            .v
            .borrow())
                == 2))
            && ((*(*(lts.as_pointer() as Ptr<Lt>)
                .offset(2_usize)
                .upgrade()
                .deref())
            .v
            .borrow())
                == 3)
    );
    let eqs: Value<Vec<Eq>> = Rc::new(RefCell::new(vec![
        Eq {
            v: Rc::new(RefCell::new(1)),
        },
        Eq {
            v: Rc::new(RefCell::new(2)),
        },
        Eq {
            v: Rc::new(RefCell::new(3)),
        },
    ]));
    let two: Value<Eq> = Rc::new(RefCell::new(Eq {
        v: Rc::new(RefCell::new(2)),
    }));
    let nine: Value<Eq> = Rc::new(RefCell::new(Eq {
        v: Rc::new(RefCell::new(9)),
    }));
    assert!(
        ((((eqs.as_pointer() as Ptr<Eq>)
            .offset(
                (eqs.as_pointer() as Ptr<Eq>)
                    .clone()
                    .into_iter()
                    .enumerate()
                    .position(|(index_0, value_0)| {
                        index_0 < (eqs.as_pointer() as Ptr<Eq>).to_end().get_offset() as usize
                            && value_0.read() == (*two.borrow())
                    })
                    .unwrap_or((eqs.as_pointer() as Ptr<Eq>).to_end().get_offset() as usize)
                    as isize,
            )
            .get_offset() as isize)
            - ((eqs.as_pointer() as Ptr<Eq>).get_offset() as isize))
            == 1_i64)
    );
    assert!(
        (eqs.as_pointer() as Ptr<Eq>).offset(
            (eqs.as_pointer() as Ptr<Eq>)
                .clone()
                .into_iter()
                .enumerate()
                .position(|(index_0, value_0)| {
                    index_0 < (eqs.as_pointer() as Ptr<Eq>).to_end().get_offset() as usize
                        && value_0.read() == (*nine.borrow())
                })
                .unwrap_or((eqs.as_pointer() as Ptr<Eq>).to_end().get_offset() as usize)
                as isize,
        ) == (eqs.as_pointer() as Ptr<Eq>).to_end()
    );
    let cmps: Value<Vec<Cmp>> = Rc::new(RefCell::new(vec![
        Cmp {
            v: Rc::new(RefCell::new(3)),
        },
        Cmp {
            v: Rc::new(RefCell::new(1)),
        },
        Cmp {
            v: Rc::new(RefCell::new(2)),
        },
    ]));
    (cmps.as_pointer() as Ptr<Cmp>).sort((cmps.as_pointer() as Ptr<Cmp>).to_end().get_offset());
    assert!(
        ((*(*(cmps.as_pointer() as Ptr<Cmp>)
            .offset(0_usize)
            .upgrade()
            .deref())
        .v
        .borrow())
            == 1)
            && ((*(*(cmps.as_pointer() as Ptr<Cmp>)
                .offset(2_usize)
                .upgrade()
                .deref())
            .v
            .borrow())
                == 3)
    );
    let three: Value<Cmp> = Rc::new(RefCell::new(Cmp {
        v: Rc::new(RefCell::new(3)),
    }));
    assert!(
        ((((cmps.as_pointer() as Ptr<Cmp>)
            .offset(
                (cmps.as_pointer() as Ptr<Cmp>)
                    .clone()
                    .into_iter()
                    .enumerate()
                    .position(|(index_0, value_0)| {
                        index_0 < (cmps.as_pointer() as Ptr<Cmp>).to_end().get_offset() as usize
                            && value_0.read() == (*three.borrow())
                    })
                    .unwrap_or((cmps.as_pointer() as Ptr<Cmp>).to_end().get_offset() as usize)
                    as isize,
            )
            .get_offset() as isize)
            - ((cmps.as_pointer() as Ptr<Cmp>).get_offset() as isize))
            == 2_i64)
    );
    let frees: Value<Vec<Free>> = Rc::new(RefCell::new(vec![
        Free {
            v: Rc::new(RefCell::new(2)),
        },
        Free {
            v: Rc::new(RefCell::new(1)),
        },
    ]));
    (frees.as_pointer() as Ptr<Free>).sort((frees.as_pointer() as Ptr<Free>).to_end().get_offset());
    assert!(
        ((*(*(frees.as_pointer() as Ptr<Free>)
            .offset(0_usize)
            .upgrade()
            .deref())
        .v
        .borrow())
            == 1)
    );
    let ftwo: Value<Free> = Rc::new(RefCell::new(Free {
        v: Rc::new(RefCell::new(2)),
    }));
    assert!(
        ((((frees.as_pointer() as Ptr<Free>)
            .offset(
                (frees.as_pointer() as Ptr<Free>)
                    .clone()
                    .into_iter()
                    .enumerate()
                    .position(|(index_0, value_0)| {
                        index_0 < (frees.as_pointer() as Ptr<Free>).to_end().get_offset() as usize
                            && value_0.read() == (*ftwo.borrow())
                    })
                    .unwrap_or((frees.as_pointer() as Ptr<Free>).to_end().get_offset() as usize)
                    as isize,
            )
            .get_offset() as isize)
            - ((frees.as_pointer() as Ptr<Free>).get_offset() as isize))
            == 1_i64)
    );
    let m: Value<BTreeMap<Lt, Value<i32>>> = Rc::new(RefCell::new(BTreeMap::new()));
    (m.as_pointer() as Ptr<BTreeMap<Lt, Value<i32>>>)
        .with_mut(|__v: &mut BTreeMap<Lt, Value<i32>>| {
            __v.entry(
                Lt {
                    v: Rc::new(RefCell::new(2)),
                }
                .clone(),
            )
            .or_insert_with(|| Rc::new(RefCell::new(<i32>::default())))
            .as_pointer()
        })
        .write(20);
    (m.as_pointer() as Ptr<BTreeMap<Lt, Value<i32>>>)
        .with_mut(|__v: &mut BTreeMap<Lt, Value<i32>>| {
            __v.entry(
                Lt {
                    v: Rc::new(RefCell::new(1)),
                }
                .clone(),
            )
            .or_insert_with(|| Rc::new(RefCell::new(<i32>::default())))
            .as_pointer()
        })
        .write(10);
    assert!(
        ((*RefcountMapIter::begin((m.as_pointer() as Ptr<BTreeMap<Lt, Value<i32>>>))
            .second()
            .borrow())
            == 10)
    );
    assert!(
        (((m.as_pointer() as Ptr<BTreeMap<Lt, Value<i32>>>)
            .with_mut(|__v: &mut BTreeMap<Lt, Value<i32>>| {
                __v.entry(
                    Lt {
                        v: Rc::new(RefCell::new(2)),
                    }
                    .clone(),
                )
                .or_insert_with(|| Rc::new(RefCell::new(<i32>::default())))
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
        return (*(*(*self).upgrade().deref()).v.borrow())
            .cmp(&(*(*o.upgrade().deref()).v.borrow()));
    }
    fn operator_eq(&self, o: Ptr<Cmp>) -> bool {
        return {
            let _lhs = (*(*(*self).upgrade().deref()).v.borrow());
            _lhs == (*(*o.upgrade().deref()).v.borrow())
        };
    }
}
pub trait EqImpl {
    fn operator_eq(&self, o: Ptr<Eq>) -> bool;
}
impl EqImpl for Ptr<Eq> {
    fn operator_eq(&self, o: Ptr<Eq>) -> bool {
        return {
            let _lhs = (*(*(*self).upgrade().deref()).v.borrow());
            _lhs == (*(*o.upgrade().deref()).v.borrow())
        };
    }
}
pub trait LtImpl {
    fn operator_lt(&self, o: Ptr<Lt>) -> bool;
}
impl LtImpl for Ptr<Lt> {
    fn operator_lt(&self, o: Ptr<Lt>) -> bool {
        return {
            let _lhs = (*(*(*self).upgrade().deref()).v.borrow());
            _lhs < (*(*o.upgrade().deref()).v.borrow())
        };
    }
}
