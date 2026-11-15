extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct S {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(28)]
pub struct W {
    #[offset(0)]
    pub h: i32,
    #[offset(4)]
    #[byte_size(24)]
    pub arr: Box<[S]>,
}
impl Default for W {
    fn default() -> Self {
        {
            W {
                h: 0_i32,
                arr: (0..3).map(|_| <S>::default()).collect::<Box<[S]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(32)]
pub struct P {
    #[offset(0)]
    #[byte_size(8)]
    pub a: Ptr<i32>,
    #[offset(8)]
    #[byte_size(24)]
    pub arr: Box<[Ptr<i32>]>,
}
impl Default for P {
    fn default() -> Self {
        {
            P {
                a: Ptr::<i32>::null(),
                arr: (0..3)
                    .map(|_| Ptr::<i32>::null())
                    .collect::<Box<[Ptr<i32>]>>(),
            }
        }
    }
}
pub fn typed_0() {
    let w: Value<W> = Rc::new(RefCell::new(<W>::default()));
    let mut p: Ptr<S> = (array_field_ptr!(w.as_pointer(), arr).offset((2) as isize));
    let mut q: Ptr<S> = ((w.as_pointer())
        .reinterpret_cast::<i8>()
        .offset((::std::mem::size_of::<i32>()) as isize)
        .offset(((2_usize).wrapping_mul((8usize as usize))) as isize))
    .reinterpret_cast::<S>();
    assert!(({ (p).clone() } == { (q).clone() }));
    assert!(({ (array_field_ptr!(w.as_pointer(), arr).offset((1) as isize)) } < { (q).clone() }));
    assert!(
        (((q).clone() - (array_field_ptr!(w.as_pointer(), arr).offset((0) as isize))) as i64
            == 2_i64)
    );
    let v: Value<i32> = Rc::new(RefCell::new(0));
    let ptrs: Value<Box<[Ptr<i32>]>> = Rc::new(RefCell::new(Box::new([
        (v.as_pointer()),
        (v.as_pointer()),
        (v.as_pointer()),
        (v.as_pointer()),
    ])));
    let mut e: Ptr<Ptr<i32>> = (ptrs.decay().offset(2));
    let mut f: Ptr<Ptr<i32>> = (ptrs
        .decay()
        .reinterpret_cast::<i8>()
        .offset(((2_usize).wrapping_mul((8usize as usize))) as isize))
    .reinterpret_cast::<Ptr<i32>>();
    assert!(({ (e).clone() } == { (f).clone() }));
    assert!(({ (ptrs.decay().offset(1)) } < { (f).clone() }));
    assert!(({ (f).clone() } < { (ptrs.decay().offset(3)) }));
    assert!((((f).clone() - ptrs.decay()) as i64 == 2_i64));
    let s: Value<P> = Rc::new(RefCell::new(<P>::default()));
    let mut a: Ptr<Ptr<i32>> = (field_ptr!(s.as_pointer(), a));
    let mut b: Ptr<Ptr<i32>> = (array_field_ptr!(s.as_pointer(), arr).offset((1) as isize));
    assert!(({ (a).clone() } < { (b).clone() }));
    assert!(
        ({
            ((s.as_pointer())
                .reinterpret_cast::<i8>()
                .offset(((2_usize).wrapping_mul((8usize as usize))) as isize))
            .reinterpret_cast::<Ptr<i32>>()
        } == { (b).clone() })
    );
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let i1: Value<i32> = Rc::new(RefCell::new(42));
    let mut ptr1: Ptr<i32> = (i1.as_pointer());
    let mut ptr2: Ptr<i8> = (i1.as_pointer()).reinterpret_cast::<i8>();
    let mut vptr1: AnyPtr = (ptr1).to_any();
    let mut vptr2: AnyPtr = (ptr2).to_any();
    assert!(({ (vptr1).clone() } == { (vptr2).clone() }));
    ({ typed_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
