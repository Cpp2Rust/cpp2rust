extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive()]
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
#[derive()]
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
    assert!(
        ((unsafe {
            (|| {
                return (((c.copies) * (10)) + (c.moves));
            })()
        }) == (10))
    );
    let mut g: _ = (|| {
        return (((c.copies) * (10)) + (c.moves));
    })
    .clone();
    assert!(((unsafe { g() }) == (20)));
    assert!(
        ((unsafe {
            (|| {
                return (((c.copies) * (10)) + (c.moves));
            })()
        }) == (10))
    );
    let mut h: _ = (|| {
        return (((c.copies) * (10)) + (c.moves));
    });
    assert!(((unsafe { h() }) == (11)));
    let mut returned: i32 = (unsafe {
        (|| {
            return Counted::copy_from({ &c });
        })()
    })
    .copies;
    assert!(((returned) == (2)));
    let mut arr: [Counted; 2] = std::array::from_fn::<_, 2, _>(|_| Counted::new());
    assert!(
        ((unsafe {
            (|| {
                return ((arr[(0) as usize].copies) + (arr[(1) as usize].copies));
            })()
        }) == (2))
    );
    let mut a2: _ = (|| {
        return ((arr[(0) as usize].copies) + (arr[(1) as usize].copies));
    })
    .clone();
    assert!(((unsafe { a2() }) == (4)));
    {
        let mut m2: _ = (|| {});
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut drops_0)) == (2)));
    {
        let mut k2: _ = (|| {}).clone();
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut drops_0)) == (4)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const drops_0);
}
