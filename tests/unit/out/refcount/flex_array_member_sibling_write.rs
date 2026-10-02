extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct S {
    #[offset(0)]
    pub n: i32,
    #[offset(4)]
    #[byte_size(1)]
    pub name: Value<Box<[u8]>>,
}
impl Default for S {
    fn default() -> Self {
        S {
            n: 0_i32,
            name: Rc::new(RefCell::new((0..1).map(|_| 0_u8).collect::<Box<[u8]>>())),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct E {
    #[offset(0)]
    pub id: i32,
    #[offset(4)]
    pub w: i32,
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct T {
    #[offset(0)]
    pub n: i32,
    #[offset(4)]
    pub cap: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub a: Value<Box<[E]>>,
}
impl Default for T {
    fn default() -> Self {
        T {
            n: 0_i32,
            cap: 0_i32,
            a: Rc::new(RefCell::new(
                (0..1).map(|_| <E>::default()).collect::<Box<[E]>>(),
            )),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<Ptr<S>> = Rc::new(RefCell::new(
        libcc2rs::calloc_refcount(1_usize, (8usize as usize).wrapping_add(8_usize))
            .reinterpret_cast::<S>(),
    ));
    assert!((((!((*s.borrow()).is_null())) as i32) != 0));
    {
        ((array_field_ptr!((*s.borrow()), name) as Ptr<u8>) as Ptr<u8>)
            .to_any()
            .memcpy(
                &Ptr::<u8>::from_string_literal(b"abcdefg").to_any(),
                8_usize as usize,
            );
        ((array_field_ptr!((*s.borrow()), name) as Ptr<u8>) as Ptr<u8>).to_any()
    };
    field!((*s.borrow()), n).write(5);
    assert!(((((*s.borrow()).with(|__s| __s.n) == 5) as i32) != 0));
    assert!(
        ((({
            let mut __it1 =
                (array_field_ptr!((*s.borrow()), name) as Ptr<u8>).to_c_string_iterator();
            let mut __it2 = Ptr::<u8>::from_string_literal(b"abcdefg").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as i32) - (__c2.unwrap_or(0) as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0) as i32)
            != 0)
    );
    libcc2rs::free_refcount((*s.borrow()).to_any());
    let t: Value<Ptr<T>> = Rc::new(RefCell::new(
        libcc2rs::malloc_refcount((16usize as usize).wrapping_add((8usize as usize)))
            .reinterpret_cast::<T>(),
    ));
    assert!((((!((*t.borrow()).is_null())) as i32) != 0));
    field!((*t.borrow()), n).write(2);
    field!((*t.borrow()), cap).write(2);
    field!(elem!((array_field_ptr!((*t.borrow()), a) as Ptr<E>), 0), id).write(10);
    field!(elem!((array_field_ptr!((*t.borrow()), a) as Ptr<E>), 1), w).write(20);
    field!((*t.borrow()), n).write(3);
    assert!(
        (((((({
            (*elem!((array_field_ptr!((*t.borrow()), a) as Ptr<E>), 0)
                .upgrade()
                .deref())
            .id
        } == 10) as i32)
            != 0)
            && ((({
                (*elem!((array_field_ptr!((*t.borrow()), a) as Ptr<E>), 1)
                    .upgrade()
                    .deref())
                .w
            } == 20) as i32)
                != 0)) as i32)
            != 0)
    );
    let tail: Value<Ptr<E>> = Rc::new(RefCell::new(
        ((*t.borrow()).offset((1) as isize)).reinterpret_cast::<E>(),
    ));
    assert!(
        ((({ (*tail.borrow()).clone() } == {
            ((array_field_ptr!((*t.borrow()), a) as Ptr<E>).offset((1) as isize))
        }) as i32)
            != 0)
    );
    field!(elem!((*tail.borrow()), 0), id).write(30);
    field!((*t.borrow()), cap).write(4);
    assert!(
        (((((({
            (*elem!((array_field_ptr!((*t.borrow()), a) as Ptr<E>), 1)
                .upgrade()
                .deref())
            .id
        } == 30) as i32)
            != 0)
            && ((({
                (*elem!((array_field_ptr!((*t.borrow()), a) as Ptr<E>), 1)
                    .upgrade()
                    .deref())
                .w
            } == 20) as i32)
                != 0)) as i32)
            != 0)
    );
    assert!(
        (((((((*t.borrow()).with(|__s| __s.n) == 3) as i32) != 0)
            && ((((*t.borrow()).with(|__s| __s.cap) == 4) as i32) != 0)) as i32)
            != 0)
    );
    libcc2rs::free_refcount((*t.borrow()).to_any());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
