extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static copyCount_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CppFrom, CopyAssign, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct HasConstexprCtor {
    #[offset(0)]
    pub m: i32,
}
impl HasConstexprCtor {
    pub fn reset() {
        copyCount_0.with(|rc| *rc.borrow_mut() = 0);
    }
    pub fn new(mut v: i32) -> Self {
        Self { m: v }
    }
    pub fn copy_from(o: Ptr<HasConstexprCtor>) -> Self {
        let __this: HasConstexprCtor = Self {
            m: o.with(|__s| __s.m),
        };
        (*copyCount_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for HasConstexprCtor {
    fn clone(&self) -> Self {
        let __src: Value<HasConstexprCtor> =
            Rc::new(RefCell::new(HasConstexprCtor { m: self.m.clone() }));
        HasConstexprCtor::copy_from(__src.as_pointer())
    }
}
thread_local!(
    pub static ctorCount_1: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static copyCount_2: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static dtorCount_3: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static useCount_4: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(4)]
pub struct LifetimeTracked {
    #[offset(0)]
    pub uses: i32,
}
impl LifetimeTracked {
    pub fn reset() {
        ctorCount_1.with(|rc| {
            *rc.borrow_mut() = {
                copyCount_2.with(|rc| {
                    *rc.borrow_mut() = {
                        dtorCount_3.with(|rc| {
                            *rc.borrow_mut() = {
                                useCount_4.with(|rc| *rc.borrow_mut() = 0);
                                useCount_4.with(|rc| *rc.borrow())
                            }
                        });
                        dtorCount_3.with(|rc| *rc.borrow())
                    }
                });
                copyCount_2.with(|rc| *rc.borrow())
            }
        });
    }
    pub fn new() -> Self {
        let __this: LifetimeTracked = Self { uses: 0 };
        (*ctorCount_1.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(_a0: Ptr<LifetimeTracked>) -> Self {
        let __this: LifetimeTracked = Self { uses: 0 };
        (*copyCount_2.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for LifetimeTracked {
    fn clone(&self) -> Self {
        let __src: Value<LifetimeTracked> = Rc::new(RefCell::new(LifetimeTracked {
            uses: self.uses.clone(),
        }));
        LifetimeTracked::copy_from(__src.as_pointer())
    }
}
impl Default for LifetimeTracked {
    fn default() -> Self {
        { LifetimeTracked::new() }
    }
}
pub fn test_5() {
    ({ HasConstexprCtor::reset() });
    let mut arr: [HasConstexprCtor; 2] =
        [HasConstexprCtor::new({ 3 }), HasConstexprCtor::new({ 4 })];
    let f: Value<FnPtr<fn() -> i32>> =
        Rc::new(RefCell::new(FnPtr::<fn() -> i32>::new(|| -> i32 {
            {
                return [HasConstexprCtor::new({ 3 }), HasConstexprCtor::new({ 4 })][(1) as usize]
                    .m;
            }
        })));
    assert!((copyCount_0.with(|rc| *rc.borrow()) == 0));
    assert!(({ (({ (*f.borrow()).call() }) == 4) }));
    assert!((copyCount_0.with(|rc| *rc.borrow()) == 0));
}
pub fn test_6() {
    ({ HasConstexprCtor::reset() });
    let mut k: HasConstexprCtor = HasConstexprCtor::new({ 5 });
    let f: Value<FnPtr<fn() -> i32>> =
        Rc::new(RefCell::new(FnPtr::<fn() -> i32>::new(|| -> i32 {
            {
                return HasConstexprCtor::new({ 5 }).m;
            }
        })));
    assert!((copyCount_0.with(|rc| *rc.borrow()) == 0));
    assert!(({ (({ (*f.borrow()).call() }) == 5) }));
    assert!((copyCount_0.with(|rc| *rc.borrow()) == 0));
}
pub fn test_7() {
    ({ HasConstexprCtor::reset() });
    let mut k1: HasConstexprCtor = HasConstexprCtor::new({ 7 });
    let mut k2: HasConstexprCtor = HasConstexprCtor::new({ 9 });
    let f: Value<FnPtr<fn(bool) -> i32>> = Rc::new(RefCell::new(FnPtr::<fn(bool) -> i32>::new(
        |mut b: bool| -> i32 {
            {
                return {
                    (if b {
                        HasConstexprCtor::new({ 7 })
                    } else {
                        HasConstexprCtor::new({ 9 })
                    })
                    .m
                };
            }
        },
    )));
    assert!((copyCount_0.with(|rc| *rc.borrow()) == 0));
    assert!(({ (({ (*f.borrow()).call(true,) }) == 7) }));
    assert!(({ (({ (*f.borrow()).call(false,) }) == 9) }));
    assert!((copyCount_0.with(|rc| *rc.borrow()) == 0));
}
thread_local!(
    pub static left_8: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn test_9() {
    ({ HasConstexprCtor::reset() });
    let k: Value<HasConstexprCtor> = Rc::new(RefCell::new(HasConstexprCtor::new({ 5 })));
    let f: Value<FnPtr<fn() -> i32>> =
        Rc::new(RefCell::new(FnPtr::<fn() -> i32>::new(|| -> i32 {
            {
                return {
                    {
                        (*left_8.with(Value::clone).borrow_mut()).prefix_inc();
                        HasConstexprCtor::new({ 5 })
                    }
                    .m
                };
            }
        })));
    assert!((copyCount_0.with(|rc| *rc.borrow()) == 0));
    assert!((left_8.with(|rc| *rc.borrow()) == 0));
    assert!(({ (({ (*f.borrow()).call() }) == 5) }));
    assert!((left_8.with(|rc| *rc.borrow()) == 1));
    assert!((copyCount_0.with(|rc| *rc.borrow()) == 0));
}
thread_local!(
    pub static derived_calls_10: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub trait p4_1_pure_virtual_without_definition_HasPureVirtual {
    fn f(&mut self);
}
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct p4_1_pure_virtual_without_definition_OverridesPureVirtual {}
pub fn test_11() {
    let d: Value<p4_1_pure_virtual_without_definition_OverridesPureVirtual> = Rc::new(
        RefCell::new(<p4_1_pure_virtual_without_definition_OverridesPureVirtual>::default()),
    );
    let mut p: PtrDyn<dyn p4_1_pure_virtual_without_definition_HasPureVirtual> =
        (d.as_pointer()).to_dyn::<dyn p4_1_pure_virtual_without_definition_HasPureVirtual>(|w| w);
    ({ (*p.upgrade().deref_mut()).f() });
    assert!((derived_calls_10.with(|rc| *rc.borrow()) == 1));
}
thread_local!(
    pub static global_12: Value<LifetimeTracked> = {
        let __static = Rc::new(RefCell::new(LifetimeTracked::new()));
        libcc2rs::register_static_destructor(|| {
            global_12.with(Value::clone).as_pointer().destructor()
        });
        __static
    };
);
pub fn test_13() {
    ({ LifetimeTracked::reset() });
    let r: Ptr<LifetimeTracked> = global_12.with(|v| v.as_pointer());
    let f: Value<FnPtr<fn()>> = Rc::new(RefCell::new(FnPtr::<fn()>::new(|| {
        ({ LifetimeTrackedImpl::use_(&global_12.with(|v| v.as_pointer())) });
    })));
    assert!((copyCount_2.with(|rc| *rc.borrow()) == 0));
    ({ (*f.borrow()).call() });
    assert!((copyCount_2.with(|rc| *rc.borrow()) == 0));
    assert!(({ ({ (*global_12.with(Value::clone).borrow()).uses } == 1) }));
    assert!((ctorCount_1.with(|rc| *rc.borrow()) == 0));
}
pub fn test_14() {
    ({ LifetimeTracked::reset() });
    let t: Value<LifetimeTracked> = Rc::new(RefCell::new(LifetimeTracked::new()));
    let _dtor_t = std::mem::DropGuard::new(t.clone(), |__p| __p.as_pointer().destructor());
    let r: Ptr<LifetimeTracked> = t.as_pointer();
    let f: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
        {
            let r: Value<LifetimeTracked> =
                Rc::new(RefCell::new(LifetimeTracked::copy_from({ (r).clone() })));
        },
        || {
            ({ LifetimeTrackedImpl::use_(&r.as_pointer()) });
        }
    )));
    let _dtor_f =
        std::mem::DropGuard::new(f.clone(), |__p| __p.as_pointer().with(|__f| __f.destroy()));
    assert!((copyCount_2.with(|rc| *rc.borrow()) == 1));
    ({ (*f.borrow()).call() });
    assert!((copyCount_2.with(|rc| *rc.borrow()) == 1));
    assert!((useCount_4.with(|rc| *rc.borrow()) == 1));
    assert!(({ ({ (*t.borrow()).uses } == 0) }));
}
pub fn test_15() {
    let n: Value<i32> = Rc::new(RefCell::new(5));
    let f: Value<FnPtr<fn() -> Ptr<i32>>> = Rc::new(RefCell::new(lambda!(
        {
            let n: Value<i32> = Rc::new(RefCell::new((*n.borrow())));
        },
        || -> Ptr<i32> {
            return (n.as_pointer());
        }
    )));
    assert!(({ (({ (*f.borrow()).call() }) != (n.as_pointer())) }));
    assert!(({ ((({ (*f.borrow()).call() }).read()) == 5) }));
}
pub fn test_16() {
    ({ HasConstexprCtor::reset() });
    let k: Value<HasConstexprCtor> = Rc::new(RefCell::new(HasConstexprCtor::new({ 5 })));
    let f: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let k: Value<HasConstexprCtor> = Rc::new(RefCell::new(HasConstexprCtor::copy_from({
                k.as_pointer()
            })));
        },
        || -> i32 {
            let mut c: HasConstexprCtor = HasConstexprCtor::copy_from({ k.as_pointer() });
            return c.m;
        }
    )));
    assert!((copyCount_0.with(|rc| *rc.borrow()) == 1));
    assert!(({ (({ (*f.borrow()).call() }) == 5) }));
    assert!((copyCount_0.with(|rc| *rc.borrow()) == 2));
}
pub fn test_17() {
    let mut n: i32 = 5;
    let f: Value<FnPtr<fn() -> i32>> =
        Rc::new(RefCell::new(FnPtr::<fn() -> i32>::new(|| -> i32 {
            {
                return 5;
            }
        })));
    assert!(({ (({ (*f.borrow()).call() }) == 5) }));
}
thread_local!(
    pub static copies_18: Value<i32> = Rc::new(RefCell::new(0_i32));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CppFrom, CopyAssign, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(8)]
pub struct p5_2_mutable_subobject_is_odr_used_HasMutableMember {
    #[offset(0)]
    pub value: i32,
    #[offset(4)]
    pub scratch: i32,
}
impl p5_2_mutable_subobject_is_odr_used_HasMutableMember {
    pub fn new(mut v: i32) -> Self {
        Self {
            value: v,
            scratch: 0,
        }
    }
    pub fn copy_from(o: Ptr<p5_2_mutable_subobject_is_odr_used_HasMutableMember>) -> Self {
        let __this: p5_2_mutable_subobject_is_odr_used_HasMutableMember = Self {
            value: o.with(|__s| __s.value),
            scratch: o.with(|__s| __s.scratch),
        };
        (*copies_18.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for p5_2_mutable_subobject_is_odr_used_HasMutableMember {
    fn clone(&self) -> Self {
        let __src: Value<p5_2_mutable_subobject_is_odr_used_HasMutableMember> = Rc::new(
            RefCell::new(p5_2_mutable_subobject_is_odr_used_HasMutableMember {
                value: self.value.clone(),
                scratch: self.scratch.clone(),
            }),
        );
        p5_2_mutable_subobject_is_odr_used_HasMutableMember::copy_from(__src.as_pointer())
    }
}
pub fn test_19() {
    let k: Value<p5_2_mutable_subobject_is_odr_used_HasMutableMember> = Rc::new(RefCell::new(
        p5_2_mutable_subobject_is_odr_used_HasMutableMember::new({ 5 }),
    ));
    let f: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let k: Value<p5_2_mutable_subobject_is_odr_used_HasMutableMember> =
                Rc::new(RefCell::new(
                    p5_2_mutable_subobject_is_odr_used_HasMutableMember::copy_from({
                        k.as_pointer()
                    }),
                ));
        },
        || -> i32 {
            return { (*k.borrow()).value };
        }
    )));
    assert!((copies_18.with(|rc| *rc.borrow()) == 1));
    assert!(({ (({ (*f.borrow()).call() }) == 5) }));
    assert!((copies_18.with(|rc| *rc.borrow()) == 1));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor, Default)]
#[byte_size(8)]
pub struct p6_unevaluated_binding_needs_no_capture_HasTwoMembers {
    #[offset(0)]
    #[byte_size(4)]
    pub a: LifetimeTracked,
    #[offset(4)]
    #[byte_size(4)]
    pub b: LifetimeTracked,
}
pub fn test_20() {
    ({ LifetimeTracked::reset() });
    let p: Value<p6_unevaluated_binding_needs_no_capture_HasTwoMembers> = Rc::new(RefCell::new(
        p6_unevaluated_binding_needs_no_capture_HasTwoMembers {
            a: LifetimeTracked::new(),
            b: LifetimeTracked::new(),
        },
    ));
    let _dtor_p = std::mem::DropGuard::new(p.clone(), |__p| __p.as_pointer().destructor());
    let __decomp_21: Ptr<p6_unevaluated_binding_needs_no_capture_HasTwoMembers> = p.as_pointer();
    let f: Value<FnPtr<fn() -> usize>> =
        Rc::new(RefCell::new(FnPtr::<fn() -> usize>::new(|| -> usize {
            {
                return 4usize;
            }
        })));
    assert!((copyCount_2.with(|rc| *rc.borrow()) == 0));
    assert!(({ (({ (*f.borrow()).call() }) == 4usize) }));
    assert!((copyCount_2.with(|rc| *rc.borrow()) == 0));
    assert!((useCount_4.with(|rc| *rc.borrow()) == 0));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(8)]
pub struct p7_member_access_captures_this_HasMemberUsedInLambda {
    #[offset(0)]
    #[byte_size(4)]
    pub x: LifetimeTracked,
    #[offset(4)]
    pub n: i32,
}
impl Default for p7_member_access_captures_this_HasMemberUsedInLambda {
    fn default() -> Self {
        {
            p7_member_access_captures_this_HasMemberUsedInLambda {
                x: <LifetimeTracked>::default(),
                n: 1,
            }
        }
    }
}
pub fn test_22() {
    ({ LifetimeTracked::reset() });
    let c: Value<p7_member_access_captures_this_HasMemberUsedInLambda> = Rc::new(RefCell::new(
        <p7_member_access_captures_this_HasMemberUsedInLambda>::default(),
    ));
    let _dtor_c = std::mem::DropGuard::new(c.clone(), |__p| __p.as_pointer().destructor());
    let f: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(
        ({ p7_member_access_captures_this_HasMemberUsedInLambdaImpl::makeLambda(&c.as_pointer()) }),
    ));
    assert!((copyCount_2.with(|rc| *rc.borrow()) == 0));
    assert!(({ (({ (*f.borrow()).call() }) == 1) }));
    assert!(({ ({ (*c.borrow()).x.uses } == 1) }));
    (*c.borrow_mut()).n = 10;
    assert!(({ (({ (*f.borrow()).call() }) == 10) }));
    assert!(({ ({ (*c.borrow()).x.uses } == 2) }));
    assert!((copyCount_2.with(|rc| *rc.borrow()) == 0));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor, Default)]
#[byte_size(4)]
pub struct p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda {
    #[offset(0)]
    #[byte_size(4)]
    pub x: LifetimeTracked,
}
pub fn test_23() {
    ({ LifetimeTracked::reset() });
    let c: Value<p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda> = Rc::new(
        RefCell::new(<p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda>::default()),
    );
    let _dtor_c = std::mem::DropGuard::new(c.clone(), |__p| __p.as_pointer().destructor());
    let f: Value<FnPtr<fn() -> usize>> = Rc::new(RefCell::new(
        ({
            p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambdaImpl::sizer(&c.as_pointer())
        }),
    ));
    assert!(({ (({ (*f.borrow()).call() }) == 4usize) }));
    assert!((copyCount_2.with(|rc| *rc.borrow()) == 0));
    assert!((useCount_4.with(|rc| *rc.borrow()) == 0));
}
thread_local!(
    pub static noisy_calls_24: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn noisy_25() -> i32 {
    (*noisy_calls_24.with(Value::clone).borrow_mut()).prefix_inc();
    return 1;
}
pub fn test_26() {
    ({ LifetimeTracked::reset() });
    assert!((4usize == 4usize));
    assert!((::std::mem::size_of::<i32>() == ::std::mem::size_of::<i32>()));
    assert!(!(false));
    assert!(!(false));
    let mut n: i32 = 3;
    assert!((n == 3));
    assert!((noisy_calls_24.with(|rc| *rc.borrow()) == 0));
    assert!((ctorCount_1.with(|rc| *rc.borrow()) == 0));
    assert!((({ noisy_25() }) == 1));
    assert!((noisy_calls_24.with(|rc| *rc.borrow()) == 1));
}
thread_local!(
    pub static t_ctors_27: Value<i32> = Rc::new(RefCell::new(0_i32));
);
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct p9_destructor_not_potentially_invoked_HasUndefinedDestructor {}
impl p9_destructor_not_potentially_invoked_HasUndefinedDestructor {
    pub fn new() -> Self {
        let __this: p9_destructor_not_potentially_invoked_HasUndefinedDestructor = Self {};
        (*t_ctors_27.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Default for p9_destructor_not_potentially_invoked_HasUndefinedDestructor {
    fn default() -> Self {
        { p9_destructor_not_potentially_invoked_HasUndefinedDestructor::new() }
    }
}
pub fn test_28() {
    let mut p: Ptr<p9_destructor_not_potentially_invoked_HasUndefinedDestructor> =
        Ptr::alloc(p9_destructor_not_potentially_invoked_HasUndefinedDestructor::new());
    assert!(!((p).is_null()));
    assert!((t_ctors_27.with(|rc| *rc.borrow()) == 1));
}
thread_local!(
    pub static ctor_calls_29: Value<i32> = Rc::new(RefCell::new(0_i32));
);
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(4)]
pub struct p9_unused_copy_constructor_HasUndefinedCopyCtor {
    #[offset(0)]
    pub value: i32,
}
impl p9_unused_copy_constructor_HasUndefinedCopyCtor {
    pub fn new() -> Self {
        let __this: p9_unused_copy_constructor_HasUndefinedCopyCtor = Self { value: 3 };
        (*ctor_calls_29.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Default for p9_unused_copy_constructor_HasUndefinedCopyCtor {
    fn default() -> Self {
        { p9_unused_copy_constructor_HasUndefinedCopyCtor::new() }
    }
}
pub fn read_30(object: Ptr<p9_unused_copy_constructor_HasUndefinedCopyCtor>) -> i32 {
    return object.with(|__s| __s.value);
}
pub fn test_31() {
    let a: Value<p9_unused_copy_constructor_HasUndefinedCopyCtor> = Rc::new(RefCell::new(
        p9_unused_copy_constructor_HasUndefinedCopyCtor::new(),
    ));
    assert!((({ read_30(a.as_pointer(),) }) == 3));
    assert!((ctor_calls_29.with(|rc| *rc.borrow()) == 1));
}
thread_local!(
    pub static m_ctors_32: Value<i32> = Rc::new(RefCell::new(0_i32));
);
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct p9_unused_implicit_assignment_HasUndefinedCopyAssignment {}
impl p9_unused_implicit_assignment_HasUndefinedCopyAssignment {
    pub fn new() -> Self {
        let __this: p9_unused_implicit_assignment_HasUndefinedCopyAssignment = Self {};
        (*m_ctors_32.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Default for p9_unused_implicit_assignment_HasUndefinedCopyAssignment {
    fn default() -> Self {
        { p9_unused_implicit_assignment_HasUndefinedCopyAssignment::new() }
    }
}
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct p9_unused_implicit_assignment_HasMemberWithUndefinedCopyAssignment {
    #[offset(0)]
    #[byte_size(1)]
    pub m: p9_unused_implicit_assignment_HasUndefinedCopyAssignment,
}
pub fn test_33() {
    let mut a: p9_unused_implicit_assignment_HasMemberWithUndefinedCopyAssignment =
        <p9_unused_implicit_assignment_HasMemberWithUndefinedCopyAssignment>::default();
    let mut b: p9_unused_implicit_assignment_HasMemberWithUndefinedCopyAssignment =
        <p9_unused_implicit_assignment_HasMemberWithUndefinedCopyAssignment>::default();
    assert!((m_ctors_32.with(|rc| *rc.borrow()) == 2));
}
thread_local!(
    pub static else_runs_34: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn g_35() {
    {
        (*else_runs_34.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub fn test_36() {
    ({ LifetimeTracked::reset() });
    ({ g_35() });
    assert!((else_runs_34.with(|rc| *rc.borrow()) == 1));
    assert!((ctorCount_1.with(|rc| *rc.borrow()) == 0));
    assert!((useCount_4.with(|rc| *rc.borrow()) == 0));
}
thread_local!(
    pub static else_runs_37: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn g_38(mut _a0: i32) {
    {
        (*else_runs_37.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub fn g_39(mut _a0: i8) {
    {
        (*else_runs_37.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub fn test_40() {
    ({ LifetimeTracked::reset() });
    ({ g_38(1) });
    ({ g_39(('c' as i8)) });
    assert!((else_runs_37.with(|rc| *rc.borrow()) == 2));
    assert!((ctorCount_1.with(|rc| *rc.borrow()) == 0));
    assert!((useCount_4.with(|rc| *rc.borrow()) == 0));
}
thread_local!(
    pub static else_runs_41: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn g_43() {
    {
        (*else_runs_41.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub fn test_44() {
    ({ LifetimeTracked::reset() });
    ({ g_43() });
    assert!((else_runs_41.with(|rc| *rc.borrow()) == 1));
    assert!((ctorCount_1.with(|rc| *rc.borrow()) == 0));
    assert!((useCount_4.with(|rc| *rc.borrow()) == 0));
}
thread_local!(
    pub static g_calls_45: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn g_46() {
    {}
    (*g_calls_45.with(Value::clone).borrow_mut()).prefix_inc();
}
pub fn test_47() {
    ({ g_46() });
    assert!((g_calls_45.with(|rc| *rc.borrow()) == 1));
}
thread_local!(
    pub static x1_48: Value<Ptr<p13_pointer_to_incomplete_class_X>> = Rc::new(RefCell::new(Ptr::<
        p13_pointer_to_incomplete_class_X,
    >::null(
    )));
);
thread_local!(
    pub static x2_49: Value<Ptr<p13_pointer_to_incomplete_class_X>> = Rc::new(RefCell::new(Ptr::<
        p13_pointer_to_incomplete_class_X,
    >::null(
    )));
);
pub fn identity_50(
    mut p: Ptr<p13_pointer_to_incomplete_class_X>,
) -> Ptr<p13_pointer_to_incomplete_class_X> {
    return (p).clone();
}
pub fn test_51() {
    assert!(({ (*x1_48.with(Value::clone).borrow()).is_null() }));
    assert!(
        ({
            ({ ({ identity_50((*x2_49.with(Value::clone).borrow()).clone()) }) } == {
                (*x1_48.with(Value::clone).borrow()).clone()
            })
        })
    );
    let mut pp: Ptr<Ptr<p13_pointer_to_incomplete_class_X>> = (x1_48.with(|v| v.as_pointer()));
    assert!((pp.read()).is_null());
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_5() });
    ({ test_6() });
    ({ test_7() });
    ({ test_9() });
    ({ test_11() });
    ({ test_13() });
    ({ test_14() });
    ({ test_15() });
    ({ test_16() });
    ({ test_17() });
    ({ test_19() });
    ({ test_20() });
    ({ test_22() });
    ({ test_23() });
    ({ test_26() });
    ({ test_28() });
    ({ test_31() });
    ({ test_33() });
    ({ test_36() });
    ({ test_40() });
    ({ test_44() });
    ({ test_47() });
    ({ test_51() });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct p13_pointer_to_incomplete_class_X;
impl p4_1_pure_virtual_without_definition_HasPureVirtual
    for p4_1_pure_virtual_without_definition_OverridesPureVirtual
{
    fn f(&mut self) {
        (*derived_calls_10.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub trait HasConstexprCtorImpl {
    fn copy_assign(&self, _a0: Ptr<HasConstexprCtor>) -> Ptr<HasConstexprCtor>;
}
impl HasConstexprCtorImpl for Ptr<HasConstexprCtor> {
    fn copy_assign(&self, _a0: Ptr<HasConstexprCtor>) -> Ptr<HasConstexprCtor> {
        field!((*self), m).write({ _a0.with(|__s| __s.m) });
        return (*self).clone();
    }
}
pub trait LifetimeTrackedImpl {
    fn destructor(&self);
    fn use_(&self);
    fn copy_assign(&self, _a0: Ptr<LifetimeTracked>) -> Ptr<LifetimeTracked>;
}
impl LifetimeTrackedImpl for Ptr<LifetimeTracked> {
    fn destructor(&self) {
        (*dtorCount_3.with(Value::clone).borrow_mut()).prefix_inc();
    }
    fn use_(&self) {
        field!((*self), uses).with_mut(|__v| {
            __v.prefix_inc();
        });
        (*useCount_4.with(Value::clone).borrow_mut()).prefix_inc();
    }
    fn copy_assign(&self, _a0: Ptr<LifetimeTracked>) -> Ptr<LifetimeTracked> {
        field!((*self), uses).write({ _a0.with(|__s| __s.uses) });
        return (*self).clone();
    }
}
pub trait p5_2_mutable_subobject_is_odr_used_HasMutableMemberImpl {
    fn copy_assign(
        &self,
        _a0: Ptr<p5_2_mutable_subobject_is_odr_used_HasMutableMember>,
    ) -> Ptr<p5_2_mutable_subobject_is_odr_used_HasMutableMember>;
}
impl p5_2_mutable_subobject_is_odr_used_HasMutableMemberImpl
    for Ptr<p5_2_mutable_subobject_is_odr_used_HasMutableMember>
{
    fn copy_assign(
        &self,
        _a0: Ptr<p5_2_mutable_subobject_is_odr_used_HasMutableMember>,
    ) -> Ptr<p5_2_mutable_subobject_is_odr_used_HasMutableMember> {
        field!((*self), value).write({ _a0.with(|__s| __s.value) });
        field!((*self), scratch).write({ _a0.with(|__s| __s.scratch) });
        return (*self).clone();
    }
}
pub trait p6_unevaluated_binding_needs_no_capture_HasTwoMembersImpl {
    fn copy_assign(
        &self,
        _a0: Ptr<p6_unevaluated_binding_needs_no_capture_HasTwoMembers>,
    ) -> Ptr<p6_unevaluated_binding_needs_no_capture_HasTwoMembers>;
    fn destructor(&self);
}
impl p6_unevaluated_binding_needs_no_capture_HasTwoMembersImpl
    for Ptr<p6_unevaluated_binding_needs_no_capture_HasTwoMembers>
{
    fn copy_assign(
        &self,
        _a0: Ptr<p6_unevaluated_binding_needs_no_capture_HasTwoMembers>,
    ) -> Ptr<p6_unevaluated_binding_needs_no_capture_HasTwoMembers> {
        ({
            let _arg0: Ptr<LifetimeTracked> = field_ptr!(_a0, a);
            LifetimeTrackedImpl::copy_assign(&field_ptr!((*self), a), _arg0)
        });
        ({
            let _arg0: Ptr<LifetimeTracked> = field_ptr!(_a0, b);
            LifetimeTrackedImpl::copy_assign(&field_ptr!((*self), b), _arg0)
        });
        return (*self).clone();
    }
    fn destructor(&self) {
        field_ptr!(self, b).destructor();
        field_ptr!(self, a).destructor();
    }
}
pub trait p7_member_access_captures_this_HasMemberUsedInLambdaImpl {
    fn makeLambda(&self) -> FnPtr<fn() -> i32>;
    fn copy_assign(
        &self,
        _a0: Ptr<p7_member_access_captures_this_HasMemberUsedInLambda>,
    ) -> Ptr<p7_member_access_captures_this_HasMemberUsedInLambda>;
    fn destructor(&self);
}
impl p7_member_access_captures_this_HasMemberUsedInLambdaImpl
    for Ptr<p7_member_access_captures_this_HasMemberUsedInLambda>
{
    fn makeLambda(&self) -> FnPtr<fn() -> i32> {
        return lambda!(
            {
                let this_: Value<Ptr<p7_member_access_captures_this_HasMemberUsedInLambda>> =
                    Rc::new(RefCell::new((*self).clone()));
            },
            || -> i32 {
                ({ LifetimeTrackedImpl::use_(&field_ptr!((*this_.borrow()), x)) });
                return (*this_.borrow()).with(|__s| __s.n);
            }
        );
    }
    fn copy_assign(
        &self,
        _a0: Ptr<p7_member_access_captures_this_HasMemberUsedInLambda>,
    ) -> Ptr<p7_member_access_captures_this_HasMemberUsedInLambda> {
        ({
            let _arg0: Ptr<LifetimeTracked> = field_ptr!(_a0, x);
            LifetimeTrackedImpl::copy_assign(&field_ptr!((*self), x), _arg0)
        });
        field!((*self), n).write({ _a0.with(|__s| __s.n) });
        return (*self).clone();
    }
    fn destructor(&self) {
        field_ptr!(self, x).destructor();
    }
}
pub trait p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambdaImpl {
    fn sizer(&self) -> FnPtr<fn() -> usize>;
    fn copy_assign(
        &self,
        _a0: Ptr<p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda>,
    ) -> Ptr<p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda>;
    fn destructor(&self);
}
impl p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambdaImpl
    for Ptr<p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda>
{
    fn sizer(&self) -> FnPtr<fn() -> usize> {
        return FnPtr::<fn() -> usize>::new(|| -> usize {
            {
                return 4usize;
            }
        });
    }
    fn copy_assign(
        &self,
        _a0: Ptr<p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda>,
    ) -> Ptr<p7_unevaluated_this_needs_no_capture_HasMemberUsedInLambda> {
        ({
            let _arg0: Ptr<LifetimeTracked> = field_ptr!(_a0, x);
            LifetimeTrackedImpl::copy_assign(&field_ptr!((*self), x), _arg0)
        });
        return (*self).clone();
    }
    fn destructor(&self) {
        field_ptr!(self, x).destructor();
    }
}
pub trait p9_destructor_not_potentially_invoked_HasUndefinedDestructorImpl {
    fn destructor(&self) {
        unimplemented!()
    }
}
impl p9_destructor_not_potentially_invoked_HasUndefinedDestructorImpl
    for Ptr<p9_destructor_not_potentially_invoked_HasUndefinedDestructor>
{
}
pub trait p9_unused_implicit_assignment_HasUndefinedCopyAssignmentImpl {
    fn copy_assign(
        &self,
        _a0: Ptr<p9_unused_implicit_assignment_HasUndefinedCopyAssignment>,
    ) -> Ptr<p9_unused_implicit_assignment_HasUndefinedCopyAssignment> {
        unimplemented!()
    }
}
impl p9_unused_implicit_assignment_HasUndefinedCopyAssignmentImpl
    for Ptr<p9_unused_implicit_assignment_HasUndefinedCopyAssignment>
{
}
pub fn __cpp2rust_init_globals() {
    let _ = copyCount_0.with(|_| ());
    let _ = ctorCount_1.with(|_| ());
    let _ = copyCount_2.with(|_| ());
    let _ = dtorCount_3.with(|_| ());
    let _ = useCount_4.with(|_| ());
    let _ = left_8.with(|_| ());
    let _ = derived_calls_10.with(|_| ());
    let _ = global_12.with(|_| ());
    let _ = copies_18.with(|_| ());
    let _ = noisy_calls_24.with(|_| ());
    let _ = t_ctors_27.with(|_| ());
    let _ = ctor_calls_29.with(|_| ());
    let _ = m_ctors_32.with(|_| ());
    let _ = else_runs_34.with(|_| ());
    let _ = else_runs_37.with(|_| ());
    let _ = else_runs_41.with(|_| ());
    let _ = g_calls_45.with(|_| ());
    let _ = x1_48.with(|_| ());
    let _ = x2_49.with(|_| ());
}
