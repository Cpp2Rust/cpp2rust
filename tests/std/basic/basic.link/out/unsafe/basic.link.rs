extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn pick_0() -> i32 {
    return 1;
}
pub unsafe fn test_1() {
    assert!(((unsafe { pick_0() }) == (1)));
    assert!(((unsafe { pick_in_b_2() }) == (2)));
}
pub static mut limit_3: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 10 });
pub unsafe fn test_4() {
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut limit_3)) == (10)));
    assert!(((unsafe { limit_in_b_5() }) == (20)));
    assert!(
        ((&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut limit_3)) as *const i32)
            != (unsafe { limit_address_in_b_6() }))
    );
}
pub unsafe fn test_8() {
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut threshold_7)) == (30)));
    assert!(
        ((&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut threshold_7)) as *const i32)
            == (unsafe { threshold_address_in_b_9() }))
    );
}
pub unsafe fn test_11() {
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut ceiling_10)) == (40)));
    assert!(
        ((&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut ceiling_10)) as *const i32)
            == (unsafe { ceiling_address_in_b_12() }))
    );
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_13 {
    pub whole: i32,
    pub fraction: f32,
}
impl Default for anon_13 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub static mut anon_13: std::cell::LazyCell<anon_13> =
    std::cell::LazyCell::new(|| unsafe { <anon_13>::default() });
pub unsafe fn test_14() {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut anon_13)).whole = 5;
    (unsafe { set_whole_in_b_15(6) });
    assert!((((*std::cell::LazyCell::force_mut(&mut *&raw mut anon_13)).whole) == (5)));
    assert!(((unsafe { whole_in_b_16() }) == (6)));
}
pub static mut counter_17: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 1 });
pub unsafe fn bump_18() -> i32 {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut counter_17)).prefix_inc();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p4_unnamed_namespaces_in_two_files_Record {
    pub id: i32,
}
impl Default for p4_unnamed_namespaces_in_two_files_Record {
    fn default() -> Self {
        unsafe { p4_unnamed_namespaces_in_two_files_Record { id: 100 } }
    }
}
pub static mut depth_19: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 3 });
pub unsafe fn test_20() {
    assert!(((unsafe { bump_18() }) == (2)));
    assert!(((unsafe { bump_in_b_21() }) == (51)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut counter_17)) == (2)));
    let mut record: p4_unnamed_namespaces_in_two_files_Record =
        <p4_unnamed_namespaces_in_two_files_Record>::default();
    assert!(((record.id) == (100)));
    assert!(((unsafe { record_id_in_b_22() }) == (200)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut depth_19)) == (3)));
    assert!(((unsafe { depth_in_b_23() }) == (4)));
}
pub unsafe fn test_25() {
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut shared_total_24)) == (5)));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut shared_total_24)) = 7;
    (unsafe { add_in_b_26(3) });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut shared_total_24)) == (10)));
}
pub unsafe fn test_28() {
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut level_27)) == (1)));
    assert!(((unsafe { raise_29(2,) }) == (3)));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut level_27)) = 10;
    assert!(((unsafe { raise_29(1,) }) == (11)));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p5_members_of_classes_in_unnamed_namespaces_Tally {}
impl p5_members_of_classes_in_unnamed_namespaces_Tally {
    pub unsafe fn add(&mut self, mut amount: i32) -> i32 {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut total_30)) += amount;
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut total_30));
    }
}
pub static mut total_30: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 10 });
pub unsafe fn test_31() {
    let mut tally: p5_members_of_classes_in_unnamed_namespaces_Tally =
        <p5_members_of_classes_in_unnamed_namespaces_Tally>::default();
    assert!(
        ((unsafe { p5_members_of_classes_in_unnamed_namespaces_Tally::add(&mut tally, 1,) })
            == (11))
    );
    assert!(((unsafe { add_in_b_32(1,) }) == (102)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut total_30)) == (11)));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p5_members_of_class_with_external_linkage_Ledger {}
pub unsafe fn test_34() {
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut balance_33)) == (50)));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut balance_33)) = 60;
    assert!(((unsafe { p5_members_of_class_with_external_linkage_Ledger::deposit(5,) }) == (65)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut balance_33)) == (65)));
}
pub unsafe fn read_remote_35() -> i32 {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut remote_36));
}
pub unsafe fn test_37() {
    assert!(((unsafe { read_remote_35() }) == (70)));
    (unsafe { set_remote_in_b_38(71) });
    assert!(((unsafe { read_remote_35() }) == (71)));
}
pub static mut hidden_39: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 80 });
pub unsafe fn read_hidden_through_extern_40() -> i32 {
    let mut hidden: i32 = 1;
    {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut hidden_39));
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_41() {
    assert!(((unsafe { read_hidden_through_extern_40() }) == (80)));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut hidden_39)) = 81;
    assert!(((unsafe { read_hidden_through_extern_40() }) == (81)));
    assert!(((unsafe { hidden_in_b_42() }) == (90)));
}
pub unsafe fn next_even_43() -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_44)) += 2;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut count_44));
}
pub unsafe fn next_odd_45() -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_46)) += 2;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut count_46));
}
pub unsafe fn test_47() {
    assert!(((unsafe { next_even_43() }) == (2)));
    assert!(((unsafe { next_odd_45() }) == (3)));
    assert!(((unsafe { next_in_b_48() }) == (1000)));
    assert!(((unsafe { next_even_43() }) == (4)));
    assert!(((unsafe { next_odd_45() }) == (5)));
    assert!(((unsafe { next_in_b_48() }) == (2000)));
}
static mut count_44: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
static mut count_46: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 1 });
pub unsafe fn first_key_49() -> i32 {
    let mut entry: Entry_50 = <Entry_50>::default();
    return entry.key;
}
pub unsafe fn second_key_plus_value_51() -> i32 {
    let mut entry: Entry_52 = <Entry_52>::default();
    return ((entry.key) + (entry.value));
}
pub unsafe fn test_53() {
    assert!(((unsafe { first_key_49() }) == (1)));
    assert!(((unsafe { second_key_plus_value_51() }) == (32)));
    assert!(((unsafe { key_in_b_54() }) == (400)));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Entry_50 {
    pub key: i32,
}
impl Default for Entry_50 {
    fn default() -> Self {
        unsafe { Entry_50 { key: 1 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Entry_52 {
    pub key: i32,
    pub value: i32,
}
impl Default for Entry_52 {
    fn default() -> Self {
        unsafe { Entry_52 { key: 2, value: 30 } }
    }
}
pub unsafe fn test_56() {
    assert!((((*std::cell::LazyCell::force_mut(&mut *&raw mut table_55))[(1) as usize]) == (2)));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut table_55))[(2) as usize] = 30;
    assert!(((unsafe { table_sum_in_b_57() }) == (33)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_1() });
    (unsafe { test_4() });
    (unsafe { test_8() });
    (unsafe { test_11() });
    (unsafe { test_14() });
    (unsafe { test_20() });
    (unsafe { test_25() });
    (unsafe { test_28() });
    (unsafe { test_31() });
    (unsafe { test_34() });
    (unsafe { test_37() });
    (unsafe { test_41() });
    (unsafe { test_47() });
    (unsafe { test_53() });
    (unsafe { test_56() });
    return 0;
}
pub unsafe fn pick_58() -> i32 {
    return 2;
}
pub unsafe fn pick_in_b_2() -> i32 {
    return (unsafe { pick_58() });
}
pub static mut limit_59: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 20 });
pub unsafe fn limit_in_b_5() -> i32 {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut limit_59));
}
pub unsafe fn limit_address_in_b_6() -> *const i32 {
    return (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut limit_59)) as *const i32);
}
pub static mut threshold_7: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 30 });
pub unsafe fn threshold_address_in_b_9() -> *const i32 {
    return (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut threshold_7))
        as *const i32);
}
pub static mut ceiling_10: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 40 });
pub unsafe fn ceiling_address_in_b_12() -> *const i32 {
    return (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut ceiling_10)) as *const i32);
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_60 {
    pub whole: i32,
    pub fraction: f32,
}
impl Default for anon_60 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub static mut anon_60: std::cell::LazyCell<anon_60> =
    std::cell::LazyCell::new(|| unsafe { <anon_60>::default() });
pub unsafe fn set_whole_in_b_15(mut value: i32) {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut anon_60)).whole = value;
}
pub unsafe fn whole_in_b_16() -> i32 {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut anon_60)).whole;
}
pub static mut counter_61: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 50 });
pub unsafe fn bump_62() -> i32 {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut counter_61)).prefix_inc();
}
pub static mut depth_63: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 4 });
pub unsafe fn bump_in_b_21() -> i32 {
    return (unsafe { bump_62() });
}
pub unsafe fn record_id_in_b_22() -> i32 {
    let mut record: p4_unnamed_namespaces_in_two_files_Record =
        <p4_unnamed_namespaces_in_two_files_Record>::default();
    return (record.id as i32);
}
pub unsafe fn depth_in_b_23() -> i32 {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut depth_63));
}
pub static mut shared_total_24: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 5 });
pub unsafe fn add_in_b_26(mut amount: i32) {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut shared_total_24)) += amount;
}
pub static mut level_27: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 1 });
pub unsafe fn raise_29(mut by: i32) -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut level_27)) += by;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut level_27));
}
impl p5_members_of_classes_in_unnamed_namespaces_Tally {
    pub unsafe fn add(&mut self, mut amount: i32) -> i32 {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut total_64)) += ((amount) * (2));
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut total_64));
    }
}
pub static mut total_64: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 100 });
pub unsafe fn add_in_b_32(mut amount: i32) -> i32 {
    let mut tally: p5_members_of_classes_in_unnamed_namespaces_Tally =
        <p5_members_of_classes_in_unnamed_namespaces_Tally>::default();
    return (unsafe { p5_members_of_classes_in_unnamed_namespaces_Tally::add(&mut tally, amount) });
}
pub static mut balance_33: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 50 });
impl p5_members_of_class_with_external_linkage_Ledger {
    pub unsafe fn deposit(mut amount: i32) -> i32 {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut balance_33)) += amount;
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut balance_33));
    }
}
pub static mut remote_36: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 70 });
pub unsafe fn set_remote_in_b_38(mut value: i32) {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut remote_36)) = value;
}
pub static mut hidden_65: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 90 });
pub unsafe fn hidden_in_b_42() -> i32 {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut hidden_65));
}
pub unsafe fn next_in_b_48() -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_66)) += 1000;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut count_66));
}
static mut count_66: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub unsafe fn key_in_b_54() -> i32 {
    let mut entry: Entry_67 = <Entry_67>::default();
    return (entry.key as i32);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Entry_67 {
    pub key: i64,
}
impl Default for Entry_67 {
    fn default() -> Self {
        unsafe { Entry_67 { key: 400_i64 } }
    }
}
pub static mut table_55: std::cell::LazyCell<[i32; 3]> =
    std::cell::LazyCell::new(|| unsafe { [1, 2, 3] });
pub unsafe fn table_sum_in_b_57() -> i32 {
    return ((((*std::cell::LazyCell::force_mut(&mut *&raw mut table_55))[(0) as usize])
        + ((*std::cell::LazyCell::force_mut(&mut *&raw mut table_55))[(1) as usize]))
        + ((*std::cell::LazyCell::force_mut(&mut *&raw mut table_55))[(2) as usize]));
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const limit_3);
    std::cell::LazyCell::force(&*&raw const anon_13);
    std::cell::LazyCell::force(&*&raw const counter_17);
    std::cell::LazyCell::force(&*&raw const depth_19);
    std::cell::LazyCell::force(&*&raw const total_30);
    std::cell::LazyCell::force(&*&raw const hidden_39);
    std::cell::LazyCell::force(&*&raw const limit_59);
    std::cell::LazyCell::force(&*&raw const threshold_7);
    std::cell::LazyCell::force(&*&raw const ceiling_10);
    std::cell::LazyCell::force(&*&raw const anon_60);
    std::cell::LazyCell::force(&*&raw const counter_61);
    std::cell::LazyCell::force(&*&raw const depth_63);
    std::cell::LazyCell::force(&*&raw const shared_total_24);
    std::cell::LazyCell::force(&*&raw const level_27);
    std::cell::LazyCell::force(&*&raw const total_64);
    std::cell::LazyCell::force(&*&raw const balance_33);
    std::cell::LazyCell::force(&*&raw const remote_36);
    std::cell::LazyCell::force(&*&raw const hidden_65);
    std::cell::LazyCell::force(&*&raw const table_55);
}
