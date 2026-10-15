extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(1)]
pub struct Guard__lambda_at_lambda_as_field_cpp_34_13__ {
    #[offset(0)]
    #[byte_size(1)]
    pub f: FnPtr<fn()>,
}
impl Default for Guard__lambda_at_lambda_as_field_cpp_34_13__ {
    fn default() -> Self {
        {
            Guard__lambda_at_lambda_as_field_cpp_34_13__ {
                f: FnPtr::<fn()>::new(|| {
                    (*count_0.with(Value::clone).borrow_mut()).postfix_inc();
                }),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(8)]
pub struct Guard__lambda_at_lambda_as_field_cpp_45_13__ {
    #[offset(0)]
    #[byte_size(8)]
    pub f: FnPtr<fn()>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Holder__lambda_at_lambda_as_field_cpp_39_12__ {
    #[offset(0)]
    #[byte_size(1)]
    pub f: FnPtr<fn(i32) -> i32>,
    #[offset(4)]
    pub calls: i32,
}
impl Default for Holder__lambda_at_lambda_as_field_cpp_39_12__ {
    fn default() -> Self {
        {
            Holder__lambda_at_lambda_as_field_cpp_39_12__ {
                f: FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
                    {
                        return (x + 1);
                    }
                }),
                calls: 0_i32,
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Holder__lambda_at_lambda_as_field_cpp_51_12__ {
    #[offset(0)]
    #[byte_size(4)]
    pub f: FnPtr<fn(i32) -> i32>,
    #[offset(4)]
    pub calls: i32,
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor)]
#[byte_size(8)]
pub struct Moving__lambda_at_lambda_as_field_cpp_64_12__ {
    #[offset(0)]
    #[byte_size(1)]
    pub f: FnPtr<fn(i32) -> i32>,
    #[offset(4)]
    pub moves: i32,
}
impl Moving__lambda_at_lambda_as_field_cpp_64_12__ {
    pub fn new(f: FnPtr<fn(i32) -> i32>) -> Self {
        let f: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(f));
        Self {
            f: (*f.borrow()).copy_from(),
            moves: 0,
        }
    }
    pub fn move_from(o: Ptr<Moving__lambda_at_lambda_as_field_cpp_64_12__>) -> Self {
        Self {
            f: (*o.upgrade().deref()).f.move_from(),
            moves: (o.with(|__s| __s.moves) + 1),
        }
    }
}
impl Default for Moving__lambda_at_lambda_as_field_cpp_64_12__ {
    fn default() -> Self {
        {
            Moving__lambda_at_lambda_as_field_cpp_64_12__ {
                f: FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
                    {
                        return (x * 2);
                    }
                }),
                moves: 0,
            }
        }
    }
}
pub fn wrap_1(fn_: FnPtr<fn(i32) -> i32>) -> FnPtr<fn(i32) -> i32> {
    let fn_: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(fn_));
    return lambda!(
        {
            let fn_: Value<FnPtr<fn(i32) -> i32>> =
                Rc::new(RefCell::new((*fn_.borrow()).copy_from()));
        },
        |mut x: i32| -> i32 {
            return (({ (*fn_.borrow()).call(x) }) + 1);
        }
    );
}
pub fn wrap_2(fn_: FnPtr<fn(i32) -> i32>) -> FnPtr<fn(i32) -> i32> {
    let fn_: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(fn_));
    return lambda!(
        {
            let fn_: Value<FnPtr<fn(i32) -> i32>> =
                Rc::new(RefCell::new((*fn_.borrow()).copy_from()));
        },
        |mut x: i32| -> i32 {
            return (({ (*fn_.borrow()).call(x) }) + 1);
        }
    );
}
thread_local!(
    pub static count_0: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        let g: Value<Guard__lambda_at_lambda_as_field_cpp_34_13__> =
            Rc::new(RefCell::new(Guard__lambda_at_lambda_as_field_cpp_34_13__ {
                f: FnPtr::<fn()>::new(|| {
                    (*count_0.with(Value::clone).borrow_mut()).postfix_inc();
                }),
            }));
        let _dtor_g = ScopedDestructor::new(&g, |__p| __p.destructor());
        assert!((count_0.with(|rc| *rc.borrow()) == 0));
    }
    assert!((count_0.with(|rc| *rc.borrow()) == 1));
    let n: Value<Holder__lambda_at_lambda_as_field_cpp_39_12__> = Rc::new(RefCell::new(
        Holder__lambda_at_lambda_as_field_cpp_39_12__ {
            f: FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
                {
                    return (x + 1);
                }
            }),
            calls: 0,
        },
    ));
    assert!(
        (({ Holder__lambda_at_lambda_as_field_cpp_39_12__Impl::call(&n.as_pointer(), 2,) }) == 3)
    );
    assert!(({ (*n.borrow()).calls } == 1));
    let cleaned: Value<i32> = Rc::new(RefCell::new(0));
    {
        let g: Value<Guard__lambda_at_lambda_as_field_cpp_45_13__> =
            Rc::new(RefCell::new(Guard__lambda_at_lambda_as_field_cpp_45_13__ {
                f: lambda!(
                    {
                        let cleaned: Ptr<i32> = cleaned.as_pointer();
                    },
                    || {
                        cleaned.with_mut(|__v| __v.postfix_inc());
                    }
                ),
            }));
        let _dtor_g = ScopedDestructor::new(&g, |__p| __p.destructor());
        assert!(((*cleaned.borrow()) == 0));
    }
    assert!(((*cleaned.borrow()) == 1));
    let factor: Value<i32> = Rc::new(RefCell::new(3));
    let h: Value<Holder__lambda_at_lambda_as_field_cpp_51_12__> = Rc::new(RefCell::new(
        Holder__lambda_at_lambda_as_field_cpp_51_12__ {
            f: lambda!(
                {
                    let factor: Value<i32> = Rc::new(RefCell::new((*factor.borrow())));
                },
                |mut x: i32| -> i32 {
                    return (x * (*factor.borrow()));
                }
            ),
            calls: 0,
        },
    ));
    (*factor.borrow_mut()) = 100;
    assert!(
        (({ Holder__lambda_at_lambda_as_field_cpp_51_12__Impl::call(&h.as_pointer(), 2,) }) == 6)
    );
    assert!(
        (({ Holder__lambda_at_lambda_as_field_cpp_51_12__Impl::call(&h.as_pointer(), 5,) }) == 15)
    );
    assert!(({ (*h.borrow()).calls } == 2));
    let w: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(
        ({
            wrap_1(lambda!(
                {
                    let factor: Value<i32> = Rc::new(RefCell::new((*factor.borrow())));
                },
                |mut x: i32| -> i32 {
                    return (x * (*factor.borrow()));
                }
            ))
        }),
    ));
    (*factor.borrow_mut()) = 7;
    assert!((({ (*w.borrow()).call(2,) }) == 201));
    let ww: Value<FnPtr<fn(i32) -> i32>> =
        Rc::new(RefCell::new(({ wrap_2((*w.borrow()).copy_from()) })));
    assert!((({ (*ww.borrow()).call(2,) }) == 202));
    let m: Value<Moving__lambda_at_lambda_as_field_cpp_64_12__> = Rc::new(RefCell::new(
        Moving__lambda_at_lambda_as_field_cpp_64_12__::new({
            FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
                {
                    return (x * 2);
                }
            })
        }),
    ));
    let m2: Value<Moving__lambda_at_lambda_as_field_cpp_64_12__> = Rc::new(RefCell::new(
        Moving__lambda_at_lambda_as_field_cpp_64_12__::move_from({ m.as_pointer() }),
    ));
    assert!((({ (*m2.borrow()).f.call(3,) }) == 6));
    assert!(({ (*m2.borrow()).moves } == 1));
    return 0;
}
pub trait Guard__lambda_at_lambda_as_field_cpp_34_13__Impl {
    fn destructor(&self);
}
impl Guard__lambda_at_lambda_as_field_cpp_34_13__Impl
    for Ptr<Guard__lambda_at_lambda_as_field_cpp_34_13__>
{
    fn destructor(&self) {
        ({ (*(*self).upgrade().deref()).f.call() });
    }
}
pub trait Guard__lambda_at_lambda_as_field_cpp_45_13__Impl {
    fn destructor(&self);
}
impl Guard__lambda_at_lambda_as_field_cpp_45_13__Impl
    for Ptr<Guard__lambda_at_lambda_as_field_cpp_45_13__>
{
    fn destructor(&self) {
        ({ (*(*self).upgrade().deref()).f.call() });
    }
}
pub trait Holder__lambda_at_lambda_as_field_cpp_39_12__Impl {
    fn call(&self, x: i32) -> i32;
}
impl Holder__lambda_at_lambda_as_field_cpp_39_12__Impl
    for Ptr<Holder__lambda_at_lambda_as_field_cpp_39_12__>
{
    fn call(&self, mut x: i32) -> i32 {
        field!((*self), calls).with_mut(|__v| __v.postfix_inc());
        return ({ (*(*self).upgrade().deref()).f.call(x) });
    }
}
pub trait Holder__lambda_at_lambda_as_field_cpp_51_12__Impl {
    fn call(&self, x: i32) -> i32;
}
impl Holder__lambda_at_lambda_as_field_cpp_51_12__Impl
    for Ptr<Holder__lambda_at_lambda_as_field_cpp_51_12__>
{
    fn call(&self, mut x: i32) -> i32 {
        field!((*self), calls).with_mut(|__v| __v.postfix_inc());
        return ({ (*(*self).upgrade().deref()).f.call(x) });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = count_0.with(|_| ());
}
