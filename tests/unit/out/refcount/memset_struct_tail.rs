extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(32)]
pub struct S {
    #[offset(0)]
    pub keep: i32,
    #[offset(4)]
    pub a: i32,
    #[offset(8)]
    pub b: i64,
    #[offset(16)]
    #[byte_size(5)]
    pub c: Box<[i8]>,
    #[offset(24)]
    pub last: i32,
}
impl Default for S {
    fn default() -> Self {
        {
            S {
                keep: 0_i32,
                a: 0_i32,
                b: 0_i64,
                c: (0..5).map(|_| 0_i8).collect::<Box<[i8]>>(),
                last: 0_i32,
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut p: Ptr<S> = libcc2rs::malloc_refcount(32usize).reinterpret_cast::<S>();
    assert!((((!((p).is_null())) as i32) != 0));
    match ({ (p).to_any() },) {
        (mut __tmp_0,) => {
            __tmp_0.memset((255) as u8, 32usize as usize);
            (__tmp_0).clone()
        }
    };
    field!(p, keep).write(7);
    match ({ ((field_ptr!(p, a)) as Ptr<i32>).to_any() },) {
        (mut __tmp_1,) => {
            __tmp_1.memset(
                (0) as u8,
                (32usize as usize).wrapping_sub((4_usize as usize)) as usize,
            );
            (__tmp_1).clone()
        }
    };
    assert!((((p.with(|__s| __s.keep) == 7) as i32) != 0));
    assert!(
        ((((((((((((p.with(|__s| __s.a) == 0) as i32) != 0)
            && (((p.with(|__s| __s.b) == 0_i64) as i32) != 0)) as i32)
            != 0)
            && (((((array_elem!(p, c, 4).read()) as i32) == 0) as i32) != 0)) as i32)
            != 0)
            && (((p.with(|__s| __s.last) == 0) as i32) != 0)) as i32)
            != 0)
    );
    field!(p, a).write(1);
    field!(p, b).write(2_i64);
    array_elem!(p, c, 0).write((('x' as i32) as i8));
    field!(p, last).write(3);
    match ({ ((field_ptr!(p, b)) as Ptr<i64>).to_any() },) {
        (mut __tmp_2,) => {
            __tmp_2.memset(
                (0) as u8,
                (24_usize as usize).wrapping_sub((8_usize as usize)) as usize,
            );
            (__tmp_2).clone()
        }
    };
    assert!(
        ((((((p.with(|__s| __s.keep) == 7) as i32) != 0)
            && (((p.with(|__s| __s.a) == 1) as i32) != 0)) as i32)
            != 0)
    );
    assert!(
        ((((((p.with(|__s| __s.b) == 0_i64) as i32) != 0)
            && (((((array_elem!(p, c, 0).read()) as i32) == 0) as i32) != 0)) as i32)
            != 0)
    );
    assert!((((p.with(|__s| __s.last) == 3) as i32) != 0));
    libcc2rs::free_refcount((p).to_any());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
