extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn apply_int_0(mut fn_: FnPtr<fn(i32) -> i32>, mut x: i32) -> i32 {
    return (unsafe { fn_.call(x) });
}
pub unsafe fn apply_int_1(mut fn_: FnPtr<Generic>, mut x: i32) -> i32 {
    return (unsafe { fn_.spec::<fn(i32) -> i32>(0).call(x) });
}
pub unsafe fn apply_int_2(mut fn_: FnPtr<Generic>, mut x: i32) -> i32 {
    return (unsafe { fn_.spec::<fn(i32) -> i32>(0).call(x) });
}
pub unsafe fn apply_int_3(mut fn_: FnPtr<Generic>, mut x: i32) -> i32 {
    return (unsafe { fn_.spec::<fn(i32) -> i32>(0).call(x) });
}
pub unsafe fn apply_double_4(mut fn_: FnPtr<Generic>, mut x: f64) -> f64 {
    return (unsafe { fn_.spec::<fn(f64) -> f64>(1).call(x) });
}
pub unsafe fn apply_double_5(mut fn_: FnPtr<Generic>, mut x: f64) -> f64 {
    return (unsafe { fn_.spec::<fn(f64) -> f64>(1).call(x) });
}
pub unsafe fn apply_twice_6(mut fn_: FnPtr<fn(i32) -> i32>, mut x: i32) -> i32 {
    return (unsafe {
        let _x: i32 = (unsafe { fn_.call(x) });
        fn_.call(_x)
    });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut factor: i32 = 3;
    let mut scale: FnPtr<fn(i32) -> i32> = lambda_unsafe!(
        {
            let factor: i32 = factor;
        },
        |x: i32| -> i32 {
            return ((x) * (factor));
        }
    );
    assert!(((unsafe { apply_twice_6(scale.copy_from(), 4,) }) == (36)));
    assert!(
        ((unsafe {
            apply_int_0(
                FnPtr::<fn(i32) -> i32>::new(|x: i32| -> i32 {
                    unsafe {
                        return -x;
                    }
                }),
                9,
            )
        }) == (-9_i32))
    );
    let mut generic_scale: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let factor: i32 = factor;
        },
        |x: i32| -> i32 {
            return ((x) * (factor));
        },
        |x: f64| -> f64 {
            return ((x) * (factor as f64));
        }
    );
    assert!(((unsafe { apply_int_1(generic_scale.copy_from(), 4,) }) == (12)));
    assert!(((unsafe { apply_double_4(generic_scale.copy_from(), 1.5_f64,) }) == (4.5_f64)));
    assert!(
        ((unsafe {
            apply_int_2(
                lambda_unsafe!(Generic, {}, |x: i32| -> i32 {
                    return -x;
                }),
                9,
            )
        }) == (-9_i32))
    );
    let mut offset: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let factor: i32 = factor;
        },
        |x: i32| -> i32 {
            return ((x) + (factor));
        },
        |x: f64| -> f64 {
            return ((x) + (factor as f64));
        }
    );
    assert!(((unsafe { apply_int_3(offset.copy_from(), 4,) }) == (7)));
    assert!(((unsafe { apply_double_5(offset.copy_from(), 1.5_f64,) }) == (4.5_f64)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
