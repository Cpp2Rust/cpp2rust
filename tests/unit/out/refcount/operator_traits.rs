extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
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
impl ByteRepr for Lt {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.v.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
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
impl ByteRepr for Eq {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.v.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
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
impl ByteRepr for Cmp {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.v.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
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
impl ByteRepr for Free {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.v.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
pub fn operator_lt_0(a: Ptr<Free>, b: Ptr<Free>) -> bool {
    return {
        let _lhs = a.with(|__s: &Free| __s.v);
        _lhs < b.with(|__s: &Free| __s.v)
    };
}
pub fn operator_eq_1(a: Ptr<Free>, b: Ptr<Free>) -> bool {
    return {
        let _lhs = a.with(|__s: &Free| __s.v);
        _lhs == b.with(|__s: &Free| __s.v)
    };
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Wrapped_int_ {
    #[offset(0)]
    pub v: i32,
}
impl ByteRepr for Wrapped_int_ {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.v.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let w: Value<Wrapped_int_> = Rc::new(RefCell::new(Wrapped_int_ { v: 2 }));
    assert!(({ (*w.borrow()).v } == 2));
    let lts: Value<Vec<Lt>> = Rc::new(RefCell::new(vec![Lt { v: 3 }, Lt { v: 1 }, Lt { v: 2 }]));
    (lts.as_pointer() as Ptr<Lt>).sort((lts.as_pointer() as Ptr<Lt>).to_end().get_offset());
    assert!(
        (((lts.as_pointer() as Ptr<Lt>)
            .offset(0_usize)
            .with(|__s: &Lt| __s.v)
            == 1)
            && ((lts.as_pointer() as Ptr<Lt>)
                .offset(1_usize)
                .with(|__s: &Lt| __s.v)
                == 2))
            && ((lts.as_pointer() as Ptr<Lt>)
                .offset(2_usize)
                .with(|__s: &Lt| __s.v)
                == 3)
    );
    let eqs: Value<Vec<Eq>> = Rc::new(RefCell::new(vec![Eq { v: 1 }, Eq { v: 2 }, Eq { v: 3 }]));
    let two: Value<Eq> = Rc::new(RefCell::new(Eq { v: 2 }));
    let nine: Value<Eq> = Rc::new(RefCell::new(Eq { v: 9 }));
    assert!(
        ({
            ((eqs.as_pointer() as Ptr<Eq>)
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
                .read())
            .v
        } == 2)
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
    let cmps: Value<Vec<Cmp>> =
        Rc::new(RefCell::new(vec![Cmp { v: 3 }, Cmp { v: 1 }, Cmp { v: 2 }]));
    (cmps.as_pointer() as Ptr<Cmp>).sort((cmps.as_pointer() as Ptr<Cmp>).to_end().get_offset());
    assert!(
        ((cmps.as_pointer() as Ptr<Cmp>)
            .offset(0_usize)
            .with(|__s: &Cmp| __s.v)
            == 1)
            && ((cmps.as_pointer() as Ptr<Cmp>)
                .offset(2_usize)
                .with(|__s: &Cmp| __s.v)
                == 3)
    );
    let three: Value<Cmp> = Rc::new(RefCell::new(Cmp { v: 3 }));
    assert!(
        ({
            ((cmps.as_pointer() as Ptr<Cmp>)
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
                .read())
            .v
        } == 3)
    );
    let frees: Value<Vec<Free>> = Rc::new(RefCell::new(vec![Free { v: 2 }, Free { v: 1 }]));
    (frees.as_pointer() as Ptr<Free>).sort((frees.as_pointer() as Ptr<Free>).to_end().get_offset());
    assert!(
        ((frees.as_pointer() as Ptr<Free>)
            .offset(0_usize)
            .with(|__s: &Free| __s.v)
            == 1)
    );
    let ftwo: Value<Free> = Rc::new(RefCell::new(Free { v: 2 }));
    assert!(
        ({
            ((frees.as_pointer() as Ptr<Free>)
                .offset(
                    (frees.as_pointer() as Ptr<Free>)
                        .clone()
                        .into_iter()
                        .enumerate()
                        .position(|(index_0, value_0)| {
                            index_0
                                < (frees.as_pointer() as Ptr<Free>).to_end().get_offset() as usize
                                && value_0.read() == (*ftwo.borrow())
                        })
                        .unwrap_or((frees.as_pointer() as Ptr<Free>).to_end().get_offset() as usize)
                        as isize,
                )
                .read())
            .v
        } == 2)
    );
    let m: Value<BTreeMap<Lt, Value<i32>>> = Rc::new(RefCell::new(BTreeMap::new()));
    (m.as_pointer() as Ptr<BTreeMap<Lt, Value<i32>>>)
        .with_mut(|__v: &mut BTreeMap<Lt, Value<i32>>| {
            __v.entry(Lt { v: 2 })
                .or_insert_with(|| Rc::new(RefCell::new(<i32>::default())))
                .as_pointer()
        })
        .write(20);
    (m.as_pointer() as Ptr<BTreeMap<Lt, Value<i32>>>)
        .with_mut(|__v: &mut BTreeMap<Lt, Value<i32>>| {
            __v.entry(Lt { v: 1 })
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
                __v.entry(Lt { v: 2 })
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
        return std::cmp::Ord::cmp(
            &((*self).with(|__s: &Cmp| __s.v)),
            &(o.with(|__s: &Cmp| __s.v)),
        );
    }
    fn operator_eq(&self, o: Ptr<Cmp>) -> bool {
        return {
            let _lhs = (*self).with(|__s: &Cmp| __s.v);
            _lhs == o.with(|__s: &Cmp| __s.v)
        };
    }
}
pub trait EqImpl {
    fn operator_eq(&self, o: Ptr<Eq>) -> bool;
}
impl EqImpl for Ptr<Eq> {
    fn operator_eq(&self, o: Ptr<Eq>) -> bool {
        return {
            let _lhs = (*self).with(|__s: &Eq| __s.v);
            _lhs == o.with(|__s: &Eq| __s.v)
        };
    }
}
pub trait LtImpl {
    fn operator_lt(&self, o: Ptr<Lt>) -> bool;
}
impl LtImpl for Ptr<Lt> {
    fn operator_lt(&self, o: Ptr<Lt>) -> bool {
        return {
            let _lhs = (*self).with(|__s: &Lt| __s.v);
            _lhs < o.with(|__s: &Lt| __s.v)
        };
    }
}
pub fn __cpp2rust_init_globals() {}
