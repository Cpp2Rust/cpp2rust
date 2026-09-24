extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive()]
pub struct Sorted__lambda_at_lambda_decltype_type_cpp______ {
    pub items: Value<Box<[i32]>>,
    pub size: Value<i32>,
    pub cmp: Value<_>,
}
impl Clone for Sorted__lambda_at_lambda_decltype_type_cpp______ {
    fn clone(&self) -> Self {
        let __this: Value<Sorted__lambda_at_lambda_decltype_type_cpp______> =
            Rc::new(RefCell::new(Self {
                items: Rc::new(RefCell::new(Box::new(std::array::from_fn::<_, 4, _>(
                    |__i: usize| (*self.items.borrow())[(__i) as usize],
                )))),
                size: Rc::new(RefCell::new((*self.size.borrow()))),
                cmp: Rc::new(RefCell::new((*self.cmp.borrow()).clone())),
            }));
        let this: Ptr<Sorted__lambda_at_lambda_decltype_type_cpp______> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Sorted__lambda_at_lambda_decltype_type_cpp______ {
    fn default() -> Self {
        Sorted__lambda_at_lambda_decltype_type_cpp______ {
            items: Rc::new(RefCell::new((0..4).map(|_| 0_i32).collect::<Box<[i32]>>())),
            size: Rc::new(RefCell::new(0)),
            cmp: <Value<_>>::default(),
        }
    }
}
impl ByteRepr for Sorted__lambda_at_lambda_decltype_type_cpp______ {
    fn byte_size() -> usize {
        24
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.items.borrow()).to_bytes(&mut buf[0..16]);
        (*self.size.borrow()).to_bytes(&mut buf[16..20]);
        (*self.cmp.borrow()).to_bytes(&mut buf[20..21]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            items: Rc::new(RefCell::new(<Box<[i32]>>::from_bytes(&buf[0..16]))),
            size: Rc::new(RefCell::new(<i32>::from_bytes(&buf[16..20]))),
            cmp: Rc::new(RefCell::new(<_>::from_bytes(&buf[20..21]))),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let greater: Value<_> = Rc::new(RefCell::new(
        (|a: i32, b: i32| {
            let a: Value<i32> = Rc::new(RefCell::new(a));
            let b: Value<i32> = Rc::new(RefCell::new(b));
            return ((*a.borrow()) > (*b.borrow()));
        }),
    ));
    let desc: Value<Sorted__lambda_at_lambda_decltype_type_cpp______> = Rc::new(RefCell::new(
        <Sorted__lambda_at_lambda_decltype_type_cpp______>::default(),
    ));
    ({ Sorted__lambda_at_lambda_decltype_type_cpp______Impl::insert(&desc.as_pointer(), 2) });
    ({ Sorted__lambda_at_lambda_decltype_type_cpp______Impl::insert(&desc.as_pointer(), 7) });
    ({ Sorted__lambda_at_lambda_decltype_type_cpp______Impl::insert(&desc.as_pointer(), 4) });
    assert!(((*(*desc.borrow()).items.borrow())[(0) as usize] == 7));
    assert!(((*(*desc.borrow()).items.borrow())[(1) as usize] == 4));
    assert!(((*(*desc.borrow()).items.borrow())[(2) as usize] == 2));
    let asc: Value<Sorted__lambda_at_lambda_decltype_type_cpp______> = Rc::new(RefCell::new(
        <Sorted__lambda_at_lambda_decltype_type_cpp______>::default(),
    ));
    ({ Sorted__lambda_at_lambda_decltype_type_cpp______Impl::insert(&asc.as_pointer(), 2) });
    ({ Sorted__lambda_at_lambda_decltype_type_cpp______Impl::insert(&asc.as_pointer(), 7) });
    ({ Sorted__lambda_at_lambda_decltype_type_cpp______Impl::insert(&asc.as_pointer(), 4) });
    assert!(((*(*asc.borrow()).items.borrow())[(0) as usize] == 2));
    assert!(((*(*asc.borrow()).items.borrow())[(1) as usize] == 4));
    assert!(((*(*asc.borrow()).items.borrow())[(2) as usize] == 7));
    let fresh: Value<_> = Rc::new(RefCell::new(<_>::default()));
    assert!((({ (*fresh.borrow_mut())(3, 1,) }) as bool));
    assert!(!({ (*fresh.borrow_mut())(1, 3,) }));
    let assigned: Value<_> = Rc::new(RefCell::new(<_>::default()));
    (*assigned.borrow_mut()) = (*greater.borrow()).clone();
    assert!((({ (*assigned.borrow_mut())(5, 4,) }) as bool));
    let less: Value<_> = Rc::new(RefCell::new(<_>::default()));
    assert!((({ (*less.borrow_mut())(1, 3,) }) as bool));
    return 0;
}
pub trait Sorted__lambda_at_lambda_decltype_type_cpp______Impl {
    fn insert(&self, v: i32);
}
impl Sorted__lambda_at_lambda_decltype_type_cpp______Impl
    for Ptr<Sorted__lambda_at_lambda_decltype_type_cpp______>
{
    fn insert(&self, v: i32) {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let i: Value<i32> = Rc::new(RefCell::new((*(*(*self).upgrade().deref()).size.borrow())));
        'loop_: while ((*i.borrow()) > 0)
            && ({
                let _b: i32 =
                    (*(*(*self).upgrade().deref()).items.borrow())[((*i.borrow()) - 1) as usize];
                (*(*(*self).upgrade().deref()).cmp.borrow_mut())((*v.borrow()), _b)
            })
        {
            let __rhs =
                (*(*(*self).upgrade().deref()).items.borrow())[((*i.borrow()) - 1) as usize];
            (*(*(*self).upgrade().deref()).items.borrow_mut())[(*i.borrow()) as usize] = __rhs;
            (*i.borrow_mut()).postfix_dec();
        }
        (*(*(*self).upgrade().deref()).items.borrow_mut())[(*i.borrow()) as usize] = (*v.borrow());
        (*(*(*self).upgrade().deref()).size.borrow_mut()).postfix_inc();
    }
    fn insert(&self, v: i32) {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        let i: Value<i32> = Rc::new(RefCell::new((*(*(*self).upgrade().deref()).size.borrow())));
        'loop_: while ((*i.borrow()) > 0)
            && ({
                let _b: i32 =
                    (*(*(*self).upgrade().deref()).items.borrow())[((*i.borrow()) - 1) as usize];
                (*(*(*self).upgrade().deref()).cmp.borrow_mut())((*v.borrow()), _b)
            })
        {
            let __rhs =
                (*(*(*self).upgrade().deref()).items.borrow())[((*i.borrow()) - 1) as usize];
            (*(*(*self).upgrade().deref()).items.borrow_mut())[(*i.borrow()) as usize] = __rhs;
            (*i.borrow_mut()).postfix_dec();
        }
        (*(*(*self).upgrade().deref()).items.borrow_mut())[(*i.borrow()) as usize] = (*v.borrow());
        (*(*(*self).upgrade().deref()).size.borrow_mut()).postfix_inc();
    }
}
pub fn __cpp2rust_init_globals() {}
