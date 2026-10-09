extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static global_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(1)]
pub struct S {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(1)]
pub struct Defaulted {
    #[offset(0)]
    #[byte_size(1)]
    pub s: S,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(1)]
pub struct Middle {
    #[offset(0)]
    #[byte_size(1)]
    pub s: S,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(1)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(1)]
    pub m: Middle,
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(3)]
pub struct ArrayMember {
    #[offset(0)]
    #[byte_size(3)]
    pub items: Value<Box<[S]>>,
}
impl Default for ArrayMember {
    fn default() -> Self {
        {
            ArrayMember {
                items: Rc::new(RefCell::new(
                    (0..3).map(|_| <S>::default()).collect::<Box<[S]>>(),
                )),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(1)]
pub struct EmptyBody {
    #[offset(0)]
    #[byte_size(1)]
    pub s: S,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(1)]
pub struct Templated_char_ {
    #[offset(0)]
    pub v: i8,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct Templated_int_ {
    #[offset(0)]
    pub v: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct Copied {
    #[offset(0)]
    pub v: i32,
}
thread_local!(
    pub static order_1: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..3).map(|_| 0_i32).collect::<Box<[i32]>>()));
);
thread_local!(
    pub static order_count_2: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct Tagged {
    #[offset(0)]
    pub tag: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(20)]
pub struct Ordered {
    #[offset(0)]
    #[byte_size(4)]
    pub first: Tagged,
    #[offset(4)]
    pub dummy1: i32,
    #[offset(8)]
    #[byte_size(4)]
    pub second: Tagged,
    #[offset(12)]
    pub dummy2: i32,
    #[offset(16)]
    #[byte_size(4)]
    pub third: Tagged,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        let s: Value<S> = Rc::new(RefCell::new(<S>::default()));
        let _dtor_s = ScopedDestructor::new(&s, |__p| __p.destructor());
    }
    assert!((global_0.with(|rc| *rc.borrow()) == 1));
    {
        let s: Value<S> = Rc::new(RefCell::new(<S>::default()));
        let _dtor_s = ScopedDestructor::new(&s, |__p| __p.destructor());
    }
    assert!((global_0.with(|rc| *rc.borrow()) == 2));
    {
        let d: Value<Defaulted> = Rc::new(RefCell::new(<Defaulted>::default()));
        let _dtor_d = ScopedDestructor::new(&d, |__p| __p.destructor());
    }
    assert!((global_0.with(|rc| *rc.borrow()) == 3));
    {
        let o: Value<Outer> = Rc::new(RefCell::new(<Outer>::default()));
        let _dtor_o = ScopedDestructor::new(&o, |__p| __p.destructor());
    }
    assert!((global_0.with(|rc| *rc.borrow()) == 4));
    {
        let am: Value<ArrayMember> = Rc::new(RefCell::new(<ArrayMember>::default()));
        let _dtor_am = ScopedDestructor::new(&am, |__p| __p.destructor());
    }
    assert!((global_0.with(|rc| *rc.borrow()) == 7));
    {
        let e: Value<EmptyBody> = Rc::new(RefCell::new(<EmptyBody>::default()));
        let _dtor_e = ScopedDestructor::new(&e, |__p| __p.destructor());
    }
    assert!((global_0.with(|rc| *rc.borrow()) == 8));
    {
        let tc: Value<Templated_char_> = Rc::new(RefCell::new(<Templated_char_>::default()));
        let _dtor_tc = ScopedDestructor::new(&tc, |__p| __p.destructor());
        let ti: Value<Templated_int_> = Rc::new(RefCell::new(<Templated_int_>::default()));
        let _dtor_ti = ScopedDestructor::new(&ti, |__p| __p.destructor());
    }
    assert!((global_0.with(|rc| *rc.borrow()) == 13));
    {
        let a: Value<Copied> = Rc::new(RefCell::new(Copied { v: 5 }));
        let _dtor_a = ScopedDestructor::new(&a, |__p| __p.destructor());
        let b: Value<Copied> = Rc::new(RefCell::new((*a.borrow()).clone()));
        let _dtor_b = ScopedDestructor::new(&b, |__p| __p.destructor());
        assert!(({ (*b.borrow()).v } == 5));
    }
    assert!((global_0.with(|rc| *rc.borrow()) == 15));
    {
        let o: Value<Ordered> = Rc::new(RefCell::new(Ordered {
            first: Tagged { tag: 1 },
            dummy1: 0,
            second: Tagged { tag: 2 },
            dummy2: 0,
            third: Tagged { tag: 3 },
        }));
        let _dtor_o = ScopedDestructor::new(&o, |__p| __p.destructor());
    }
    assert!((order_count_2.with(|rc| *rc.borrow()) == 3));
    assert!(
        (({
            let __idx = (0) as usize;
            order_1.with(|rc| rc.borrow()[__idx])
        }) == 3)
    );
    assert!(
        (({
            let __idx = (1) as usize;
            order_1.with(|rc| rc.borrow()[__idx])
        }) == 2)
    );
    assert!(
        (({
            let __idx = (2) as usize;
            order_1.with(|rc| rc.borrow()[__idx])
        }) == 1)
    );
    return 0;
}
pub trait ArrayMemberImpl {
    fn destructor(&self);
}
impl ArrayMemberImpl for Ptr<ArrayMember> {
    fn destructor(&self) {
        {
            let __p = (*self.upgrade().deref()).items.as_pointer();
            for __i in 0..__p.len() {
                SImpl::destructor(&__p.offset(__i as isize));
            }
        }
    }
}
pub trait CopiedImpl {
    fn destructor(&self);
}
impl CopiedImpl for Ptr<Copied> {
    fn destructor(&self) {
        (*global_0.with(Value::clone).borrow_mut()).postfix_inc();
    }
}
pub trait DefaultedImpl {
    fn destructor(&self);
}
impl DefaultedImpl for Ptr<Defaulted> {
    fn destructor(&self) {
        field_ptr!(self, s).destructor();
    }
}
pub trait EmptyBodyImpl {
    fn destructor(&self);
}
impl EmptyBodyImpl for Ptr<EmptyBody> {
    fn destructor(&self) {
        field_ptr!(self, s).destructor();
    }
}
pub trait MiddleImpl {
    fn destructor(&self);
}
impl MiddleImpl for Ptr<Middle> {
    fn destructor(&self) {
        field_ptr!(self, s).destructor();
    }
}
pub trait OrderedImpl {
    fn destructor(&self);
}
impl OrderedImpl for Ptr<Ordered> {
    fn destructor(&self) {
        field_ptr!(self, third).destructor();
        field_ptr!(self, second).destructor();
        field_ptr!(self, first).destructor();
    }
}
pub trait OuterImpl {
    fn destructor(&self);
}
impl OuterImpl for Ptr<Outer> {
    fn destructor(&self) {
        field_ptr!(self, m).destructor();
    }
}
pub trait SImpl {
    fn destructor(&self);
}
impl SImpl for Ptr<S> {
    fn destructor(&self) {
        (*global_0.with(Value::clone).borrow_mut()).postfix_inc();
    }
}
pub trait TaggedImpl {
    fn destructor(&self);
}
impl TaggedImpl for Ptr<Tagged> {
    fn destructor(&self) {
        (*order_1.with(Value::clone).borrow_mut())
            [((*order_count_2.with(Value::clone).borrow_mut()).postfix_inc()) as usize] =
            (*self).with(|__s| __s.tag);
    }
}
pub trait Templated_char_Impl {
    fn destructor(&self);
}
impl Templated_char_Impl for Ptr<Templated_char_> {
    fn destructor(&self) {
        global_0.with(|rc| {
            *rc.borrow_mut() = {
                ((global_0.with(|rc| *rc.borrow()) as usize)
                    .wrapping_add((::std::mem::size_of::<i8>() as usize))) as i32
            }
        });
    }
}
pub trait Templated_int_Impl {
    fn destructor(&self);
}
impl Templated_int_Impl for Ptr<Templated_int_> {
    fn destructor(&self) {
        global_0.with(|rc| {
            *rc.borrow_mut() = {
                ((global_0.with(|rc| *rc.borrow()) as usize)
                    .wrapping_add((::std::mem::size_of::<i32>() as usize))) as i32
            }
        });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = global_0.with(|_| ());
    let _ = order_1.with(|_| ());
    let _ = order_count_2.with(|_| ());
}
