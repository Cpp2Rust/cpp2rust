extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static scaled_calls_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static rate_1: Value<i32> = Rc::new(RefCell::new(1));
);
pub fn scaled_2(mut amount: i32) -> i32 {
    (*scaled_calls_0.with(Value::clone).borrow_mut()).prefix_inc();
    return (amount * 10);
}
pub fn total_3(mut base: i32) -> i32 {
    return base;
}
pub fn test_4() {
    rate_1.with(|rc| *rc.borrow_mut() = 2);
    {
        let mut rate: i32 = 3;
        assert!((({ total_3(({ scaled_2(rate_1.with(|rc| *rc.borrow()),) }),) }) == 20));
        assert!((scaled_calls_0.with(|rc| *rc.borrow()) == 1));
        assert!((rate == 3));
    }
    assert!((({ total_3(5,) }) == 5));
    assert!((scaled_calls_0.with(|rc| *rc.borrow()) == 1));
}
thread_local!(
    pub static next_ticket_calls_5: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn next_ticket_6() -> i32 {
    return ((*(*next_ticket_calls_5.with(Value::clone).borrow_mut()).prefix_inc()) * 100);
}
pub fn issue_7(mut ticket: i32) -> i32 {
    return ticket;
}
pub fn test_8() {
    assert!((next_ticket_calls_5.with(|rc| *rc.borrow()) == 0));
    assert!((({ issue_7(({ next_ticket_6() }),) }) == 100));
    assert!((next_ticket_calls_5.with(|rc| *rc.borrow()) == 1));
    assert!((({ issue_7(7,) }) == 7));
    assert!((next_ticket_calls_5.with(|rc| *rc.borrow()) == 1));
    assert!((({ issue_7(({ next_ticket_6() }),) }) == 200));
    assert!((next_ticket_calls_5.with(|rc| *rc.borrow()) == 2));
}
thread_local!(
    pub static step_9: Value<i32> = Rc::new(RefCell::new(100));
);
thread_local!(
    pub static step_10: Value<i32> = Rc::new(RefCell::new(4));
);
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct p9_static_member_hides_namespace_variable_HasStaticStep {
    #[offset(0)]
    pub total: i32,
}
impl Default for p9_static_member_hides_namespace_variable_HasStaticStep {
    fn default() -> Self {
        { p9_static_member_hides_namespace_variable_HasStaticStep { total: 1 } }
    }
}
pub fn test_11() {
    let object: Value<p9_static_member_hides_namespace_variable_HasStaticStep> = Rc::new(
        RefCell::new(<p9_static_member_hides_namespace_variable_HasStaticStep>::default()),
    );
    assert!(
        (({
            p9_static_member_hides_namespace_variable_HasStaticStepImpl::advance(
                &object.as_pointer(),
                step_10.with(|rc| *rc.borrow()),
            )
        }) == 5)
    );
    step_10.with(|rc| *rc.borrow_mut() = 6);
    assert!(
        (({
            p9_static_member_hides_namespace_variable_HasStaticStepImpl::advance(
                &object.as_pointer(),
                step_10.with(|rc| *rc.borrow()),
            )
        }) == 11)
    );
    assert!(
        (({
            p9_static_member_hides_namespace_variable_HasStaticStepImpl::advance(
                &object.as_pointer(),
                2,
            )
        }) == 13)
    );
    assert!((step_9.with(|rc| *rc.borrow()) == 100));
}
thread_local!(
    pub static defaults_evaluated_12: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn count_default_13() -> i32 {
    return (*(*defaults_evaluated_12.with(Value::clone).borrow_mut()).prefix_inc());
}
pub fn combine_14(mut first: i32, mut second: i32) -> i32 {
    return ((first * 100) + second);
}
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct p9_evaluated_each_time_called_Widget {
    #[offset(0)]
    pub id: i32,
}
impl p9_evaluated_each_time_called_Widget {
    pub fn new(mut id: i32) -> Self {
        Self { id: id }
    }
}
impl Default for p9_evaluated_each_time_called_Widget {
    fn default() -> Self {
        { p9_evaluated_each_time_called_Widget::new({ ({ count_default_13() }) }) }
    }
}
thread_local!(
    static base_15: Value<i32> = Rc::new(RefCell::new(10));
);
pub type anon_16 = u32;
pub const anon_16_kBonus: anon_16 = 5;
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct p9_evaluated_each_time_called_Account {
    #[offset(0)]
    pub balance: i32,
}
impl p9_evaluated_each_time_called_Account {
    fn twice(mut value: i32) -> i32 {
        return (2 * value);
    }
    pub fn set_base(mut value: i32) {
        base_15.with(|rc| *rc.borrow_mut() = value);
    }
}
impl Default for p9_evaluated_each_time_called_Account {
    fn default() -> Self {
        { p9_evaluated_each_time_called_Account { balance: 0 } }
    }
}
pub fn test_17() {
    defaults_evaluated_12.with(|rc| *rc.borrow_mut() = 0);
    assert!((({ combine_14(1, ({ count_default_13() }),) }) == 101));
    assert!((({ combine_14(2, ({ count_default_13() }),) }) == 202));
    assert!((({ combine_14(3, 9,) }) == 309));
    assert!((defaults_evaluated_12.with(|rc| *rc.borrow()) == 2));
    let mut first: i32 = 0;
    assert!((({ combine_14((*first.prefix_inc()), ({ count_default_13() }),) }) == 103));
    assert!((first == 1));
    assert!((defaults_evaluated_12.with(|rc| *rc.borrow()) == 3));
    let mut a: p9_evaluated_each_time_called_Widget =
        p9_evaluated_each_time_called_Widget::new({ ({ count_default_13() }) });
    let mut b: p9_evaluated_each_time_called_Widget =
        p9_evaluated_each_time_called_Widget::new({ 40 });
    let mut c: p9_evaluated_each_time_called_Widget =
        p9_evaluated_each_time_called_Widget::new({ ({ count_default_13() }) });
    assert!((a.id == 4));
    assert!((b.id == 40));
    assert!((c.id == 5));
    assert!((defaults_evaluated_12.with(|rc| *rc.borrow()) == 5));
    let account: Value<p9_evaluated_each_time_called_Account> = Rc::new(RefCell::new(
        <p9_evaluated_each_time_called_Account>::default(),
    ));
    assert!(
        (({
            p9_evaluated_each_time_called_AccountImpl::deposit(
                &account.as_pointer(),
                (({
                    p9_evaluated_each_time_called_Account::twice(base_15.with(|rc| *rc.borrow()))
                }) + (anon_16_kBonus as i32)),
            )
        }) == 25)
    );
    ({ p9_evaluated_each_time_called_Account::set_base(1) });
    assert!(
        (({
            p9_evaluated_each_time_called_AccountImpl::deposit(
                &account.as_pointer(),
                (({
                    p9_evaluated_each_time_called_Account::twice(base_15.with(|rc| *rc.borrow()))
                }) + (anon_16_kBonus as i32)),
            )
        }) == 32)
    );
    assert!(
        (({ p9_evaluated_each_time_called_AccountImpl::deposit(&account.as_pointer(), 3,) }) == 35)
    );
}
pub fn add_18(mut a: i32, mut b: i32) -> i32 {
    return (a + b);
}
#[derive(Copy, Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct p9_not_part_of_the_type_Scale {}
impl p9_not_part_of_the_type_Scale {
    pub fn times(mut a: i32, mut b: i32) -> i32 {
        return (a * b);
    }
}
pub fn descending_19(a: Ptr<i32>, b: Ptr<i32>) -> bool {
    return ({ (a.read()) } > { (b.read()) });
}
pub fn test_20() {
    let mut p: FnPtr<fn(i32, i32) -> i32> = FnPtr::<fn(i32, i32) -> i32>::new(add_18);
    let mut q: FnPtr<fn(i32, i32) -> i32> = (FnPtr::<fn(i32, i32) -> i32>::new(add_18));
    assert!(({ (p).clone() } == { (q).clone() }));
    assert!((({ p.call(1, 5,) }) == 6));
    assert!((({ q.call(2, 5,) }) == 7));
    assert!((({ add_18(1, 2,) }) == 3));
    let mut s: FnPtr<fn(i32, i32) -> i32> =
        (FnPtr::<fn(i32, i32) -> i32>::new(p9_not_part_of_the_type_Scale::times));
    assert!((({ s.call(2, 5,) }) == 10));
    assert!((({ p9_not_part_of_the_type_Scale::times(2, 3,) }) == 6));
    let values: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([3, -1_i32, 4, 1, 5])));
    match ({ values.decay().begin() },) {
        (mut __tmp_0,) => {
            let __n = values.decay().end().iter_distance(&__tmp_0);
            let mut __order: Vec<isize> = (0..__n).collect();
            __order.sort_by(|&__x, &__y| {
                let __a = __tmp_0.iter_at(__x);
                let __b = __tmp_0.iter_at(__y);
                if descending_19.call(__a.clone(), __b.clone()) {
                    std::cmp::Ordering::Less
                } else if descending_19.call(__b, __a) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });
            reorder(__order, |__i| __tmp_0.iter_at(__i));
        }
    };
    assert!(({ ({ (*values.borrow())[0_usize] } == 5) }));
    assert!(({ ({ (*values.borrow())[1_usize] } == 4) }));
    assert!(({ ({ (*values.borrow())[2_usize] } == 3) }));
    assert!(({ ({ (*values.borrow())[3_usize] } == 1) }));
    assert!(({ ({ (*values.borrow())[4_usize] } == -1_i32) }));
    assert!(
        ({
            let _a: Value<i32> = Rc::new(RefCell::new(1));
            let _b: Value<i32> = Rc::new(RefCell::new(0));
            descending_19(_a.as_pointer(), _b.as_pointer())
        })
    );
    assert!(
        !({
            let _a: Value<i32> = Rc::new(RefCell::new(-1_i32));
            let _b: Value<i32> = Rc::new(RefCell::new(0));
            descending_19(_a.as_pointer(), _b.as_pointer())
        })
    );
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_4() });
    ({ test_8() });
    ({ test_11() });
    ({ test_17() });
    ({ test_20() });
    return 0;
}
pub trait p9_evaluated_each_time_called_AccountImpl {
    fn deposit(&self, amount: i32) -> i32;
}
impl p9_evaluated_each_time_called_AccountImpl for Ptr<p9_evaluated_each_time_called_Account> {
    fn deposit(&self, mut amount: i32) -> i32 {
        {
            let __rhs = amount;
            field!((*self), balance).with_mut(|__v| *__v = *__v + __rhs)
        };
        return (*self).with(|__s| __s.balance);
    }
}
pub trait p9_static_member_hides_namespace_variable_HasStaticStepImpl {
    fn advance(&self, amount: i32) -> i32;
}
impl p9_static_member_hides_namespace_variable_HasStaticStepImpl
    for Ptr<p9_static_member_hides_namespace_variable_HasStaticStep>
{
    fn advance(&self, mut amount: i32) -> i32 {
        {
            let __rhs = amount;
            field!((*self), total).with_mut(|__v| *__v = *__v + __rhs)
        };
        return (*self).with(|__s| __s.total);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = scaled_calls_0.with(|_| ());
    let _ = rate_1.with(|_| ());
    let _ = next_ticket_calls_5.with(|_| ());
    let _ = step_9.with(|_| ());
    let _ = step_10.with(|_| ());
    let _ = defaults_evaluated_12.with(|_| ());
    let _ = base_15.with(|_| ());
}
