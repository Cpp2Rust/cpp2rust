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
    return (*counter_17.with(Value::clone).borrow_mut()).prefix_inc();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct p4_unnamed_namespaces_in_two_files_Record {
    #[offset(0)]
    pub id: i32,
}
impl Default for p4_unnamed_namespaces_in_two_files_Record {
    fn default() -> Self {
        { p4_unnamed_namespaces_in_two_files_Record { id: 100 } }
    }
}
thread_local!(
    pub static depth_19: Value<i32> = Rc::new(RefCell::new(3));
);
pub fn test_20() {
    assert!((({ bump_18() }) == 2));
    assert!((({ bump_in_b_21() }) == 51));
    assert!((counter_17.with(|rc| *rc.borrow()) == 2));
    let mut record: p4_unnamed_namespaces_in_two_files_Record =
        <p4_unnamed_namespaces_in_two_files_Record>::default();
    assert!((record.id == 100));
    assert!((({ record_id_in_b_22() }) == 200));
    assert!((depth_19.with(|rc| *rc.borrow()) == 3));
    assert!((({ depth_in_b_23() }) == 4));
}
pub fn test_25() {
    assert!((shared_total_24.with(|rc| *rc.borrow()) == 5));
    shared_total_24.with(|rc| *rc.borrow_mut() = 7);
    ({ add_in_b_26(3) });
    assert!((shared_total_24.with(|rc| *rc.borrow()) == 10));
}
pub fn test_28() {
    assert!((level_27.with(|rc| *rc.borrow()) == 1));
    assert!((({ raise_29(2,) }) == 3));
    level_27.with(|rc| *rc.borrow_mut() = 10);
    assert!((({ raise_29(1,) }) == 11));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct p5_members_of_classes_in_unnamed_namespaces_Tally {}
thread_local!(
    pub static total_30: Value<i32> = Rc::new(RefCell::new(10));
);
pub fn test_31() {
    let tally: Value<p5_members_of_classes_in_unnamed_namespaces_Tally> = Rc::new(RefCell::new(
        <p5_members_of_classes_in_unnamed_namespaces_Tally>::default(),
    ));
    assert!(
        (({ p5_members_of_classes_in_unnamed_namespaces_TallyImpl::add(&tally.as_pointer(), 1,) })
            == 11)
    );
    assert!((({ add_in_b_32(1,) }) == 102));
    assert!((total_30.with(|rc| *rc.borrow()) == 11));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct p5_members_of_class_with_external_linkage_Ledger {}
pub fn test_34() {
    assert!((balance_33.with(|rc| *rc.borrow()) == 50));
    balance_33.with(|rc| *rc.borrow_mut() = 60);
    assert!((({ p5_members_of_class_with_external_linkage_Ledger::deposit(5,) }) == 65));
    assert!((balance_33.with(|rc| *rc.borrow()) == 65));
}
pub fn read_remote_35() -> i32 {
    let mut remote: i32 = 0_i32;
    return remote;
}
pub fn test_36() {
    assert!((({ read_remote_35() }) == 70));
    ({ set_remote_in_b_37(71) });
    assert!((({ read_remote_35() }) == 71));
}
thread_local!(
    pub static hidden_38: Value<i32> = Rc::new(RefCell::new(80));
);
pub fn read_hidden_through_extern_39() -> i32 {
    let mut hidden: i32 = 1;
    {
        let mut hidden: i32 = 0_i32;
        return hidden;
    }
    panic!("ub: non-void function does not return a value")
}
pub fn test_40() {
    assert!((({ read_hidden_through_extern_39() }) == 80));
    hidden_38.with(|rc| *rc.borrow_mut() = 81);
    assert!((({ read_hidden_through_extern_39() }) == 81));
    assert!((({ hidden_in_b_41() }) == 90));
}
pub fn next_even_42() -> i32 {
    count_43.with(|rc| *rc.borrow_mut() += 2);
    return count_43.with(|rc| *rc.borrow());
}
pub fn next_odd_44() -> i32 {
    count_45.with(|rc| *rc.borrow_mut() += 2);
    return count_45.with(|rc| *rc.borrow());
}
pub fn test_46() {
    assert!((({ next_even_42() }) == 2));
    assert!((({ next_odd_44() }) == 3));
    assert!((({ next_in_b_47() }) == 1000));
    assert!((({ next_even_42() }) == 4));
    assert!((({ next_odd_44() }) == 5));
    assert!((({ next_in_b_47() }) == 2000));
}
thread_local!(
    static count_43: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    static count_45: Value<i32> = Rc::new(RefCell::new(1));
);
pub fn first_key_48() -> i32 {
    let mut entry: Entry_49 = <Entry_49>::default();
    return entry.key;
}
pub fn second_key_plus_value_50() -> i32 {
    let mut entry: Entry_51 = <Entry_51>::default();
    return (entry.key + entry.value);
}
pub fn test_52() {
    assert!((({ first_key_48() }) == 1));
    assert!((({ second_key_plus_value_50() }) == 32));
    assert!((({ key_in_b_53() }) == 400));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct Entry_49 {
    #[offset(0)]
    pub key: i32,
}
impl Default for Entry_49 {
    fn default() -> Self {
        { Entry_49 { key: 1 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Entry_51 {
    #[offset(0)]
    pub key: i32,
    #[offset(4)]
    pub value: i32,
}
impl Default for Entry_51 {
    fn default() -> Self {
        { Entry_51 { key: 2, value: 30 } }
    }
}
pub fn test_55() {
    assert!(
        (({
            let __idx0 = (1) as usize;
            table_54.with(|rc| rc.borrow()[__idx0])
        }) == 2)
    );
    (*table_54.with(Value::clone).borrow_mut())[(2) as usize] = 30;
    assert!((({ table_sum_in_b_56() }) == 33));
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
    ({ test_20() });
    ({ test_25() });
    ({ test_28() });
    ({ test_31() });
    ({ test_34() });
    ({ test_36() });
    ({ test_40() });
    ({ test_46() });
    ({ test_52() });
    ({ test_55() });
    return 0;
}
pub fn pick_57() -> i32 {
    return 2;
}
pub fn pick_in_b_2() -> i32 {
    return ({ pick_57() });
}
thread_local!(
    pub static limit_58: Value<i32> = Rc::new(RefCell::new(20));
);
pub fn limit_in_b_5() -> i32 {
    return limit_58.with(|rc| *rc.borrow());
}
pub fn limit_address_in_b_6() -> Ptr<i32> {
    return (limit_58.with(|v| v.as_pointer()));
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
pub struct anon_59 {
    #[offset(0)]
    #[byte_size(4)]
    __bytes: Box<[u8]>,
}
impl anon_59 {
    pub fn whole(this: Ptr<Self>) -> Ptr<i32> {
        this.reinterpret_cast()
    }
    pub fn fraction(this: Ptr<Self>) -> Ptr<f32> {
        this.reinterpret_cast()
    }
}
impl Default for anon_59 {
    fn default() -> Self {
        anon_59 {
            __bytes: Box::from([0u8; 4]),
        }
    }
}
pub fn set_whole_in_b_15(mut value: i32) {
    anon_59::whole(anon_59.with(|v| v.as_pointer())).write(value);
}
pub fn whole_in_b_16() -> i32 {
    return (anon_59::whole(anon_59.with(|v| v.as_pointer())).read());
}
thread_local!(
    pub static counter_60: Value<i32> = Rc::new(RefCell::new(50));
);
pub fn bump_61() -> i32 {
    return (*counter_60.with(Value::clone).borrow_mut()).prefix_inc();
}
thread_local!(
    pub static depth_62: Value<i32> = Rc::new(RefCell::new(4));
);
pub fn bump_in_b_21() -> i32 {
    return ({ bump_61() });
}
pub fn record_id_in_b_22() -> i32 {
    let mut record: p4_unnamed_namespaces_in_two_files_Record =
        <p4_unnamed_namespaces_in_two_files_Record>::default();
    return (record.id as i32);
}
pub fn depth_in_b_23() -> i32 {
    return depth_62.with(|rc| *rc.borrow());
}
thread_local!(
    pub static shared_total_24: Value<i32> = Rc::new(RefCell::new(5));
);
pub fn add_in_b_26(mut amount: i32) {
    shared_total_24.with(|rc| *rc.borrow_mut() += amount);
}
thread_local!(
    pub static level_27: Value<i32> = Rc::new(RefCell::new(1));
);
pub fn raise_29(mut by: i32) -> i32 {
    level_27.with(|rc| *rc.borrow_mut() += by);
    return level_27.with(|rc| *rc.borrow());
}
thread_local!(
    pub static total_63: Value<i32> = Rc::new(RefCell::new(100));
);
pub fn add_in_b_32(mut amount: i32) -> i32 {
    let tally: Value<p5_members_of_classes_in_unnamed_namespaces_Tally> = Rc::new(RefCell::new(
        <p5_members_of_classes_in_unnamed_namespaces_Tally>::default(),
    ));
    return ({
        p5_members_of_classes_in_unnamed_namespaces_TallyImpl::add(&tally.as_pointer(), amount)
    });
}
thread_local!(
    pub static balance_33: Value<i32> = Rc::new(RefCell::new(50));
);
impl p5_members_of_class_with_external_linkage_Ledger {
    pub fn deposit(mut amount: i32) -> i32 {
        balance_33.with(|rc| *rc.borrow_mut() += amount);
        return balance_33.with(|rc| *rc.borrow());
    }
}
thread_local!(
    pub static remote_64: Value<i32> = Rc::new(RefCell::new(70));
);
pub fn set_remote_in_b_37(mut value: i32) {
    remote_64.with(|rc| *rc.borrow_mut() = value);
}
thread_local!(
    pub static hidden_65: Value<i32> = Rc::new(RefCell::new(90));
);
pub fn hidden_in_b_41() -> i32 {
    return hidden_65.with(|rc| *rc.borrow());
}
pub fn next_in_b_47() -> i32 {
    count_66.with(|rc| *rc.borrow_mut() += 1000);
    return count_66.with(|rc| *rc.borrow());
}
thread_local!(
    static count_66: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn key_in_b_53() -> i32 {
    let mut entry: Entry_67 = <Entry_67>::default();
    return (entry.key as i32);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Entry_67 {
    #[offset(0)]
    pub key: i64,
}
impl Default for Entry_67 {
    fn default() -> Self {
        { Entry_67 { key: 400_i64 } }
    }
}
thread_local!(
    pub static table_54: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
);
pub fn table_sum_in_b_56() -> i32 {
    return ((({
        let __idx0 = (0) as usize;
        table_54.with(|rc| rc.borrow()[__idx0])
    }) + ({
        let __idx0 = (1) as usize;
        table_54.with(|rc| rc.borrow()[__idx0])
    })) + ({
        let __idx0 = (2) as usize;
        table_54.with(|rc| rc.borrow()[__idx0])
    }));
}
pub trait p5_members_of_classes_in_unnamed_namespaces_TallyImpl {
    fn add(&self, amount: i32) -> i32;
}
impl p5_members_of_classes_in_unnamed_namespaces_TallyImpl
    for Ptr<p5_members_of_classes_in_unnamed_namespaces_Tally>
{
    fn add(&self, mut amount: i32) -> i32 {
        total_30.with(|rc| *rc.borrow_mut() += amount);
        return total_30.with(|rc| *rc.borrow());
    }
    fn add(&self, mut amount: i32) -> i32 {
        total_63.with(|rc| *rc.borrow_mut() += (amount * 2));
        return total_63.with(|rc| *rc.borrow());
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = limit_3.with(|_| ());
    let _ = counter_17.with(|_| ());
    let _ = depth_19.with(|_| ());
    let _ = total_30.with(|_| ());
    let _ = hidden_38.with(|_| ());
    let _ = limit_58.with(|_| ());
    let _ = threshold_7.with(|_| ());
    let _ = ceiling_10.with(|_| ());
    let _ = counter_60.with(|_| ());
    let _ = depth_62.with(|_| ());
    let _ = shared_total_24.with(|_| ());
    let _ = level_27.with(|_| ());
    let _ = total_63.with(|_| ());
    let _ = balance_33.with(|_| ());
    let _ = remote_64.with(|_| ());
    let _ = hidden_65.with(|_| ());
    let _ = table_54.with(|_| ());
}
