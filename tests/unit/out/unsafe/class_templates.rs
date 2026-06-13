extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, Default)]
pub struct MyContainer_int_ {
    vec_: Vec<i32>,
}
impl MyContainer_int_ {
    pub unsafe fn empty(&self) -> bool {
        return self.vec_.is_empty();
    }
    pub unsafe fn size(&self) -> usize {
        return self.vec_.len();
    }
    pub unsafe fn back_const(&self) -> *const i32 {
        return ((self.vec_).last().unwrap());
    }
    pub unsafe fn back(&mut self) -> *mut i32 {
        return ((self.vec_).last_mut().unwrap());
    }
    pub unsafe fn pop_back(&mut self) {
        self.vec_.pop();
        return;
    }
    pub unsafe fn push_back(&mut self, item: *const i32) {
        {
            let a0_clone = (*item).clone();
            self.vec_.push(a0_clone)
        };
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct MyContainer_char_ {
    vec_: Vec<u8>,
}
impl MyContainer_char_ {
    pub unsafe fn empty(&self) -> bool {
        return self.vec_.is_empty();
    }
    pub unsafe fn size(&self) -> usize {
        return self.vec_.len();
    }
    pub unsafe fn back_const(&self) -> *const u8 {
        return ((self.vec_).last().unwrap());
    }
    pub unsafe fn back(&mut self) -> *mut u8 {
        return ((self.vec_).last_mut().unwrap());
    }
    pub unsafe fn pop_back(&mut self) {
        self.vec_.pop();
        return;
    }
    pub unsafe fn push_back(&mut self, item: *const u8) {
        {
            let a0_clone = (*item).clone();
            self.vec_.push(a0_clone)
        };
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct MyContainer_float_ {
    vec_: Vec<f32>,
}
impl MyContainer_float_ {
    pub unsafe fn empty(&self) -> bool {
        return self.vec_.is_empty();
    }
    pub unsafe fn size(&self) -> usize {
        return self.vec_.len();
    }
    pub unsafe fn back_const(&self) -> *const f32 {
        return ((self.vec_).last().unwrap());
    }
    pub unsafe fn back(&mut self) -> *mut f32 {
        return ((self.vec_).last_mut().unwrap());
    }
    pub unsafe fn pop_back(&mut self) {
        self.vec_.pop();
        return;
    }
    pub unsafe fn push_back(&mut self, item: *const f32) {
        {
            let a0_clone = (*item).clone();
            self.vec_.push(a0_clone)
        };
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut imc: MyContainer_int_ = <MyContainer_int_>::default();
    assert!((unsafe { imc.empty() }));
    (unsafe {
        let mut _item = 1;
        imc.push_back(&mut _item)
    });
    assert!(((unsafe { imc.size() }) == (1_usize)) && ((*(unsafe { imc.back() })) == (1)));
    (unsafe { imc.pop_back() });
    assert!((unsafe { imc.empty() }));
    let mut cmc: MyContainer_char_ = <MyContainer_char_>::default();
    assert!((unsafe { cmc.empty() }));
    (unsafe {
        let mut _item = ('a' as u8);
        cmc.push_back(&mut _item)
    });
    assert!(
        ((unsafe { cmc.size() }) == (1_usize))
            && (((*(unsafe { cmc.back() })) as i32) == (('a' as u8) as i32))
    );
    (unsafe { cmc.pop_back() });
    assert!((unsafe { cmc.empty() }));
    let mut fmc: MyContainer_float_ = <MyContainer_float_>::default();
    assert!((unsafe { fmc.empty() }));
    (unsafe {
        let mut _item = (('a' as u8) as f32);
        fmc.push_back(&mut _item)
    });
    assert!(
        ((unsafe { fmc.size() }) == (1_usize))
            && ((*(unsafe { fmc.back() })) == ((('a' as u8) as i32) as f32))
    );
    (unsafe { fmc.pop_back() });
    assert!((unsafe { fmc.empty() }));
    return 0;
}
