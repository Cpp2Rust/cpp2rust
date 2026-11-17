extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static allocations_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct p1_allocation_function_is_static_Allocates {}
impl p1_allocation_function_is_static_Allocates {
    pub fn operator_new(mut size: usize) -> AnyPtr {
        (*allocations_0.with(Value::clone).borrow_mut()).prefix_inc();
        return libcc2rs::malloc_refcount(size);
    }
    pub fn operator_new_arr(mut size: usize) -> AnyPtr {
        allocations_0.with(|rc| *rc.borrow_mut() += 10);
        return libcc2rs::malloc_refcount(size);
    }
}
pub fn test_1() {
    let mut one: Ptr<i32> = ({
        p1_allocation_function_is_static_Allocates::operator_new(::std::mem::size_of::<i32>())
    })
    .reinterpret_cast::<i32>();
    one.write(7);
    assert!(((one.read()) == 7));
    assert!((allocations_0.with(|rc| *rc.borrow()) == 1));
    libcc2rs::free_refcount((one).to_any());
    let mut many: Ptr<i32> = ({
        p1_allocation_function_is_static_Allocates::operator_new_arr(
            (::std::mem::size_of::<i32>() as usize).wrapping_mul(2_usize),
        )
    })
    .reinterpret_cast::<i32>();
    elem!(many, 0).write(1);
    elem!(many, 1).write(2);
    assert!((((elem!(many, 0).read()) + (elem!(many, 1).read())) == 3));
    assert!((allocations_0.with(|rc| *rc.borrow()) == 11));
    libcc2rs::free_refcount((many).to_any());
}
thread_local!(
    pub static deallocations_2: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct p3_deallocation_function_is_static_Deallocates {}
impl p3_deallocation_function_is_static_Deallocates {
    pub fn operator_delete(mut ptr: AnyPtr) {
        (*deallocations_2.with(Value::clone).borrow_mut()).prefix_inc();
        libcc2rs::free_refcount((ptr).clone());
    }
    pub fn operator_delete_arr(mut ptr: AnyPtr) {
        deallocations_2.with(|rc| *rc.borrow_mut() += 10);
        libcc2rs::free_refcount((ptr).clone());
    }
}
pub fn test_3() {
    let mut one: Ptr<i32> =
        libcc2rs::malloc_refcount(::std::mem::size_of::<i32>()).reinterpret_cast::<i32>();
    ({ p3_deallocation_function_is_static_Deallocates::operator_delete((one).to_any()) });
    assert!((deallocations_2.with(|rc| *rc.borrow()) == 1));
    let mut many: Ptr<i32> =
        libcc2rs::malloc_refcount((::std::mem::size_of::<i32>() as usize).wrapping_mul(2_usize))
            .reinterpret_cast::<i32>();
    ({ p3_deallocation_function_is_static_Deallocates::operator_delete_arr((many).to_any()) });
    assert!((deallocations_2.with(|rc| *rc.borrow()) == 11));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_1() });
    ({ test_3() });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = allocations_0.with(|_| ());
    let _ = deallocations_2.with(|_| ());
}
