extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut copies_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe, MoveAssignUnsafe)]
pub struct Tracked {
    pub v: i32,
}
impl Tracked {
    pub unsafe fn new_1() -> Self {
        let mut this = Self { v: 0 };
        this
    }
    pub unsafe fn new_2(mut v: i32) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn copy_from(o: *const Tracked) -> Self {
        let mut this = Self { v: (*o).v };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
        this
    }
    pub unsafe fn move_from(o: *mut Tracked) -> Self {
        let mut this = Self { v: (*o).v };
        this
    }
    pub unsafe fn copy_assign(&mut self, o: *const Tracked) -> *mut Tracked {
        self.v = (*o).v;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
        return &mut (*(self as *mut Tracked));
    }
    pub unsafe fn move_assign(&mut self, o: *mut Tracked) -> *mut Tracked {
        self.v = (*o).v;
        return &mut (*(self as *mut Tracked));
    }
}
impl std::cmp::Ord for Tracked {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_1(self as *const Tracked, other as *const Tracked) {
                std::cmp::Ordering::Less
            } else if operator_lt_1(other as *const Tracked, self as *const Tracked) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Tracked {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Tracked {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(operator_lt_1(self as *const Tracked, other as *const Tracked))
                && !(operator_lt_1(other as *const Tracked, self as *const Tracked))
        }
    }
}
impl std::cmp::Eq for Tracked {}
impl Clone for Tracked {
    fn clone(&self) -> Self {
        unsafe { Tracked::copy_from(self as *const Tracked) }
    }
}
impl Default for Tracked {
    fn default() -> Self {
        unsafe { Tracked::new_1() }
    }
}
pub unsafe fn operator_lt_1(a: *const Tracked, b: *const Tracked) -> bool {
    return (((*a).v) < ((*b).v));
}
pub unsafe fn greater_2(a: *const Tracked, b: *const Tracked) -> bool {
    return (((*a).v) > ((*b).v));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut arr: [Tracked; 5] = [
        Tracked::new_2({ 3 }),
        Tracked::new_2({ 1 }),
        Tracked::new_2({ 4 }),
        Tracked::new_2({ 1 }),
        Tracked::new_2({ 5 }),
    ];
    {
        let __n = arr
            .as_mut_ptr()
            .offset((5) as isize)
            .iter_distance(&arr.as_mut_ptr());
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| {
            (*arr.as_mut_ptr().iter_at(__x)).cmp(&*arr.as_mut_ptr().iter_at(__y))
        });
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| std::ptr::read(arr.as_mut_ptr().iter_at(__i)))
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            std::ptr::write(arr.as_mut_ptr().iter_at(__i), __v);
        }
    };
    assert!(
        (((((arr[(0) as usize].v) == (1)) && ((arr[(1) as usize].v) == (1)))
            && ((arr[(2) as usize].v) == (3)))
            && ((arr[(3) as usize].v) == (4)))
            && ((arr[(4) as usize].v) == (5))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    let mut vec_: Vec<Tracked> = Vec::new();
    if 4_usize as usize > vec_.capacity() as usize {
        let len_0 = vec_.len();
        vec_.reserve_exact(4_usize as usize - len_0 as usize);
    };
    {
        let __init = Tracked::new_2({ 2 });
        vec_.push(__init)
    };
    {
        let __init = Tracked::new_2({ 7 });
        vec_.push(__init)
    };
    {
        let __init = Tracked::new_2({ 1 });
        vec_.push(__init)
    };
    {
        let __init = Tracked::new_2({ 8 });
        vec_.push(__init)
    };
    {
        let __n = vec_.end().iter_distance(&vec_.begin());
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| (*vec_.begin().iter_at(__x)).cmp(&*vec_.begin().iter_at(__y)));
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| std::ptr::read(vec_.begin().iter_at(__i)))
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            std::ptr::write(vec_.begin().iter_at(__i), __v);
        }
    };
    assert!(
        ((((vec_[(0_usize)].v) == (1)) && ((vec_[(1_usize)].v) == (2)))
            && ((vec_[(2_usize)].v) == (7)))
            && ((vec_[(3_usize)].v) == (8))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    {
        let __n = vec_.end().iter_distance(&vec_.begin());
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| {
            let __a = vec_.begin().iter_at(__x) as *const _;
            let __b = vec_.begin().iter_at(__y) as *const _;
            if (greater_2 as unsafe fn(*const Tracked, *const Tracked) -> bool).call(__a, __b) {
                std::cmp::Ordering::Less
            } else if (greater_2 as unsafe fn(*const Tracked, *const Tracked) -> bool)
                .call(__b, __a)
            {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| std::ptr::read(vec_.begin().iter_at(__i)))
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            std::ptr::write(vec_.begin().iter_at(__i), __v);
        }
    };
    assert!(((vec_[(0_usize)].v) == (8)) && ((vec_[(3_usize)].v) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    {
        let __n = vec_.end().iter_distance(&vec_.begin());
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| {
            let __a = vec_.begin().iter_at(__x) as *const _;
            let __b = vec_.begin().iter_at(__y) as *const _;
            if FnPtr::<fn(*const Tracked, *const Tracked) -> bool>::new(
                |a: *const Tracked, b: *const Tracked| -> bool {
                    unsafe {
                        return (((*a).v) < ((*b).v));
                    }
                },
            )
            .call(__a, __b)
            {
                std::cmp::Ordering::Less
            } else if FnPtr::<fn(*const Tracked, *const Tracked) -> bool>::new(
                |a: *const Tracked, b: *const Tracked| -> bool {
                    unsafe {
                        return (((*a).v) < ((*b).v));
                    }
                },
            )
            .call(__b, __a)
            {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| std::ptr::read(vec_.begin().iter_at(__i)))
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            std::ptr::write(vec_.begin().iter_at(__i), __v);
        }
    };
    assert!(((vec_[(0_usize)].v) == (1)) && ((vec_[(3_usize)].v) == (8)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    {
        let __n = vec_.end().iter_distance(&vec_.begin().iter_add(1_isize));
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| {
            let __a = vec_.begin().iter_add(1_isize).iter_at(__x) as *const _;
            let __b = vec_.begin().iter_add(1_isize).iter_at(__y) as *const _;
            if (greater_2 as unsafe fn(*const Tracked, *const Tracked) -> bool).call(__a, __b) {
                std::cmp::Ordering::Less
            } else if (greater_2 as unsafe fn(*const Tracked, *const Tracked) -> bool)
                .call(__b, __a)
            {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| std::ptr::read(vec_.begin().iter_add(1_isize).iter_at(__i)))
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            std::ptr::write(vec_.begin().iter_add(1_isize).iter_at(__i), __v);
        }
    };
    assert!(
        (((vec_[(0_usize)].v) == (1)) && ((vec_[(1_usize)].v) == (8)))
            && ((vec_[(3_usize)].v) == (2))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (0)));
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0));
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copies_0);
}
