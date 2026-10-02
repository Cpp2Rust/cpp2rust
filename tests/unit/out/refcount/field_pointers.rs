extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct Inner {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    #[byte_size(8)]
    pub name: Value<Box<[u8]>>,
}
impl Default for Inner {
    fn default() -> Self {
        Inner {
            a: 0_i32,
            name: Rc::new(RefCell::new((0..8).map(|_| 0_u8).collect::<Box<[u8]>>())),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Header {
    #[offset(0)]
    pub tag: i32,
    #[offset(4)]
    pub size: i16,
}
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(104)]
pub struct Outer {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    #[byte_size(12)]
    pub inner: Inner,
    #[offset(16)]
    #[byte_size(36)]
    pub items: Value<Box<[Inner]>>,
    #[offset(56)]
    #[byte_size(24)]
    pub v: Value<Vec<i32>>,
    #[offset(80)]
    #[byte_size(8)]
    pub cursor: Ptr<i32>,
    #[offset(88)]
    #[byte_size(16)]
    pub buf: Value<Box<[i32]>>,
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
pub fn set_0(p: Ptr<i32>, value: i32) {
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(p));
    let value: Value<i32> = Rc::new(RefCell::new(value));
    (*p.borrow()).write({ (*value.borrow()) });
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
            (array_field_ptr!(o.as_pointer(), items) as Ptr<Inner>).offset((1) as isize),
            a
        )),
    ));
    (*pi.borrow()).write(4);
    assert!(
        ({
            (*elem!((array_field_ptr!(o.as_pointer(), items) as Ptr<Inner>), 1)
                .upgrade()
                .deref())
            .a
        } == 4)
    );
    assert!(
        ({
            (field_ptr!(
                (array_field_ptr!(o.as_pointer(), items) as Ptr<Inner>).offset((0) as isize),
                a
            ))
        } != { (*pi.borrow()).clone() })
    );
    let name: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (array_field_ptr!(field_ptr!(o.as_pointer(), inner), name) as Ptr<u8>),
    ));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 3) {
        elem!((*name.borrow()), (*i.borrow()))
            .write({ (((('a' as u8) as i32) + (*i.borrow())) as u8) });
        (*i.borrow_mut()).prefix_inc();
    }
    assert!(
        ((array_field_ptr!(field_ptr!(o.as_pointer(), inner), name) as Ptr::<u8>)
            .to_c_string_iterator()
            .count()
            == 3_usize)
    );
    assert!(
        ({ (*name.borrow()).offset((3) as isize) } == {
            ((array_field_ptr!(field_ptr!(o.as_pointer(), inner), name) as Ptr<u8>)
                .offset((3) as isize))
        })
    );
    (*o.borrow_mut()).cursor =
        { ((array_field_ptr!(o.as_pointer(), buf) as Ptr<i32>).offset((1) as isize)) };
    { (*o.borrow()).cursor.clone() }.write(5);
    {
        let _ptr = (*o.borrow_mut()).cursor.postfix_inc();
        _ptr.write(_ptr.read() + 1)
    };
    assert!(
        ((elem!((array_field_ptr!(o.as_pointer(), buf) as Ptr::<i32>), 1).read()) == 6)
            && ({ { (*o.borrow()).cursor.clone() } } == {
                ((array_field_ptr!(o.as_pointer(), buf) as Ptr<i32>).offset((2) as isize))
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
            && ((elem!(({ (*o.borrow()).v.as_pointer() } as Ptr<i32>), 0_usize).read()) == 7)
    );
    assert!((({ OuterImpl::sum(&o.as_pointer(),) }) == 7));
    let y: Value<i32> = Rc::new(RefCell::new(0));
    {
        ((field_ptr!(
            (array_field_ptr!(o.as_pointer(), items) as Ptr<Inner>).offset((2) as isize),
            a
        )) as Ptr<i32>)
            .to_any()
            .memcpy(
                &((field_ptr!(
                    (array_field_ptr!(o.as_pointer(), items) as Ptr<Inner>).offset((1) as isize),
                    a
                )) as Ptr<i32>)
                    .to_any(),
                ::std::mem::size_of::<i32>() as usize,
            );
        ((field_ptr!(
            (array_field_ptr!(o.as_pointer(), items) as Ptr<Inner>).offset((2) as isize),
            a
        )) as Ptr<i32>)
            .to_any()
    };
    {
        ((y.as_pointer()) as Ptr<i32>).to_any().memcpy(
            &((field_ptr!(
                (array_field_ptr!(o.as_pointer(), items) as Ptr<Inner>).offset((2) as isize),
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
        return (elem!(
            (array_field_ptr!((*self), buf) as Ptr::<i32>),
            field!((*self), x).with_mut(|__v| __v.postfix_inc())
        )
        .read());
    }
    fn sum(&self) -> i32 {
        return ((*self).with(|__s| __s.x) + (*self).with(|__s| __s.inner.a));
    }
    fn push(&self, k: i32) {
        let k: Value<i32> = Rc::new(RefCell::new(k));
        {
            let __a1 = ((*k.borrow()) + (*self).with(|__s| __s.x));
            (*(*self).with(|__s| __s.v.clone()).borrow_mut()).push(__a1)
        };
    }
}
pub fn __cpp2rust_init_globals() {}
