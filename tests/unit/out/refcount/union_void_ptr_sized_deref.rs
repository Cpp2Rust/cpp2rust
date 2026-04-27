extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum Width {
    #[default]
    W_64 = 0,
    W_32 = 1,
    W_16 = 2,
}
#[derive(Default)]
pub union Sink_anon_0 {
    pub text: Value<Ptr<u8>>,
    pub handle: Value<AnyPtr>,
    pub signed_n: Value<i64>,
    pub f: Value<f64>,
}
impl Default for Sink_anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl ByteRepr for Sink_anon_0 {}
#[derive(Default)]
pub struct Sink {
    pub width: Value<Width>,
    pub out: Value<Sink_anon_0>,
}
impl ByteRepr for Sink {}
pub fn write_count_0(s: Ptr<Sink>, count: i64) {
    let s: Value<Ptr<Sink>> = Rc::new(RefCell::new(s));
    let count: Value<i64> = Rc::new(RefCell::new(count));
    'switch: {
        let __match_cond = ((*(*(*s.borrow()).upgrade().deref()).width.borrow()) as u32);
        match __match_cond {
            v if v == (Width::W_64 as u32) => {
                (*(*(*(*s.borrow()).upgrade().deref()).out.borrow())
                    .handle
                    .borrow())
                .cast::<i64>()
                .expect("ub:wrong type")
                .write((*count.borrow()));
                break 'switch;
            }
            v if v == (Width::W_32 as u32) => {
                (*(*(*(*s.borrow()).upgrade().deref()).out.borrow())
                    .handle
                    .borrow())
                .cast::<i32>()
                .expect("ub:wrong type")
                .write(((*count.borrow()) as i32));
                break 'switch;
            }
            v if v == (Width::W_16 as u32) => {
                (*(*(*(*s.borrow()).upgrade().deref()).out.borrow())
                    .handle
                    .borrow())
                .cast::<i16>()
                .expect("ub:wrong type")
                .write(((*count.borrow()) as i16));
                break 'switch;
            }
            _ => {}
        }
    };
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let buf64: Value<i64> = Rc::new(RefCell::new(0_i64));
    let buf32: Value<i32> = Rc::new(RefCell::new(0));
    let buf16: Value<i16> = Rc::new(RefCell::new(0_i16));
    let s: Value<Sink> = <Value<Sink>>::default();
    (*(*s.borrow()).width.borrow_mut()) = (Width::W_64 as Width);
    (*(*(*s.borrow()).out.borrow()).handle.borrow_mut()) =
        ((buf64.as_pointer()) as Ptr<i64>).to_any();
    ({
        let _s: Ptr<Sink> = (s.as_pointer());
        let _count: i64 = 1234605616436508552_i64;
        write_count_0(_s, _count)
    });
    assert!(((*buf64.borrow()) == 1234605616436508552_i64));
    (*(*s.borrow()).width.borrow_mut()) = (Width::W_32 as Width);
    (*(*(*s.borrow()).out.borrow()).handle.borrow_mut()) =
        ((buf32.as_pointer()) as Ptr<i32>).to_any();
    ({
        let _s: Ptr<Sink> = (s.as_pointer());
        let _count: i64 = 305419896_i64;
        write_count_0(_s, _count)
    });
    assert!(((*buf32.borrow()) == 305419896));
    (*(*s.borrow()).width.borrow_mut()) = (Width::W_16 as Width);
    (*(*(*s.borrow()).out.borrow()).handle.borrow_mut()) =
        ((buf16.as_pointer()) as Ptr<i16>).to_any();
    ({
        let _s: Ptr<Sink> = (s.as_pointer());
        let _count: i64 = 4660_i64;
        write_count_0(_s, _count)
    });
    assert!((((*buf16.borrow()) as i32) == 4660));
    return 0;
}
