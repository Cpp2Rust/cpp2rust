extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn pick_0() -> i32 {
    return 1;
}
pub fn test_1() {
    assert!((({ pick_0() }) == 1));
    assert!((({ pick_in_b_2() }) == 2));
}
thread_local!(
    pub static limit_3: Value<i32> = Rc::new(RefCell::new(10));
);
pub fn test_4() {
    assert!((limit_3.with(|rc| *rc.borrow()) == 10));
    assert!((({ limit_in_b_5() }) == 20));
    assert!(((limit_3.with(|v| v.as_pointer())) != ({ limit_address_in_b_6() })));
}
pub fn test_8() {
    assert!((threshold_7.with(|rc| *rc.borrow()) == 30));
    assert!(((threshold_7.with(|v| v.as_pointer())) == ({ threshold_address_in_b_9() })));
}
pub fn test_11() {
    assert!((ceiling_10.with(|rc| *rc.borrow()) == 40));
    assert!(((ceiling_10.with(|v| v.as_pointer())) == ({ ceiling_address_in_b_12() })));
}
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(4)]
pub struct anon_13 {
    #[offset(0)]
    #[byte_size(4)]
    __bytes: Box<[u8]>,
}
impl anon_13 {
    pub fn whole(this: Ptr<Self>) -> Ptr<i32> {
        this.reinterpret_cast()
    }
    pub fn fraction(this: Ptr<Self>) -> Ptr<f32> {
        this.reinterpret_cast()
    }
}
impl Default for anon_13 {
    fn default() -> Self {
        anon_13 {
            __bytes: Box::from([0u8; 4]),
        }
    }
}
thread_local!(
    pub static anon_13: Value<anon_13> = Rc::new(RefCell::new(<anon_13>::default()));
);
pub fn test_14() {
    anon_13::whole(anon_13.with(|v| v.as_pointer())).write(5);
    ({ set_whole_in_b_15(6) });
    assert!(((anon_13::whole(anon_13.with(|v| v.as_pointer())).read()) == 5));
    assert!((({ whole_in_b_16() }) == 6));
}
thread_local!(
    pub static counter_17: Value<i32> = Rc::new(RefCell::new(1));
);
pub fn bump_18() -> i32 {
    return (*(*counter_17.with(Value::clone).borrow_mut()).prefix_inc());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct Record_19 {
    #[offset(0)]
    pub id: i32,
}
impl Default for Record_19 {
    fn default() -> Self {
        { Record_19 { id: 100 } }
    }
}
thread_local!(
    pub static depth_20: Value<i32> = Rc::new(RefCell::new(3));
);
pub fn test_21() {
    assert!((({ bump_18() }) == 2));
    assert!((({ bump_in_b_22() }) == 51));
    assert!((counter_17.with(|rc| *rc.borrow()) == 2));
    let mut record: Record_19 = <Record_19>::default();
    assert!((record.id == 100));
    assert!((({ record_id_in_b_23() }) == 200));
    assert!((depth_20.with(|rc| *rc.borrow()) == 3));
    assert!((({ depth_in_b_24() }) == 4));
}
pub fn test_26() {
    assert!((shared_total_25.with(|rc| *rc.borrow()) == 5));
    shared_total_25.with(|rc| *rc.borrow_mut() = 7);
    ({ add_in_b_27(3) });
    assert!((shared_total_25.with(|rc| *rc.borrow()) == 10));
}
pub fn test_29() {
    assert!((level_28.with(|rc| *rc.borrow()) == 1));
    assert!((({ raise_30(2,) }) == 3));
    level_28.with(|rc| *rc.borrow_mut() = 10);
    assert!((({ raise_30(1,) }) == 11));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Tally_31 {}
thread_local!(
    pub static total_32: Value<i32> = Rc::new(RefCell::new(10));
);
pub fn test_33() {
    let tally: Value<Tally_31> = Rc::new(RefCell::new(<Tally_31>::default()));
    assert!((({ Tally_31Impl::add(&tally.as_pointer(), 1,) }) == 11));
    assert!((({ add_in_b_34(1,) }) == 102));
    assert!((total_32.with(|rc| *rc.borrow()) == 11));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct p5_members_of_class_with_external_linkage_Ledger {}
pub fn test_36() {
    assert!((balance_35.with(|rc| *rc.borrow()) == 50));
    balance_35.with(|rc| *rc.borrow_mut() = 60);
    assert!((({ p5_members_of_class_with_external_linkage_Ledger::deposit(5,) }) == 65));
    assert!((balance_35.with(|rc| *rc.borrow()) == 65));
}
pub fn read_remote_37() -> i32 {
    return remote_38.with(|rc| *rc.borrow());
}
pub fn test_39() {
    assert!((({ read_remote_37() }) == 70));
    ({ set_remote_in_b_40(71) });
    assert!((({ read_remote_37() }) == 71));
}
thread_local!(
    pub static hidden_41: Value<i32> = Rc::new(RefCell::new(80));
);
pub fn read_hidden_through_extern_42() -> i32 {
    let mut hidden: i32 = 1;
    {
        return hidden_41.with(|rc| *rc.borrow());
    }
    panic!("ub: non-void function does not return a value")
}
pub fn test_43() {
    assert!((({ read_hidden_through_extern_42() }) == 80));
    hidden_41.with(|rc| *rc.borrow_mut() = 81);
    assert!((({ read_hidden_through_extern_42() }) == 81));
    assert!((({ hidden_in_b_44() }) == 90));
}
pub fn next_even_45() -> i32 {
    count_46.with(|rc| *rc.borrow_mut() += 2);
    return count_46.with(|rc| *rc.borrow());
}
pub fn next_odd_47() -> i32 {
    count_48.with(|rc| *rc.borrow_mut() += 2);
    return count_48.with(|rc| *rc.borrow());
}
pub fn test_49() {
    assert!((({ next_even_45() }) == 2));
    assert!((({ next_odd_47() }) == 3));
    assert!((({ next_in_b_50() }) == 1000));
    assert!((({ next_even_45() }) == 4));
    assert!((({ next_odd_47() }) == 5));
    assert!((({ next_in_b_50() }) == 2000));
}
thread_local!(
    static count_46: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    static count_48: Value<i32> = Rc::new(RefCell::new(1));
);
pub fn first_key_51() -> i32 {
    let mut entry: Entry_52 = <Entry_52>::default();
    return entry.key;
}
pub fn second_key_plus_value_53() -> i32 {
    let mut entry: Entry_54 = <Entry_54>::default();
    return (entry.key + entry.value);
}
pub fn test_55() {
    assert!((({ first_key_51() }) == 1));
    assert!((({ second_key_plus_value_53() }) == 32));
    assert!((({ key_in_b_56() }) == 400));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct Entry_52 {
    #[offset(0)]
    pub key: i32,
}
impl Default for Entry_52 {
    fn default() -> Self {
        { Entry_52 { key: 1 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Entry_54 {
    #[offset(0)]
    pub key: i32,
    #[offset(4)]
    pub value: i32,
}
impl Default for Entry_54 {
    fn default() -> Self {
        { Entry_54 { key: 2, value: 30 } }
    }
}
pub fn test_58() {
    assert!(
        (({
            let __idx0 = (1) as usize;
            table_57.with(|rc| rc.borrow()[__idx0])
        }) == 2)
    );
    (*table_57.with(Value::clone).borrow_mut())[(2) as usize] = 30;
    assert!((({ table_sum_in_b_59() }) == 33));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_1() });
    ({ test_4() });
    ({ test_8() });
    ({ test_11() });
    ({ test_14() });
    ({ test_21() });
    ({ test_26() });
    ({ test_29() });
    ({ test_33() });
    ({ test_36() });
    ({ test_39() });
    ({ test_43() });
    ({ test_49() });
    ({ test_55() });
    ({ test_58() });
    return 0;
}
pub fn pick_60() -> i32 {
    return 2;
}
pub fn pick_in_b_2() -> i32 {
    return ({ pick_60() });
}
thread_local!(
    pub static limit_61: Value<i32> = Rc::new(RefCell::new(20));
);
pub fn limit_in_b_5() -> i32 {
    return limit_61.with(|rc| *rc.borrow());
}
pub fn limit_address_in_b_6() -> Ptr<i32> {
    return (limit_61.with(|v| v.as_pointer()));
}
thread_local!(
    pub static threshold_7: Value<i32> = Rc::new(RefCell::new(30));
);
pub fn threshold_address_in_b_9() -> Ptr<i32> {
    return (threshold_7.with(|v| v.as_pointer()));
}
thread_local!(
    pub static ceiling_10: Value<i32> = Rc::new(RefCell::new(40));
);
pub fn ceiling_address_in_b_12() -> Ptr<i32> {
    return (ceiling_10.with(|v| v.as_pointer()));
}
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(4)]
pub struct anon_62 {
    #[offset(0)]
    #[byte_size(4)]
    __bytes: Box<[u8]>,
}
impl anon_62 {
    pub fn whole(this: Ptr<Self>) -> Ptr<i32> {
        this.reinterpret_cast()
    }
    pub fn fraction(this: Ptr<Self>) -> Ptr<f32> {
        this.reinterpret_cast()
    }
}
impl Default for anon_62 {
    fn default() -> Self {
        anon_62 {
            __bytes: Box::from([0u8; 4]),
        }
    }
}
thread_local!(
    pub static anon_62: Value<anon_62> = Rc::new(RefCell::new(<anon_62>::default()));
);
pub fn set_whole_in_b_15(mut value: i32) {
    anon_62::whole(anon_62.with(|v| v.as_pointer())).write(value);
}
pub fn whole_in_b_16() -> i32 {
    return (anon_62::whole(anon_62.with(|v| v.as_pointer())).read());
}
thread_local!(
    pub static counter_63: Value<i32> = Rc::new(RefCell::new(50));
);
pub fn bump_64() -> i32 {
    return (*(*counter_63.with(Value::clone).borrow_mut()).prefix_inc());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct Record_65 {
    #[offset(0)]
    pub id: i64,
    #[offset(8)]
    pub extra: i32,
}
impl Default for Record_65 {
    fn default() -> Self {
        {
            Record_65 {
                id: 200_i64,
                extra: 1,
            }
        }
    }
}
thread_local!(
    pub static depth_66: Value<i32> = Rc::new(RefCell::new(4));
);
pub fn bump_in_b_22() -> i32 {
    return ({ bump_64() });
}
pub fn record_id_in_b_23() -> i32 {
    let mut record: Record_65 = <Record_65>::default();
    return (record.id as i32);
}
pub fn depth_in_b_24() -> i32 {
    return depth_66.with(|rc| *rc.borrow());
}
thread_local!(
    pub static shared_total_25: Value<i32> = Rc::new(RefCell::new(5));
);
pub fn add_in_b_27(mut amount: i32) {
    shared_total_25.with(|rc| *rc.borrow_mut() += amount);
}
thread_local!(
    pub static level_28: Value<i32> = Rc::new(RefCell::new(1));
);
pub fn raise_30(mut by: i32) -> i32 {
    level_28.with(|rc| *rc.borrow_mut() += by);
    return level_28.with(|rc| *rc.borrow());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Tally_67 {}
thread_local!(
    pub static total_68: Value<i32> = Rc::new(RefCell::new(100));
);
pub fn add_in_b_34(mut amount: i32) -> i32 {
    let tally: Value<Tally_67> = Rc::new(RefCell::new(<Tally_67>::default()));
    return ({ Tally_67Impl::add(&tally.as_pointer(), amount) });
}
thread_local!(
    pub static balance_35: Value<i32> = Rc::new(RefCell::new(50));
);
impl p5_members_of_class_with_external_linkage_Ledger {
    pub fn deposit(mut amount: i32) -> i32 {
        balance_35.with(|rc| *rc.borrow_mut() += amount);
        return balance_35.with(|rc| *rc.borrow());
    }
}
thread_local!(
    pub static remote_38: Value<i32> = Rc::new(RefCell::new(70));
);
pub fn set_remote_in_b_40(mut value: i32) {
    remote_38.with(|rc| *rc.borrow_mut() = value);
}
thread_local!(
    pub static hidden_69: Value<i32> = Rc::new(RefCell::new(90));
);
pub fn hidden_in_b_44() -> i32 {
    return hidden_69.with(|rc| *rc.borrow());
}
pub fn next_in_b_50() -> i32 {
    count_70.with(|rc| *rc.borrow_mut() += 1000);
    return count_70.with(|rc| *rc.borrow());
}
thread_local!(
    static count_70: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn key_in_b_56() -> i32 {
    let mut entry: Entry_71 = <Entry_71>::default();
    return (entry.key as i32);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Entry_71 {
    #[offset(0)]
    pub key: i64,
}
impl Default for Entry_71 {
    fn default() -> Self {
        { Entry_71 { key: 400_i64 } }
    }
}
thread_local!(
    pub static table_57: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
);
pub fn table_sum_in_b_59() -> i32 {
    return ((({
        let __idx0 = (0) as usize;
        table_57.with(|rc| rc.borrow()[__idx0])
    }) + ({
        let __idx0 = (1) as usize;
        table_57.with(|rc| rc.borrow()[__idx0])
    })) + ({
        let __idx0 = (2) as usize;
        table_57.with(|rc| rc.borrow()[__idx0])
    }));
}
pub trait Tally_31Impl {
    fn add(&self, amount: i32) -> i32;
}
impl Tally_31Impl for Ptr<Tally_31> {
    fn add(&self, mut amount: i32) -> i32 {
        total_32.with(|rc| *rc.borrow_mut() += amount);
        return total_32.with(|rc| *rc.borrow());
    }
}
pub trait Tally_67Impl {
    fn add(&self, amount: i32) -> i32;
}
impl Tally_67Impl for Ptr<Tally_67> {
    fn add(&self, mut amount: i32) -> i32 {
        total_68.with(|rc| *rc.borrow_mut() += (amount * 2));
        return total_68.with(|rc| *rc.borrow());
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = limit_3.with(|_| ());
    let _ = anon_13.with(|_| ());
    let _ = counter_17.with(|_| ());
    let _ = depth_20.with(|_| ());
    let _ = total_32.with(|_| ());
    let _ = hidden_41.with(|_| ());
    let _ = limit_61.with(|_| ());
    let _ = threshold_7.with(|_| ());
    let _ = ceiling_10.with(|_| ());
    let _ = anon_62.with(|_| ());
    let _ = counter_63.with(|_| ());
    let _ = depth_66.with(|_| ());
    let _ = shared_total_25.with(|_| ());
    let _ = level_28.with(|_| ());
    let _ = total_68.with(|_| ());
    let _ = balance_35.with(|_| ());
    let _ = remote_38.with(|_| ());
    let _ = hidden_69.with(|_| ());
    let _ = table_57.with(|_| ());
}
