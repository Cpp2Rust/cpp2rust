extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut destroyed_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut last_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct S {
    pub v: i32,
}
impl S {
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)).postfix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut last_1)) = self.v;
    }
    pub unsafe fn get(&self) -> i32 {
        return self.v;
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Outer {
    pub inner: S,
    pub w: i32,
}
impl Outer {
    pub unsafe fn destructor(&mut self) {
        S::destructor(&mut self.inner);
    }
}
pub unsafe fn make_2(mut v: i32) -> S {
    return S { v: v };
}
pub unsafe fn make_outer_3() -> Outer {
    return Outer {
        inner: S { v: 7 },
        w: 8,
    };
}
pub unsafe fn use_4(s: *const S) -> i32 {
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) == (0)));
    return (*s).v;
}
pub unsafe fn value_5(s: *const S) -> i32 {
    return (*s).v;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { make_2(1) });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut last_1)) == (1))
    );
    (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) = 0;
    &(unsafe { make_2(2) });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut last_1)) == (2))
    );
    (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) = 0;
    let mut x: i32 = (unsafe { make_2(3) }).v;
    assert!(
        (((x) == (3)) && ((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) == (1)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut last_1)) == (3))
    );
    (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) = 0;
    x = (unsafe { S::get(&(unsafe { make_2(4) })) });
    assert!(
        (((x) == (4)) && ((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) == (1)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut last_1)) == (4))
    );
    (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) = 0;
    x = (unsafe { make_outer_3() }).inner.v;
    assert!(
        (((x) == (7)) && ((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) == (1)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut last_1)) == (7))
    );
    (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) = 0;
    x = S { v: 5 }.v;
    assert!(((x) == (5)) && ((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) == (1)));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) = 0;
    x = (unsafe {
        let mut _s: S = (unsafe { make_2(6) });
        use_4(&mut _s)
    });
    assert!(
        (((x) == (6)) && ((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) == (1)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut last_1)) == (6))
    );
    (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) = 0;
    x = ((unsafe {
        let mut _s: S = S { v: 1 };
        value_5(&mut _s)
    }) + (unsafe {
        let mut _s: S = S { v: 2 };
        value_5(&mut _s)
    }));
    assert!(((x) == (3)) && ((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) == (2)));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) = 0;
    x = (if ((x) == (3)) {
        (unsafe { make_2(9) })
    } else {
        S { v: 10 }
    })
    .v;
    assert!(
        (((x) == (9)) && ((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) == (1)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut last_1)) == (9))
    );
    (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) = 0;
    {
        let mut __tmp_0: S = (unsafe { make_2(11) });
        let r: *const S = &mut __tmp_0;
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) == (0))
                && (((*r).v) == (11))
        );
    }
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut last_1)) == (11))
    );
    (*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) = 0;
    {
        let mut s: S = S { v: 12 };
        let _dtor_s = ScopedDestructorUnsafe::new(&raw mut s, S::destructor);
        s = (unsafe { make_2(13) });
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) == (1))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut last_1)) == (13)))
                && ((s.v) == (13))
        );
    }
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut destroyed_0)) == (2))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut last_1)) == (13))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destroyed_0);
    std::cell::LazyCell::force(&*&raw const last_1);
}
