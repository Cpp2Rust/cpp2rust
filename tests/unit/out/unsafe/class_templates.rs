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
    vec_: Vec<libc::c_char>,
}
impl MyContainer_char_ {
    pub unsafe fn empty(&self) -> bool {
        return self.vec_.is_empty();
    }
    pub unsafe fn size(&self) -> usize {
        return self.vec_.len();
    }
    pub unsafe fn back(&mut self) -> *mut libc::c_char {
        return ((self.vec_).last_mut().unwrap());
    }
    pub unsafe fn pop_back(&mut self) {
        self.vec_.pop();
        return;
    }
    pub unsafe fn push_back(&mut self, item: *const libc::c_char) {
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
    assert!((unsafe { MyContainer_int_::empty(&imc,) }));
    (unsafe {
        let mut _item: i32 = 1;
        MyContainer_int_::push_back(&mut imc, &mut _item)
    });
    assert!(
        ((unsafe { MyContainer_int_::size(&imc,) }) == (1_usize))
            && ((*(unsafe { MyContainer_int_::back(&mut imc,) })) == (1))
    );
    (unsafe { MyContainer_int_::pop_back(&mut imc) });
    assert!((unsafe { MyContainer_int_::empty(&imc,) }));
    let mut cmc: MyContainer_char_ = <MyContainer_char_>::default();
    assert!((unsafe { MyContainer_char_::empty(&cmc,) }));
    (unsafe {
        let mut _item: libc::c_char = ('a' as libc::c_char);
        MyContainer_char_::push_back(&mut cmc, &mut _item)
    });
    assert!(
        ((unsafe { MyContainer_char_::size(&cmc,) }) == (1_usize))
            && (((*(unsafe { MyContainer_char_::back(&mut cmc,) })) as i32)
                == (('a' as libc::c_char) as i32))
    );
    (unsafe { MyContainer_char_::pop_back(&mut cmc) });
    assert!((unsafe { MyContainer_char_::empty(&cmc,) }));
    let mut fmc: MyContainer_float_ = <MyContainer_float_>::default();
    assert!((unsafe { MyContainer_float_::empty(&fmc,) }));
    (unsafe {
        let mut _item: f32 = (1.0E+0 as f32);
        MyContainer_float_::push_back(&mut fmc, &mut _item)
    });
    assert!(
        ((unsafe { MyContainer_float_::size(&fmc,) }) == (1_usize))
            && (((*(unsafe { MyContainer_float_::back(&mut fmc,) })) as f64) == (1.0E+0))
    );
    (unsafe { MyContainer_float_::pop_back(&mut fmc) });
    assert!((unsafe { MyContainer_float_::empty(&fmc,) }));
    return 0;
}
