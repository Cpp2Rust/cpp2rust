extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static calls_0: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static xs_1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2])));
);
pub fn pick_2(mut i: i32) -> Ptr<i32> {
    (*calls_0.with(Value::clone).borrow_mut()).prefix_inc();
    return (xs_1.with(|v| v.as_pointer()) as Ptr<i32>).offset(i);
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CppFrom, CopyAssign, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct Bag {
    #[offset(0)]
    pub n: i32,
}
impl Bag {
    pub fn new(mut n: i32) -> Self {
        Self { n: n }
    }
    pub fn copy_from(o: Ptr<Bag>) -> Self {
        let __this: Bag = Self {
            n: o.with(|__s| __s.n),
        };
        (*calls_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl CppLess for Bag {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_3(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for Bag {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(operator_lt_3(__lhs.clone(), __rhs.clone()))
            && !(operator_lt_3(__rhs.clone(), __lhs.clone()))
    }
}
impl Clone for Bag {
    fn clone(&self) -> Self {
        let __src: Value<Bag> = Rc::new(RefCell::new(Bag { n: self.n.clone() }));
        Bag::copy_from(__src.as_pointer())
    }
}
pub fn operator_lt_3(a: Ptr<Bag>, b: Ptr<Bag>) -> bool {
    return ({ a.with(|__s| __s.n) } < { b.with(|__s| __s.n) });
}
pub fn make_4(mut n: i32) -> Bag {
    return Bag::new({ n });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x1: Value<i32> = Rc::new(RefCell::new(1));
    let x2: Value<i32> = Rc::new(RefCell::new(2));
    let x3: Value<i32> = Rc::new(RefCell::new(10));
    let x4: Value<i32> = Rc::new(RefCell::new(20));
    let mut p1: Ptr<i32> = (x1.as_pointer());
    let mut p2: Ptr<i32> = (x2.as_pointer());
    let mut r1: i32 = ({
        let __a = x1.as_pointer();
        let __b = x2.as_pointer();
        if CppLess::less(&__a, &__b) { __b } else { __a }
    }
    .read());
    let mut r2: i32 = ({
        let __a = x3.as_pointer();
        let __b = x4.as_pointer();
        if CppLess::less(&__b, &__a) { __b } else { __a }
    }
    .read());
    let mut r3: i32 = ({
        let __a = (p1).clone();
        let __b = x2.as_pointer();
        if CppLess::less(&__a, &__b) { __b } else { __a }
    }
    .read());
    let mut r4: i32 = ({
        let __a = (p2).clone();
        let __b = x3.as_pointer();
        if CppLess::less(&__b, &__a) { __b } else { __a }
    }
    .read());
    let mut r5: i32 = {
        let __tmp_0: Value<i32> = Rc::new(RefCell::new(30));
        let __tmp_1: Value<i32> = Rc::new(RefCell::new(40));
        ({
            let __a = __tmp_0.as_pointer();
            let __b = __tmp_1.as_pointer();
            if CppLess::less(&__a, &__b) { __b } else { __a }
        }
        .read())
    };
    assert!((((((r1 + r2) + r3) + r4) + r5) == 56));
    let values: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 7, 7, 3])));
    let mut max: Ptr<i32> = {
        let __first = (values.as_pointer() as Ptr<i32>);
        let __last = (values.as_pointer() as Ptr<i32>).offset((4) as isize);
        let mut __best = __first.clone();
        if __first != __last {
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if CppLess::less(&__best.iter_deref(), &__it.iter_deref()) {
                    __best = __it.clone();
                }
                __it.iter_inc();
            }
        }
        __best
    };
    assert!(({ (max).clone() } == { (values.as_pointer() as Ptr::<i32>).offset((1) as isize) }));
    assert!(
        (({
            let __a = ({ pick_2(0) });
            let __b = ({ pick_2(1) });
            if CppLess::less(&__a, &__b) { __b } else { __a }
        }
        .read())
            == 2)
            && (calls_0.with(|rc| *rc.borrow()) == 2)
    );
    assert!(
        (({
            let __a = ({ pick_2(1) });
            let __b = ({ pick_2(0) });
            if CppLess::less(&__b, &__a) { __b } else { __a }
        }
        .read())
            == 1)
            && (calls_0.with(|rc| *rc.borrow()) == 4)
    );
    calls_0.with(|rc| *rc.borrow_mut() = 0);
    assert!(
        ({
            let __tmp_2: Value<Bag> = Rc::new(RefCell::new(({ make_4(1) })));
            let __tmp_3: Value<Bag> = Rc::new(RefCell::new(({ make_4(2) })));
            {
                let __a = __tmp_2.as_pointer();
                let __b = __tmp_3.as_pointer();
                if CppLess::less(&__a, &__b) { __b } else { __a }
            }
            .with(|__s| __s.n)
        } == 2)
            && ({
                let __tmp_4: Value<Bag> = Rc::new(RefCell::new(({ make_4(1) })));
                let __tmp_5: Value<Bag> = Rc::new(RefCell::new(({ make_4(2) })));
                {
                    let __a = __tmp_4.as_pointer();
                    let __b = __tmp_5.as_pointer();
                    if CppLess::less(&__b, &__a) { __b } else { __a }
                }
                .with(|__s| __s.n)
            } == 1)
    );
    assert!((calls_0.with(|rc| *rc.borrow()) == 0));
    return 0;
}
pub trait BagImpl {
    fn copy_assign(&self, _a0: Ptr<Bag>) -> Ptr<Bag>;
}
impl BagImpl for Ptr<Bag> {
    fn copy_assign(&self, _a0: Ptr<Bag>) -> Ptr<Bag> {
        field!((*self), n).write({ _a0.with(|__s| __s.n) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = calls_0.with(|_| ());
    let _ = xs_1.with(|_| ());
}
