extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct F {
    #[offset(0)]
    pub n: i32,
    #[offset(4)]
    #[byte_size(0)]
    pub tail: Box<[i8]>,
}
impl Default for F {
    fn default() -> Self {
        {
            F {
                n: 0_i32,
                tail: Box::<[i8]>::default(),
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((((4usize == ::std::mem::size_of::<i32>()) as i32) != 0));
    let mut f: Ptr<F> =
        libcc2rs::malloc_refcount((4usize as usize).wrapping_add(4_usize)).reinterpret_cast::<F>();
    assert!((((!((f).is_null())) as i32) != 0));
    field!(f, n).write(4);
    match ({ ((array_field_ptr!(f, tail) as Ptr<i8>) as Ptr<i8>).to_any() },) {
        (mut __tmp_0,) => {
            __tmp_0.memcpy(
                &Ptr::<i8>::from_string_literal(b"xyz").to_any(),
                4_usize as usize,
            );
            (__tmp_0).clone()
        }
    };
    assert!((((f.with(|__s| __s.n) == 4) as i32) != 0));
    assert!(
        ((({
            let mut __it1 = (array_field_ptr!(f, tail) as Ptr<i8>).to_c_string_iterator();
            let mut __it2 = Ptr::<i8>::from_string_literal(b"xyz").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    assert!((((((array_elem!(f, tail, 1).read()) as i32) == ('y' as i32)) as i32) != 0));
    libcc2rs::free_refcount((f).to_any());
    let mut g: Ptr<F> =
        libcc2rs::malloc_refcount((4usize as usize).wrapping_add(8_usize)).reinterpret_cast::<F>();
    assert!((((!((g).is_null())) as i32) != 0));
    field!(g, n).write(8);
    match ({ ((array_field_ptr!(g, tail) as Ptr<i8>) as Ptr<i8>).to_any() },) {
        (mut __tmp_1,) => {
            __tmp_1.memcpy(
                &Ptr::<i8>::from_string_literal(b"abcdefg").to_any(),
                8_usize as usize,
            );
            (__tmp_1).clone()
        }
    };
    assert!((((((array_elem!(g, tail, 6).read()) as i32) == ('g' as i32)) as i32) != 0));
    assert!((((((array_elem!(g, tail, 7).read()) as i32) == ('\0' as i32)) as i32) != 0));
    libcc2rs::free_refcount((g).to_any());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
