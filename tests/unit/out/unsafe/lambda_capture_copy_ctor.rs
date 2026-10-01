extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(VaArg, FnPtrArg)]
pub struct Counted {
    pub copies: i32,
    pub moves: i32,
}
impl Counted {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            copies: 0,
            moves: 0,
        };
        this
    }
    pub unsafe fn copy_from(o: *const Counted) -> Self {
        let mut this = Self {
            copies: (((*o).copies) + (1)),
            moves: (*o).moves,
        };
        this
    }
    pub unsafe fn move_from(o: *mut Counted) -> Self {
        let mut this = Self {
            copies: (*o).copies,
            moves: (((*o).moves) + (1)),
        };
        this
    }
}
impl Clone for Counted {
    fn clone(&self) -> Self {
        unsafe { Counted::copy_from(self as *const Counted) }
    }
}
impl Default for Counted {
    fn default() -> Self {
        unsafe { Counted::new() }
    }
}
pub static mut drops_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(VaArg, FnPtrArg)]
pub struct Dropped {}
impl Dropped {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn copy_from(_a0: *const Dropped) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn move_from(_a0: *mut Dropped) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut drops_0)).postfix_inc();
    }
}
impl Clone for Dropped {
    fn clone(&self) -> Self {
        unsafe { Dropped::copy_from(self as *const Dropped) }
    }
}
impl Default for Dropped {
    fn default() -> Self {
        unsafe { Dropped::new() }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut c: Counted = Counted::new();
    let mut f: FnPtr<fn() -> i32> = lambda_unsafe!(
        {
            let c: Counted = Counted::copy_from({ &c });
        },
        || -> i32 {
            return (((self.c.copies) * (10)) + (self.c.moves));
        }
    );
    assert!(((unsafe { f.call() }) == (10)));
    let mut g: FnPtr<fn() -> i32> = f.clone();
    assert!(((unsafe { g.call() }) == (20)));
    assert!(((unsafe { f.call() }) == (10)));
    let mut h: FnPtr<fn() -> i32> = f;
    assert!(((unsafe { h.call() }) == (11)));
    let mut returned: i32 = (unsafe {
        lambda_unsafe!(
            {
                let c: Counted = Counted::copy_from({ &c });
            },
            || -> Counted {
                return Counted::copy_from({ &self.c });
            }
        )
        .call()
    })
    .copies;
    assert!(((returned) == (2)));
    let mut arr: [Counted; 2] = std::array::from_fn::<_, 2, _>(|_| Counted::new());
    let mut a: FnPtr<fn() -> i32> = lambda_unsafe!(
        {
            let arr: [Counted; 2] =
                std::array::from_fn::<_, 2, _>(|__i: usize| Counted::copy_from({ &arr[(__i)] }));
        },
        || -> i32 {
            return ((self.arr[(0) as usize].copies) + (self.arr[(1) as usize].copies));
        }
    );
    assert!(((unsafe { a.call() }) == (2)));
    let mut a2: FnPtr<fn() -> i32> = a.clone();
    assert!(((unsafe { a2.call() }) == (4)));
    {
        let mut m: FnPtr<fn()> = lambda_unsafe!(
            {
                let d: Dropped = Dropped::new();
            },
            || {}
        );
        let mut m2: FnPtr<fn()> = m;
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut drops_0)) == (2)));
    {
        let mut k: FnPtr<fn()> = lambda_unsafe!(
            {
                let d: Dropped = Dropped::new();
            },
            || {}
        );
        let mut k2: FnPtr<fn()> = k.clone();
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut drops_0)) == (4)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const drops_0);
}
