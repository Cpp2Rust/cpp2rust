extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn int_cmp_0(v1: AnyPtr, v2: AnyPtr) -> i32 {
    let v1: Value<AnyPtr> = Rc::new(RefCell::new(v1));
    let v2: Value<AnyPtr> = Rc::new(RefCell::new(v2));
    return {
        let _lhs = ((*v1.borrow()).reinterpret_cast::<i32>().read());
        _lhs - ((*v2.borrow()).reinterpret_cast::<i32>().read())
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
    let vptr1: Value<AnyPtr> = Rc::new(RefCell::new({
        let __base = ((a1.as_pointer() as Ptr<i32>) as Ptr<i32>)
            .to_any()
            .reinterpret_cast::<u8>();
        let mut __lo: isize = 0;
        let mut __hi: isize = 1_usize as isize - 1;
        let mut __found = AnyPtr::default();
        while __lo <= __hi && __found.is_null() {
            let __mid = __lo + (__hi - __lo) / 2;
            let __elem = __base.offset(__mid as usize * ::std::mem::size_of::<i32>());
            let __r = int_cmp_0(
                (((a1.as_pointer() as Ptr<i32>).offset(0)) as Ptr<i32>)
                    .to_any()
                    .clone(),
                __elem.to_any(),
            );
            if __r == 0 {
                __found = __elem.to_any();
            } else if __r < 0 {
                __hi = __mid - 1;
            } else {
                __lo = __mid + 1;
            }
        }
        __found
    }));
    assert!({
        let _lhs = (*vptr1.borrow()).clone();
        _lhs == (((a1.as_pointer() as Ptr<i32>).offset(0)) as Ptr<i32>).to_any()
    });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
