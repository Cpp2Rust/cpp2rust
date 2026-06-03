extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
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
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct S {
    pub a: i32,
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
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct WithAnonField {
    pub a: i32,
    pub field: anon_2,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
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
    assert!(((((anon_0::FIRST_A as i32) != (anon_0::FIRST_B as i32)) as i32) != 0));
    assert!(((((anon_1::SECOND_A as i32) != (anon_1::SECOND_B as i32)) as i32) != 0));
    assert!(((((anon_3::THIRD_A as i32) != (anon_3::THIRD_B as i32)) as i32) != 0));
    let mut td: TdEnum_enum = TdEnum_enum::TD_A;
    assert!(((((td as u32) == ((TdEnum_enum::TD_A as i32) as u32)) as i32) != 0));
    td = (TdEnum_enum::TD_B).clone();
    assert!(((((td as u32) == ((TdEnum_enum::TD_B as i32) as u32)) as i32) != 0));
    let mut w: WithAnonField = <WithAnonField>::default();
    w.field = anon_2::FIELD_A;
    assert!(((((w.field as u32) == ((anon_2::FIELD_A as i32) as u32)) as i32) != 0));
    w.field = (anon_2::FIELD_B).clone();
    assert!(((((w.field as u32) == ((anon_2::FIELD_B as i32) as u32)) as i32) != 0));
    return 0;
}
