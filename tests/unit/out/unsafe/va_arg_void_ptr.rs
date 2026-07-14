extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct registry {
    pub slot: *mut ::libc::c_void,
    pub level: i64,
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum field {
    #[default]
    FIELD_SLOT = 0,
    FIELD_LEVEL = 1,
}
impl From<i32> for field {
    fn from(n: i32) -> field {
        match n {
            0 => field::FIELD_SLOT,
            1 => field::FIELD_LEVEL,
            _ => panic!("invalid field value: {}", n),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(field);
pub unsafe fn registry_update_0(mut r: *mut registry, mut field: field, __args: &[VaArg]) -> i32 {
    let mut result: i32 = 0;
    let mut ap: VaList = VaList::default();
    ap = VaList::new(__args);
    'switch: {
        let __match_cond = (field as u32);
        match __match_cond {
            __v if __v == ((field::FIELD_SLOT as i32) as u32) => {
                (*r).slot = ap.arg::<*mut ::libc::c_void>();
                break 'switch;
            }
            __v if __v == ((field::FIELD_LEVEL as i32) as u32) => {
                (*r).level = (ap.arg::<i64>()).clone();
                break 'switch;
            }
            _ => {
                result = 1;
                break 'switch;
            }
        }
    };
    return result;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut r: registry = registry {
        slot: std::ptr::null_mut(),
        level: 0_i64,
    };
    let mut payload: i32 = 7;
    assert!(
        ((((unsafe {
            registry_update_0(
                (&mut r as *mut registry),
                field::FIELD_SLOT,
                &[(&mut payload as *mut i32).into()],
            )
        }) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            registry_update_0(
                (&mut r as *mut registry),
                field::FIELD_LEVEL,
                &[(5_i64).into()],
            )
        }) == (0)) as i32)
            != 0)
    );
    assert!(((((r.slot) == ((&mut payload as *mut i32) as *mut ::libc::c_void)) as i32) != 0));
    assert!(((((*(r.slot as *mut i32)) == (7)) as i32) != 0));
    assert!(((((r.level) == (5_i64)) as i32) != 0));
    return 0;
}
