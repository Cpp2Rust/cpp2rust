extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(24)]
pub struct Inner {
    #[offset(0)]
    #[byte_size(24)]
    pub values: Vec<i32>,
}
impl Inner {
    pub fn move_from(_a0: Ptr<Inner>) -> Self {
        Self {
            values: field!(_a0, values).with_mut(std::mem::take),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(56)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(24)]
    pub values: Vec<i32>,
    #[offset(24)]
    #[byte_size(24)]
    pub inner: Inner,
    #[offset(48)]
    #[byte_size(8)]
    pub name: Box<[i8]>,
}
impl Outer {
    pub fn move_from(_a0: Ptr<Outer>) -> Self {
        Self {
            values: field!(_a0, values).with_mut(std::mem::take),
            inner: Inner::move_from({ field_ptr!(_a0, inner) }),
            name: Box::new(std::array::from_fn::<_, 8, _>(|__i: usize| {
                (*_a0.upgrade().deref()).name[(__i) as usize]
            })),
        }
    }
}
impl Default for Outer {
    fn default() -> Self {
        {
            Outer {
                values: Default::default(),
                inner: <Inner>::default(),
                name: (0..8).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
        }
    }
}
pub fn make_0(calls: Ptr<i32>) -> Outer {
    calls.with_mut(|__v| __v.prefix_inc());
    return Outer {
        values: Vec::from([1, 2, 3]),
        inner: Inner {
            values: Vec::from([4, 5, 6]),
        },
        name: i8::array_from_literal(b"abc\0\0\0\0\0"),
    };
}
pub fn sum_1(values: Ptr<Vec<i32>>) -> i32 {
    let mut total: i32 = 0;
    'loop_: for value in IterRange::new(values.decay().begin(), values.decay().end()) {
        let mut value: i32 = (value.iter_deref().read());
        total += value;
    }
    return total;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let calls: Value<i32> = Rc::new(RefCell::new(0));
    let mut member: i32 = ({
        sum_1(
            (match Rc::new(RefCell::new(({ make_0(calls.as_pointer()) }))).as_pointer() {
                __tmp_1 => field_ref!(__tmp_1, values).as_pointer(),
            }),
        )
    });
    let mut nested: i32 = ({
        sum_1(
            (match Rc::new(RefCell::new(({ make_0(calls.as_pointer()) }))).as_pointer() {
                __tmp_2 => field_ref!(field_ptr!(__tmp_2, inner), values).as_pointer(),
            }),
        )
    });
    let mut element: i32 = (match (
        (match Rc::new(RefCell::new(({ make_0(calls.as_pointer()) }))).as_pointer() {
            __tmp_4 => field_ref!(__tmp_4, values).as_pointer(),
        }),
        { (((*calls.borrow()) - 2) as usize) },
    ) {
        (__tmp_3, mut __tmp_5) => {
            if __tmp_5 as usize >= __tmp_3.decay().len() {
                panic!("out of bounds access")
            } else {
                __tmp_3.decay().offset(__tmp_5 as isize)
            }
        }
    }
    .read());
    let mut length: usize =
        ((match Rc::new(RefCell::new(({ make_0(calls.as_pointer()) }))).as_pointer() {
            __tmp_6 => array_field_ptr!(__tmp_6, name),
        }) as Ptr<i8>)
            .to_c_string_iterator()
            .count();
    let mut aggregate: i32 = ({
        sum_1(
            (match Rc::new(RefCell::new(Outer {
                values: Vec::from([7]),
                inner: Inner {
                    values: Vec::from([8, 9]),
                },
                name: i8::array_from_literal(b"x\0\0\0\0\0\0\0"),
            }))
            .as_pointer()
            {
                __tmp_7 => field_ref!(field_ptr!(__tmp_7, inner), values).as_pointer(),
            }),
        )
    });
    println!(
        "{} {} {} {} {} {}",
        member,
        nested,
        element,
        length,
        aggregate,
        (*calls.borrow())
    );
    return 0;
}
pub trait InnerImpl {
    fn move_assign(&self, _a0: Ptr<Inner>) -> Ptr<Inner>;
}
impl InnerImpl for Ptr<Inner> {
    fn move_assign(&self, _a0: Ptr<Inner>) -> Ptr<Inner> {
        field_ref!((*self), values)
            .as_pointer()
            .write(field!(_a0, values).with_mut(std::mem::take));
        return (*self).clone();
    }
}
pub trait OuterImpl {
    fn move_assign(&self, _a0: Ptr<Outer>) -> Ptr<Outer>;
}
impl OuterImpl for Ptr<Outer> {
    fn move_assign(&self, _a0: Ptr<Outer>) -> Ptr<Outer> {
        field_ref!((*self), values)
            .as_pointer()
            .write(field!(_a0, values).with_mut(std::mem::take));
        ({
            let _arg0: Ptr<Inner> = field_ptr!(_a0, inner);
            InnerImpl::move_assign(&field_ptr!((*self), inner), _arg0)
        });
        match ({ ((array_field_ptr!((*self), name)) as Ptr<i8>).to_any() },) {
            (mut __tmp_0,) => {
                __tmp_0.memcpy(
                    &((array_field_ptr!(_a0, name)) as Ptr<i8>).to_any(),
                    8_usize as usize,
                );
                (__tmp_0).clone()
            }
        };
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
