extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct RecordsConstructorName {
    #[offset(0)]
    #[byte_size(8)]
    pub name: Ptr<i8>,
}
impl RecordsConstructorName {
    pub fn new() -> Self {
        let __this: Value<RecordsConstructorName> = Rc::new(RefCell::new(Self {
            name: Ptr::<i8>::from_string_literal(b"RecordsConstructorName"),
        }));
        let this: Ptr<RecordsConstructorName> = __this.as_pointer();
        assert!(
            (this.with(|__s| __s.name.clone())
                == Ptr::<i8>::from_string_literal(b"RecordsConstructorName"))
        );
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for RecordsConstructorName {
    fn default() -> Self {
        { RecordsConstructorName::new() }
    }
}
pub fn plain_function_0() -> Ptr<i8> {
    return Ptr::<i8>::from_string_literal(b"plain_function");
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        ({
            let mut __it1 = ({ plain_function_0() }).to_c_string_iterator();
            let mut __it2 =
                Ptr::<i8>::from_string_literal(b"plain_function").to_c_string_iterator();
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
        } == 0)
    );
    assert!((({ plain_function_0() }) == ({ plain_function_0() })));
    let mut object: RecordsConstructorName = RecordsConstructorName::new();
    assert!(
        ({
            let mut __it1 = object.name.to_c_string_iterator();
            let mut __it2 =
                Ptr::<i8>::from_string_literal(b"RecordsConstructorName").to_c_string_iterator();
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
        } == 0)
    );
    let mut enclosing: Ptr<i8> = Ptr::<i8>::from_string_literal(b"main");
    let lambda: Value<FnPtr<fn() -> Ptr<i8>>> = Rc::new(RefCell::new(
        FnPtr::<fn() -> Ptr<i8>>::new(|| -> Ptr<i8> {
            {
                return Ptr::<i8>::from_string_literal(b"operator()");
            }
        }),
    ));
    assert!(({ ({ ({ (*lambda.borrow()).call() }).clone() } != { (enclosing).clone() }) }));
    assert!(
        ({
            let mut __it1 = enclosing.to_c_string_iterator();
            let mut __it2 = Ptr::<i8>::from_string_literal(b"main").to_c_string_iterator();
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
        } == 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
