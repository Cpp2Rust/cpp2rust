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
pub struct node {
    pub data: i32,
    pub next: *mut node,
}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum opt {
    #[default]
    OPT_STRING_OUT = 0,
    OPT_FILE = 1,
    OPT_NODE = 2,
    OPT_NODE_OUT = 3,
}
impl From<i32> for opt {
    fn from(n: i32) -> opt {
        match n {
            0 => opt::OPT_STRING_OUT,
            1 => opt::OPT_FILE,
            2 => opt::OPT_NODE,
            3 => opt::OPT_NODE_OUT,
            _ => panic!("invalid opt value: {}", n),
        }
    }
}
pub unsafe fn dispatch_0(mut option: i32, __args: &[VaArg]) -> i32 {
    let mut ap: VaList = VaList::default();
    ap = VaList::new(__args);
    let mut result: i32 = 0;
    'switch: {
        let __match_cond = option;
        match __match_cond {
            v if v == (opt::OPT_STRING_OUT as i32) => {
                let mut out: *mut *const u8 = ap.arg::<*mut *const u8>();
                (*out) = b"hello\0".as_ptr().cast_mut().cast_const();
                result = 1;
                break 'switch;
            }
            v if v == (opt::OPT_FILE as i32) => {
                let mut f: *mut ::std::fs::File = ap.arg::<*mut ::std::fs::File>();
                result = ((!((f).is_null())) as i32).clone();
                break 'switch;
            }
            v if v == (opt::OPT_NODE as i32) => {
                let mut n: *mut node = ap.arg::<*mut node>();
                result = (*n).data;
                break 'switch;
            }
            v if v == (opt::OPT_NODE_OUT as i32) => {
                let mut out: *mut *mut node = ap.arg::<*mut *mut node>();
                (*out) = std::ptr::null_mut();
                result = 2;
                break 'switch;
            }
            _ => {}
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
    let mut s: *const u8 = std::ptr::null();
    assert!(
        ((((unsafe {
            let _option: i32 = (opt::OPT_STRING_OUT as i32);
            dispatch_0(_option, &[(&mut s as *mut *const u8).into()])
        }) == (1)) as i32)
            != 0)
    );
    assert!((((!((s).is_null())) as i32) != 0));
    assert!(
        ((((unsafe {
            let _option: i32 = (opt::OPT_FILE as i32);
            dispatch_0(_option, &[libcc2rs::cout_unsafe().into()])
        }) == (1)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            let _option: i32 = (opt::OPT_FILE as i32);
            dispatch_0(
                _option,
                &[((0 as *mut ::libc::c_void) as *mut ::std::fs::File).into()],
            )
        }) == (0)) as i32)
            != 0)
    );
    let mut head: node = node {
        data: 42,
        next: std::ptr::null_mut(),
    };
    assert!(
        ((((unsafe {
            let _option: i32 = (opt::OPT_NODE as i32);
            dispatch_0(_option, &[(&mut head as *mut node).into()])
        }) == (42)) as i32)
            != 0)
    );
    let mut outp: *mut node = (&mut head as *mut node);
    assert!(
        ((((unsafe {
            let _option: i32 = (opt::OPT_NODE_OUT as i32);
            dispatch_0(_option, &[(&mut outp as *mut *mut node).into()])
        }) == (2)) as i32)
            != 0)
    );
    assert!(((((outp).is_null()) as i32) != 0));
    return 0;
}
