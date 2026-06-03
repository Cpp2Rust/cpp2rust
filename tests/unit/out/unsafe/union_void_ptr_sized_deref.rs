extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum Width_enum {
    #[default]
    W_64 = 0,
    W_32 = 1,
    W_16 = 2,
}
impl From<i32> for Width_enum {
    fn from(n: i32) -> Width_enum {
        match n {
            0 => Width_enum::W_64,
            1 => Width_enum::W_32,
            2 => Width_enum::W_16,
            _ => panic!("invalid Width_enum value: {}", n),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(Width_enum);
#[repr(C)]
#[derive(Copy, Clone)]
pub union anon_0 {
    pub text: *const u8,
    pub handle: *mut ::libc::c_void,
    pub signed_n: i64,
    pub f: f64,
}
impl Default for anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Sink {
    pub width: Width_enum,
    pub out: anon_0,
}
pub unsafe fn write_count_1(mut s: *mut Sink, mut count: i64) {
    'switch: {
        let __match_cond = ((*s).width as u32);
        match __match_cond {
            v if v == ((Width_enum::W_64 as i32) as u32) => {
                (*((*s).out.handle as *mut i64)) = count;
                break 'switch;
            }
            v if v == ((Width_enum::W_32 as i32) as u32) => {
                (*((*s).out.handle as *mut i32)) = (count as i32);
                break 'switch;
            }
            v if v == ((Width_enum::W_16 as i32) as u32) => {
                (*((*s).out.handle as *mut i16)) = (count as i16);
                break 'switch;
            }
            _ => {}
        }
    };
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut buf64: i64 = 0_i64;
    let mut buf32: i32 = 0;
    let mut buf16: i16 = 0_i16;
    let mut s: Sink = <Sink>::default();
    s.width = Width_enum::W_64;
    s.out.handle = ((&mut buf64 as *mut i64) as *mut i64 as *mut ::libc::c_void);
    (unsafe {
        let _s: *mut Sink = (&mut s as *mut Sink);
        let _count: i64 = 1234605616436508552_i64;
        write_count_1(_s, _count)
    });
    assert!(((((buf64) == (1234605616436508552_i64)) as i32) != 0));
    s.width = Width_enum::W_32;
    s.out.handle = ((&mut buf32 as *mut i32) as *mut i32 as *mut ::libc::c_void);
    (unsafe {
        let _s: *mut Sink = (&mut s as *mut Sink);
        let _count: i64 = 305419896_i64;
        write_count_1(_s, _count)
    });
    assert!(((((buf32) == (305419896)) as i32) != 0));
    s.width = Width_enum::W_16;
    s.out.handle = ((&mut buf16 as *mut i16) as *mut i16 as *mut ::libc::c_void);
    (unsafe {
        let _s: *mut Sink = (&mut s as *mut Sink);
        let _count: i64 = 4660_i64;
        write_count_1(_s, _count)
    });
    assert!(((((buf16 as i32) == (4660)) as i32) != 0));
    return 0;
}
