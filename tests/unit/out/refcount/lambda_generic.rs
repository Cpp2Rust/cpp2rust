extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let twice: Value<lambda_0> = Rc::new(RefCell::new((<lambda_0>::default())));
    assert!((({ lambda_0::operator_call_i32__int_const(4,) }) == 8));
    assert!((({ lambda_0::operator_call_f64__double_const(1.5E+0,) }) == 3.0E+0));
    let base: Value<i32> = Rc::new(RefCell::new(10));
    let add_base: Value<lambda_1> = Rc::new(RefCell::new(
        (lambda_1 {
            base: Rc::new(RefCell::new((*base.borrow()))),
        }),
    ));
    assert!((({ lambda_1::operator_call_i32__int_const(&(*add_base.borrow_mut()), 5,) }) == 15));
    assert!(
        (({ lambda_1::operator_call_f64__double_const(&(*add_base.borrow_mut()), 2.5E+0,) })
            == 1.25E+1)
    );
    let total: Value<i32> = Rc::new(RefCell::new(0));
    let accumulate: Value<lambda_2> = Rc::new(RefCell::new(
        (lambda_2 {
            total: total.as_pointer(),
        }),
    ));
    ({ lambda_2::operator_call_i32_i32__int_int_const(&(*accumulate.borrow_mut()), 2, 3) });
    ({
        lambda_2::operator_call_u32_u32__unsigned_int_unsigned_int_const(
            &(*accumulate.borrow_mut()),
            4_u32,
            5_u32,
        )
    });
    assert!(((*total.borrow()) == 26));
    let sub: Value<lambda_3> = Rc::new(RefCell::new((<lambda_3>::default())));
    assert!((({ lambda_3::operator_call_i32_i32__int_const(9, 4,) }) == 5));
    assert!((({ lambda_3::operator_call_f64_f64__double_const(2.5E+0, 1.0E+0,) }) == 1.5E+0));
    let mixed: Value<lambda_4> = Rc::new(RefCell::new(
        (lambda_4 {
            base: Rc::new(RefCell::new((*base.borrow()))),
        }),
    ));
    assert!(
        (({ lambda_4::operator_call_i32_i32__int_int_const(&(*mixed.borrow_mut()), 2, 3,) }) == 16)
    );
    assert!(
        (({
            lambda_4::operator_call_i32_f64__int_double_const(&(*mixed.borrow_mut()), 2, 5.0E-1)
        }) == 1.1E+1)
    );
    let cast_to: Value<lambda_5> = Rc::new(RefCell::new((<lambda_5>::default())));
    assert!((({ lambda_5::operator_call_i32__int_const(5,) }) == 2));
    assert!((({ lambda_5::operator_call_i32__double_const(5,) }) == 2.5E+0));
    return 0;
}
#[derive(Clone, ByteRepr, Default)]
pub struct lambda_0 {}
impl lambda_0 {
    pub fn operator_call_i32__int_const(x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*x.borrow()) + (*x.borrow()));
    }
    pub fn operator_call_f64__double_const(x: f64) -> f64 {
        let x: Value<f64> = Rc::new(RefCell::new(x));
        return ((*x.borrow()) + (*x.borrow()));
    }
}
impl Callable1<i32, i32> for lambda_0 {
    fn call(&self, a1: i32) -> i32 {
        { lambda_0::operator_call_i32__int_const(a1) }
    }
}
impl Callable1<f64, f64> for lambda_0 {
    fn call(&self, a1: f64) -> f64 {
        { lambda_0::operator_call_f64__double_const(a1) }
    }
}
#[derive(Default)]
pub struct lambda_1 {
    base: Value<i32>,
}
impl lambda_1 {
    pub fn operator_call_i32__int_const(&self, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*x.borrow()) + (*self.base.borrow()));
    }
    pub fn operator_call_f64__double_const(&self, x: f64) -> f64 {
        let x: Value<f64> = Rc::new(RefCell::new(x));
        return ((*x.borrow()) + ((*self.base.borrow()) as f64));
    }
}
impl Clone for lambda_1 {
    fn clone(&self) -> Self {
        Self {
            base: Rc::new(RefCell::new((*self.base.borrow()).clone())),
        }
    }
}
impl ByteRepr for lambda_1 {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.base.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            base: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
impl Callable1<i32, i32> for lambda_1 {
    fn call(&self, a1: i32) -> i32 {
        { lambda_1::operator_call_i32__int_const(self, a1) }
    }
}
impl Callable1<f64, f64> for lambda_1 {
    fn call(&self, a1: f64) -> f64 {
        { lambda_1::operator_call_f64__double_const(self, a1) }
    }
}
#[derive(Clone, Default)]
pub struct lambda_2 {
    total: Ptr<i32>,
}
impl lambda_2 {
    pub fn operator_call_i32_i32__int_int_const(&self, x: i32, y: i32) {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let y: Value<i32> = Rc::new(RefCell::new(y));
        {
            let _ptr = self.total.clone();
            _ptr.write(_ptr.read() + ((*x.borrow()) * (*y.borrow())))
        };
    }
    pub fn operator_call_u32_u32__unsigned_int_unsigned_int_const(&self, x: u32, y: u32) {
        let x: Value<u32> = Rc::new(RefCell::new(x));
        let y: Value<u32> = Rc::new(RefCell::new(y));
        {
            let rhs_0 = (((self.total.read()) as u32)
                .wrapping_add((*x.borrow()).wrapping_mul((*y.borrow()))))
                as i32;
            self.total.write(rhs_0)
        };
    }
}
impl ByteRepr for lambda_2 {}
impl Callable2<i32, i32, ()> for lambda_2 {
    fn call(&self, a1: i32, a2: i32) -> () {
        { lambda_2::operator_call_i32_i32__int_int_const(self, a1, a2) }
    }
}
impl Callable2<u32, u32, ()> for lambda_2 {
    fn call(&self, a1: u32, a2: u32) -> () {
        { lambda_2::operator_call_u32_u32__unsigned_int_unsigned_int_const(self, a1, a2) }
    }
}
#[derive(Clone, ByteRepr, Default)]
pub struct lambda_3 {}
impl lambda_3 {
    pub fn operator_call_i32_i32__int_const(x: i32, y: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let y: Value<i32> = Rc::new(RefCell::new(y));
        return ((*x.borrow()) - (*y.borrow()));
    }
    pub fn operator_call_f64_f64__double_const(x: f64, y: f64) -> f64 {
        let x: Value<f64> = Rc::new(RefCell::new(x));
        let y: Value<f64> = Rc::new(RefCell::new(y));
        return ((*x.borrow()) - (*y.borrow()));
    }
}
impl Callable2<i32, i32, i32> for lambda_3 {
    fn call(&self, a1: i32, a2: i32) -> i32 {
        { lambda_3::operator_call_i32_i32__int_const(a1, a2) }
    }
}
impl Callable2<f64, f64, f64> for lambda_3 {
    fn call(&self, a1: f64, a2: f64) -> f64 {
        { lambda_3::operator_call_f64_f64__double_const(a1, a2) }
    }
}
#[derive(Default)]
pub struct lambda_4 {
    base: Value<i32>,
}
impl lambda_4 {
    pub fn operator_call_i32_i32__int_int_const(&self, x: i32, y: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let y: Value<i32> = Rc::new(RefCell::new(y));
        return (((*x.borrow()) * (*y.borrow())) + (*self.base.borrow()));
    }
    pub fn operator_call_i32_f64__int_double_const(&self, x: i32, y: f64) -> f64 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        let y: Value<f64> = Rc::new(RefCell::new(y));
        return ((((*x.borrow()) as f64) * (*y.borrow())) + ((*self.base.borrow()) as f64));
    }
}
impl Clone for lambda_4 {
    fn clone(&self) -> Self {
        Self {
            base: Rc::new(RefCell::new((*self.base.borrow()).clone())),
        }
    }
}
impl ByteRepr for lambda_4 {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.base.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            base: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
impl Callable2<i32, i32, i32> for lambda_4 {
    fn call(&self, a1: i32, a2: i32) -> i32 {
        { lambda_4::operator_call_i32_i32__int_int_const(self, a1, a2) }
    }
}
impl Callable2<i32, f64, f64> for lambda_4 {
    fn call(&self, a1: i32, a2: f64) -> f64 {
        { lambda_4::operator_call_i32_f64__int_double_const(self, a1, a2) }
    }
}
#[derive(Clone, ByteRepr, Default)]
pub struct lambda_5 {}
impl lambda_5 {
    pub fn operator_call_i32__int_const(x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return (((*x.borrow()) as i32) / 2);
    }
    pub fn operator_call_i32__double_const(x: i32) -> f64 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return (((*x.borrow()) as f64) / 2_f64);
    }
}
pub fn __cpp2rust_init_globals() {}
