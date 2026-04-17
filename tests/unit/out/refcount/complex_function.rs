extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::Seek;
use std::io::{Read, Write};
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
#[derive(Default)]
pub struct X1 {
    pub v: Value<i32>,
}
impl Clone for X1 {
    fn clone(&self) -> Self {
        let mut this = Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        };
        this
    }
}
impl ByteRepr for X1 {}
#[derive(Default)]
pub struct X2 {
    pub v: Ptr<X1>,
}
impl X2 {
    pub fn get(&self) -> Ptr<X1> {
        return (self.v).clone();
    }
}
impl Clone for X2 {
    fn clone(&self) -> Self {
        let mut this = Self {
            v: (self.v).clone(),
        };
        this
    }
}
impl ByteRepr for X2 {}
#[derive(Default)]
pub struct X3 {
    pub v: Value<Ptr<X2>>,
}
impl X3 {
    pub fn get(&self) -> Ptr<X2> {
        return (*self.v.borrow()).clone();
    }
}
impl Clone for X3 {
    fn clone(&self) -> Self {
        let mut this = Self {
            v: Rc::new(RefCell::new((*self.v.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for X3 {}
#[derive(Default)]
pub struct X4 {
    pub v: Value<X3>,
}
impl X4 {
    pub fn get(&self) -> Ptr<X3> {
        return self.v.as_pointer();
    }
}
impl Clone for X4 {
    fn clone(&self) -> Self {
        let mut this = Self {
            v: Rc::new(RefCell::new((*self.v.borrow()).clone())),
        };
        this
    }
}
impl ByteRepr for X4 {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x1: Value<i32> = Rc::new(RefCell::new(0));
    let x2: Value<i32> = Rc::new(RefCell::new(
        ({
            let _x: i32 = (*x1.borrow());
            foo_0(_x)
        }),
    ));
    let x3: Value<i32> = Rc::new(RefCell::new(
        ((({
            let _x: i32 = (*x2.borrow());
            foo_0(_x)
        }) + ({
            let _x: i32 = (*x1.borrow());
            foo_0(_x)
        })) + 1),
    ));
    (*x2.borrow_mut()) += 1;
    (*x2.borrow_mut()) += ({
        let _x: i32 = (*x1.borrow());
        foo_0(_x)
    });
    let __rhs = ((({
        let _x: i32 = (*x2.borrow());
        foo_0(_x)
    }) + ({
        let _x: i32 = (*x3.borrow());
        foo_0(_x)
    })) + 1);
    (*x3.borrow_mut()) += __rhs;
    let p1: Value<Ptr<i32>> = Rc::new(RefCell::new((x1.as_pointer())));
    let p2: Value<Ptr<i32>> = Rc::new(RefCell::new(
        ({
            let _x: Ptr<i32> = (*p1.borrow()).clone();
            ptr_1(_x)
        }),
    ));
    (*p1.borrow_mut()) = (*p2.borrow()).clone();
    (*p2.borrow_mut()) = ({
        let _x: Ptr<i32> = (*p1.borrow()).clone();
        ptr_1(_x)
    });
    let r1: Ptr<i32> = x1.as_pointer();
    let r2: Ptr<i32> = ({
        let _x: Ptr<i32> = x1.as_pointer();
        bar_2(_x)
    });
    let r3: Ptr<i32> = ({
        let _x: Ptr<i32> = (r1).clone();
        bar_2(_x)
    });
    let __rhs = (*x1.borrow());
    {
        let __ptr = r2.clone();
        let __tmp = __ptr.read() + __rhs;
        __ptr.write(__tmp)
    };
    let __rhs = (r1.read());
    {
        let __ptr = r3.clone();
        let __tmp = __ptr.read() + __rhs;
        __ptr.write(__tmp)
    };
    let x4: Value<i32> = Rc::new(RefCell::new(
        ((({
            let _x: i32 = (*x3.borrow());
            foo_0(_x)
        }) + (({
            let _x: Ptr<i32> = (x3.as_pointer());
            ptr_1(_x)
        })
        .read()))
            + (({
                let _x: Ptr<i32> = x2.as_pointer();
                bar_2(_x)
            })
            .read())),
    ));
    let a: Value<X1> = Rc::new(RefCell::new(X1 {
        v: Rc::new(RefCell::new(0)),
    }));
    let b: Value<X2> = Rc::new(RefCell::new(X2 { v: a.as_pointer() }));
    let c: Value<X3> = Rc::new(RefCell::new(X3 {
        v: Rc::new(RefCell::new((b.as_pointer()))),
    }));
    let d: Value<X4> = Rc::new(RefCell::new(X4 {
        v: Rc::new(RefCell::new((*c.borrow()).clone())),
    }));
    (*(*(*(*(*(*d.borrow()).v.borrow()).v.borrow()).upgrade().deref())
        .v
        .upgrade()
        .deref())
    .v
    .borrow_mut()) = 0;
    (*(*({
        (*({ (*({ (*d.borrow()).get() }).upgrade().deref()).get() })
            .upgrade()
            .deref())
        .get()
    })
    .upgrade()
    .deref())
    .v
    .borrow_mut()) = 0;
    (*(*(*d.borrow()).v.borrow()).v.borrow_mut()) = (b.as_pointer());
    let r4: Ptr<i32> = (*({
        (*({ (*({ (*d.borrow()).get() }).upgrade().deref()).get() })
            .upgrade()
            .deref())
        .get()
    })
    .upgrade()
    .deref())
    .v
    .as_pointer();
    let r5: Ptr<X1> = ({
        (*({ (*({ (*d.borrow()).get() }).upgrade().deref()).get() })
            .upgrade()
            .deref())
        .get()
    });
    let p: Value<Ptr<X2>> = Rc::new(RefCell::new(
        ({ (*({ (*d.borrow()).get() }).upgrade().deref()).get() }),
    ));
    let r6: Ptr<X3> = ({ (*d.borrow()).get() });
    let r7: Ptr<X3> = (*d.borrow()).v.as_pointer();
    let r8: Ptr<i32> = (*({ (*({ (*(*d.borrow()).v.borrow()).get() }).upgrade().deref()).get() })
        .upgrade()
        .deref())
    .v
    .as_pointer();
    let x5: Value<i32> = Rc::new(RefCell::new(
        (*(*({
            (*({ (*({ (*d.borrow()).get() }).upgrade().deref()).get() })
                .upgrade()
                .deref())
            .get()
        })
        .upgrade()
        .deref())
        .v
        .borrow()),
    ));
    {
        let __ptr = ({
            let _x: Ptr<i32> = x1.as_pointer();
            bar_2(_x)
        })
        .clone();
        let __tmp = __ptr.read() + 10;
        __ptr.write(__tmp)
    };
    ({
        let _x: Ptr<i32> = x1.as_pointer();
        bar_2(_x)
    })
    .with_mut(|__v| __v.postfix_inc());
    let bar_out: Value<i32> = Rc::new(RefCell::new(
        (({
            let _x: Ptr<i32> = (*({
                (*({ (*({ (*d.borrow()).get() }).upgrade().deref()).get() })
                    .upgrade()
                    .deref())
                .get()
            })
            .upgrade()
            .deref())
            .v
            .as_pointer();
            bar_2(_x)
        })
        .read()),
    ));
    let bar_inc: Value<i32> = Rc::new(RefCell::new(
        ({
            let _x: Ptr<i32> = x1.as_pointer();
            bar_2(_x)
        })
        .with_mut(|__v| __v.prefix_inc()),
    ));
    (*bar_inc.borrow_mut()) = ({
        let _x: Ptr<i32> = x1.as_pointer();
        bar_2(_x)
    })
    .with_mut(|__v| __v.postfix_inc());
    (*bar_inc.borrow_mut()) = (((({
        let _x: Ptr<i32> = x1.as_pointer();
        bar_2(_x)
    })
    .read())
        + ({
            let _x: i32 = (*x4.borrow());
            foo_0(_x)
        }))
        + 1);
    {
        let __ptr = ({
            let _x: Ptr<i32> = (*({
                (*({ (*({ (*d.borrow()).get() }).upgrade().deref()).get() })
                    .upgrade()
                    .deref())
                .get()
            })
            .upgrade()
            .deref())
            .v
            .as_pointer();
            bar_2(_x)
        })
        .clone();
        let __tmp = __ptr.read() + 10;
        __ptr.write(__tmp)
    };
    ({
        let _x: Ptr<i32> = (*({
            (*({ (*({ (*d.borrow()).get() }).upgrade().deref()).get() })
                .upgrade()
                .deref())
            .get()
        })
        .upgrade()
        .deref())
        .v
        .as_pointer();
        bar_2(_x)
    })
    .with_mut(|__v| __v.postfix_inc());
    let bar_inc2: Value<i32> = Rc::new(RefCell::new(
        ({
            let _x: Ptr<i32> = (*({
                (*({ (*({ (*d.borrow()).get() }).upgrade().deref()).get() })
                    .upgrade()
                    .deref())
                .get()
            })
            .upgrade()
            .deref())
            .v
            .as_pointer();
            bar_2(_x)
        })
        .with_mut(|__v| __v.prefix_inc()),
    ));
    (*bar_inc2.borrow_mut()) = ({
        let _x: Ptr<i32> = (*({
            (*({ (*({ (*d.borrow()).get() }).upgrade().deref()).get() })
                .upgrade()
                .deref())
            .get()
        })
        .upgrade()
        .deref())
        .v
        .as_pointer();
        bar_2(_x)
    })
    .with_mut(|__v| __v.postfix_inc());
    ({
        let _x: Ptr<i32> = (x1.as_pointer());
        ptr_1(_x)
    })
    .with_mut(|__v| __v.prefix_inc());
    {
        let __ptr = ({
            let _x: Ptr<i32> = (x1.as_pointer());
            ptr_1(_x)
        })
        .clone();
        let __tmp = __ptr.read() + 1;
        __ptr.write(__tmp)
    };
    ({
        let _x: Ptr<i32> = ((*({
            (*({ (*({ (*d.borrow()).get() }).upgrade().deref()).get() })
                .upgrade()
                .deref())
            .get()
        })
        .upgrade()
        .deref())
        .v
        .as_pointer());
        ptr_1(_x)
    })
    .with_mut(|__v| __v.prefix_inc());
    {
        let __ptr = ({
            let _x: Ptr<i32> = ((*({
                (*({ (*({ (*d.borrow()).get() }).upgrade().deref()).get() })
                    .upgrade()
                    .deref())
                .get()
            })
            .upgrade()
            .deref())
            .v
            .as_pointer());
            ptr_1(_x)
        })
        .clone();
        let __tmp = __ptr.read() + 1;
        __ptr.write(__tmp)
    };
    {
        let __ptr = ({
            let _x: Ptr<i32> = ((*({
                (*({ (*({ (*d.borrow()).get() }).upgrade().deref()).get() })
                    .upgrade()
                    .deref())
                .get()
            })
            .upgrade()
            .deref())
            .v
            .as_pointer());
            ptr_1(_x)
        })
        .clone();
        let __tmp = __ptr.read() + 1;
        __ptr.write(__tmp)
    };
    let ptr1: Value<i32> = Rc::new(RefCell::new(
        ({
            let _x: Ptr<i32> = ((*({
                (*({ (*({ (*d.borrow()).get() }).upgrade().deref()).get() })
                    .upgrade()
                    .deref())
                .get()
            })
            .upgrade()
            .deref())
            .v
            .as_pointer());
            ptr_1(_x)
        })
        .with_mut(|__v| __v.postfix_inc()),
    ));
    let ptr2: Ptr<i32> = ({
        let _x: Ptr<i32> = ((*({
            (*({ (*({ (*d.borrow()).get() }).upgrade().deref()).get() })
                .upgrade()
                .deref())
            .get()
        })
        .upgrade()
        .deref())
        .v
        .as_pointer());
        ptr_1(_x)
    });
    let ptr3: Value<Ptr<i32>> = Rc::new(RefCell::new(
        ({
            let _x: Ptr<i32> = ((*({
                (*({ (*({ (*d.borrow()).get() }).upgrade().deref()).get() })
                    .upgrade()
                    .deref())
                .get()
            })
            .upgrade()
            .deref())
            .v
            .as_pointer());
            ptr_1(_x)
        }),
    ));
    let vptr: Value<i32> = Rc::new(RefCell::new(
        (({
            let _x: Ptr<i32> = ((*({
                (*({ (*({ (*d.borrow()).get() }).upgrade().deref()).get() })
                    .upgrade()
                    .deref())
                .get()
            })
            .upgrade()
            .deref())
            .v
            .as_pointer());
            ptr_1(_x)
        })
        .read()),
    ));
    let pref: Value<Ptr<i32>> = Rc::new(RefCell::new(
        ({
            let _x: Ptr<i32> = (*({
                (*({ (*({ (*d.borrow()).get() }).upgrade().deref()).get() })
                    .upgrade()
                    .deref())
                .get()
            })
            .upgrade()
            .deref())
            .v
            .as_pointer();
            bar_2(_x)
        }),
    ));
    ({
        let _x: Ptr<i32> = (*({
            (*({ (*({ (*d.borrow()).get() }).upgrade().deref()).get() })
                .upgrade()
                .deref())
            .get()
        })
        .upgrade()
        .deref())
        .v
        .as_pointer();
        bar_2(_x)
    })
    .with_mut(|__v| __v.postfix_inc());
    return (((({
        let _x: Ptr<i32> = ((*({
            (*({ (*({ (*d.borrow()).get() }).upgrade().deref()).get() })
                .upgrade()
                .deref())
            .get()
        })
        .upgrade()
        .deref())
        .v
        .as_pointer());
        ptr_1(_x)
    })
    .read())
        + (({
            let _x: Ptr<i32> = (*({
                (*({ (*({ (*d.borrow()).get() }).upgrade().deref()).get() })
                    .upgrade()
                    .deref())
                .get()
            })
            .upgrade()
            .deref())
            .v
            .as_pointer();
            bar_2(_x)
        })
        .read()))
        + ({
            let _x: i32 = (*(*({
                (*({ (*({ (*d.borrow()).get() }).upgrade().deref()).get() })
                    .upgrade()
                    .deref())
                .get()
            })
            .upgrade()
            .deref())
            .v
            .borrow());
            foo_0(_x)
        }));
}
