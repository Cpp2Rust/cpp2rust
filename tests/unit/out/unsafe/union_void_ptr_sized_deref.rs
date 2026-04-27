extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum Width {
    #[default]
    W_64 = 0,
    W_32 = 1,
    W_16 = 2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Sink_anon_14_3 {
    pub text: *const u8,
    pub handle: *mut ::libc::c_void,
    pub signed_n: i64,
    pub f: f64,
}
impl Default for Sink_anon_14_3 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Sink {
    pub width: Width,
    pub out: Sink_anon_14_3,
}
pub unsafe fn write_count_0(mut s: *mut Sink, mut count: i64) {
    'switch: {
        let __match_cond = ((*s).width as u32);
        match __match_cond {
            v if v == (Width::W_64 as u32) => {
                (*((*s).out.handle as *mut i64)) = count;
                break 'switch;
            }
            v if v == (Width::W_32 as u32) => {
                (*((*s).out.handle as *mut i32)) = (count as i32);
                break 'switch;
            }
            v if v == (Width::W_16 as u32) => {
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
    s.width = (Width::W_64 as Width);
    s.out.handle = ((&mut buf64 as *mut i64) as *mut i64 as *mut ::libc::c_void);
    (unsafe {
        let _s: *mut Sink = (&mut s as *mut Sink);
        let _count: i64 = 1234605616436508552_i64;
        write_count_0(_s, _count)
    });
    assert!(((buf64) == (1234605616436508552_i64)));
    s.width = (Width::W_32 as Width);
    s.out.handle = ((&mut buf32 as *mut i32) as *mut i32 as *mut ::libc::c_void);
    (unsafe {
        let _s: *mut Sink = (&mut s as *mut Sink);
        let _count: i64 = 305419896_i64;
        write_count_0(_s, _count)
    });
    assert!(((buf32) == (305419896)));
    s.width = (Width::W_16 as Width);
    s.out.handle = ((&mut buf16 as *mut i16) as *mut i16 as *mut ::libc::c_void);
    (unsafe {
        let _s: *mut Sink = (&mut s as *mut Sink);
        let _count: i64 = 4660_i64;
        write_count_0(_s, _count)
    });
    assert!(((buf16 as i32) == (4660)));
    return 0;
}
