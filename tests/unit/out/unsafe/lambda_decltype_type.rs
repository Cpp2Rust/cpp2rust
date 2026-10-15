extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Sorted__lambda_at_lambda_decltype_type_cpp_22_18__ {
    pub items: [i32; 4],
    pub size: i32,
    pub cmp: FnPtr<fn(i32, i32) -> bool>,
}
impl Sorted__lambda_at_lambda_decltype_type_cpp_22_18__ {
    pub unsafe fn insert(&mut self, mut v: i32) {
        let mut i: i32 = self.size;
        'loop_: while ((i) > (0))
            && (unsafe {
                let _b: i32 = self.items[((i) - (1)) as usize];
                self.cmp.call(v, _b)
            })
        {
            self.items[(i) as usize] = self.items[((i) - (1)) as usize];
            i.postfix_dec();
        }
        self.items[(i) as usize] = v;
        self.size.postfix_inc();
    }
}
impl Default for Sorted__lambda_at_lambda_decltype_type_cpp_22_18__ {
    fn default() -> Self {
        unsafe {
            Sorted__lambda_at_lambda_decltype_type_cpp_22_18__ {
                items: [0_i32; 4],
                size: 0,
                cmp: FnPtr::<fn(i32, i32) -> bool>::new(|mut a: i32, mut b: i32| -> bool {
                    unsafe {
                        return ((a) > (b));
                    }
                }),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Sorted__lambda_at_lambda_decltype_type_cpp_19_23__ {
    pub items: [i32; 4],
    pub size: i32,
    pub cmp: FnPtr<fn(i32, i32) -> bool>,
}
impl Sorted__lambda_at_lambda_decltype_type_cpp_19_23__ {
    pub unsafe fn insert(&mut self, mut v: i32) {
        let mut i: i32 = self.size;
        'loop_: while ((i) > (0))
            && (unsafe {
                let _b: i32 = self.items[((i) - (1)) as usize];
                self.cmp.call(v, _b)
            })
        {
            self.items[(i) as usize] = self.items[((i) - (1)) as usize];
            i.postfix_dec();
        }
        self.items[(i) as usize] = v;
        self.size.postfix_inc();
    }
}
impl Default for Sorted__lambda_at_lambda_decltype_type_cpp_19_23__ {
    fn default() -> Self {
        unsafe {
            Sorted__lambda_at_lambda_decltype_type_cpp_19_23__ {
                items: [0_i32; 4],
                size: 0,
                cmp: FnPtr::<fn(i32, i32) -> bool>::new(|mut a: i32, mut b: i32| -> bool {
                    unsafe {
                        return ((a) < (b));
                    }
                }),
            }
        }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut greater: FnPtr<fn(i32, i32) -> bool> =
        FnPtr::<fn(i32, i32) -> bool>::new(|mut a: i32, mut b: i32| -> bool {
            unsafe {
                return ((a) > (b));
            }
        });
    let mut desc: Sorted__lambda_at_lambda_decltype_type_cpp_22_18__ =
        <Sorted__lambda_at_lambda_decltype_type_cpp_22_18__>::default();
    (unsafe { Sorted__lambda_at_lambda_decltype_type_cpp_22_18__::insert(&mut desc, 2) });
    (unsafe { Sorted__lambda_at_lambda_decltype_type_cpp_22_18__::insert(&mut desc, 7) });
    (unsafe { Sorted__lambda_at_lambda_decltype_type_cpp_22_18__::insert(&mut desc, 4) });
    assert!(((desc.items[(0) as usize]) == (7)));
    assert!(((desc.items[(1) as usize]) == (4)));
    assert!(((desc.items[(2) as usize]) == (2)));
    let mut asc: Sorted__lambda_at_lambda_decltype_type_cpp_19_23__ =
        <Sorted__lambda_at_lambda_decltype_type_cpp_19_23__>::default();
    (unsafe { Sorted__lambda_at_lambda_decltype_type_cpp_19_23__::insert(&mut asc, 2) });
    (unsafe { Sorted__lambda_at_lambda_decltype_type_cpp_19_23__::insert(&mut asc, 7) });
    (unsafe { Sorted__lambda_at_lambda_decltype_type_cpp_19_23__::insert(&mut asc, 4) });
    assert!(((asc.items[(0) as usize]) == (2)));
    assert!(((asc.items[(1) as usize]) == (4)));
    assert!(((asc.items[(2) as usize]) == (7)));
    let mut fresh: FnPtr<fn(i32, i32) -> bool> =
        FnPtr::<fn(i32, i32) -> bool>::new(|mut a: i32, mut b: i32| -> bool {
            unsafe {
                return ((a) > (b));
            }
        });
    assert!(((unsafe { fresh.call(3, 1,) }) as bool));
    assert!(!(unsafe { fresh.call(1, 3,) }));
    let mut assigned: FnPtr<fn(i32, i32) -> bool> =
        FnPtr::<fn(i32, i32) -> bool>::new(|mut a: i32, mut b: i32| -> bool {
            unsafe {
                return ((a) > (b));
            }
        });
    assigned = greater;
    assert!(((unsafe { assigned.call(5, 4,) }) as bool));
    let mut less: FnPtr<fn(i32, i32) -> bool> =
        FnPtr::<fn(i32, i32) -> bool>::new(|mut a: i32, mut b: i32| -> bool {
            unsafe {
                return ((a) < (b));
            }
        });
    assert!(((unsafe { less.call(1, 3,) }) as bool));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
