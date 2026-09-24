extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, Default)]
pub struct Guard__lambda_at_lambda_as_field_cpp______ {
    pub f: _,
}
impl Guard__lambda_at_lambda_as_field_cpp______ {
    pub unsafe fn destructor(&mut self) {
        (unsafe { self.f() });
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct Holder__lambda_at_lambda_as_field_cpp______ {
    pub f: _,
    pub calls: i32,
}
impl Holder__lambda_at_lambda_as_field_cpp______ {
    pub unsafe fn call(&mut self, mut x: i32) -> i32 {
        self.calls.postfix_inc();
        return (unsafe { self.f(x) });
    }
}
pub unsafe fn wrap_0(mut fn_: impl Fn(i32) -> i32) -> _ {
    return (|x: i32| {
        return ((unsafe { fn_(x) }) + (1));
    });
}
pub unsafe fn wrap_1(mut fn_: impl Fn(i32) -> i32) -> _ {
    return (|x: i32| {
        return ((unsafe { fn_(x) }) + (1));
    });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut cleaned: i32 = 0;
    {
        let mut g: Guard__lambda_at_lambda_as_field_cpp______ =
            Guard__lambda_at_lambda_as_field_cpp______ {
                f: (|| {
                    cleaned.postfix_inc();
                }),
            };
        let _dtor_g = ScopedDestructorUnsafe::new(
            &raw mut g,
            Guard__lambda_at_lambda_as_field_cpp______::destructor,
        );
        assert!(((cleaned) == (0)));
    }
    assert!(((cleaned) == (1)));
    let mut factor: i32 = 3;
    let mut h: Holder__lambda_at_lambda_as_field_cpp______ =
        Holder__lambda_at_lambda_as_field_cpp______ {
            f: (|x: i32| {
                return ((x) * (factor));
            }),
            calls: 0,
        };
    factor = 100;
    assert!(((unsafe { Holder__lambda_at_lambda_as_field_cpp______::call(&mut h, 2,) }) == (6)));
    assert!(((unsafe { Holder__lambda_at_lambda_as_field_cpp______::call(&mut h, 5,) }) == (15)));
    assert!(((h.calls) == (2)));
    let mut w: _ = (unsafe {
        wrap_0(
            (|x: i32| {
                return ((x) * (factor));
            }),
        )
    });
    factor = 7;
    assert!(((unsafe { w(2,) }) == (201)));
    let mut ww: _ = (unsafe { wrap_1(w.clone()) });
    assert!(((unsafe { ww(2,) }) == (202)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
