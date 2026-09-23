extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Val {
    pub x: Value<i32>,
}
impl Clone for Val {
    fn clone(&self) -> Self {
        let __this: Value<Val> = Rc::new(RefCell::new(Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
        }));
        let this: Ptr<Val> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Val {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn sum_0(a: Val, b: Val) -> i32 {
    let a: Value<Val> = Rc::new(RefCell::new(a));
    let b: Value<Val> = Rc::new(RefCell::new(b));
    return ((*(*a.borrow()).x.borrow()) + (*(*b.borrow()).x.borrow()));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let total: Value<i32> = Rc::new(RefCell::new(0));
    let tally: Value<lambda_1> = Rc::new(RefCell::new(
        (lambda_1 {
            total: total.as_pointer(),
        }),
    ));
    ({ (*tally.borrow()).operator_call_char_char_const() });
    ({ (*tally.borrow()).operator_call_int_char_const() });
    assert!(((*total.borrow()) == 7));
    let v: Value<Val> = Rc::new(RefCell::new(Val {
        x: Rc::new(RefCell::new(5)),
    }));
    let acc: Value<i32> = Rc::new(RefCell::new(0));
    let pick: Value<lambda_2> = Rc::new(RefCell::new(
        (lambda_2 {
            v: v.as_pointer(),
            acc: acc.as_pointer(),
        }),
    ));
    ({ (*pick.borrow()).operator_call_struct_Val_ref_const() });
    ({ (*pick.borrow()).operator_call_const_struct_Val_ref_const() });
    ({ (*pick.borrow()).operator_call_struct_Val_refref_const() });
    assert!(((*acc.borrow()) == 30));
    return 0;
}
#[derive(Clone, Default)]
pub struct lambda_1 {
    total: Ptr<i32>,
}
impl lambda_1 {
    pub fn operator_call_char_char_const(&self) {
        {
            let rhs_0 = (((self.total.read()) as usize).wrapping_add(
                ((::std::mem::size_of::<u8>() as usize)
                    .wrapping_add((::std::mem::size_of::<u8>() as usize))
                    as usize),
            )) as i32;
            self.total.write(rhs_0)
        };
    }
    pub fn operator_call_int_char_const(&self) {
        {
            let rhs_0 = (((self.total.read()) as usize).wrapping_add(
                ((::std::mem::size_of::<i32>() as usize)
                    .wrapping_add((::std::mem::size_of::<u8>() as usize))
                    as usize),
            )) as i32;
            self.total.write(rhs_0)
        };
    }
}
impl ByteRepr for lambda_1 {}
#[derive(Clone, Default)]
pub struct lambda_2 {
    v: Ptr<Val>,
    acc: Ptr<i32>,
}
impl lambda_2 {
    pub fn operator_call_struct_Val_ref_const(&self) {
        {
            let _ptr = self.acc.clone();
            _ptr.write(
                _ptr.read()
                    + ({
                        let _a: Val = (*self.v.upgrade().deref()).clone();
                        let _b: Val = (*self.v.upgrade().deref()).clone();
                        sum_0(_a, _b)
                    }),
            )
        };
    }
    pub fn operator_call_const_struct_Val_ref_const(&self) {
        {
            let _ptr = self.acc.clone();
            _ptr.write(
                _ptr.read()
                    + ({
                        let _a: Val = (*self.v.upgrade().deref()).clone();
                        let _b: Val = (*self.v.upgrade().deref()).clone();
                        sum_0(_a, _b)
                    }),
            )
        };
    }
    pub fn operator_call_struct_Val_refref_const(&self) {
        {
            let _ptr = self.acc.clone();
            _ptr.write(
                _ptr.read()
                    + ({
                        let _a: Val = (*self.v.upgrade().deref()).clone();
                        let _b: Val = (*self.v.upgrade().deref()).clone();
                        sum_0(_a, _b)
                    }),
            )
        };
    }
}
impl ByteRepr for lambda_2 {}
pub fn __cpp2rust_init_globals() {}
