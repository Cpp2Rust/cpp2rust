extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct Guard__lambda_at_lambda_as_field_cpp______ {
    pub f: FnPtr<fn()>,
}
impl Guard__lambda_at_lambda_as_field_cpp______ {
    pub unsafe fn destructor(&mut self) {
        (unsafe { self.f.call() });
    }
}
impl Default for Guard__lambda_at_lambda_as_field_cpp______ {
    fn default() -> Self {
        unsafe {
            Guard__lambda_at_lambda_as_field_cpp______ {
                f: FnPtr::<fn()>::new(|| unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).postfix_inc();
                }),
            }
        }
    }
}
impl Guard__lambda_at_lambda_as_field_cpp______ {
    pub unsafe fn destructor(&mut self) {
        (unsafe { self.f.call() });
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct Holder__lambda_at_lambda_as_field_cpp______ {
    pub f: FnPtr<fn(i32) -> i32>,
    pub calls: i32,
}
impl Holder__lambda_at_lambda_as_field_cpp______ {
    pub unsafe fn call(&mut self, mut x: i32) -> i32 {
        self.calls.postfix_inc();
        return (unsafe { self.f.call(x) });
    }
}
impl Default for Holder__lambda_at_lambda_as_field_cpp______ {
    fn default() -> Self {
        unsafe {
            Holder__lambda_at_lambda_as_field_cpp______ {
                f: FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
                    unsafe {
                        return ((x) + (1));
                    }
                }),
                calls: 0_i32,
            }
        }
    }
}
impl Holder__lambda_at_lambda_as_field_cpp______ {
    pub unsafe fn call(&mut self, mut x: i32) -> i32 {
        self.calls.postfix_inc();
        return (unsafe { self.f.call(x) });
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe)]
pub struct Moving__lambda_at_lambda_as_field_cpp______ {
    pub f: FnPtr<fn(i32) -> i32>,
    pub moves: i32,
}
impl Moving__lambda_at_lambda_as_field_cpp______ {
    pub unsafe fn new(mut f: FnPtr<fn(i32) -> i32>) -> Self {
        let mut this = Self {
            f: f.copy_from(),
            moves: 0,
        };
        this
    }
    pub unsafe fn move_from(o: *mut Moving__lambda_at_lambda_as_field_cpp______) -> Self {
        let mut this = Self {
            f: (*o).f.move_from(),
            moves: (((*o).moves) + (1)),
        };
        this
    }
}
impl Default for Moving__lambda_at_lambda_as_field_cpp______ {
    fn default() -> Self {
        unsafe {
            Moving__lambda_at_lambda_as_field_cpp______ {
                f: FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
                    unsafe {
                        return ((x) * (2));
                    }
                }),
                moves: 0,
            }
        }
    }
}
pub unsafe fn wrap_1(mut fn_: FnPtr<fn(i32) -> i32>) -> FnPtr<fn(i32) -> i32> {
    return lambda_unsafe!(
        {
            let fn_: FnPtr<fn(i32) -> i32> = fn_.copy_from();
        },
        |mut x: i32| -> i32 {
            return ((unsafe { fn_.call(x) }) + (1));
        }
    );
}
pub unsafe fn wrap_2(mut fn_: FnPtr<fn(i32) -> i32>) -> FnPtr<fn(i32) -> i32> {
    return lambda_unsafe!(
        {
            let fn_: FnPtr<fn(i32) -> i32> = fn_.copy_from();
        },
        |mut x: i32| -> i32 {
            return ((unsafe { fn_.call(x) }) + (1));
        }
    );
}
pub static mut count_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    {
        let mut g: Guard__lambda_at_lambda_as_field_cpp______ =
            Guard__lambda_at_lambda_as_field_cpp______ {
                f: FnPtr::<fn()>::new(|| unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).postfix_inc();
                }),
            };
        let _dtor_g = ScopedDestructorUnsafe::new(
            &raw mut g,
            Guard__lambda_at_lambda_as_field_cpp______::destructor,
        );
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)) == (0)));
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)) == (1)));
    let mut n: Holder__lambda_at_lambda_as_field_cpp______ =
        Holder__lambda_at_lambda_as_field_cpp______ {
            f: FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
                unsafe {
                    return ((x) + (1));
                }
            }),
            calls: 0,
        };
    assert!(((unsafe { Holder__lambda_at_lambda_as_field_cpp______::call(&mut n, 2,) }) == (3)));
    assert!(((n.calls) == (1)));
    let mut cleaned: i32 = 0;
    {
        let mut g: Guard__lambda_at_lambda_as_field_cpp______ =
            Guard__lambda_at_lambda_as_field_cpp______ {
                f: lambda_unsafe!(
                    {
                        let cleaned: *mut i32 = &mut cleaned;
                    },
                    || {
                        (*cleaned).postfix_inc();
                    }
                ),
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
            f: lambda_unsafe!(
                {
                    let factor: i32 = factor;
                },
                |mut x: i32| -> i32 {
                    return ((x) * (factor));
                }
            ),
            calls: 0,
        };
    factor = 100;
    assert!(((unsafe { Holder__lambda_at_lambda_as_field_cpp______::call(&mut h, 2,) }) == (6)));
    assert!(((unsafe { Holder__lambda_at_lambda_as_field_cpp______::call(&mut h, 5,) }) == (15)));
    assert!(((h.calls) == (2)));
    let mut w: FnPtr<fn(i32) -> i32> = (unsafe {
        wrap_1(lambda_unsafe!(
            {
                let factor: i32 = factor;
            },
            |mut x: i32| -> i32 {
                return ((x) * (factor));
            }
        ))
    });
    factor = 7;
    assert!(((unsafe { w.call(2,) }) == (201)));
    let mut ww: FnPtr<fn(i32) -> i32> = (unsafe { wrap_2(w.copy_from()) });
    assert!(((unsafe { ww.call(2,) }) == (202)));
    let mut m: Moving__lambda_at_lambda_as_field_cpp______ =
        Moving__lambda_at_lambda_as_field_cpp______::new({
            FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
                unsafe {
                    return ((x) * (2));
                }
            })
        });
    let mut m2: Moving__lambda_at_lambda_as_field_cpp______ =
        Moving__lambda_at_lambda_as_field_cpp______::move_from({ &mut m });
    assert!(((unsafe { m2.f.call(3,) }) == (6)));
    assert!(((m2.moves) == (1)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const count_0);
}
