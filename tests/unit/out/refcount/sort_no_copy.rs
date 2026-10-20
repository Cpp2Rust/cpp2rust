extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static copies_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign, MoveAssign)]
#[byte_size(4)]
pub struct Tracked {
    #[offset(0)]
    pub v: i32,
}
impl Tracked {
    pub fn new_1() -> Self {
        Self { v: 0 }
    }
    pub fn new_2(mut v: i32) -> Self {
        Self { v: v }
    }
    pub fn copy_from(o: Ptr<Tracked>) -> Self {
        let __this: Tracked = Self {
            v: o.with(|__s| __s.v),
        };
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn move_from(o: Ptr<Tracked>) -> Self {
        Self {
            v: o.with(|__s| __s.v),
        }
    }
}
impl std::cmp::Ord for Tracked {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_1(
                Rc::new(RefCell::new(Tracked { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Tracked { v: other.v.clone() })).as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_1(
                Rc::new(RefCell::new(Tracked { v: other.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Tracked { v: self.v.clone() })).as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Tracked {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Tracked {
    fn eq(&self, other: &Self) -> bool {
        {
            !(operator_lt_1(
                Rc::new(RefCell::new(Tracked { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Tracked { v: other.v.clone() })).as_pointer(),
            )) && !(operator_lt_1(
                Rc::new(RefCell::new(Tracked { v: other.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Tracked { v: self.v.clone() })).as_pointer(),
            ))
        }
    }
}
impl std::cmp::Eq for Tracked {}
impl Clone for Tracked {
    fn clone(&self) -> Self {
        let __src: Value<Tracked> = Rc::new(RefCell::new(Tracked { v: self.v.clone() }));
        Tracked::copy_from(__src.as_pointer())
    }
}
impl Default for Tracked {
    fn default() -> Self {
        { Tracked::new_1() }
    }
}
pub fn operator_lt_1(a: Ptr<Tracked>, b: Ptr<Tracked>) -> bool {
    return ({ a.with(|__s| __s.v) } < { b.with(|__s| __s.v) });
}
pub fn greater_2(a: Ptr<Tracked>, b: Ptr<Tracked>) -> bool {
    return ({ a.with(|__s| __s.v) } > { b.with(|__s| __s.v) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let arr: Value<Box<[Tracked]>> = Rc::new(RefCell::new(Box::new([
        Tracked::new_2({ 3 }),
        Tracked::new_2({ 1 }),
        Tracked::new_2({ 4 }),
        Tracked::new_2({ 1 }),
        Tracked::new_2({ 5 }),
    ])));
    {
        let __n = (arr.as_pointer() as Ptr<Tracked>)
            .offset((5) as isize)
            .iter_distance(&(arr.as_pointer() as Ptr<Tracked>));
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| {
            (arr.as_pointer() as Ptr<Tracked>).iter_at(__x).with(|__a| {
                (arr.as_pointer() as Ptr<Tracked>)
                    .iter_at(__y)
                    .with(|__b| __a.cmp(__b))
            })
        });
        reorder(__order, |__i| {
            (arr.as_pointer() as Ptr<Tracked>).iter_at(__i)
        });
    };
    assert!(
        (((({ (*arr.borrow())[(0) as usize].v } == 1)
            && ({ (*arr.borrow())[(1) as usize].v } == 1))
            && ({ (*arr.borrow())[(2) as usize].v } == 3))
            && ({ (*arr.borrow())[(3) as usize].v } == 4))
            && ({ (*arr.borrow())[(4) as usize].v } == 5)
    );
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    let vec_: Value<Vec<Tracked>> = Rc::new(RefCell::new(Vec::new()));
    if 4_usize as usize > (*vec_.borrow()).capacity() as usize {
        let len_0 = (*vec_.borrow()).len();
        (*vec_.borrow_mut()).reserve_exact(4_usize as usize - len_0 as usize);
    };
    {
        let __init = Tracked::new_2({ 2 });
        (*vec_.borrow_mut()).push(__init)
    };
    {
        let __init = Tracked::new_2({ 7 });
        (*vec_.borrow_mut()).push(__init)
    };
    {
        let __init = Tracked::new_2({ 1 });
        (*vec_.borrow_mut()).push(__init)
    };
    {
        let __init = Tracked::new_2({ 8 });
        (*vec_.borrow_mut()).push(__init)
    };
    {
        let __n = (vec_.as_pointer() as Ptr<Tracked>)
            .end()
            .iter_distance(&(vec_.as_pointer() as Ptr<Tracked>).begin());
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| {
            (vec_.as_pointer() as Ptr<Tracked>)
                .begin()
                .iter_at(__x)
                .with(|__a| {
                    (vec_.as_pointer() as Ptr<Tracked>)
                        .begin()
                        .iter_at(__y)
                        .with(|__b| __a.cmp(__b))
                })
        });
        reorder(__order, |__i| {
            (vec_.as_pointer() as Ptr<Tracked>).begin().iter_at(__i)
        });
    };
    assert!(
        ((({ (*vec_.borrow())[0_usize].v } == 1) && ({ (*vec_.borrow())[1_usize].v } == 2))
            && ({ (*vec_.borrow())[2_usize].v } == 7))
            && ({ (*vec_.borrow())[3_usize].v } == 8)
    );
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    {
        let __n = (vec_.as_pointer() as Ptr<Tracked>)
            .end()
            .iter_distance(&(vec_.as_pointer() as Ptr<Tracked>).begin());
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| {
            let __a = (vec_.as_pointer() as Ptr<Tracked>).begin().iter_at(__x);
            let __b = (vec_.as_pointer() as Ptr<Tracked>).begin().iter_at(__y);
            if greater_2.call(__a.clone(), __b.clone()) {
                std::cmp::Ordering::Less
            } else if greater_2.call(__b, __a) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        reorder(__order, |__i| {
            (vec_.as_pointer() as Ptr<Tracked>).begin().iter_at(__i)
        });
    };
    assert!(({ (*vec_.borrow())[0_usize].v } == 8) && ({ (*vec_.borrow())[3_usize].v } == 1));
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    {
        let __n = (vec_.as_pointer() as Ptr<Tracked>)
            .end()
            .iter_distance(&(vec_.as_pointer() as Ptr<Tracked>).begin());
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| {
            let __a = (vec_.as_pointer() as Ptr<Tracked>).begin().iter_at(__x);
            let __b = (vec_.as_pointer() as Ptr<Tracked>).begin().iter_at(__y);
            if FnPtr::<fn(Ptr<Tracked>, Ptr<Tracked>) -> bool>::new(
                |a: Ptr<Tracked>, b: Ptr<Tracked>| -> bool {
                    {
                        return ({ a.with(|__s| __s.v) } < { b.with(|__s| __s.v) });
                    }
                },
            )
            .call(__a.clone(), __b.clone())
            {
                std::cmp::Ordering::Less
            } else if FnPtr::<fn(Ptr<Tracked>, Ptr<Tracked>) -> bool>::new(
                |a: Ptr<Tracked>, b: Ptr<Tracked>| -> bool {
                    {
                        return ({ a.with(|__s| __s.v) } < { b.with(|__s| __s.v) });
                    }
                },
            )
            .call(__b, __a)
            {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        reorder(__order, |__i| {
            (vec_.as_pointer() as Ptr<Tracked>).begin().iter_at(__i)
        });
    };
    assert!(({ (*vec_.borrow())[0_usize].v } == 1) && ({ (*vec_.borrow())[3_usize].v } == 8));
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    {
        let __n = (vec_.as_pointer() as Ptr<Tracked>).end().iter_distance(
            &(vec_.as_pointer() as Ptr<Tracked>)
                .begin()
                .iter_add(1_isize),
        );
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| {
            let __a = (vec_.as_pointer() as Ptr<Tracked>)
                .begin()
                .iter_add(1_isize)
                .iter_at(__x);
            let __b = (vec_.as_pointer() as Ptr<Tracked>)
                .begin()
                .iter_add(1_isize)
                .iter_at(__y);
            if greater_2.call(__a.clone(), __b.clone()) {
                std::cmp::Ordering::Less
            } else if greater_2.call(__b, __a) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        reorder(__order, |__i| {
            (vec_.as_pointer() as Ptr<Tracked>)
                .begin()
                .iter_add(1_isize)
                .iter_at(__i)
        });
    };
    assert!(
        (({ (*vec_.borrow())[0_usize].v } == 1) && ({ (*vec_.borrow())[1_usize].v } == 8))
            && ({ (*vec_.borrow())[3_usize].v } == 2)
    );
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    return copies_0.with(|rc| *rc.borrow());
}
pub trait TrackedImpl {
    fn copy_assign(&self, o: Ptr<Tracked>) -> Ptr<Tracked>;
    fn move_assign(&self, o: Ptr<Tracked>) -> Ptr<Tracked>;
}
impl TrackedImpl for Ptr<Tracked> {
    fn copy_assign(&self, o: Ptr<Tracked>) -> Ptr<Tracked> {
        field!((*self), v).write({ o.with(|__s| __s.v) });
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
        return (*self).clone();
    }
    fn move_assign(&self, o: Ptr<Tracked>) -> Ptr<Tracked> {
        field!((*self), v).write({ o.with(|__s| __s.v) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = copies_0.with(|_| ());
}
