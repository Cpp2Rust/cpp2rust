extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return (*x.borrow());
}
pub fn ptr_1(x: Ptr<i32>) -> Ptr<i32> {
    let x: Value<Ptr<i32>> = Rc::new(RefCell::new(x));
    return (*x.borrow()).clone();
}
pub fn bar_2(x: Ptr<i32>) -> Ptr<i32> {
    return (x).clone();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct X1 {
    #[offset(0)]
    pub v: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct X2 {
    #[offset(0)]
    #[byte_size(8)]
    pub v: Ptr<X1>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct X3 {
    #[offset(0)]
    #[byte_size(8)]
    pub v: Ptr<X2>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct X4 {
    #[offset(0)]
    #[byte_size(8)]
    pub v: X3,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x1: Value<i32> = Rc::new(RefCell::new(0));
    let x2: Value<i32> = Rc::new(RefCell::new(({ foo_0((*x1.borrow())) })));
    let x3: Value<i32> = Rc::new(RefCell::new(
        ((({ foo_0((*x2.borrow())) }) + ({ foo_0((*x1.borrow())) })) + 1),
    ));
    (*x2.borrow_mut()) += 1;
    (*x2.borrow_mut()) += ({ foo_0((*x1.borrow())) });
    let __rhs = ((({ foo_0((*x2.borrow())) }) + ({ foo_0((*x3.borrow())) })) + 1);
    (*x3.borrow_mut()) += __rhs;
    let p1: Value<Ptr<i32>> = Rc::new(RefCell::new((x1.as_pointer())));
    let p2: Value<Ptr<i32>> = Rc::new(RefCell::new(({ ptr_1((*p1.borrow()).clone()) })));
    (*p1.borrow_mut()) = (*p2.borrow()).clone();
    (*p2.borrow_mut()) = ({ ptr_1((*p1.borrow()).clone()) });
    let r1: Ptr<i32> = x1.as_pointer();
    let r2: Ptr<i32> = ({ bar_2(x1.as_pointer()) });
    let r3: Ptr<i32> = ({ bar_2((r1).clone()) });
    {
        let _ptr = r2.clone();
        _ptr.write(_ptr.read() + { (*x1.borrow()) })
    };
    {
        let _ptr = r3.clone();
        _ptr.write(_ptr.read() + { (r1.read()) })
    };
    let x4: Value<i32> = Rc::new(RefCell::new(
        ((({ foo_0((*x3.borrow())) }) + (({ ptr_1((x3.as_pointer())) }).read()))
            + (({ bar_2(x2.as_pointer()) }).read())),
    ));
    let a: Value<X1> = Rc::new(RefCell::new(X1 { v: 0 }));
    let b: Value<X2> = Rc::new(RefCell::new(X2 { v: a.as_pointer() }));
    let c: Value<X3> = Rc::new(RefCell::new(X3 {
        v: (b.as_pointer()),
    }));
    let d: Value<X4> = Rc::new(RefCell::new(X4 {
        v: (*c.borrow()).clone(),
    }));
    field!({ (*d.borrow()).v.v.clone() }.with(|__s| __s.v.clone()), v).write(0);
    field!(
        ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
        v
    )
    .write(0);
    (*d.borrow_mut()).v.v = (b.as_pointer());
    let r4: Ptr<i32> = field_ptr!(
        ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
        v
    );
    let r5: Ptr<X1> = ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer()) })) })) });
    let p: Value<Ptr<X2>> = Rc::new(RefCell::new(
        ({ X3Impl::get(&({ X4Impl::get(&d.as_pointer()) })) }),
    ));
    let r6: Ptr<X3> = ({ X4Impl::get(&d.as_pointer()) });
    let r7: Ptr<X3> = field_ptr!(d.as_pointer(), v);
    let r8: Ptr<i32> = field_ptr!(
        ({ X2Impl::get(&({ X3Impl::get(&field_ptr!(d.as_pointer(), v),) }),) }),
        v
    );
    let x5: Value<i32> = Rc::new(RefCell::new(
        ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer()) })) })) })
            .with(|__s| __s.v),
    ));
    {
        let _ptr = ({ bar_2(x1.as_pointer()) });
        _ptr.write(_ptr.read() + 10)
    };
    ({ bar_2(x1.as_pointer()) }).with_mut(|__v| __v.postfix_inc());
    let bar_out: Value<i32> = Rc::new(RefCell::new(
        (({
            bar_2(field_ptr!(
                ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
                v
            ))
        })
        .read()),
    ));
    let bar_inc: Value<i32> = Rc::new(RefCell::new(
        ({ bar_2(x1.as_pointer()) }).with_mut(|__v| __v.prefix_inc()),
    ));
    (*bar_inc.borrow_mut()) = ({ bar_2(x1.as_pointer()) }).with_mut(|__v| __v.postfix_inc());
    (*bar_inc.borrow_mut()) =
        (((({ bar_2(x1.as_pointer()) }).read()) + ({ foo_0((*x4.borrow())) })) + 1);
    {
        let _ptr = ({
            bar_2(field_ptr!(
                ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
                v
            ))
        });
        _ptr.write(_ptr.read() + 10)
    };
    ({
        bar_2(field_ptr!(
            ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
            v
        ))
    })
    .with_mut(|__v| __v.postfix_inc());
    let bar_inc2: Value<i32> = Rc::new(RefCell::new(
        ({
            bar_2(field_ptr!(
                ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
                v
            ))
        })
        .with_mut(|__v| __v.prefix_inc()),
    ));
    (*bar_inc2.borrow_mut()) = ({
        bar_2(field_ptr!(
            ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
            v
        ))
    })
    .with_mut(|__v| __v.postfix_inc());
    ({ ptr_1((x1.as_pointer())) }).with_mut(|__v| __v.prefix_inc());
    {
        let _ptr = ({ ptr_1((x1.as_pointer())) });
        _ptr.write(_ptr.read() + 1)
    };
    ({
        ptr_1(
            (field_ptr!(
                ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
                v
            )),
        )
    })
    .with_mut(|__v| __v.prefix_inc());
    {
        let _ptr = ({
            ptr_1(
                (field_ptr!(
                    ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
                    v
                )),
            )
        });
        _ptr.write(_ptr.read() + 1)
    };
    {
        let _ptr = ({
            ptr_1(
                (field_ptr!(
                    ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
                    v
                )),
            )
        });
        _ptr.write(_ptr.read() + 1)
    };
    let ptr1: Value<i32> = Rc::new(RefCell::new(
        ({
            ptr_1(
                (field_ptr!(
                    ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
                    v
                )),
            )
        })
        .with_mut(|__v| __v.postfix_inc()),
    ));
    let ptr2: Ptr<i32> = ({
        ptr_1(
            (field_ptr!(
                ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
                v
            )),
        )
    });
    let ptr3: Value<Ptr<i32>> = Rc::new(RefCell::new(
        ({
            ptr_1(
                (field_ptr!(
                    ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
                    v
                )),
            )
        }),
    ));
    let vptr: Value<i32> = Rc::new(RefCell::new(
        (({
            ptr_1(
                (field_ptr!(
                    ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
                    v
                )),
            )
        })
        .read()),
    ));
    let pref: Value<Ptr<i32>> = Rc::new(RefCell::new(
        ({
            bar_2(field_ptr!(
                ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
                v
            ))
        }),
    ));
    ({
        bar_2(field_ptr!(
            ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
            v
        ))
    })
    .with_mut(|__v| __v.postfix_inc());
    assert!(
        ((((({
            ptr_1(
                (field_ptr!(
                    ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
                    v
                )),
            )
        })
        .read())
            + (({
                bar_2(field_ptr!(
                    ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer(),) }),) }),) }),
                    v
                ))
            })
            .read()))
            + ({
                foo_0(
                    ({ X2Impl::get(&({ X3Impl::get(&({ X4Impl::get(&d.as_pointer()) })) })) })
                        .with(|__s| __s.v),
                )
            }))
            == 54)
    );
    return 0;
}
pub trait X2Impl {
    fn get(&self) -> Ptr<X1>;
}
impl X2Impl for Ptr<X2> {
    fn get(&self) -> Ptr<X1> {
        return ((*self).with(|__s| __s.v.clone())).clone();
    }
}
pub trait X3Impl {
    fn get(&self) -> Ptr<X2>;
}
impl X3Impl for Ptr<X3> {
    fn get(&self) -> Ptr<X2> {
        return (*self).with(|__s| __s.v.clone());
    }
}
pub trait X4Impl {
    fn get(&self) -> Ptr<X3>;
}
impl X4Impl for Ptr<X4> {
    fn get(&self) -> Ptr<X3> {
        return field_ptr!((*self), v);
    }
}
pub fn __cpp2rust_init_globals() {}
