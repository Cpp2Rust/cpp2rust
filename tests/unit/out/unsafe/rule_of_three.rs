extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut alive_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut copies_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive()]
pub struct Buffer {
    pub data: [i32; 4],
    pub size: i32,
}
impl Buffer {
    pub unsafe fn new(mut size: i32) -> Self {
        let mut this = Self {
            data: [0_i32; 4],
            size: size,
        };
        let mut i: i32 = 0;
        'loop_: while ((i) < (4)) {
            this.data[(i) as usize] = if ((i) < (size)) { i } else { -1_i32 };
            i.prefix_inc();
        }
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alive_0)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alive_0)).prefix_dec();
    }
    pub unsafe fn Buffer_pconstBuffer(o: *const Buffer) -> Self {
        let mut this = Self {
            data: [0_i32; 4],
            size: (*o).size,
        };
        let mut i: i32 = 0;
        'loop_: while ((i) < (4)) {
            this.data[(i) as usize] = (*o).data[(i) as usize];
            i.prefix_inc();
        }
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alive_0)).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_1)).prefix_inc();
        this
    }
    pub unsafe fn operator_assign(&mut self, o: *const Buffer) -> *mut Buffer {
        if (((self as *mut Buffer).cast_const()) == (o)) {
            return &mut (*(self as *mut Buffer));
        }
        self.size = (*o).size;
        let mut i: i32 = 0;
        'loop_: while ((i) < (4)) {
            self.data[(i) as usize] = (*o).data[(i) as usize];
            i.prefix_inc();
        }
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_1)).prefix_inc();
        return &mut (*(self as *mut Buffer));
    }
}
impl Clone for Buffer {
    fn clone(&self) -> Self {
        unsafe { Buffer::Buffer_pconstBuffer(self as *const Buffer) }
    }
}
impl Default for Buffer {
    fn default() -> Self {
        Buffer {
            data: [0_i32; 4],
            size: 0_i32,
        }
    }
}
pub unsafe fn sum_2(b: *const Buffer) -> i32 {
    let mut s: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while ((i) < ((*b).size)) {
        s += (*b).data[(i) as usize];
        i.prefix_inc();
    }
    return s;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    {
        let mut a: Buffer = Buffer::new({ 4 });
        let _dtor_a = ScopedDestructorUnsafe::new(&raw mut a, Buffer::destructor);
        let mut b: Buffer = Buffer::Buffer_pconstBuffer({ &a });
        let _dtor_b = ScopedDestructorUnsafe::new(&raw mut b, Buffer::destructor);
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alive_0)) == (2))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_1)) == (1))
        );
        b.data[(0) as usize] = 100;
        assert!(((a.data[(0) as usize]) == (0)));
        let mut c: Buffer = Buffer::new({ 2 });
        let _dtor_c = ScopedDestructorUnsafe::new(&raw mut c, Buffer::destructor);
        (unsafe { Buffer::operator_assign(&mut c, &a) });
        assert!(((c.size) == (4)) && ((c.data[(3) as usize]) == (3)));
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alive_0)) == (3))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_1)) == (2))
        );
        (unsafe {
            let _o: *const Buffer = &c;
            Buffer::operator_assign(&mut c, _o)
        });
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_1)) == (2)));
        assert!(((unsafe { sum_2(&a,) }) == (6)));
        assert!(((unsafe { sum_2(&b,) }) == (106)));
        let mut d: Buffer = Buffer::Buffer_pconstBuffer({ &a });
        let _dtor_d = ScopedDestructorUnsafe::new(&raw mut d, Buffer::destructor);
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alive_0)) == (4))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_1)) == (3))
        );
        assert!(((a.size) == (4)) && ((a.data[(3) as usize]) == (3)));
        (unsafe { Buffer::operator_assign(&mut d, &b) });
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_1)) == (4)));
        assert!(((b.data[(0) as usize]) == (100)) && ((d.data[(0) as usize]) == (100)));
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut alive_0)) == (0)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const alive_0);
    std::cell::LazyCell::force(&*&raw const copies_1);
}
