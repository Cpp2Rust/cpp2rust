extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut copyCount_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CppFromUnsafe, CopyAssignUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct HasConstexprCtor {
    pub m: i32,
}
impl HasConstexprCtor {
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_0)) = 0;
    }
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { m: v };
        this
    }
    pub unsafe fn copy_from(o: *const HasConstexprCtor) -> Self {
        let mut this = Self { m: (*o).m };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_0)).prefix_inc();
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const HasConstexprCtor) -> *mut HasConstexprCtor {
        self.m = (*_a0).m;
        return &mut (*(self as *mut HasConstexprCtor));
    }
}
impl Clone for HasConstexprCtor {
    fn clone(&self) -> Self {
        unsafe { HasConstexprCtor::copy_from(self as *const HasConstexprCtor) }
    }
}
pub static mut ctorCount_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut copyCount_2: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut dtorCount_3: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut useCount_4: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct LifetimeTracked {
    pub uses: i32,
}
impl LifetimeTracked {
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_1)) = {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_2)) = {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_3)) = {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut useCount_4)) = 0;
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut useCount_4))
                };
                (*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_3))
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_2))
        };
    }
    pub unsafe fn new() -> Self {
        let mut this = Self { uses: 0 };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_1)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(_a0: *const LifetimeTracked) -> Self {
        let mut this = Self { uses: 0 };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_2)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_3)).prefix_inc();
    }
    pub unsafe fn use_(&self) {
        (*(&raw const self.uses).cast_mut()).prefix_inc();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut useCount_4)).prefix_inc();
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const LifetimeTracked) -> *mut LifetimeTracked {
        self.uses = (*_a0).uses;
        return &mut (*(self as *mut LifetimeTracked));
    }
}
impl Clone for LifetimeTracked {
    fn clone(&self) -> Self {
        unsafe { LifetimeTracked::copy_from(self as *const LifetimeTracked) }
    }
}
impl Default for LifetimeTracked {
    fn default() -> Self {
        unsafe { LifetimeTracked::new() }
    }
}
pub unsafe fn test_5() {
    (unsafe { HasConstexprCtor::reset() });
    let arr: [HasConstexprCtor; 2] = [HasConstexprCtor::new({ 3 }), HasConstexprCtor::new({ 4 })];
    let mut f: FnPtr<fn() -> i32> = FnPtr::<fn() -> i32>::new(|| -> i32 {
        unsafe {
            return [HasConstexprCtor::new({ 3 }), HasConstexprCtor::new({ 4 })][(1) as usize].m;
        }
    });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_0)) == (0)));
    assert!(((unsafe { f.call() }) == (4)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_0)) == (0)));
}
pub unsafe fn test_6() {
    (unsafe { HasConstexprCtor::reset() });
    let k: HasConstexprCtor = HasConstexprCtor::new({ 5 });
    let mut f: FnPtr<fn() -> i32> = FnPtr::<fn() -> i32>::new(|| -> i32 {
        unsafe {
            return HasConstexprCtor::new({ 5 }).m;
        }
    });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_0)) == (0)));
    assert!(((unsafe { f.call() }) == (5)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_0)) == (0)));
}
pub unsafe fn test_7() {
    (unsafe { HasConstexprCtor::reset() });
    let k1: HasConstexprCtor = HasConstexprCtor::new({ 7 });
    let k2: HasConstexprCtor = HasConstexprCtor::new({ 9 });
    let mut f: FnPtr<fn(bool) -> i32> = FnPtr::<fn(bool) -> i32>::new(|mut b: bool| -> i32 {
        unsafe {
            return (if b {
                HasConstexprCtor::new({ 7 })
            } else {
                HasConstexprCtor::new({ 9 })
            })
            .m;
        }
    });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_0)) == (0)));
    assert!(((unsafe { f.call(true,) }) == (7)));
    assert!(((unsafe { f.call(false,) }) == (9)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_0)) == (0)));
}
pub static mut left_8: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub unsafe fn test_9() {
    (unsafe { HasConstexprCtor::reset() });
    let k: HasConstexprCtor = HasConstexprCtor::new({ 5 });
    let mut f: FnPtr<fn() -> i32> = FnPtr::<fn() -> i32>::new(|| -> i32 {
        unsafe {
            return {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut left_8)).prefix_inc();
                HasConstexprCtor::new({ 5 })
            }
            .m;
        }
    });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_0)) == (0)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut left_8)) == (0)));
    assert!(((unsafe { f.call() }) == (5)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut left_8)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_0)) == (0)));
}
pub static mut derived_calls_10: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub unsafe trait p4_1_pure_virtual_without_definition_HasPureVirtual {
    unsafe fn f(&mut self);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p4_1_pure_virtual_without_definition_OverridesPureVirtual {}
pub unsafe fn test_11() {
    let mut d: p4_1_pure_virtual_without_definition_OverridesPureVirtual =
        <p4_1_pure_virtual_without_definition_OverridesPureVirtual>::default();
    let mut p: *mut dyn p4_1_pure_virtual_without_definition_HasPureVirtual =
        (&mut d as *mut p4_1_pure_virtual_without_definition_OverridesPureVirtual);
    (unsafe { (*p).f() });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_calls_10)) == (1)));
}
pub static mut global_12: std::cell::LazyCell<LifetimeTracked> =
    std::cell::LazyCell::new(|| unsafe {
        {
            let __static = LifetimeTracked::new();
            libcc2rs::register_static_destructor(|| {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut global_12)).destructor()
            });
            __static
        }
    });
pub unsafe fn test_13() {
    (unsafe { LifetimeTracked::reset() });
    let r: *mut LifetimeTracked = &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut global_12));
    let mut f: FnPtr<fn()> = FnPtr::<fn()>::new(|| unsafe {
        (unsafe {
            LifetimeTracked::use_(&(*std::cell::LazyCell::force_mut(&mut *&raw mut global_12)))
        });
    });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_2)) == (0)));
    (unsafe { f.call() });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_2)) == (0)));
    assert!((((*std::cell::LazyCell::force_mut(&mut *&raw mut global_12)).uses) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_1)) == (0)));
}
pub unsafe fn test_14() {
    (unsafe { LifetimeTracked::reset() });
    let mut t: LifetimeTracked = LifetimeTracked::new();
    let _dtor_t = std::mem::DropGuard::new(&raw mut t, |__p| unsafe { (*__p).destructor() });
    let r: *mut LifetimeTracked = &mut t;
    let mut f: FnPtr<fn()> = lambda_unsafe!(
        {
            let r: LifetimeTracked = LifetimeTracked::copy_from({ r });
        },
        || {
            (unsafe { LifetimeTracked::use_(&r) });
        }
    );
    let _dtor_f = std::mem::DropGuard::new(&raw mut f, |__p| unsafe { (*__p).destroy() });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_2)) == (1)));
    (unsafe { f.call() });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_2)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut useCount_4)) == (1)));
    assert!(((t.uses) == (0)));
}
pub unsafe fn test_15() {
    let n: i32 = 5;
    let mut f: FnPtr<fn() -> *const i32> = lambda_unsafe!(
        {
            let n: i32 = n;
        },
        || -> *const i32 {
            return (&n as *const i32);
        }
    );
    assert!(((unsafe { f.call() }) != (&n as *const i32)));
    assert!(((*(unsafe { f.call() })) == (5)));
}
pub unsafe fn test_16() {
    (unsafe { HasConstexprCtor::reset() });
    let k: HasConstexprCtor = HasConstexprCtor::new({ 5 });
    let mut f: FnPtr<fn() -> i32> = lambda_unsafe!(
        {
            let k: HasConstexprCtor = HasConstexprCtor::copy_from({ &k });
        },
        || -> i32 {
            let mut c: HasConstexprCtor = HasConstexprCtor::copy_from({ &k });
            return c.m;
        }
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_0)) == (1)));
    assert!(((unsafe { f.call() }) == (5)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_0)) == (2)));
}
pub unsafe fn test_17() {
    let n: i32 = 5;
    let mut f: FnPtr<fn() -> i32> = FnPtr::<fn() -> i32>::new(|| -> i32 {
        unsafe {
            return 5;
        }
    });
    assert!(((unsafe { f.call() }) == (5)));
}
pub static mut copies_18: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CppFromUnsafe, CopyAssignUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct p5_2_mutable_subobject_is_odr_used_HasMutableMember {
    pub value: i32,
    pub scratch: i32,
}
impl p5_2_mutable_subobject_is_odr_used_HasMutableMember {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self {
            value: v,
            scratch: 0,
        };
        this
    }
    pub unsafe fn copy_from(o: *const p5_2_mutable_subobject_is_odr_used_HasMutableMember) -> Self {
        let mut this = Self {
            value: (*o).value,
            scratch: (*o).scratch,
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_18)).prefix_inc();
        this
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const p5_2_mutable_subobject_is_odr_used_HasMutableMember,
    ) -> *mut p5_2_mutable_subobject_is_odr_used_HasMutableMember {
        self.value = (*_a0).value;
        self.scratch = (*_a0).scratch;
        return &mut (*(self as *mut p5_2_mutable_subobject_is_odr_used_HasMutableMember));
    }
}
impl Clone for p5_2_mutable_subobject_is_odr_used_HasMutableMember {
    fn clone(&self) -> Self {
        unsafe {
            p5_2_mutable_subobject_is_odr_used_HasMutableMember::copy_from(
                self as *const p5_2_mutable_subobject_is_odr_used_HasMutableMember,
            )
        }
    }
}
pub unsafe fn test_19() {
    let k: p5_2_mutable_subobject_is_odr_used_HasMutableMember =
        p5_2_mutable_subobject_is_odr_used_HasMutableMember::new({ 5 });
    let mut f: FnPtr<fn() -> i32> = lambda_unsafe!(
        {
            let k: p5_2_mutable_subobject_is_odr_used_HasMutableMember =
                p5_2_mutable_subobject_is_odr_used_HasMutableMember::copy_from({ &k });
        },
        || -> i32 {
            return k.value;
        }
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_18)) == (1)));
    assert!(((unsafe { f.call() }) == (5)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_18)) == (1)));
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe, Default)]
pub struct p6_unevaluated_binding_needs_no_capture_HasTwoMembers {
    pub a: LifetimeTracked,
    pub b: LifetimeTracked,
}
impl p6_unevaluated_binding_needs_no_capture_HasTwoMembers {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const p6_unevaluated_binding_needs_no_capture_HasTwoMembers,
    ) -> *mut p6_unevaluated_binding_needs_no_capture_HasTwoMembers {
        (unsafe {
            let _arg0: *const LifetimeTracked = &(*_a0).a;
            LifetimeTracked::copy_assign(&mut self.a, _arg0)
        });
        (unsafe {
            let _arg0: *const LifetimeTracked = &(*_a0).b;
            LifetimeTracked::copy_assign(&mut self.b, _arg0)
        });
        return &mut (*(self as *mut p6_unevaluated_binding_needs_no_capture_HasTwoMembers));
    }
}
impl p6_unevaluated_binding_needs_no_capture_HasTwoMembers {
    pub unsafe fn destructor(&mut self) {
        LifetimeTracked::destructor(&mut self.b);
        LifetimeTracked::destructor(&mut self.a);
    }
}
pub unsafe fn test_20() {
    (unsafe { LifetimeTracked::reset() });
    let mut p: p6_unevaluated_binding_needs_no_capture_HasTwoMembers =
        p6_unevaluated_binding_needs_no_capture_HasTwoMembers {
            a: LifetimeTracked::new(),
            b: LifetimeTracked::new(),
        };
    let _dtor_p = std::mem::DropGuard::new(&raw mut p, |__p| unsafe { (*__p).destructor() });
    let __decomp_21: *mut p6_unevaluated_binding_needs_no_capture_HasTwoMembers = &mut p;
    let mut f: FnPtr<fn() -> usize> = FnPtr::<fn() -> usize>::new(|| -> usize {
        unsafe {
            return ::std::mem::size_of::<LifetimeTracked>();
        }
    });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_2)) == (0)));
    assert!(((unsafe { f.call() }) == (::std::mem::size_of::<LifetimeTracked>())));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_2)) == (0)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut useCount_4)) == (0)));
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct p7_member_access_captures_this_HasMemberUsedInLambda {
    pub x: LifetimeTracked,
    pub n: i32,
}
impl p7_member_access_captures_this_HasMemberUsedInLambda {
    pub unsafe fn makeLambda(&mut self) -> FnPtr<fn() -> i32> {
        return lambda_unsafe!(
            {
                let this_: *mut p7_member_access_captures_this_HasMemberUsedInLambda =
                    (self as *mut p7_member_access_captures_this_HasMemberUsedInLambda);
            },
            || -> i32 {
                (unsafe { LifetimeTracked::use_(&(*this_).x) });
                return (*this_).n;
            }
        );
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const p7_member_access_captures_this_HasMemberUsedInLambda,
    ) -> *mut p7_member_access_captures_this_HasMemberUsedInLambda {
        (unsafe {
            let _arg0: *const LifetimeTracked = &(*_a0).x;
            LifetimeTracked::copy_assign(&mut self.x, _arg0)
        });
        self.n = (*_a0).n;
        return &mut (*(self as *mut p7_member_access_captures_this_HasMemberUsedInLambda));
    }
}
impl p7_member_access_captures_this_HasMemberUsedInLambda {
    pub unsafe fn destructor(&mut self) {
        LifetimeTracked::destructor(&mut self.x);
    }
}
impl Default for p7_member_access_captures_this_HasMemberUsedInLambda {
    fn default() -> Self {
        unsafe {
            p7_member_access_captures_this_HasMemberUsedInLambda {
                x: <LifetimeTracked>::default(),
                n: 1,
            }
        }
    }
}
pub unsafe fn test_22() {
    (unsafe { LifetimeTracked::reset() });
    let mut c: p7_member_access_captures_this_HasMemberUsedInLambda =
        <p7_member_access_captures_this_HasMemberUsedInLambda>::default();
    let _dtor_c = std::mem::DropGuard::new(&raw mut c, |__p| unsafe { (*__p).destructor() });
    let mut f: FnPtr<fn() -> i32> =
        (unsafe { p7_member_access_captures_this_HasMemberUsedInLambda::makeLambda(&mut c) });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_2)) == (0)));
    assert!(((unsafe { f.call() }) == (1)));
    assert!(((c.x.uses) == (1)));
    c.n = 10;
    assert!(((unsafe { f.call() }) == (10)));
    assert!(((c.x.uses) == (2)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_2)) == (0)));
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe, Default)]
pub struct p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda {
    pub x: LifetimeTracked,
}
impl p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda {
    pub unsafe fn sizer(&mut self) -> FnPtr<fn() -> usize> {
        return FnPtr::<fn() -> usize>::new(|| -> usize {
            unsafe {
                return ::std::mem::size_of::<LifetimeTracked>();
            }
        });
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda,
    ) -> *mut p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda {
        (unsafe {
            let _arg0: *const LifetimeTracked = &(*_a0).x;
            LifetimeTracked::copy_assign(&mut self.x, _arg0)
        });
        return &mut (*(self as *mut p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda));
    }
}
impl p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda {
    pub unsafe fn destructor(&mut self) {
        LifetimeTracked::destructor(&mut self.x);
    }
}
pub unsafe fn test_23() {
    (unsafe { LifetimeTracked::reset() });
    let mut c: p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda =
        <p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda>::default();
    let _dtor_c = std::mem::DropGuard::new(&raw mut c, |__p| unsafe { (*__p).destructor() });
    let mut f: FnPtr<fn() -> usize> =
        (unsafe { p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda::sizer(&mut c) });
    assert!(((unsafe { f.call() }) == (::std::mem::size_of::<LifetimeTracked>())));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut copyCount_2)) == (0)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut useCount_4)) == (0)));
}
pub static mut noisy_calls_24: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub unsafe fn noisy_25() -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut noisy_calls_24)).prefix_inc();
    return 1;
}
pub unsafe fn test_26() {
    (unsafe { LifetimeTracked::reset() });
    assert!(
        ((::std::mem::size_of::<LifetimeTracked>()) == (::std::mem::size_of::<LifetimeTracked>()))
    );
    assert!(((::std::mem::size_of::<i32>()) == (::std::mem::size_of::<i32>())));
    assert!(!(false));
    assert!(!(false));
    let mut n: i32 = 3;
    assert!(((n) == (3)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut noisy_calls_24)) == (0)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_1)) == (0)));
    assert!(((unsafe { noisy_25() }) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut noisy_calls_24)) == (1)));
}
pub static mut t_ctors_27: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p9_destructor_not_potentially_invoked_HasUndefinedDestructor {}
impl p9_destructor_not_potentially_invoked_HasUndefinedDestructor {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut t_ctors_27)).prefix_inc();
        this
    }
}
impl Default for p9_destructor_not_potentially_invoked_HasUndefinedDestructor {
    fn default() -> Self {
        unsafe { p9_destructor_not_potentially_invoked_HasUndefinedDestructor::new() }
    }
}
pub unsafe fn test_28() {
    let mut p: *mut p9_destructor_not_potentially_invoked_HasUndefinedDestructor = (Box::leak(
        Box::new(p9_destructor_not_potentially_invoked_HasUndefinedDestructor::new()),
    )
        as *mut p9_destructor_not_potentially_invoked_HasUndefinedDestructor);
    assert!(!((p).is_null()));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut t_ctors_27)) == (1)));
}
pub static mut ctor_calls_29: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(FnPtrArg)]
pub struct p9_unused_copy_constructor_HasUndefinedCopyCtor {
    pub value: i32,
}
impl p9_unused_copy_constructor_HasUndefinedCopyCtor {
    pub unsafe fn new() -> Self {
        let mut this = Self { value: 3 };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut ctor_calls_29)).prefix_inc();
        this
    }
}
impl Default for p9_unused_copy_constructor_HasUndefinedCopyCtor {
    fn default() -> Self {
        unsafe { p9_unused_copy_constructor_HasUndefinedCopyCtor::new() }
    }
}
pub unsafe fn read_30(object: *const p9_unused_copy_constructor_HasUndefinedCopyCtor) -> i32 {
    return (*object).value;
}
pub unsafe fn test_31() {
    let mut a: p9_unused_copy_constructor_HasUndefinedCopyCtor =
        p9_unused_copy_constructor_HasUndefinedCopyCtor::new();
    assert!(((unsafe { read_30(&a,) }) == (3)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut ctor_calls_29)) == (1)));
}
pub static mut m_ctors_32: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p9_unused_implicit_assignment_HasUndefinedCopyAssignment {}
impl p9_unused_implicit_assignment_HasUndefinedCopyAssignment {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut m_ctors_32)).prefix_inc();
        this
    }
}
impl Default for p9_unused_implicit_assignment_HasUndefinedCopyAssignment {
    fn default() -> Self {
        unsafe { p9_unused_implicit_assignment_HasUndefinedCopyAssignment::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p9_unused_implicit_assignment_HasMemberWithUndefinedCopyAssignment {
    pub m: p9_unused_implicit_assignment_HasUndefinedCopyAssignment,
}
pub unsafe fn test_33() {
    let mut a: p9_unused_implicit_assignment_HasMemberWithUndefinedCopyAssignment =
        <p9_unused_implicit_assignment_HasMemberWithUndefinedCopyAssignment>::default();
    let mut b: p9_unused_implicit_assignment_HasMemberWithUndefinedCopyAssignment =
        <p9_unused_implicit_assignment_HasMemberWithUndefinedCopyAssignment>::default();
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut m_ctors_32)) == (2)));
}
pub static mut else_runs_34: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub unsafe fn g_35() {
    {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut else_runs_34)).prefix_inc();
    }
}
pub unsafe fn test_36() {
    (unsafe { LifetimeTracked::reset() });
    (unsafe { g_35() });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut else_runs_34)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_1)) == (0)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut useCount_4)) == (0)));
}
pub static mut else_runs_37: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub unsafe fn g_38(mut _a0: i32) {
    {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut else_runs_37)).prefix_inc();
    }
}
pub unsafe fn g_39(mut _a0: libc::c_char) {
    {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut else_runs_37)).prefix_inc();
    }
}
pub unsafe fn test_40() {
    (unsafe { LifetimeTracked::reset() });
    (unsafe { g_38(1) });
    (unsafe { g_39(('c' as libc::c_char)) });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut else_runs_37)) == (2)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_1)) == (0)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut useCount_4)) == (0)));
}
pub static mut else_runs_41: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub unsafe fn g_43() {
    {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut else_runs_41)).prefix_inc();
    }
}
pub unsafe fn test_44() {
    (unsafe { LifetimeTracked::reset() });
    (unsafe { g_43() });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut else_runs_41)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_1)) == (0)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut useCount_4)) == (0)));
}
pub static mut g_calls_45: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub unsafe fn g_46() {
    {}
    (*std::cell::LazyCell::force_mut(&mut *&raw mut g_calls_45)).prefix_inc();
}
pub unsafe fn test_47() {
    (unsafe { g_46() });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut g_calls_45)) == (1)));
}
pub static mut x1_48: std::cell::LazyCell<*mut p13_pointer_to_incomplete_class_X> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null_mut() });
pub static mut x2_49: std::cell::LazyCell<*mut p13_pointer_to_incomplete_class_X> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null_mut() });
pub unsafe fn identity_50(
    mut p: *mut p13_pointer_to_incomplete_class_X,
) -> *mut p13_pointer_to_incomplete_class_X {
    return p;
}
pub unsafe fn test_51() {
    assert!((*std::cell::LazyCell::force_mut(&mut *&raw mut x1_48)).is_null());
    assert!(
        ((unsafe { identity_50((*std::cell::LazyCell::force_mut(&mut *&raw mut x2_49)),) })
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut x1_48)))
    );
    let mut pp: *mut *mut p13_pointer_to_incomplete_class_X =
        (&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut x1_48))
            as *mut *mut p13_pointer_to_incomplete_class_X);
    assert!((*pp).is_null());
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_5() });
    (unsafe { test_6() });
    (unsafe { test_7() });
    (unsafe { test_9() });
    (unsafe { test_11() });
    (unsafe { test_13() });
    (unsafe { test_14() });
    (unsafe { test_15() });
    (unsafe { test_16() });
    (unsafe { test_17() });
    (unsafe { test_19() });
    (unsafe { test_20() });
    (unsafe { test_22() });
    (unsafe { test_23() });
    (unsafe { test_26() });
    (unsafe { test_28() });
    (unsafe { test_31() });
    (unsafe { test_33() });
    (unsafe { test_36() });
    (unsafe { test_40() });
    (unsafe { test_44() });
    (unsafe { test_47() });
    (unsafe { test_51() });
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct p13_pointer_to_incomplete_class_X;
unsafe impl p4_1_pure_virtual_without_definition_HasPureVirtual
    for p4_1_pure_virtual_without_definition_OverridesPureVirtual
{
    unsafe fn f(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut derived_calls_10)).prefix_inc();
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copyCount_0);
    std::cell::LazyCell::force(&*&raw const ctorCount_1);
    std::cell::LazyCell::force(&*&raw const copyCount_2);
    std::cell::LazyCell::force(&*&raw const dtorCount_3);
    std::cell::LazyCell::force(&*&raw const useCount_4);
    std::cell::LazyCell::force(&*&raw const left_8);
    std::cell::LazyCell::force(&*&raw const derived_calls_10);
    std::cell::LazyCell::force(&*&raw const global_12);
    std::cell::LazyCell::force(&*&raw const copies_18);
    std::cell::LazyCell::force(&*&raw const noisy_calls_24);
    std::cell::LazyCell::force(&*&raw const t_ctors_27);
    std::cell::LazyCell::force(&*&raw const ctor_calls_29);
    std::cell::LazyCell::force(&*&raw const m_ctors_32);
    std::cell::LazyCell::force(&*&raw const else_runs_34);
    std::cell::LazyCell::force(&*&raw const else_runs_37);
    std::cell::LazyCell::force(&*&raw const else_runs_41);
    std::cell::LazyCell::force(&*&raw const g_calls_45);
    std::cell::LazyCell::force(&*&raw const x1_48);
    std::cell::LazyCell::force(&*&raw const x2_49);
}
