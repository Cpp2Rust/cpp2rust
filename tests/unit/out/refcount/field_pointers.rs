extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, VaArg, FnPtrArg)]
pub struct Inner {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub name: Value<Box<[u8]>>,
}
impl Clone for Inner {
    fn clone(&self) -> Self {
        Self {
            a: self.a.clone(),
            name: Rc::new(RefCell::new((*self.name.borrow()).clone())),
        }
    }
}
impl Default for Inner {
    fn default() -> Self {
        Inner {
            a: 0_i32,
            name: Rc::new(RefCell::new((0..8).map(|_| 0_u8).collect::<Box<[u8]>>())),
        }
    }
}
impl ByteRepr for Inner {
    fn byte_size() -> usize {
        12
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.a.to_bytes(&mut buf[0..4]);
        (*self.name.borrow()).to_bytes(&mut buf[4..12]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: <i32>::from_bytes(&buf[0..4]),
            name: Rc::new(RefCell::new(<Box<[u8]>>::from_bytes(&buf[4..12]))),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Header {
    #[offset(0)]
    pub tag: i32,
    #[offset(4)]
    pub size: i16,
}
impl ByteRepr for Header {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.tag.to_bytes(&mut buf[0..4]);
        self.size.to_bytes(&mut buf[4..6]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            tag: <i32>::from_bytes(&buf[0..4]),
            size: <i16>::from_bytes(&buf[4..6]),
        }
    }
}
#[derive(Record, VaArg, FnPtrArg)]
pub struct Outer {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub inner: Inner,
    #[offset(16)]
    pub items: Value<Box<[Inner]>>,
    #[offset(56)]
    pub v: Value<Vec<i32>>,
    #[offset(80)]
    pub cursor: Ptr<i32>,
    #[offset(88)]
    pub buf: Value<Box<[i32]>>,
}
impl Clone for Outer {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            inner: self.inner.clone(),
            items: Rc::new(RefCell::new((*self.items.borrow()).clone())),
            v: Rc::new(RefCell::new((*self.v.borrow()).clone())),
            cursor: self.cursor.clone(),
            buf: Rc::new(RefCell::new((*self.buf.borrow()).clone())),
        }
    }
}
impl Default for Outer {
    fn default() -> Self {
        Outer {
            x: 0_i32,
            inner: <Inner>::default(),
            items: Rc::new(RefCell::new(
                (0..3).map(|_| <Inner>::default()).collect::<Box<[Inner]>>(),
            )),
            v: Rc::new(RefCell::new(Default::default())),
            cursor: Ptr::<i32>::null(),
            buf: Rc::new(RefCell::new((0..4).map(|_| 0_i32).collect::<Box<[i32]>>())),
        }
    }
}
impl ByteRepr for Outer {
    fn byte_size() -> usize {
        104
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x.to_bytes(&mut buf[0..4]);
        self.inner.to_bytes(&mut buf[4..16]);
        (*self.items.borrow()).to_bytes(&mut buf[16..52]);
        (*self.v.borrow()).to_bytes(&mut buf[56..80]);
        self.cursor.to_bytes(&mut buf[80..88]);
        (*self.buf.borrow()).to_bytes(&mut buf[88..104]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: <i32>::from_bytes(&buf[0..4]),
            inner: <Inner>::from_bytes(&buf[4..16]),
            items: Rc::new(RefCell::new(<Box<[Inner]>>::from_bytes(&buf[16..52]))),
            v: Rc::new(RefCell::new(<Vec<i32>>::from_bytes(&buf[56..80]))),
            cursor: <Ptr<i32>>::from_bytes(&buf[80..88]),
            buf: Rc::new(RefCell::new(<Box<[i32]>>::from_bytes(&buf[88..104]))),
        }
    }
}
pub fn set_0(p: Ptr<i32>, value: i32) {
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(p));
    let value: Value<i32> = Rc::new(RefCell::new(value));
    let __rhs = (*value.borrow());
    (*p.borrow()).write(__rhs);
}
pub fn bump_1(o: Ptr<Outer>) -> i32 {
    let o: Value<Ptr<Outer>> = Rc::new(RefCell::new(o));
    field!((*o.borrow()), x).with_mut(|__v| __v.postfix_inc());
    return 1;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let o: Value<Outer> = Rc::new(RefCell::new(<Outer>::default()));
    (*o.borrow_mut()).x = 1;
    let px: Value<Ptr<i32>> = Rc::new(RefCell::new((field_ptr!(o.as_pointer(), x))));
    (*px.borrow()).write(2);
    assert!(({ (*o.borrow()).x } == 2));
    let pa: Value<Ptr<i32>> = Rc::new(RefCell::new(
        (field_ptr!(field_ptr!(o.as_pointer(), inner), a)),
    ));
    ({
        let _p: Ptr<i32> = (*pa.borrow()).clone();
        set_0(_p, 3)
    });
    assert!(({ (*o.borrow()).inner.a } == 3));
    let pi: Value<Ptr<i32>> = Rc::new(RefCell::new(
        (field_ptr!(
            ({ (*o.borrow()).items.clone() }.as_pointer() as Ptr<Inner>).offset(1),
            a
        )),
    ));
    (*pi.borrow()).write(4);
    assert!(({ (*{ (*o.borrow()).items.clone() }.borrow())[(1) as usize].a } == 4));
    assert!({
        let _lhs = (field_ptr!(
            ({ (*o.borrow()).items.clone() }.as_pointer() as Ptr<Inner>).offset(0),
            a
        ));
        _lhs != (*pi.borrow()).clone()
    });
    let name: Value<Ptr<u8>> = Rc::new(RefCell::new(
        ({ (*o.borrow()).inner.name.clone() }.as_pointer() as Ptr<u8>),
    ));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 3) {
        let __rhs = (((('a' as u8) as i32) + (*i.borrow())) as u8);
        (*name.borrow()).offset((*i.borrow()) as isize).write(__rhs);
        (*i.borrow_mut()).prefix_inc();
    }
    assert!(
        (({ (*o.borrow()).inner.name.clone() }.as_pointer() as Ptr::<u8>)
            .to_c_string_iterator()
            .count()
            == 3_usize)
    );
    assert!({
        let _lhs = (*name.borrow()).offset((3) as isize);
        _lhs == (({ (*o.borrow()).inner.name.clone() }.as_pointer() as Ptr<u8>).offset(3))
    });
    let __rhs = (({ (*o.borrow()).buf.clone() }.as_pointer() as Ptr<i32>).offset(1));
    (*o.borrow_mut()).cursor = __rhs;
    { (*o.borrow()).cursor.clone() }.write(5);
    {
        let _ptr = (*o.borrow_mut()).cursor.postfix_inc();
        _ptr.write(_ptr.read() + 1)
    };
    assert!(
        ((*{ (*o.borrow()).buf.clone() }.borrow())[(1) as usize] == 6)
            && ({
                let _lhs = { (*o.borrow()).cursor.clone() };
                _lhs == (({ (*o.borrow()).buf.clone() }.as_pointer() as Ptr<i32>).offset(2))
            })
    );
    (*o.borrow_mut()).x = 0;
    let first: Value<i32> = Rc::new(RefCell::new(({ OuterImpl::next(&o.as_pointer()) })));
    assert!(((*first.borrow()) == 0) && ({ (*o.borrow()).x } == 1));
    let __rhs = ({ (*o.borrow()).inner.a } + ({ bump_1((o.as_pointer())) }));
    (*o.borrow_mut()).x = __rhs;
    assert!(({ (*o.borrow()).x } == 4));
    ({
        let _k: i32 = { (*o.borrow()).inner.a };
        OuterImpl::push(&o.as_pointer(), _k)
    });
    assert!(
        ((*{ (*o.borrow()).v.clone() }.borrow()).len() == 1_usize)
            && ((({ (*o.borrow()).v.clone() }.as_pointer() as Ptr<i32>)
                .offset(0_usize)
                .read())
                == 7)
    );
    assert!((({ OuterImpl::sum(&o.as_pointer(),) }) == 7));
    let y: Value<i32> = Rc::new(RefCell::new(0));
    {
        ((field_ptr!(
            ({ (*o.borrow()).items.clone() }.as_pointer() as Ptr<Inner>).offset(2),
            a
        )) as Ptr<i32>)
            .to_any()
            .memcpy(
                &((field_ptr!(
                    ({ (*o.borrow()).items.clone() }.as_pointer() as Ptr<Inner>).offset(1),
                    a
                )) as Ptr<i32>)
                    .to_any(),
                ::std::mem::size_of::<i32>() as usize,
            );
        ((field_ptr!(
            ({ (*o.borrow()).items.clone() }.as_pointer() as Ptr<Inner>).offset(2),
            a
        )) as Ptr<i32>)
            .to_any()
    };
    {
        ((y.as_pointer()) as Ptr<i32>).to_any().memcpy(
            &((field_ptr!(
                ({ (*o.borrow()).items.clone() }.as_pointer() as Ptr<Inner>).offset(2),
                a
            )) as Ptr<i32>)
                .to_any(),
            ::std::mem::size_of::<i32>() as usize,
        );
        ((y.as_pointer()) as Ptr<i32>).to_any()
    };
    assert!(((*y.borrow()) == 4));
    let bytes: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
    ])));
    let view: Value<Ptr<Header>> = Rc::new(RefCell::new(
        (bytes.as_pointer() as Ptr<u8>).reinterpret_cast::<Header>(),
    ));
    field!((*view.borrow()), tag).write(16909060);
    assert!(
        (((*bytes.borrow())[(0) as usize] as i32) == 4)
            || (((*bytes.borrow())[(3) as usize] as i32) == 4)
    );
    ({ set_0((field_ptr!((*view.borrow()), tag)), 0) });
    assert!(
        (((*bytes.borrow())[(0) as usize] as i32) == 0)
            && (((*bytes.borrow())[(3) as usize] as i32) == 0)
    );
    field!((*view.borrow()), size).write(1_i16);
    assert!(
        ((((*bytes.borrow())[(4) as usize] as i32) + ((*bytes.borrow())[(5) as usize] as i32))
            == 1)
    );
    return 0;
}
pub trait OuterImpl {
    fn next(&self) -> i32;
    fn sum(&self) -> i32;
    fn push(&self, k: i32);
}
impl OuterImpl for Ptr<Outer> {
    fn next(&self) -> i32 {
        return (*(*self).with(|__s| __s.buf.clone()).borrow())
            [(field!((*self), x).with_mut(|__v| __v.postfix_inc())) as usize];
    }
    fn sum(&self) -> i32 {
        return ((*self).with(|__s| __s.x) + (*self).with(|__s| __s.inner.a));
    }
    fn push(&self, k: i32) {
        let k: Value<i32> = Rc::new(RefCell::new(k));
        (*(*self).with(|__s| __s.v.clone()).borrow_mut())
            .push(((*k.borrow()) + (*self).with(|__s| __s.x)));
    }
}
pub fn __cpp2rust_init_globals() {}
