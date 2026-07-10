extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum anon_0 {
    #[default]
    FIRST_A = 0,
    FIRST_B = 1,
}
impl From<i32> for anon_0 {
    fn from(n: i32) -> anon_0 {
        match n {
            0 => anon_0::FIRST_A,
            1 => anon_0::FIRST_B,
            _ => panic!("invalid anon_0 value: {}", n),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(anon_0);
impl ByteRepr for anon_0 {
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self as i32).to_bytes(buf);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        <anon_0>::from(i32::from_bytes(buf))
    }
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum anon_1 {
    #[default]
    SECOND_A = 0,
    SECOND_B = 1,
}
impl From<i32> for anon_1 {
    fn from(n: i32) -> anon_1 {
        match n {
            0 => anon_1::SECOND_A,
            1 => anon_1::SECOND_B,
            _ => panic!("invalid anon_1 value: {}", n),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(anon_1);
impl ByteRepr for anon_1 {
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self as i32).to_bytes(buf);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        <anon_1>::from(i32::from_bytes(buf))
    }
}
#[derive(Default)]
pub struct S {
    pub a: Value<i32>,
}
impl Clone for S {
    fn clone(&self) -> Self {
        Self {
            a: Rc::new(RefCell::new((*self.a.borrow()).clone())),
        }
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum TdEnum_enum {
    #[default]
    TD_A = 0,
    TD_B = 1,
}
impl From<i32> for TdEnum_enum {
    fn from(n: i32) -> TdEnum_enum {
        match n {
            0 => TdEnum_enum::TD_A,
            1 => TdEnum_enum::TD_B,
            _ => panic!("invalid TdEnum_enum value: {}", n),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(TdEnum_enum);
impl ByteRepr for TdEnum_enum {
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self as i32).to_bytes(buf);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        <TdEnum_enum>::from(i32::from_bytes(buf))
    }
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum anon_2 {
    #[default]
    FIELD_A = 0,
    FIELD_B = 1,
}
impl From<i32> for anon_2 {
    fn from(n: i32) -> anon_2 {
        match n {
            0 => anon_2::FIELD_A,
            1 => anon_2::FIELD_B,
            _ => panic!("invalid anon_2 value: {}", n),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(anon_2);
impl ByteRepr for anon_2 {
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self as i32).to_bytes(buf);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        <anon_2>::from(i32::from_bytes(buf))
    }
}
#[derive(Default)]
pub struct WithAnonField {
    pub a: Value<i32>,
    pub field: Value<anon_2>,
}
impl Clone for WithAnonField {
    fn clone(&self) -> Self {
        Self {
            a: Rc::new(RefCell::new((*self.a.borrow()).clone())),
            field: Rc::new(RefCell::new((*self.field.borrow()).clone())),
        }
    }
}
impl ByteRepr for WithAnonField {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.a.borrow()).to_bytes(&mut buf[0..4]);
        (*self.field.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            field: Rc::new(RefCell::new(<anon_2>::from_bytes(&buf[4..8]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    #[derive(Clone, Copy, PartialEq, Debug, Default)]
    enum anon_3 {
        #[default]
        THIRD_A = 0,
        THIRD_B = 1,
    }
    impl From<i32> for anon_3 {
        fn from(n: i32) -> anon_3 {
            match n {
                0 => anon_3::THIRD_A,
                1 => anon_3::THIRD_B,
                _ => panic!("invalid anon_3 value: {}", n),
            }
        }
    }
    libcc2rs::impl_enum_inc_dec!(anon_3);
    impl ByteRepr for anon_3 {
        fn to_bytes(&self, buf: &mut [u8]) {
            (*self as i32).to_bytes(buf);
        }
        fn from_bytes(buf: &[u8]) -> Self {
            <anon_3>::from(i32::from_bytes(buf))
        }
    };
    assert!(((((anon_0::FIRST_A as i32) != (anon_0::FIRST_B as i32)) as i32) != 0));
    assert!(((((anon_1::SECOND_A as i32) != (anon_1::SECOND_B as i32)) as i32) != 0));
    assert!(((((anon_3::THIRD_A as i32) != (anon_3::THIRD_B as i32)) as i32) != 0));
    let td: Value<TdEnum_enum> = Rc::new(RefCell::new(TdEnum_enum::TD_A));
    assert!((((((*td.borrow()) as u32) == ((TdEnum_enum::TD_A as i32) as u32)) as i32) != 0));
    (*td.borrow_mut()) = TdEnum_enum::TD_B;
    assert!((((((*td.borrow()) as u32) == ((TdEnum_enum::TD_B as i32) as u32)) as i32) != 0));
    let w: Value<WithAnonField> = <Value<WithAnonField>>::default();
    (*(*w.borrow()).field.borrow_mut()) = anon_2::FIELD_A;
    assert!(
        (((((*(*w.borrow()).field.borrow()) as u32) == ((anon_2::FIELD_A as i32) as u32)) as i32)
            != 0)
    );
    (*(*w.borrow()).field.borrow_mut()) = anon_2::FIELD_B;
    assert!(
        (((((*(*w.borrow()).field.borrow()) as u32) == ((anon_2::FIELD_B as i32) as u32)) as i32)
            != 0)
    );
    return 0;
}
