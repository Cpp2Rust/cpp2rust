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
#[derive(Default)]
pub struct S {}
pub trait SImpl {
    fn destructor(&self);
}
impl Clone for S {
    fn clone(&self) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {}
    fn from_bytes(buf: &[u8]) -> Self {
        Self {}
    }
}
#[derive(Default)]
pub struct Defaulted {
    pub s: Value<S>,
}
pub trait DefaultedImpl {
    fn destructor(&self);
}
impl Clone for Defaulted {
    fn clone(&self) -> Self {
        let __this: Value<Defaulted> = Rc::new(RefCell::new(Self {
            s: Rc::new(RefCell::new((*self.s.borrow()).clone())),
        }));
        let this: Ptr<Defaulted> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Defaulted {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.s.borrow()).to_bytes(&mut buf[0..1]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            s: Rc::new(RefCell::new(<S>::from_bytes(&buf[0..1]))),
        }
    }
}
#[derive(Default)]
pub struct Middle {
    pub s: Value<S>,
}
pub trait MiddleImpl {
    fn destructor(&self);
}
impl Clone for Middle {
    fn clone(&self) -> Self {
        let __this: Value<Middle> = Rc::new(RefCell::new(Self {
            s: Rc::new(RefCell::new((*self.s.borrow()).clone())),
        }));
        let this: Ptr<Middle> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Middle {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.s.borrow()).to_bytes(&mut buf[0..1]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            s: Rc::new(RefCell::new(<S>::from_bytes(&buf[0..1]))),
        }
    }
}
#[derive(Default)]
pub struct Outer {
    pub m: Value<Middle>,
}
pub trait OuterImpl {
    fn destructor(&self);
}
impl Clone for Outer {
    fn clone(&self) -> Self {
        let __this: Value<Outer> = Rc::new(RefCell::new(Self {
            m: Rc::new(RefCell::new((*self.m.borrow()).clone())),
        }));
        let this: Ptr<Outer> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Outer {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.m.borrow()).to_bytes(&mut buf[0..1]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            m: Rc::new(RefCell::new(<Middle>::from_bytes(&buf[0..1]))),
        }
    }
}
#[derive()]
pub struct ArrayMember {
    pub items: Value<Box<[S]>>,
}
pub trait ArrayMemberImpl {
    fn destructor(&self);
}
impl Clone for ArrayMember {
    fn clone(&self) -> Self {
        let __this: Value<ArrayMember> = Rc::new(RefCell::new(Self {
            items: Rc::new(RefCell::new((*self.items.borrow()).clone())),
        }));
        let this: Ptr<ArrayMember> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for ArrayMember {
    fn default() -> Self {
        ArrayMember {
            items: Rc::new(RefCell::new(
                (0..3).map(|_| <S>::default()).collect::<Box<[S]>>(),
            )),
        }
    }
}
impl ByteRepr for ArrayMember {
    fn byte_size() -> usize {
        3
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.items.borrow()).to_bytes(&mut buf[0..3]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            items: Rc::new(RefCell::new(<Box<[S]>>::from_bytes(&buf[0..3]))),
        }
    }
}
#[derive(Default)]
pub struct EmptyBody {
    pub s: Value<S>,
}
pub trait EmptyBodyImpl {
    fn destructor(&self);
}
impl Clone for EmptyBody {
    fn clone(&self) -> Self {
        let __this: Value<EmptyBody> = Rc::new(RefCell::new(Self {
            s: Rc::new(RefCell::new((*self.s.borrow()).clone())),
        }));
        let this: Ptr<EmptyBody> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for EmptyBody {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.s.borrow()).to_bytes(&mut buf[0..1]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            s: Rc::new(RefCell::new(<S>::from_bytes(&buf[0..1]))),
        }
    }
}
#[derive(Default)]
pub struct Templated_char_ {
    pub v: Value<u8>,
}
pub trait Templated_char_Impl {
    fn destructor(&self);
}
impl Clone for Templated_char_ {
    fn clone(&self) -> Self {
        let __this: Value<Templated_char_> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        }));
        let this: Ptr<Templated_char_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Templated_char_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..1]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<u8>::from_bytes(&buf[0..1]))),
        }
    }
}
#[derive(Default)]
pub struct Templated_int_ {
    pub v: Value<i32>,
}
pub trait Templated_int_Impl {
    fn destructor(&self);
}
impl Clone for Templated_int_ {
    fn clone(&self) -> Self {
        let __this: Value<Templated_int_> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        }));
        let this: Ptr<Templated_int_> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Templated_int_ {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct Copied {
    pub v: Value<i32>,
}
pub trait CopiedImpl {
    fn destructor(&self);
}
impl Clone for Copied {
    fn clone(&self) -> Self {
        let __this: Value<Copied> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        }));
        let this: Ptr<Copied> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Copied {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
thread_local!(
    pub static order_1: Value<Box<[i32]>> = Rc::new(RefCell::new(
        (0..3).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
    ));
);
thread_local!(
    pub static order_count_2: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Default)]
pub struct Tagged {
    pub tag: Value<i32>,
}
pub trait TaggedImpl {
    fn destructor(&self);
}
impl Clone for Tagged {
    fn clone(&self) -> Self {
        let __this: Value<Tagged> = Rc::new(RefCell::new(Self {
            tag: Rc::new(RefCell::new((*self.tag.borrow()))),
        }));
        let this: Ptr<Tagged> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Tagged {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.tag.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            tag: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
#[derive(Default)]
pub struct Ordered {
    pub first: Value<Tagged>,
    pub dummy1: Value<i32>,
    pub second: Value<Tagged>,
    pub dummy2: Value<i32>,
    pub third: Value<Tagged>,
}
pub trait OrderedImpl {
    fn destructor(&self);
}
impl Clone for Ordered {
    fn clone(&self) -> Self {
        let __this: Value<Ordered> = Rc::new(RefCell::new(Self {
            first: Rc::new(RefCell::new((*self.first.borrow()).clone())),
            dummy1: Rc::new(RefCell::new((*self.dummy1.borrow()))),
            second: Rc::new(RefCell::new((*self.second.borrow()).clone())),
            dummy2: Rc::new(RefCell::new((*self.dummy2.borrow()))),
            third: Rc::new(RefCell::new((*self.third.borrow()).clone())),
        }));
        let this: Ptr<Ordered> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Ordered {
    fn byte_size() -> usize {
        20
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.first.borrow()).to_bytes(&mut buf[0..4]);
        (*self.dummy1.borrow()).to_bytes(&mut buf[4..8]);
        (*self.second.borrow()).to_bytes(&mut buf[8..12]);
        (*self.dummy2.borrow()).to_bytes(&mut buf[12..16]);
        (*self.third.borrow()).to_bytes(&mut buf[16..20]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            first: Rc::new(RefCell::new(<Tagged>::from_bytes(&buf[0..4]))),
            dummy1: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
            second: Rc::new(RefCell::new(<Tagged>::from_bytes(&buf[8..12]))),
            dummy2: Rc::new(RefCell::new(<i32>::from_bytes(&buf[12..16]))),
            third: Rc::new(RefCell::new(<Tagged>::from_bytes(&buf[16..20]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        let s: Value<S> = Rc::new(RefCell::new(S {}));
        let _dtor_s = ScopedDestructor::new(&s, |__p| __p.destructor());
    }
    assert!(((*global_0.with(Value::clone).borrow()) == 1));
    {
        let s: Value<S> = Rc::new(RefCell::new(S {}));
        let _dtor_s = ScopedDestructor::new(&s, |__p| __p.destructor());
    }
    assert!(((*global_0.with(Value::clone).borrow()) == 2));
    {
        let d: Value<Defaulted> = Rc::new(RefCell::new(Defaulted {
            s: Rc::new(RefCell::new(S {})),
        }));
        let _dtor_d = ScopedDestructor::new(&d, |__p| __p.destructor());
    }
    assert!(((*global_0.with(Value::clone).borrow()) == 3));
    {
        let o: Value<Outer> = Rc::new(RefCell::new(Outer {
            m: Rc::new(RefCell::new(Middle {
                s: Rc::new(RefCell::new(S {})),
            })),
        }));
        let _dtor_o = ScopedDestructor::new(&o, |__p| __p.destructor());
    }
    assert!(((*global_0.with(Value::clone).borrow()) == 4));
    {
        let am: Value<ArrayMember> = Rc::new(RefCell::new(ArrayMember {
            items: Rc::new(RefCell::new(Box::new([S {}, S {}, S {}]))),
        }));
        let _dtor_am = ScopedDestructor::new(&am, |__p| __p.destructor());
    }
    assert!(((*global_0.with(Value::clone).borrow()) == 7));
    {
        let e: Value<EmptyBody> = Rc::new(RefCell::new(EmptyBody {
            s: Rc::new(RefCell::new(S {})),
        }));
        let _dtor_e = ScopedDestructor::new(&e, |__p| __p.destructor());
    }
    assert!(((*global_0.with(Value::clone).borrow()) == 8));
    {
        let tc: Value<Templated_char_> = Rc::new(RefCell::new(Templated_char_ {
            v: Rc::new(RefCell::new(<u8>::default())),
        }));
        let _dtor_tc = ScopedDestructor::new(&tc, |__p| __p.destructor());
        let ti: Value<Templated_int_> = Rc::new(RefCell::new(Templated_int_ {
            v: Rc::new(RefCell::new(<i32>::default())),
        }));
        let _dtor_ti = ScopedDestructor::new(&ti, |__p| __p.destructor());
    }
    assert!(((*global_0.with(Value::clone).borrow()) == 13));
    {
        let a: Value<Copied> = Rc::new(RefCell::new(Copied {
            v: Rc::new(RefCell::new(5)),
        }));
        let _dtor_a = ScopedDestructor::new(&a, |__p| __p.destructor());
        let b: Value<Copied> = Rc::new(RefCell::new((*a.borrow()).clone()));
        let _dtor_b = ScopedDestructor::new(&b, |__p| __p.destructor());
        assert!(((*(*b.borrow()).v.borrow()) == 5));
    }
    assert!(((*global_0.with(Value::clone).borrow()) == 15));
    {
        let o: Value<Ordered> = Rc::new(RefCell::new(Ordered {
            first: Rc::new(RefCell::new(Tagged {
                tag: Rc::new(RefCell::new(1)),
            })),
            dummy1: Rc::new(RefCell::new(0)),
            second: Rc::new(RefCell::new(Tagged {
                tag: Rc::new(RefCell::new(2)),
            })),
            dummy2: Rc::new(RefCell::new(0)),
            third: Rc::new(RefCell::new(Tagged {
                tag: Rc::new(RefCell::new(3)),
            })),
        }));
        let _dtor_o = ScopedDestructor::new(&o, |__p| __p.destructor());
    }
    assert!(((*order_count_2.with(Value::clone).borrow()) == 3));
    assert!(((*order_1.with(Value::clone).borrow())[(0) as usize] == 3));
    assert!(((*order_1.with(Value::clone).borrow())[(1) as usize] == 2));
    assert!(((*order_1.with(Value::clone).borrow())[(2) as usize] == 1));
    return 0;
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
impl CopiedImpl for Ptr<Copied> {
    fn destructor(&self) {
        (*global_0.with(Value::clone).borrow_mut()).postfix_inc();
    }
}
impl DefaultedImpl for Ptr<Defaulted> {
    fn destructor(&self) {
        (*self.upgrade().deref()).s.as_pointer().destructor();
    }
}
impl EmptyBodyImpl for Ptr<EmptyBody> {
    fn destructor(&self) {
        (*self.upgrade().deref()).s.as_pointer().destructor();
    }
}
impl MiddleImpl for Ptr<Middle> {
    fn destructor(&self) {
        (*self.upgrade().deref()).s.as_pointer().destructor();
    }
}
impl OrderedImpl for Ptr<Ordered> {
    fn destructor(&self) {
        (*self.upgrade().deref()).third.as_pointer().destructor();
        (*self.upgrade().deref()).second.as_pointer().destructor();
        (*self.upgrade().deref()).first.as_pointer().destructor();
    }
}
impl OuterImpl for Ptr<Outer> {
    fn destructor(&self) {
        (*self.upgrade().deref()).m.as_pointer().destructor();
    }
}
impl SImpl for Ptr<S> {
    fn destructor(&self) {
        (*global_0.with(Value::clone).borrow_mut()).postfix_inc();
    }
}
impl TaggedImpl for Ptr<Tagged> {
    fn destructor(&self) {
        (*order_1.with(Value::clone).borrow_mut())
            [((*order_count_2.with(Value::clone).borrow_mut()).postfix_inc()) as usize] =
            (*(*(*self).upgrade().deref()).tag.borrow());
    }
}
impl Templated_char_Impl for Ptr<Templated_char_> {
    fn destructor(&self) {
        {
            let rhs_0 = (((*global_0.with(Value::clone).borrow()) as usize)
                .wrapping_add((::std::mem::size_of::<u8>() as usize)))
                as i32;
            (*global_0.with(Value::clone).borrow_mut()) = rhs_0
        };
    }
}
impl Templated_int_Impl for Ptr<Templated_int_> {
    fn destructor(&self) {
        {
            let rhs_0 = (((*global_0.with(Value::clone).borrow()) as usize)
                .wrapping_add((::std::mem::size_of::<i32>() as usize)))
                as i32;
            (*global_0.with(Value::clone).borrow_mut()) = rhs_0
        };
    }
}
