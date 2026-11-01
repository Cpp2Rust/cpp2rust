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
pub fn size_of_2(mut i: i32) -> usize {
    (*calls_0.with(Value::clone).borrow_mut()).prefix_inc();
    return ((i * 3) as usize);
}
pub fn pick_3(mut i: i32) -> Ptr<i32> {
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
        operator_lt_4(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for Bag {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(operator_lt_4(__lhs.clone(), __rhs.clone()))
            && !(operator_lt_4(__rhs.clone(), __lhs.clone()))
    }
}
impl Clone for Bag {
    fn clone(&self) -> Self {
        let __src: Value<Bag> = Rc::new(RefCell::new(Bag { n: self.n.clone() }));
        Bag::copy_from(__src.as_pointer())
    }
}
pub fn operator_lt_4(a: Ptr<Bag>, b: Ptr<Bag>) -> bool {
    return ({ a.with(|__s| __s.n) } < { b.with(|__s| __s.n) });
}
pub fn make_5(mut n: i32) -> Bag {
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
    let mut r1: i32 = (if CppLess::less(&x1.as_pointer(), &x2.as_pointer()) {
        x2.as_pointer()
    } else {
        x1.as_pointer()
    }
    .read());
    let mut r2: i32 = (if CppLess::less(&x4.as_pointer(), &x3.as_pointer()) {
        x4.as_pointer()
    } else {
        x3.as_pointer()
    }
    .read());
    let mut r3: i32 = (if CppLess::less(&(p1).clone(), &x2.as_pointer()) {
        x2.as_pointer()
    } else {
        (p1).clone()
    }
    .read());
    let mut r4: i32 = (if CppLess::less(&x3.as_pointer(), &(p2).clone()) {
        x3.as_pointer()
    } else {
        (p2).clone()
    }
    .read());
    let mut r5: i32 = {
        let __tmp_0: Value<i32> = Rc::new(RefCell::new(30));
        let __tmp_1: Value<i32> = Rc::new(RefCell::new(40));
        (if CppLess::less(&__tmp_0.as_pointer(), &__tmp_1.as_pointer()) {
            __tmp_1.as_pointer()
        } else {
            __tmp_0.as_pointer()
        }
        .read())
    };
    assert!((((((r1 + r2) + r3) + r4) + r5) == 56));
    let values: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 7, 7, 3])));
    let mut max: Ptr<i32> = match ({ (values.as_pointer() as Ptr<i32>) }, {
        (values.as_pointer() as Ptr<i32>).offset((4) as isize)
    }) {
        (mut __tmp_2, mut __tmp_3) => {
            let mut __best = __tmp_2.clone();
            if __tmp_2 != __tmp_3 {
                let mut __it = (__tmp_2).clone();
                __it.iter_inc();
                while __it != __tmp_3 {
                    if CppLess::less(&__best.iter_deref(), &__it.iter_deref()) {
                        __best = __it.clone();
                    }
                    __it.iter_inc();
                }
            }
            __best
        }
    };
    assert!(({ (max).clone() } == { (values.as_pointer() as Ptr::<i32>).offset((1) as isize) }));
    assert!(
        ((match (({ pick_3(0,) }), ({ pick_3(1,) }),) {
            (__tmp_4, __tmp_5) => {
                if CppLess::less(&(__tmp_4).clone(), &(__tmp_5).clone()) {
                    (__tmp_5).clone()
                } else {
                    (__tmp_4).clone()
                }
            }
        }
        .read())
            == 2)
            && (calls_0.with(|rc| *rc.borrow()) == 2)
    );
    assert!(
        ((match (({ pick_3(1,) }), ({ pick_3(0,) }),) {
            (__tmp_6, __tmp_7) => {
                if CppLess::less(&(__tmp_7).clone(), &(__tmp_6).clone()) {
                    (__tmp_7).clone()
                } else {
                    (__tmp_6).clone()
                }
            }
        }
        .read())
            == 1)
            && (calls_0.with(|rc| *rc.borrow()) == 4)
    );
    calls_0.with(|rc| *rc.borrow_mut() = 0);
    assert!(
        (match (
            Rc::new(RefCell::new(({ make_5(1,) }))).as_pointer(),
            Rc::new(RefCell::new(({ make_5(2,) }))).as_pointer(),
        ) {
            (__tmp_8, __tmp_9) => {
                if CppLess::less(&(__tmp_8).clone(), &(__tmp_9).clone()) {
                    (__tmp_9).clone()
                } else {
                    (__tmp_8).clone()
                }
            }
        }
        .with(|__s| __s.n)
            == 2)
            && (match (
                Rc::new(RefCell::new(({ make_5(1,) }))).as_pointer(),
                Rc::new(RefCell::new(({ make_5(2,) }))).as_pointer(),
            ) {
                (__tmp_10, __tmp_11) => {
                    if CppLess::less(&(__tmp_11).clone(), &(__tmp_10).clone()) {
                        (__tmp_11).clone()
                    } else {
                        (__tmp_10).clone()
                    }
                }
            }
            .with(|__s| __s.n)
                == 1)
    );
    assert!((calls_0.with(|rc| *rc.borrow()) == 0));
    let mut largest: usize = 0_usize;
    let mut i: i32 = 0;
    'loop_: while (i < 3) {
        let __rhs = ({
            let __tmp_13: Value<u64> = Rc::new(RefCell::new((largest as u64)));
            (match (Rc::new(RefCell::new((({ size_of_2(i) }) as u64))).as_pointer(),) {
                (__tmp_12,) => {
                    if CppLess::less(&__tmp_13.as_pointer(), &(__tmp_12).clone()) {
                        (__tmp_12).clone()
                    } else {
                        __tmp_13.as_pointer()
                    }
                }
            }
            .read())
        } as usize);
        largest = __rhs;
        i.prefix_inc();
    }
    assert!((largest == 6_usize) && (calls_0.with(|rc| *rc.borrow()) == 3));
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
