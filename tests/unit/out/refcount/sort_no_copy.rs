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
impl CppLess for Tracked {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_1(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for Tracked {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(operator_lt_1(__lhs.clone(), __rhs.clone()))
            && !(operator_lt_1(__rhs.clone(), __lhs.clone()))
    }
}
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(24)]
pub struct Bag {
    #[offset(0)]
    #[byte_size(24)]
    pub items: Vec<Tracked>,
}
impl Bag {
    pub fn move_from(_a0: Ptr<Bag>) -> Self {
        Self {
            items: field!(_a0, items).with_mut(std::mem::take),
        }
    }
}
impl CppLess for Bag {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_3(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for Bag {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_4(__lhs.clone(), __rhs.clone())
    }
}
pub fn operator_lt_3(a: Ptr<Bag>, b: Ptr<Bag>) -> bool {
    return ({ (*a.upgrade().deref()).items.len() } < { (*b.upgrade().deref()).items.len() });
}
pub fn operator_eq_4(a: Ptr<Bag>, b: Ptr<Bag>) -> bool {
    return ({ (*a.upgrade().deref()).items.len() } == { (*b.upgrade().deref()).items.len() });
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
        let __first = (arr.as_pointer() as Ptr<Tracked>);
        let __n = (arr.as_pointer() as Ptr<Tracked>)
            .offset((5) as isize)
            .iter_distance(&__first);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| cpp_cmp(&__first.iter_at(__x), &__first.iter_at(__y)));
        reorder(__order, |__i| __first.iter_at(__i));
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
        let __first = vec_.decay().begin();
        let __n = vec_.decay().end().iter_distance(&__first);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| cpp_cmp(&__first.iter_at(__x), &__first.iter_at(__y)));
        reorder(__order, |__i| __first.iter_at(__i));
    };
    assert!(
        ((({ (*vec_.borrow())[0_usize].v } == 1) && ({ (*vec_.borrow())[1_usize].v } == 2))
            && ({ (*vec_.borrow())[2_usize].v } == 7))
            && ({ (*vec_.borrow())[3_usize].v } == 8)
    );
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    {
        let __first = vec_.decay().begin();
        let __n = vec_.decay().end().iter_distance(&__first);
        let __cmp = greater_2;
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| {
            let __a = __first.iter_at(__x);
            let __b = __first.iter_at(__y);
            if __cmp.call(__a.clone(), __b.clone()) {
                std::cmp::Ordering::Less
            } else if __cmp.call(__b, __a) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        reorder(__order, |__i| __first.iter_at(__i));
    };
    assert!(({ (*vec_.borrow())[0_usize].v } == 8) && ({ (*vec_.borrow())[3_usize].v } == 1));
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    {
        let __first = vec_.decay().begin();
        let __n = vec_.decay().end().iter_distance(&__first);
        let __cmp = FnPtr::<fn(Ptr<Tracked>, Ptr<Tracked>) -> bool>::new(
            |a: Ptr<Tracked>, b: Ptr<Tracked>| -> bool {
                {
                    return ({ a.with(|__s| __s.v) } < { b.with(|__s| __s.v) });
                }
            },
        );
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| {
            let __a = __first.iter_at(__x);
            let __b = __first.iter_at(__y);
            if __cmp.call(__a.clone(), __b.clone()) {
                std::cmp::Ordering::Less
            } else if __cmp.call(__b, __a) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        reorder(__order, |__i| __first.iter_at(__i));
    };
    assert!(({ (*vec_.borrow())[0_usize].v } == 1) && ({ (*vec_.borrow())[3_usize].v } == 8));
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    {
        let __first = vec_.decay().begin().iter_add(1_isize);
        let __n = vec_.decay().end().iter_distance(&__first);
        let __cmp = greater_2;
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| {
            let __a = __first.iter_at(__x);
            let __b = __first.iter_at(__y);
            if __cmp.call(__a.clone(), __b.clone()) {
                std::cmp::Ordering::Less
            } else if __cmp.call(__b, __a) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        reorder(__order, |__i| __first.iter_at(__i));
    };
    assert!(
        (({ (*vec_.borrow())[0_usize].v } == 1) && ({ (*vec_.borrow())[1_usize].v } == 8))
            && ({ (*vec_.borrow())[3_usize].v } == 2)
    );
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    let bags: Value<Box<[Bag]>> = Rc::new(RefCell::new(
        (0..3).map(|_| <Bag>::default()).collect::<Box<[Bag]>>(),
    ));
    {
        let __init = Tracked::new_2({ 1 });
        (*bags.borrow_mut())[(0) as usize].items.push(__init)
    };
    {
        let __init = Tracked::new_2({ 2 });
        (*bags.borrow_mut())[(0) as usize].items.push(__init)
    };
    {
        let __init = Tracked::new_2({ 3 });
        (*bags.borrow_mut())[(2) as usize].items.push(__init)
    };
    {
        let __first = (bags.as_pointer() as Ptr<Bag>);
        let __n = (bags.as_pointer() as Ptr<Bag>)
            .offset((3) as isize)
            .iter_distance(&__first);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| cpp_cmp(&__first.iter_at(__x), &__first.iter_at(__y)));
        reorder(__order, |__i| __first.iter_at(__i));
    };
    assert!(
        (((*bags.borrow())[(0) as usize].items.is_empty())
            && ((*bags.borrow())[(1) as usize].items.len() == 1_usize))
            && ((*bags.borrow())[(2) as usize].items.len() == 2_usize)
    );
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    let bag_vec: Value<Vec<Bag>> = Rc::new(RefCell::new(
        (0..(3_usize) as usize)
            .map(|_| <Bag>::default())
            .collect::<Vec<_>>(),
    ));
    {
        let __init = Tracked::new_2({ 4 });
        (*bag_vec.borrow_mut())[1_usize].items.push(__init)
    };
    {
        let __init = Tracked::new_2({ 5 });
        (*bag_vec.borrow_mut())[1_usize].items.push(__init)
    };
    {
        let __init = Tracked::new_2({ 6 });
        (*bag_vec.borrow_mut())[2_usize].items.push(__init)
    };
    {
        let __first = bag_vec.decay().begin();
        let __n = bag_vec.decay().end().iter_distance(&__first);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| cpp_cmp(&__first.iter_at(__x), &__first.iter_at(__y)));
        reorder(__order, |__i| __first.iter_at(__i));
    };
    assert!(
        (((*bag_vec.borrow())[0_usize].items.is_empty())
            && ((*bag_vec.borrow())[1_usize].items.len() == 1_usize))
            && (vec_elem!(bag_vec.decay().offset(2_usize), items, 1_usize).with(|__s| __s.v) == 5)
    );
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    assert!(
        {
            let mut __it = bag_vec.decay().begin();
            let __last = bag_vec.decay().end();
            let __value = (bags.as_pointer() as Ptr<Bag>).offset(1);
            while __it != __last && !CppEq::equal(&__it.iter_deref(), &__value) {
                __it.iter_inc();
            }
            __it
        } == bag_vec.decay().begin().iter_add(1_isize)
    );
    assert!(
        {
            let __first = bag_vec.decay().begin();
            let __last = bag_vec.decay().end();
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
        } == bag_vec.decay().begin().iter_add(2_isize)
    );
    assert!(
        (({
            let __a = (bags.as_pointer() as Ptr<Bag>).offset(0);
            let __b = (bags.as_pointer() as Ptr<Bag>).offset(2);
            if CppLess::less(&__b, &__a) { __b } else { __a }
        }) == ((bags.as_pointer() as Ptr<Bag>).offset(0)))
    );
    assert!(
        (({
            let __a = (bags.as_pointer() as Ptr<Bag>).offset(0);
            let __b = (bags.as_pointer() as Ptr<Bag>).offset(2);
            if CppLess::less(&__a, &__b) { __b } else { __a }
        }) == ((bags.as_pointer() as Ptr<Bag>).offset(2)))
    );
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    let bag_map: Value<Map<Bag, (Value<Bag>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    Map::subscript_move(
        bag_map.as_pointer(),
        bag_vec.decay().offset(2_usize),
        |__k| cpp_move!(__k),
    )
    .write(2);
    Map::subscript_move(
        bag_map.as_pointer(),
        bag_vec.decay().offset(0_usize),
        |__k| cpp_move!(__k),
    )
    .write(0);
    assert!(
        ((*(*bag_map
            .as_pointer()
            .with(|__m: &Map<Bag, (Value<Bag>, Value<i32>)>| __m
                .find(&(bags.as_pointer() as Ptr<Bag>).offset(2)))
            .iter_deref()
            .upgrade()
            .deref())
        .1
        .borrow())
            == 2)
    );
    assert!(
        bag_map
            .as_pointer()
            .with(|__m: &Map<Bag, (Value<Bag>, Value<i32>)>| __m
                .find(&(bags.as_pointer() as Ptr<Bag>).offset(1)))
            == bag_map
                .as_pointer()
                .with(|__m: &Map<Bag, (Value<Bag>, Value<i32>)>| __m.end())
    );
    assert!((copies_0.with(|rc| *rc.borrow()) == 0));
    return copies_0.with(|rc| *rc.borrow());
}
pub trait BagImpl {
    fn move_assign(&self, _a0: Ptr<Bag>) -> Ptr<Bag>;
}
impl BagImpl for Ptr<Bag> {
    fn move_assign(&self, _a0: Ptr<Bag>) -> Ptr<Bag> {
        field_ref!((*self), items)
            .as_pointer()
            .write(field!(_a0, items).with_mut(std::mem::take));
        return (*self).clone();
    }
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
