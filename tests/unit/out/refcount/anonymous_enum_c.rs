extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum anon_enum_3 {
    #[default]
    FIRST_A = 0,
    FIRST_B = 1,
}
impl From<i32> for anon_enum_3 {
    fn from(n: i32) -> anon_enum_3 {
        match n {
            0 => anon_enum_3::FIRST_A,
            1 => anon_enum_3::FIRST_B,
            _ => panic!("invalid anon_enum_3 value: {}", n),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum anon_enum_11 {
    #[default]
    SECOND_A = 0,
    SECOND_B = 1,
}
impl From<i32> for anon_enum_11 {
    fn from(n: i32) -> anon_enum_11 {
        match n {
            0 => anon_enum_11::SECOND_A,
            1 => anon_enum_11::SECOND_B,
            _ => panic!("invalid anon_enum_11 value: {}", n),
        }
    }
}
#[derive(Default)]
pub struct S {
    pub a: Value<i32>,
}
impl ByteRepr for S {}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum TdEnum {
    #[default]
    TD_A = 0,
    TD_B = 1,
}
impl From<i32> for TdEnum {
    fn from(n: i32) -> TdEnum {
        match n {
            0 => TdEnum::TD_A,
            1 => TdEnum::TD_B,
            _ => panic!("invalid TdEnum value: {}", n),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum anon_enum_24 {
    #[default]
    FIELD_A = 0,
    FIELD_B = 1,
}
impl From<i32> for anon_enum_24 {
    fn from(n: i32) -> anon_enum_24 {
        match n {
            0 => anon_enum_24::FIELD_A,
            1 => anon_enum_24::FIELD_B,
            _ => panic!("invalid anon_enum_24 value: {}", n),
        }
    }
}
#[derive(Default)]
pub struct WithAnonField {
    pub a: Value<i32>,
    pub field: Value<anon_enum_24>,
}
impl ByteRepr for WithAnonField {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    #[derive(Clone, Copy, PartialEq, Debug, Default)]
    enum anon_enum_31 {
        #[default]
        THIRD_A = 0,
        THIRD_B = 1,
    }
    impl From<i32> for anon_enum_31 {
        fn from(n: i32) -> anon_enum_31 {
            match n {
                0 => anon_enum_31::THIRD_A,
                1 => anon_enum_31::THIRD_B,
                _ => panic!("invalid anon_enum_31 value: {}", n),
            }
        }
    };
    assert!(((((anon_enum_3::FIRST_A as i32) != (anon_enum_3::FIRST_B as i32)) as i32) != 0));
    assert!(((((anon_enum_11::SECOND_A as i32) != (anon_enum_11::SECOND_B as i32)) as i32) != 0));
    assert!(((((anon_enum_31::THIRD_A as i32) != (anon_enum_31::THIRD_B as i32)) as i32) != 0));
    let td: Value<TdEnum> = Rc::new(RefCell::new(TdEnum::from((TdEnum::TD_A as i32))));
    assert!((((((*td.borrow()) as u32) == ((TdEnum::TD_A as i32) as u32)) as i32) != 0));
    (*td.borrow_mut()) = TdEnum::from((TdEnum::TD_B as i32));
    assert!((((((*td.borrow()) as u32) == ((TdEnum::TD_B as i32) as u32)) as i32) != 0));
    let w: Value<WithAnonField> = <Value<WithAnonField>>::default();
    (*(*w.borrow()).field.borrow_mut()) = anon_enum_24::from((anon_enum_24::FIELD_A as i32));
    assert!(
        (((((*(*w.borrow()).field.borrow()) as u32) == ((anon_enum_24::FIELD_A as i32) as u32))
            as i32)
            != 0)
    );
    (*(*w.borrow()).field.borrow_mut()) = anon_enum_24::from((anon_enum_24::FIELD_B as i32));
    assert!(
        (((((*(*w.borrow()).field.borrow()) as u32) == ((anon_enum_24::FIELD_B as i32) as u32))
            as i32)
            != 0)
    );
    return 0;
}
