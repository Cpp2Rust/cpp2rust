extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct Inner {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    #[byte_size(8)]
    pub name: Box<[i8]>,
}
impl Default for Inner {
    fn default() -> Self {
        {
            Inner {
                a: 0_i32,
                name: (0..8).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(104)]
pub struct Outer {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    #[byte_size(12)]
    pub inner: Inner,
    #[offset(16)]
    #[byte_size(36)]
    pub items: Box<[Inner]>,
    #[offset(56)]
    #[byte_size(24)]
    pub v: Vec<i32>,
    #[offset(80)]
    #[byte_size(8)]
    pub cursor: Ptr<i32>,
    #[offset(88)]
    #[byte_size(16)]
    pub buf: Box<[i32]>,
}
impl Outer {
    pub fn move_from(_a0: Ptr<Outer>) -> Self {
        Self {
            x: { (*_a0.upgrade().deref()).x },
            inner: { (*_a0.upgrade().deref()).inner.clone() },
            items: Box::new(std::array::from_fn::<_, 3, _>(|__i: usize| {
                ((*_a0.upgrade().deref()).items[(__i) as usize]).clone()
            })),
            v: field!(_a0, v).with_mut(std::mem::take),
            cursor: { (*_a0.upgrade().deref()).cursor.clone() },
            buf: Box::new(std::array::from_fn::<_, 4, _>(|__i: usize| {
                (*_a0.upgrade().deref()).buf[(__i) as usize]
            })),
        }
    }
}
impl Default for Outer {
    fn default() -> Self {
        {
            Outer {
                x: 0_i32,
                inner: <Inner>::default(),
                items: (0..3).map(|_| <Inner>::default()).collect::<Box<[Inner]>>(),
                v: Default::default(),
                cursor: Ptr::<i32>::null(),
                buf: (0..4).map(|_| 0_i32).collect::<Box<[i32]>>(),
            }
        }
    }
}
pub fn set_0(mut p: Ptr<i32>, mut value: i32) {
    p.write({ value });
}
pub fn bump_1(mut o: Ptr<Outer>) -> i32 {
    field!(o, x).with_mut(|__v| __v.postfix_inc());
    return 1;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let o: Value<Outer> = Rc::new(RefCell::new(<Outer>::default()));
    (*o.borrow_mut()).x = 1;
    let mut px: Ptr<i32> = (field_ptr!(o.as_pointer(), x));
    px.write(2);
    assert!(({ (*o.borrow()).x } == 2));
    let mut pa: Ptr<i32> = (field_ptr!(field_ptr!(o.as_pointer(), inner), a));
    ({
        let _p: Ptr<i32> = (pa).clone();
        set_0(_p, 3)
    });
    assert!(({ (*o.borrow()).inner.a } == 3));
    let mut pi: Ptr<i32> = (field_ptr!(
        (array_field_ptr!(o.as_pointer(), items) as Ptr<Inner>).offset((1) as isize),
        a
    ));
    pi.write(4);
    assert!((array_elem!(o.as_pointer(), items, 1).with(|__s| __s.a) == 4));
    assert!(
        ({
            (field_ptr!(
                (array_field_ptr!(o.as_pointer(), items) as Ptr<Inner>).offset((0) as isize),
                a
            ))
        } != { (pi).clone() })
    );
    let mut name: Ptr<i8> = (array_field_ptr!(field_ptr!(o.as_pointer(), inner), name) as Ptr<i8>);
    let mut i: i32 = 0;
    'loop_: while (i < 3) {
        elem!(name, i).write({ (((('a' as i8) as i32) + i) as i8) });
        i.prefix_inc();
    }
    assert!(
        ((array_field_ptr!(field_ptr!(o.as_pointer(), inner), name) as Ptr::<i8>)
            .to_c_string_iterator()
            .count()
            == 3_usize)
    );
    assert!(
        ({ name.offset((3) as isize) } == {
            ((array_field_ptr!(field_ptr!(o.as_pointer(), inner), name) as Ptr<i8>)
                .offset((3) as isize))
        })
    );
    (*o.borrow_mut()).cursor =
        { ((array_field_ptr!(o.as_pointer(), buf) as Ptr<i32>).offset((1) as isize)) };
    { (*o.borrow()).cursor.clone() }.write(5);
    {
        { (*o.borrow_mut()).cursor.postfix_inc() }.with_mut(|__v| *__v = *__v + 1)
    };
    assert!(
        ((array_elem!(o.as_pointer(), buf, 1).read()) == 6)
            && ({ { (*o.borrow()).cursor.clone() } } == {
                ((array_field_ptr!(o.as_pointer(), buf) as Ptr<i32>).offset((2) as isize))
            })
    );
    (*o.borrow_mut()).x = 0;
    let mut first: i32 = ({ OuterImpl::next(&o.as_pointer()) });
    assert!((first == 0) && ({ (*o.borrow()).x } == 1));
    let __rhs = ({ (*o.borrow()).inner.a } + ({ bump_1((o.as_pointer())) }));
    (*o.borrow_mut()).x = __rhs;
    assert!(({ (*o.borrow()).x } == 4));
    ({
        let _k: i32 = { (*o.borrow()).inner.a };
        OuterImpl::push(&o.as_pointer(), _k)
    });
    assert!(
        ((*o.borrow()).v.len() == 1_usize) && ((vec_elem!(o.as_pointer(), v, 0_usize).read()) == 7)
    );
    assert!((({ OuterImpl::sum(&o.as_pointer(),) }) == 7));
    let y: Value<i32> = Rc::new(RefCell::new(0));
    match ({
        ((field_ptr!(
            (array_field_ptr!(o.as_pointer(), items) as Ptr<Inner>).offset((2) as isize),
            a
        )) as Ptr<i32>)
            .to_any()
    },)
    {
        (mut __tmp_2,) => {
            __tmp_2.memcpy(
                &((field_ptr!(
                    (array_field_ptr!(o.as_pointer(), items) as Ptr<Inner>).offset((1) as isize),
                    a
                )) as Ptr<i32>)
                    .to_any(),
                ::std::mem::size_of::<i32>() as usize,
            );
            (__tmp_2).clone()
        }
    };
    match ({ ((y.as_pointer()) as Ptr<i32>).to_any() },) {
        (mut __tmp_3,) => {
            __tmp_3.memcpy(
                &((field_ptr!(
                    (array_field_ptr!(o.as_pointer(), items) as Ptr<Inner>).offset((2) as isize),
                    a
                )) as Ptr<i32>)
                    .to_any(),
                ::std::mem::size_of::<i32>() as usize,
            );
            (__tmp_3).clone()
        }
    };
    assert!(((*y.borrow()) == 4));
    let bytes: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
    ])));
    let mut view: Ptr<Header> = (bytes.as_pointer() as Ptr<u8>).reinterpret_cast::<Header>();
    field!(view, tag).write(16909060);
    assert!(
        (((*bytes.borrow())[(0) as usize] as i32) == 4)
            || (((*bytes.borrow())[(3) as usize] as i32) == 4)
    );
    ({ set_0((field_ptr!(view, tag)), 0) });
    assert!(
        (((*bytes.borrow())[(0) as usize] as i32) == 0)
            && (((*bytes.borrow())[(3) as usize] as i32) == 0)
    );
    field!(view, size).write(1_i16);
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
    fn move_assign(&self, _a0: Ptr<Outer>) -> Ptr<Outer>;
}
impl OuterImpl for Ptr<Outer> {
    fn next(&self) -> i32 {
        return (array_elem!(
            (*self),
            buf,
            field!((*self), x).with_mut(|__v| __v.postfix_inc())
        )
        .read());
    }
    fn sum(&self) -> i32 {
        return ((*self).with(|__s| __s.x) + (*self).with(|__s| __s.inner.a));
    }
    fn push(&self, mut k: i32) {
        {
            let __a1 = (k + (*self).with(|__s| __s.x));
            field!((*self), v).with_mut(|__v: &mut Vec<i32>| __v.push(__a1))
        };
    }
    fn move_assign(&self, _a0: Ptr<Outer>) -> Ptr<Outer> {
        field!((*self), x).write({ { (*_a0.upgrade().deref()).x } });
        field!((*self), inner).write({ { (*_a0.upgrade().deref()).inner.clone() } });
        match ({ ((array_field_ptr!((*self), items)) as Ptr<Inner>).to_any() },) {
            (mut __tmp_0,) => {
                __tmp_0.memcpy(
                    &((array_field_ptr!(_a0, items)) as Ptr<Inner>).to_any(),
                    36_usize as usize,
                );
                (__tmp_0).clone()
            }
        };
        field_ref!((*self), v)
            .as_pointer()
            .write(field!(_a0, v).with_mut(std::mem::take));
        field!((*self), cursor).write({ { (*_a0.upgrade().deref()).cursor.clone() } });
        match ({ ((array_field_ptr!((*self), buf)) as Ptr<i32>).to_any() },) {
            (mut __tmp_1,) => {
                __tmp_1.memcpy(
                    &((array_field_ptr!(_a0, buf)) as Ptr<i32>).to_any(),
                    16_usize as usize,
                );
                (__tmp_1).clone()
            }
        };
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
