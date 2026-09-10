extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut alive_0: i32 = unsafe { 0 };
pub static mut copies_1: i32 = unsafe { 0 };
#[repr(C)]
#[derive()]
pub struct Buffer {
    pub data: [i32; 4],
    pub size: i32,
}
impl Buffer {
    pub unsafe fn Buffer(mut size: i32) -> Self {
        let mut this = Self {
            data: [0_i32; 4],
            size: size,
        };
        let mut i: i32 = 0;
        'loop_: while ((i) < (4)) {
            this.data[(i) as usize] = if ((i) < (size)) { i } else { -1_i32 };
            i.prefix_inc();
        }
        alive_0.prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        alive_0.prefix_dec();
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
        alive_0.prefix_inc();
        copies_1.prefix_inc();
        this
    }
    pub unsafe fn operator_assign(&mut self, o: *const Buffer) -> *mut Buffer {
        if (((self as *mut Buffer).cast_const()) == (o)) {
            return &mut (*(self as *mut Buffer)) as *mut Buffer;
        }
        self.size = (*o).size;
        let mut i: i32 = 0;
        'loop_: while ((i) < (4)) {
            self.data[(i) as usize] = (*o).data[(i) as usize];
            i.prefix_inc();
        }
        copies_1.prefix_inc();
        return &mut (*(self as *mut Buffer)) as *mut Buffer;
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
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    {
        let mut a: Buffer = Buffer::Buffer({ 4 });
        let _dtor_a = ScopedDestructorUnsafe::new(&raw mut a, Buffer::destructor);
        let mut b: Buffer = Buffer::Buffer_pconstBuffer({ &a as *const Buffer });
        let _dtor_b = ScopedDestructorUnsafe::new(&raw mut b, Buffer::destructor);
        assert!(((alive_0) == (2)) && ((copies_1) == (1)));
        b.data[(0) as usize] = 100;
        assert!(((a.data[(0) as usize]) == (0)));
        let mut c: Buffer = Buffer::Buffer({ 2 });
        let _dtor_c = ScopedDestructorUnsafe::new(&raw mut c, Buffer::destructor);
        (unsafe { Buffer::operator_assign(&mut c, &a as *const Buffer) });
        assert!(((c.size) == (4)) && ((c.data[(3) as usize]) == (3)));
        assert!(((alive_0) == (3)) && ((copies_1) == (2)));
        (unsafe {
            let _o: *const Buffer = &c as *const Buffer;
            Buffer::operator_assign(&mut c, _o)
        });
        assert!(((copies_1) == (2)));
        assert!(((unsafe { sum_2(&a as *const Buffer,) }) == (6)));
        assert!(((unsafe { sum_2(&b as *const Buffer,) }) == (106)));
        let mut d: Buffer = Buffer::Buffer_pconstBuffer({ &mut a });
        let _dtor_d = ScopedDestructorUnsafe::new(&raw mut d, Buffer::destructor);
        assert!(((alive_0) == (4)) && ((copies_1) == (3)));
        assert!(((a.size) == (4)) && ((a.data[(3) as usize]) == (3)));
        (unsafe { Buffer::operator_assign(&mut d, &mut b) });
        assert!(((copies_1) == (4)));
        assert!(((b.data[(0) as usize]) == (100)) && ((d.data[(0) as usize]) == (100)));
    }
    assert!(((alive_0) == (0)));
    return 0;
}
