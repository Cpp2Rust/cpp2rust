extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone)]
pub struct Sorted__lambda_at_lambda_decltype_type_cpp______ {
    pub items: [i32; 4],
    pub size: i32,
    pub cmp: _,
}
impl Sorted__lambda_at_lambda_decltype_type_cpp______ {
    pub unsafe fn insert(&mut self, mut v: i32) {
        let mut i: i32 = self.size;
        'loop_: while ((i) > (0))
            && (unsafe {
                let _b: i32 = self.items[((i) - (1)) as usize];
                self.cmp(v, _b)
            })
        {
            self.items[(i) as usize] = self.items[((i) - (1)) as usize];
            i.postfix_dec();
        }
        self.items[(i) as usize] = v;
        self.size.postfix_inc();
    }
}
impl Default for Sorted__lambda_at_lambda_decltype_type_cpp______ {
    fn default() -> Self {
        Sorted__lambda_at_lambda_decltype_type_cpp______ {
            items: [0_i32; 4],
            size: 0,
            cmp: <_>::default(),
        }
    }
}
impl Sorted__lambda_at_lambda_decltype_type_cpp______ {
    pub unsafe fn insert(&mut self, mut v: i32) {
        let mut i: i32 = self.size;
        'loop_: while ((i) > (0))
            && (unsafe {
                let _b: i32 = self.items[((i) - (1)) as usize];
                self.cmp(v, _b)
            })
        {
            self.items[(i) as usize] = self.items[((i) - (1)) as usize];
            i.postfix_dec();
        }
        self.items[(i) as usize] = v;
        self.size.postfix_inc();
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut desc: Sorted__lambda_at_lambda_decltype_type_cpp______ =
        <Sorted__lambda_at_lambda_decltype_type_cpp______>::default();
    (unsafe { Sorted__lambda_at_lambda_decltype_type_cpp______::insert(&mut desc, 2) });
    (unsafe { Sorted__lambda_at_lambda_decltype_type_cpp______::insert(&mut desc, 7) });
    (unsafe { Sorted__lambda_at_lambda_decltype_type_cpp______::insert(&mut desc, 4) });
    assert!(((desc.items[(0) as usize]) == (7)));
    assert!(((desc.items[(1) as usize]) == (4)));
    assert!(((desc.items[(2) as usize]) == (2)));
    let mut asc: Sorted__lambda_at_lambda_decltype_type_cpp______ =
        <Sorted__lambda_at_lambda_decltype_type_cpp______>::default();
    (unsafe { Sorted__lambda_at_lambda_decltype_type_cpp______::insert(&mut asc, 2) });
    (unsafe { Sorted__lambda_at_lambda_decltype_type_cpp______::insert(&mut asc, 7) });
    (unsafe { Sorted__lambda_at_lambda_decltype_type_cpp______::insert(&mut asc, 4) });
    assert!(((asc.items[(0) as usize]) == (2)));
    assert!(((asc.items[(1) as usize]) == (4)));
    assert!(((asc.items[(2) as usize]) == (7)));
    let mut fresh: _ = <_>::default();
    assert!(((unsafe { fresh(3, 1,) }) as bool));
    assert!(!(unsafe { fresh(1, 3,) }));
    let mut assigned: _ = <_>::default();
    assigned = (|a: i32, b: i32| {
        return ((a) > (b));
    });
    assert!(((unsafe { assigned(5, 4,) }) as bool));
    let mut less: _ = <_>::default();
    assert!(((unsafe { less(1, 3,) }) as bool));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
