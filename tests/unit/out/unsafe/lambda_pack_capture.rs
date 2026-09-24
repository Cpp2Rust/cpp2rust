extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn sum_0() -> i32 {
    return 0;
}
pub unsafe fn sum_1(mut t: i32, mut u_1: i32, mut u_2: i32) -> i32 {
    return ((t) + (unsafe { sum_2(u_1, u_2) }));
}
pub unsafe fn sum_2(mut t: i32, mut u: i32) -> i32 {
    return ((t) + (unsafe { sum_3(u) }));
}
pub unsafe fn sum_3(mut t: i32) -> i32 {
    return ((t) + (unsafe { sum_0() }));
}
pub unsafe fn by_value_4() -> i32 {
    ();
    return (unsafe {
        (|| {
            return (unsafe { sum_0() });
        })()
    });
}
pub unsafe fn by_value_5(mut t_0: i32, mut t_1: i32, mut t_2: i32) -> i32 {
    {
        t_0 = 0;
        {
            t_1 = 0;
            t_2 = 0
        }
    };
    return (unsafe {
        (|| {
            return (unsafe { sum_1(t_0, t_1, t_2) });
        })()
    });
}
pub unsafe fn by_ref_6(mut t_0: i32, mut t_1: i32, mut t_2: i32) -> i32 {
    (unsafe {
        (|| {
            {
                t_0 *= 2;
                {
                    t_1 *= 2;
                    t_2 *= 2
                }
            };
        })()
    });
    return (unsafe { sum_1(t_0, t_1, t_2) });
}
pub unsafe fn init_pack_7(mut t_0: i32, mut t_1: i32, mut t_2: i32) -> i32 {
    return (unsafe {
        (|| {
            return (unsafe { sum_1(xs, xs, xs) });
        })()
    });
}
pub unsafe fn implicit_8(mut t_0: i32, mut t_1: i32) -> i32 {
    return (unsafe {
        (|| {
            return (unsafe { sum_2(t_0, t_1) });
        })()
    });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((unsafe { by_value_4() }) == (0)));
    assert!(((unsafe { by_value_5(1, 2, 3,) }) == (6)));
    assert!(((unsafe { by_ref_6(1, 2, 3,) }) == (12)));
    assert!(((unsafe { init_pack_7(1, 2, 3,) }) == (9)));
    assert!(((unsafe { implicit_8(4, 5,) }) == (9)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
