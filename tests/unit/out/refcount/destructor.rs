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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
pub struct S {}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Defaulted {
    #[offset(0)]
    pub s: S,
}
<<<<<<< HEAD
impl ByteRepr for Defaulted {}
#[derive(VaArg, FnPtrArg, Default)]
=======
impl ByteRepr for Defaulted {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.s.to_bytes(&mut buf[0..1]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            s: <S>::from_bytes(&buf[0..1]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
>>>>>>> 3ed38b58 (Remove Value<> boxing from struct fields)
pub struct Middle {
    #[offset(0)]
    pub s: S,
}
<<<<<<< HEAD
impl ByteRepr for Middle {}
#[derive(VaArg, FnPtrArg, Default)]
=======
impl ByteRepr for Middle {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.s.to_bytes(&mut buf[0..1]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            s: <S>::from_bytes(&buf[0..1]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
>>>>>>> 3ed38b58 (Remove Value<> boxing from struct fields)
pub struct Outer {
    #[offset(0)]
    pub m: Middle,
}
<<<<<<< HEAD
impl ByteRepr for Outer {}
#[derive(VaArg, FnPtrArg)]
=======
impl ByteRepr for Outer {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.m.to_bytes(&mut buf[0..1]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            m: <Middle>::from_bytes(&buf[0..1]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg)]
>>>>>>> 3ed38b58 (Remove Value<> boxing from struct fields)
pub struct ArrayMember {
    #[offset(0)]
    pub items: Box<[S]>,
}
impl Default for ArrayMember {
    fn default() -> Self {
        ArrayMember {
            items: (0..3).map(|_| <S>::default()).collect::<Box<[S]>>(),
        }
    }
}
<<<<<<< HEAD
impl ByteRepr for ArrayMember {}
#[derive(VaArg, FnPtrArg, Default)]
=======
impl ByteRepr for ArrayMember {
    fn byte_size() -> usize {
        3
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.items.to_bytes(&mut buf[0..3]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            items: <Box<[S]>>::from_bytes(&buf[0..3]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
>>>>>>> 3ed38b58 (Remove Value<> boxing from struct fields)
pub struct EmptyBody {
    #[offset(0)]
    pub s: S,
}
<<<<<<< HEAD
impl ByteRepr for EmptyBody {}
#[derive(VaArg, FnPtrArg, Default)]
=======
impl ByteRepr for EmptyBody {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.s.to_bytes(&mut buf[0..1]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            s: <S>::from_bytes(&buf[0..1]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
>>>>>>> 3ed38b58 (Remove Value<> boxing from struct fields)
pub struct Templated_char_ {
    #[offset(0)]
    pub v: u8,
}
<<<<<<< HEAD
impl ByteRepr for Templated_char_ {}
#[derive(VaArg, FnPtrArg, Default)]
=======
impl ByteRepr for Templated_char_ {
    fn byte_size() -> usize {
        1
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.v.to_bytes(&mut buf[0..1]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: <u8>::from_bytes(&buf[0..1]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
>>>>>>> 3ed38b58 (Remove Value<> boxing from struct fields)
pub struct Templated_int_ {
    #[offset(0)]
    pub v: i32,
}
<<<<<<< HEAD
impl ByteRepr for Templated_int_ {}
#[derive(VaArg, FnPtrArg, Default)]
=======
impl ByteRepr for Templated_int_ {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.v.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
>>>>>>> 3ed38b58 (Remove Value<> boxing from struct fields)
pub struct Copied {
    #[offset(0)]
    pub v: i32,
}
<<<<<<< HEAD
impl ByteRepr for Copied {}
=======
impl ByteRepr for Copied {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.v.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
>>>>>>> 3ed38b58 (Remove Value<> boxing from struct fields)
thread_local!(
    pub static order_1: Value<Box<[i32]>> =
        Rc::new(RefCell::new((0..3).map(|_| 0_i32).collect::<Box<[i32]>>()));
);
thread_local!(
    pub static order_count_2: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Tagged {
    #[offset(0)]
    pub tag: i32,
}
<<<<<<< HEAD
impl ByteRepr for Tagged {}
#[derive(VaArg, FnPtrArg, Default)]
=======
impl ByteRepr for Tagged {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.tag.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            tag: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
>>>>>>> 3ed38b58 (Remove Value<> boxing from struct fields)
pub struct Ordered {
    #[offset(0)]
    pub first: Tagged,
    #[offset(4)]
    pub dummy1: i32,
    #[offset(8)]
    pub second: Tagged,
    #[offset(12)]
    pub dummy2: i32,
    #[offset(16)]
    pub third: Tagged,
}
<<<<<<< HEAD
impl ByteRepr for Ordered {}
=======
impl ByteRepr for Ordered {
    fn byte_size() -> usize {
        20
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.first.to_bytes(&mut buf[0..4]);
        self.dummy1.to_bytes(&mut buf[4..8]);
        self.second.to_bytes(&mut buf[8..12]);
        self.dummy2.to_bytes(&mut buf[12..16]);
        self.third.to_bytes(&mut buf[16..20]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            first: <Tagged>::from_bytes(&buf[0..4]),
            dummy1: <i32>::from_bytes(&buf[4..8]),
            second: <Tagged>::from_bytes(&buf[8..12]),
            dummy2: <i32>::from_bytes(&buf[12..16]),
            third: <Tagged>::from_bytes(&buf[16..20]),
        }
    }
}
>>>>>>> 3ed38b58 (Remove Value<> boxing from struct fields)
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
            let __p: Ptr<S> = field_ptr!(self, items);
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
        SImpl::destructor(&field_ptr!(self, s));
    }
}
pub trait EmptyBodyImpl {
    fn destructor(&self);
}
impl EmptyBodyImpl for Ptr<EmptyBody> {
    fn destructor(&self) {
        SImpl::destructor(&field_ptr!(self, s));
    }
}
pub trait MiddleImpl {
    fn destructor(&self);
}
impl MiddleImpl for Ptr<Middle> {
    fn destructor(&self) {
        SImpl::destructor(&field_ptr!(self, s));
    }
}
pub trait OrderedImpl {
    fn destructor(&self);
}
impl OrderedImpl for Ptr<Ordered> {
    fn destructor(&self) {
        TaggedImpl::destructor(&field_ptr!(self, third));
        TaggedImpl::destructor(&field_ptr!(self, second));
        TaggedImpl::destructor(&field_ptr!(self, first));
    }
}
pub trait OuterImpl {
    fn destructor(&self);
}
impl OuterImpl for Ptr<Outer> {
    fn destructor(&self) {
        MiddleImpl::destructor(&field_ptr!(self, m));
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
        let __rhs = (*self).with(|__s: &Tagged| __s.tag);
        let __idx = (*order_count_2.with(Value::clone).borrow_mut()).postfix_inc();
        (*order_1.with(Value::clone).borrow_mut())[(__idx) as usize] = __rhs;
    }
}
pub trait Templated_char_Impl {
    fn destructor(&self);
}
impl Templated_char_Impl for Ptr<Templated_char_> {
    fn destructor(&self) {
        {
            let rhs_0 = ((global_0.with(|rc| *rc.borrow()) as usize)
                .wrapping_add((::std::mem::size_of::<u8>() as usize)))
                as i32;
            global_0.with(|rc| *rc.borrow_mut() = rhs_0)
        };
    }
}
pub trait Templated_int_Impl {
    fn destructor(&self);
}
impl Templated_int_Impl for Ptr<Templated_int_> {
    fn destructor(&self) {
        {
            let rhs_0 = ((global_0.with(|rc| *rc.borrow()) as usize)
                .wrapping_add((::std::mem::size_of::<i32>() as usize)))
                as i32;
            global_0.with(|rc| *rc.borrow_mut() = rhs_0)
        };
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = global_0.with(|_| ());
    let _ = order_1.with(|_| ());
    let _ = order_count_2.with(|_| ());
}
