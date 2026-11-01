extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct Sorted__lambda_at_lambda_decltype_type_cpp_22_18__ {
    #[offset(0)]
    #[byte_size(16)]
    pub items: Box<[i32]>,
    #[offset(16)]
    pub size: i32,
    #[offset(20)]
    #[byte_size(1)]
    pub cmp: FnPtr<fn(i32, i32) -> bool>,
}
impl Default for Sorted__lambda_at_lambda_decltype_type_cpp_22_18__ {
    fn default() -> Self {
        {
            Sorted__lambda_at_lambda_decltype_type_cpp_22_18__ {
                items: (0..4).map(|_| 0_i32).collect::<Box<[i32]>>(),
                size: 0,
                cmp: FnPtr::<fn(i32, i32) -> bool>::new(|mut a: i32, mut b: i32| -> bool {
                    {
                        return (a > b);
                    }
                }),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct Sorted__lambda_at_lambda_decltype_type_cpp_19_23__ {
    #[offset(0)]
    #[byte_size(16)]
    pub items: Box<[i32]>,
    #[offset(16)]
    pub size: i32,
    #[offset(20)]
    #[byte_size(1)]
    pub cmp: FnPtr<fn(i32, i32) -> bool>,
}
impl Default for Sorted__lambda_at_lambda_decltype_type_cpp_19_23__ {
    fn default() -> Self {
        {
            Sorted__lambda_at_lambda_decltype_type_cpp_19_23__ {
                items: (0..4).map(|_| 0_i32).collect::<Box<[i32]>>(),
                size: 0,
                cmp: FnPtr::<fn(i32, i32) -> bool>::new(|mut a: i32, mut b: i32| -> bool {
                    {
                        return (a < b);
                    }
                }),
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let greater: Value<FnPtr<fn(i32, i32) -> bool>> =
        Rc::new(RefCell::new(FnPtr::<fn(i32, i32) -> bool>::new(
            |mut a: i32, mut b: i32| -> bool {
                {
                    return (a > b);
                }
            },
        )));
    let desc: Value<Sorted__lambda_at_lambda_decltype_type_cpp_22_18__> = Rc::new(RefCell::new(
        <Sorted__lambda_at_lambda_decltype_type_cpp_22_18__>::default(),
    ));
    ({ Sorted__lambda_at_lambda_decltype_type_cpp_22_18__Impl::insert(&desc.as_pointer(), 2) });
    ({ Sorted__lambda_at_lambda_decltype_type_cpp_22_18__Impl::insert(&desc.as_pointer(), 7) });
    ({ Sorted__lambda_at_lambda_decltype_type_cpp_22_18__Impl::insert(&desc.as_pointer(), 4) });
    assert!(((array_elem!(desc.as_pointer(), items, 0).read()) == 7));
    assert!(((array_elem!(desc.as_pointer(), items, 1).read()) == 4));
    assert!(((array_elem!(desc.as_pointer(), items, 2).read()) == 2));
    let asc: Value<Sorted__lambda_at_lambda_decltype_type_cpp_19_23__> = Rc::new(RefCell::new(
        <Sorted__lambda_at_lambda_decltype_type_cpp_19_23__>::default(),
    ));
    ({ Sorted__lambda_at_lambda_decltype_type_cpp_19_23__Impl::insert(&asc.as_pointer(), 2) });
    ({ Sorted__lambda_at_lambda_decltype_type_cpp_19_23__Impl::insert(&asc.as_pointer(), 7) });
    ({ Sorted__lambda_at_lambda_decltype_type_cpp_19_23__Impl::insert(&asc.as_pointer(), 4) });
    assert!(((array_elem!(asc.as_pointer(), items, 0).read()) == 2));
    assert!(((array_elem!(asc.as_pointer(), items, 1).read()) == 4));
    assert!(((array_elem!(asc.as_pointer(), items, 2).read()) == 7));
    let fresh: Value<FnPtr<fn(i32, i32) -> bool>> =
        Rc::new(RefCell::new(FnPtr::<fn(i32, i32) -> bool>::new(
            |mut a: i32, mut b: i32| -> bool {
                {
                    return (a > b);
                }
            },
        )));
    assert!(({ (({ (*fresh.borrow()).call(3, 1,) }) as bool) }));
    assert!(({ !({ (*fresh.borrow()).call(1, 3,) }) }));
    let assigned: Value<FnPtr<fn(i32, i32) -> bool>> =
        Rc::new(RefCell::new(FnPtr::<fn(i32, i32) -> bool>::new(
            |mut a: i32, mut b: i32| -> bool {
                {
                    return (a > b);
                }
            },
        )));
    (*assigned.borrow_mut()) = (*greater.borrow()).clone();
    assert!(({ (({ (*assigned.borrow()).call(5, 4,) }) as bool) }));
    let less: Value<FnPtr<fn(i32, i32) -> bool>> =
        Rc::new(RefCell::new(FnPtr::<fn(i32, i32) -> bool>::new(
            |mut a: i32, mut b: i32| -> bool {
                {
                    return (a < b);
                }
            },
        )));
    assert!(({ (({ (*less.borrow()).call(1, 3,) }) as bool) }));
    return 0;
}
pub trait Sorted__lambda_at_lambda_decltype_type_cpp_19_23__Impl {
    fn insert(&self, v: i32);
}
impl Sorted__lambda_at_lambda_decltype_type_cpp_19_23__Impl
    for Ptr<Sorted__lambda_at_lambda_decltype_type_cpp_19_23__>
{
    fn insert(&self, mut v: i32) {
        let mut i: i32 = (*self).with(|__s| __s.size);
        'loop_: while (i > 0)
            && ({
                let _b: i32 = (array_elem!((*self), items, (i - 1)).read());
                (*(*self).upgrade().deref()).cmp.call(v, _b)
            })
        {
            array_elem!((*self), items, i).write({ (array_elem!((*self), items, (i - 1)).read()) });
            i.postfix_dec();
        }
        array_elem!((*self), items, i).write(v);
        field!((*self), size).with_mut(|__v| __v.postfix_inc());
    }
}
pub trait Sorted__lambda_at_lambda_decltype_type_cpp_22_18__Impl {
    fn insert(&self, v: i32);
}
impl Sorted__lambda_at_lambda_decltype_type_cpp_22_18__Impl
    for Ptr<Sorted__lambda_at_lambda_decltype_type_cpp_22_18__>
{
    fn insert(&self, mut v: i32) {
        let mut i: i32 = (*self).with(|__s| __s.size);
        'loop_: while (i > 0)
            && ({
                let _b: i32 = (array_elem!((*self), items, (i - 1)).read());
                (*(*self).upgrade().deref()).cmp.call(v, _b)
            })
        {
            array_elem!((*self), items, i).write({ (array_elem!((*self), items, (i - 1)).read()) });
            i.postfix_dec();
        }
        array_elem!((*self), items, i).write(v);
        field!((*self), size).with_mut(|__v| __v.postfix_inc());
    }
}
pub fn __cpp2rust_init_globals() {}
