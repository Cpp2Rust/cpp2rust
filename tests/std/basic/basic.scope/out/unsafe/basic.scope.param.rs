extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct p1_1_parameter_hides_member_in_constructor_HasMemberNamedLikeParameter {
    pub amount: i32,
}
impl p1_1_parameter_hides_member_in_constructor_HasMemberNamedLikeParameter {
    pub unsafe fn new(mut amount: i32) -> Self {
        let mut this = Self {
            amount: ((amount) + (1)),
        };
        amount += 10;
        assert!(((amount) == (14)));
        assert!(((this.amount) == (5)));
        this
    }
    pub unsafe fn add(&self, mut amount: i32) -> i32 {
        return ((self.amount) + (amount));
    }
}
pub unsafe fn test_0() {
    let mut object: p1_1_parameter_hides_member_in_constructor_HasMemberNamedLikeParameter =
        p1_1_parameter_hides_member_in_constructor_HasMemberNamedLikeParameter::new({ 4 });
    assert!(((object.amount) == (5)));
    assert!(
        ((unsafe {
            p1_1_parameter_hides_member_in_constructor_HasMemberNamedLikeParameter::add(&object, 20)
        }) == (25))
    );
    assert!(((object.amount) == (5)));
}
pub static mut rate_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 3 });
pub unsafe fn scale_2(mut rate: i32) -> i32 {
    rate *= 2;
    return rate;
}
pub unsafe fn read_rate_3() -> i32 {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut rate_1));
}
pub unsafe fn test_4() {
    assert!(((unsafe { scale_2(5,) }) == (10)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut rate_1)) == (3)));
    assert!(((unsafe { read_rate_3() }) == (3)));
}
pub unsafe fn test_5() {
    let mut amount: i32 = 6;
    let mut total: i32 = 0;
    let mut add_doubled: FnPtr<fn(i32) -> i32> = lambda_unsafe!(
        {
            let total: *mut i32 = &mut total;
        },
        |mut amount: i32| -> i32 {
            amount *= 2;
            (*total) += amount;
            return amount;
        }
    );
    assert!(((unsafe { add_doubled.call(4,) }) == (8)));
    assert!(((total) == (8)));
    assert!(((amount) == (6)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_0() });
    (unsafe { test_4() });
    (unsafe { test_5() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const rate_1);
}
