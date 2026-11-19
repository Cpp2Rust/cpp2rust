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
    return (*(*std::cell::LazyCell::force_mut(&mut *&raw mut counter_17)).prefix_inc());
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Record_19 {
    pub id: i32,
}
impl Default for Record_19 {
    fn default() -> Self {
        unsafe { Record_19 { id: 100 } }
    }
}
pub static mut depth_20: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 3 });
pub unsafe fn test_21() {
    assert!(((unsafe { bump_18() }) == (2)));
    assert!(((unsafe { bump_in_b_22() }) == (51)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut counter_17)) == (2)));
    let mut record: Record_19 = <Record_19>::default();
    assert!(((record.id) == (100)));
    assert!(((unsafe { record_id_in_b_23() }) == (200)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut depth_20)) == (3)));
    assert!(((unsafe { depth_in_b_24() }) == (4)));
}
pub unsafe fn test_26() {
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut shared_total_25)) == (5)));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut shared_total_25)) = 7;
    (unsafe { add_in_b_27(3) });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut shared_total_25)) == (10)));
}
pub unsafe fn test_29() {
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut level_28)) == (1)));
    assert!(((unsafe { raise_30(2,) }) == (3)));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut level_28)) = 10;
    assert!(((unsafe { raise_30(1,) }) == (11)));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Tally_31 {}
impl Tally_31 {
    pub unsafe fn add(&mut self, mut amount: i32) -> i32 {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut total_32)) += amount;
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut total_32));
    }
}
pub static mut total_32: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 10 });
pub unsafe fn test_33() {
    let mut tally: Tally_31 = <Tally_31>::default();
    assert!(((unsafe { Tally_31::add(&mut tally, 1,) }) == (11)));
    assert!(((unsafe { add_in_b_34(1,) }) == (102)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut total_32)) == (11)));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p5_members_of_class_with_external_linkage_Ledger {}
pub unsafe fn test_36() {
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut balance_35)) == (50)));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut balance_35)) = 60;
    assert!(((unsafe { p5_members_of_class_with_external_linkage_Ledger::deposit(5,) }) == (65)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut balance_35)) == (65)));
}
pub unsafe fn read_remote_37() -> i32 {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut remote_38));
}
pub unsafe fn test_39() {
    assert!(((unsafe { read_remote_37() }) == (70)));
    (unsafe { set_remote_in_b_40(71) });
    assert!(((unsafe { read_remote_37() }) == (71)));
}
pub static mut hidden_41: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 80 });
pub unsafe fn read_hidden_through_extern_42() -> i32 {
    let mut hidden: i32 = 1;
    {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut hidden_41));
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_43() {
    assert!(((unsafe { read_hidden_through_extern_42() }) == (80)));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut hidden_41)) = 81;
    assert!(((unsafe { read_hidden_through_extern_42() }) == (81)));
    assert!(((unsafe { hidden_in_b_44() }) == (90)));
}
pub unsafe fn next_even_45() -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_46)) += 2;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut count_46));
}
pub unsafe fn next_odd_47() -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_48)) += 2;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut count_48));
}
pub unsafe fn test_49() {
    assert!(((unsafe { next_even_45() }) == (2)));
    assert!(((unsafe { next_odd_47() }) == (3)));
    assert!(((unsafe { next_in_b_50() }) == (1000)));
    assert!(((unsafe { next_even_45() }) == (4)));
    assert!(((unsafe { next_odd_47() }) == (5)));
    assert!(((unsafe { next_in_b_50() }) == (2000)));
}
static mut count_46: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
static mut count_48: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 1 });
pub unsafe fn first_key_51() -> i32 {
    let mut entry: Entry_52 = <Entry_52>::default();
    return entry.key;
}
pub unsafe fn second_key_plus_value_53() -> i32 {
    let mut entry: Entry_54 = <Entry_54>::default();
    return ((entry.key) + (entry.value));
}
pub unsafe fn test_55() {
    assert!(((unsafe { first_key_51() }) == (1)));
    assert!(((unsafe { second_key_plus_value_53() }) == (32)));
    assert!(((unsafe { key_in_b_56() }) == (400)));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Entry_52 {
    pub key: i32,
}
impl Default for Entry_52 {
    fn default() -> Self {
        unsafe { Entry_52 { key: 1 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Entry_54 {
    pub key: i32,
    pub value: i32,
}
impl Default for Entry_54 {
    fn default() -> Self {
        unsafe { Entry_54 { key: 2, value: 30 } }
    }
}
pub unsafe fn test_58() {
    assert!((((*std::cell::LazyCell::force_mut(&mut *&raw mut table_57))[(1) as usize]) == (2)));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut table_57))[(2) as usize] = 30;
    assert!(((unsafe { table_sum_in_b_59() }) == (33)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_1() });
    (unsafe { test_4() });
    (unsafe { test_8() });
    (unsafe { test_11() });
    (unsafe { test_14() });
    (unsafe { test_21() });
    (unsafe { test_26() });
    (unsafe { test_29() });
    (unsafe { test_33() });
    (unsafe { test_36() });
    (unsafe { test_39() });
    (unsafe { test_43() });
    (unsafe { test_49() });
    (unsafe { test_55() });
    (unsafe { test_58() });
    return 0;
}
pub unsafe fn pick_60() -> i32 {
    return 2;
}
pub unsafe fn pick_in_b_2() -> i32 {
    return (unsafe { pick_60() });
}
pub static mut limit_61: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 20 });
pub unsafe fn limit_in_b_5() -> i32 {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut limit_61));
}
pub unsafe fn limit_address_in_b_6() -> *const i32 {
    return (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut limit_61)) as *const i32);
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
pub union anon_62 {
    pub whole: i32,
    pub fraction: f32,
}
impl Default for anon_62 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub static mut anon_62: std::cell::LazyCell<anon_62> =
    std::cell::LazyCell::new(|| unsafe { <anon_62>::default() });
pub unsafe fn set_whole_in_b_15(mut value: i32) {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut anon_62)).whole = value;
}
pub unsafe fn whole_in_b_16() -> i32 {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut anon_62)).whole;
}
pub static mut counter_63: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 50 });
pub unsafe fn bump_64() -> i32 {
    return (*(*std::cell::LazyCell::force_mut(&mut *&raw mut counter_63)).prefix_inc());
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Record_65 {
    pub id: i64,
    pub extra: i32,
}
impl Default for Record_65 {
    fn default() -> Self {
        unsafe {
            Record_65 {
                id: 200_i64,
                extra: 1,
            }
        }
    }
}
pub static mut depth_66: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 4 });
pub unsafe fn bump_in_b_22() -> i32 {
    return (unsafe { bump_64() });
}
pub unsafe fn record_id_in_b_23() -> i32 {
    let mut record: Record_65 = <Record_65>::default();
    return (record.id as i32);
}
pub unsafe fn depth_in_b_24() -> i32 {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut depth_66));
}
pub static mut shared_total_25: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 5 });
pub unsafe fn add_in_b_27(mut amount: i32) {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut shared_total_25)) += amount;
}
pub static mut level_28: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 1 });
pub unsafe fn raise_30(mut by: i32) -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut level_28)) += by;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut level_28));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Tally_67 {}
impl Tally_67 {
    pub unsafe fn add(&mut self, mut amount: i32) -> i32 {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut total_68)) += ((amount) * (2));
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut total_68));
    }
}
pub static mut total_68: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 100 });
pub unsafe fn add_in_b_34(mut amount: i32) -> i32 {
    let mut tally: Tally_67 = <Tally_67>::default();
    return (unsafe { Tally_67::add(&mut tally, amount) });
}
pub static mut balance_35: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 50 });
impl p5_members_of_class_with_external_linkage_Ledger {
    pub unsafe fn deposit(mut amount: i32) -> i32 {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut balance_35)) += amount;
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut balance_35));
    }
}
pub static mut remote_38: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 70 });
pub unsafe fn set_remote_in_b_40(mut value: i32) {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut remote_38)) = value;
}
pub static mut hidden_69: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 90 });
pub unsafe fn hidden_in_b_44() -> i32 {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut hidden_69));
}
pub unsafe fn next_in_b_50() -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_70)) += 1000;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut count_70));
}
static mut count_70: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub unsafe fn key_in_b_56() -> i32 {
    let mut entry: Entry_71 = <Entry_71>::default();
    return (entry.key as i32);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Entry_71 {
    pub key: i64,
}
impl Default for Entry_71 {
    fn default() -> Self {
        unsafe { Entry_71 { key: 400_i64 } }
    }
}
pub static mut table_57: std::cell::LazyCell<[i32; 3]> =
    std::cell::LazyCell::new(|| unsafe { [1, 2, 3] });
pub unsafe fn table_sum_in_b_59() -> i32 {
    return ((((*std::cell::LazyCell::force_mut(&mut *&raw mut table_57))[(0) as usize])
        + ((*std::cell::LazyCell::force_mut(&mut *&raw mut table_57))[(1) as usize]))
        + ((*std::cell::LazyCell::force_mut(&mut *&raw mut table_57))[(2) as usize]));
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const limit_3);
    std::cell::LazyCell::force(&*&raw const anon_13);
    std::cell::LazyCell::force(&*&raw const counter_17);
    std::cell::LazyCell::force(&*&raw const depth_20);
    std::cell::LazyCell::force(&*&raw const total_32);
    std::cell::LazyCell::force(&*&raw const hidden_41);
    std::cell::LazyCell::force(&*&raw const limit_61);
    std::cell::LazyCell::force(&*&raw const threshold_7);
    std::cell::LazyCell::force(&*&raw const ceiling_10);
    std::cell::LazyCell::force(&*&raw const anon_62);
    std::cell::LazyCell::force(&*&raw const counter_63);
    std::cell::LazyCell::force(&*&raw const depth_66);
    std::cell::LazyCell::force(&*&raw const shared_total_25);
    std::cell::LazyCell::force(&*&raw const level_28);
    std::cell::LazyCell::force(&*&raw const total_68);
    std::cell::LazyCell::force(&*&raw const balance_35);
    std::cell::LazyCell::force(&*&raw const remote_38);
    std::cell::LazyCell::force(&*&raw const hidden_69);
    std::cell::LazyCell::force(&*&raw const table_57);
}
