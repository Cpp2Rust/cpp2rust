extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut level_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 11 });
pub unsafe fn test_1() {
    let mut level: i32 = 22;
    assert!(((level) == (22)));
    {
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut level_0)) == (11)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut level_0)) = 33;
    }
    assert!(((level) == (22)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut level_0)) == (33)));
}
pub static mut function_calls_2: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub unsafe fn test_3() {
    let mut measure: i32 = 5;
    {
        assert!(((unsafe { measure_4() }) == (7)));
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut function_calls_2)) == (1)));
    }
    assert!(((measure) == (5)));
}
pub unsafe fn measure_4() -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut function_calls_2)).prefix_inc();
    return 7;
}
pub type p2_11_unscoped_enumerator_in_both_scopes_Direction = u32;
pub const p2_11_unscoped_enumerator_in_both_scopes_Direction_north:
    p2_11_unscoped_enumerator_in_both_scopes_Direction = 4;
pub const p2_11_unscoped_enumerator_in_both_scopes_Direction_south:
    p2_11_unscoped_enumerator_in_both_scopes_Direction = 9;
pub type p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum_Size = u32;
pub const p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum_Size_small:
    p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum_Size = 3;
pub const p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum_Size_large:
    p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum_Size = 8;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum {}
impl p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum {
    pub unsafe fn scale(&self, mut count: i32) -> i32 {
        return ((count)
            * (p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum_Size_large as i32));
    }
}
pub unsafe fn test_5() {
    assert!(((p2_11_unscoped_enumerator_in_both_scopes_Direction_north as i32) == (4)));
    assert!(((p2_11_unscoped_enumerator_in_both_scopes_Direction_north as i32) == (4)));
    assert!(((p2_11_unscoped_enumerator_in_both_scopes_Direction_south as i32) == (9)));
    assert!(((p2_11_unscoped_enumerator_in_both_scopes_Direction_south as i32) == (9)));
    assert!(((p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum_Size_small as i32) == (3)));
    assert!(((p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum_Size_small as i32) == (3)));
    assert!(((p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum_Size_large as i32) == (8)));
    assert!(((p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum_Size_large as i32) == (8)));
    let mut object: p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum =
        <p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum>::default();
    assert!(
        ((unsafe { p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum::scale(&object, 2,) })
            == (16))
    );
    assert!(((p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum_Size_small as i32) == (3)));
}
pub static mut count_6: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p2_12_class_name_in_own_scope_HasStaticCounter {}
impl p2_12_class_name_in_own_scope_HasStaticCounter {
    pub unsafe fn bump() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_6)).prefix_inc();
    }
}
pub unsafe fn test_7() {
    (unsafe { p2_12_class_name_in_own_scope_HasStaticCounter::bump() });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_6)) == (1)));
    (unsafe { p2_12_class_name_in_own_scope_HasStaticCounter::bump() });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_6)) == (2)));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_6)) = 6;
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_6)) == (6)));
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_8 {
    pub namespace_count: i32,
    pub namespace_ratio: f64,
}
impl Default for anon_8 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub static mut anon_8: std::cell::LazyCell<anon_8> =
    std::cell::LazyCell::new(|| unsafe { <anon_8>::default() });
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_9 {
    pub member_count: i32,
    pub member_ratio: f64,
}
impl Default for anon_9 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p2_13_anonymous_union_members_in_parent_scope_HasAnonymousUnion {
    pub tag: i32,
    pub anon_9: anon_9,
}
impl p2_13_anonymous_union_members_in_parent_scope_HasAnonymousUnion {
    pub unsafe fn read_count(&self) -> i32 {
        return self.anon_9.member_count;
    }
}
pub unsafe fn test_10() {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut anon_8)).namespace_count = 4;
    assert!((((*std::cell::LazyCell::force_mut(&mut *&raw mut anon_8)).namespace_count) == (4)));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut anon_8)).namespace_ratio = 0.25_f64;
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut anon_8)).namespace_ratio) == (0.25_f64))
    );
    let mut object: p2_13_anonymous_union_members_in_parent_scope_HasAnonymousUnion =
        <p2_13_anonymous_union_members_in_parent_scope_HasAnonymousUnion>::default();
    object.tag = 1;
    object.anon_9.member_count = 6;
    assert!(((object.anon_9.member_count) == (6)));
    assert!(
        ((unsafe {
            p2_13_anonymous_union_members_in_parent_scope_HasAnonymousUnion::read_count(&object)
        }) == (6))
    );
    object.anon_9.member_ratio = 0.5_f64;
    assert!(((object.anon_9.member_ratio) == (0.5_f64)));
    assert!(((object.tag) == (1)));
    let mut anon_11: anon_11 = <anon_11>::default();
    anon_11.block_count = 9;
    assert!(((anon_11.block_count) == (9)));
    anon_11.block_ratio = 0.75_f64;
    assert!(((anon_11.block_ratio) == (0.75_f64)));
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_11 {
    pub block_count: i32,
    pub block_ratio: f64,
}
impl Default for anon_11 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p3_2_const_qualified_member_is_distinct_HasConstOverload {
    pub plain_calls: i32,
    pub const_calls: i32,
}
impl p3_2_const_qualified_member_is_distinct_HasConstOverload {
    pub unsafe fn get_1(&mut self) -> i32 {
        self.plain_calls.prefix_inc();
        return 1;
    }
    pub unsafe fn get_2(&self) -> i32 {
        (*(&raw const self.const_calls).cast_mut()).prefix_inc();
        return 2;
    }
}
impl Default for p3_2_const_qualified_member_is_distinct_HasConstOverload {
    fn default() -> Self {
        unsafe {
            p3_2_const_qualified_member_is_distinct_HasConstOverload {
                plain_calls: 0,
                const_calls: 0,
            }
        }
    }
}
pub unsafe fn test_12() {
    let mut object: p3_2_const_qualified_member_is_distinct_HasConstOverload =
        <p3_2_const_qualified_member_is_distinct_HasConstOverload>::default();
    let view: *const p3_2_const_qualified_member_is_distinct_HasConstOverload = &object;
    assert!(
        ((unsafe {
            p3_2_const_qualified_member_is_distinct_HasConstOverload::get_1(&mut object)
        }) == (1))
    );
    assert!(((object.plain_calls) == (1)));
    assert!(((object.const_calls) == (0)));
    assert!(
        ((unsafe { p3_2_const_qualified_member_is_distinct_HasConstOverload::get_2(&(*view),) })
            == (2))
    );
    assert!(((object.plain_calls) == (1)));
    assert!(((object.const_calls) == (1)));
}
pub static mut explicit_calls_13: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut rvalue_calls_14: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload {
    pub base: i32,
}
impl p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload {
    pub unsafe fn pick_1(
        self_: *mut p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload,
        mut amount: i32,
    ) -> i32 {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut explicit_calls_13)).prefix_inc();
        return (((*self_).base) + (amount));
    }
    pub unsafe fn pick_2(&mut self, mut amount: i32) -> i32 {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut rvalue_calls_14)).prefix_inc();
        return (((self.base) + (amount)) + (100));
    }
}
impl Default for p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload {
    fn default() -> Self {
        unsafe {
            p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload {
                base: 10,
            }
        }
    }
}
pub unsafe fn test_15() {
    let mut object: p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload =
        <p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload>::default();
    assert!(
        ((unsafe {
            p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload::pick_1(
                &mut object,
                1,
            )
        }) == (11))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut explicit_calls_13)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut rvalue_calls_14)) == (0)));
    assert!(
        ((unsafe {
            p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload :: pick_2 ( &mut <p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload >::default()  , 2 , )
        }) == (112))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut explicit_calls_13)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut rvalue_calls_14)) == (1)));
}
pub static mut lvalue_calls_16: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut rvalue_calls_17: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p3_2_ref_qualified_members_are_distinct_HasRefQualifiedOverloads {}
impl p3_2_ref_qualified_members_are_distinct_HasRefQualifiedOverloads {
    pub unsafe fn get_1(&mut self) -> i32 {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut lvalue_calls_16)).prefix_inc();
        return 3;
    }
    pub unsafe fn get_2(&mut self) -> i32 {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut rvalue_calls_17)).prefix_inc();
        return 4;
    }
}
pub unsafe fn test_18() {
    let mut object: p3_2_ref_qualified_members_are_distinct_HasRefQualifiedOverloads =
        <p3_2_ref_qualified_members_are_distinct_HasRefQualifiedOverloads>::default();
    assert!(
        ((unsafe {
            p3_2_ref_qualified_members_are_distinct_HasRefQualifiedOverloads::get_1(&mut object)
        }) == (3))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut lvalue_calls_16)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut rvalue_calls_17)) == (0)));
    assert!(
        ((unsafe {
            p3_2_ref_qualified_members_are_distinct_HasRefQualifiedOverloads::get_2(
                &mut <p3_2_ref_qualified_members_are_distinct_HasRefQualifiedOverloads>::default(),
            )
        }) == (4))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut lvalue_calls_16)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut rvalue_calls_17)) == (1)));
    assert!(
        ((unsafe {
            p3_2_ref_qualified_members_are_distinct_HasRefQualifiedOverloads::get_2(&mut object)
        }) == (4))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut lvalue_calls_16)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut rvalue_calls_17)) == (2)));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverload {
    pub plain_calls: i32,
    pub const_calls: i32,
}
impl p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverload {
    pub unsafe fn get_1_int(&mut self, mut value: i32) -> i32 {
        self.plain_calls.prefix_inc();
        return ((value) + (1));
    }
    pub unsafe fn get_2_int(&self, mut value: i32) -> i32 {
        (*(&raw const self.const_calls).cast_mut()).prefix_inc();
        return ((value) + (2));
    }
}
impl Default for p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverload {
    fn default() -> Self {
        unsafe {
            p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverload {
                plain_calls: 0,
                const_calls: 0,
            }
        }
    }
}
pub unsafe fn test_19() {
    let mut object: p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverload =
        <p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverload>::default();
    let view: *const p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverload =
        &object;
    assert!(
        ((unsafe {
            p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverload::get_1_int(
                &mut object,
                10,
            )
        }) == (11))
    );
    assert!(((object.plain_calls) == (1)));
    assert!(((object.const_calls) == (0)));
    assert!(
        ((unsafe {
            p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverload::get_2_int(
                &(*view),
                10,
            )
        }) == (12))
    );
    assert!(((object.plain_calls) == (1)));
    assert!(((object.const_calls) == (1)));
}
pub static mut lvalue_calls_20: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut rvalue_calls_21: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p3_4_ref_qualified_member_templates_are_distinct_HasRefQualifiedTemplateOverloads {}
impl p3_4_ref_qualified_member_templates_are_distinct_HasRefQualifiedTemplateOverloads {
    pub unsafe fn get_1_int(&mut self, mut value: i32) -> i32 {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut lvalue_calls_20)).prefix_inc();
        return ((value) + (3));
    }
    pub unsafe fn get_2_int(&mut self, mut value: i32) -> i32 {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut rvalue_calls_21)).prefix_inc();
        return ((value) + (4));
    }
}
pub unsafe fn test_22() {
    let mut object : p3_4_ref_qualified_member_templates_are_distinct_HasRefQualifiedTemplateOverloads = <p3_4_ref_qualified_member_templates_are_distinct_HasRefQualifiedTemplateOverloads >::default() ;
    assert!(
        ((unsafe {
            p3_4_ref_qualified_member_templates_are_distinct_HasRefQualifiedTemplateOverloads :: get_1_int ( &mut object  , 10 , )
        }) == (13))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut lvalue_calls_20)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut rvalue_calls_21)) == (0)));
    assert!(
        ((unsafe {
            p3_4_ref_qualified_member_templates_are_distinct_HasRefQualifiedTemplateOverloads :: get_2_int ( &mut <p3_4_ref_qualified_member_templates_are_distinct_HasRefQualifiedTemplateOverloads >::default()  , 10 , )
        }) == (14))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut lvalue_calls_20)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut rvalue_calls_21)) == (1)));
    assert!(
        ((unsafe {
            p3_4_ref_qualified_member_templates_are_distinct_HasRefQualifiedTemplateOverloads :: get_2_int ( &mut object  , 10 , )
        }) == (14))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut lvalue_calls_20)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut rvalue_calls_21)) == (2)));
}
pub static mut friend_calls_23: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p4_friend_definition_defines_earlier_friend_declaration_DeclaresFriend {}
pub unsafe fn shared_friend_24() -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut friend_calls_23)).prefix_inc();
    return 7;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p4_friend_definition_defines_earlier_friend_declaration_DefinesFriend {}
pub unsafe fn test_25() {
    assert!(((unsafe { shared_friend_24() }) == (7)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut friend_calls_23)) == (1)));
}
pub static mut library_calls_26: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut local_calls_27: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub unsafe fn describe_28(mut amount: i32) -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut library_calls_26)).prefix_inc();
    return ((amount) + (10));
}
pub unsafe fn describe_29(mut amount: f64) -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut local_calls_27)).prefix_inc();
    return ((amount as i32) + (20));
}
pub unsafe fn test_30() {
    assert!(((unsafe { describe_28(1,) }) == (11)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut library_calls_26)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut local_calls_27)) == (0)));
    assert!(((unsafe { describe_29(2_f64,) }) == (22)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut library_calls_26)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut local_calls_27)) == (1)));
}
pub static mut function_calls_31: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p4_2_class_and_function_share_name_Reading {
    pub value: i32,
}
pub unsafe fn Reading_32(mut raw_: i32) -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut function_calls_31)).prefix_inc();
    return ((raw_) + (1));
}
pub unsafe fn test_33() {
    let mut reading: p4_2_class_and_function_share_name_Reading =
        p4_2_class_and_function_share_name_Reading { value: 4 };
    assert!(((reading.value) == (4)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut function_calls_31)) == (0)));
    assert!(((unsafe { Reading_32(reading.value,) }) == (5)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut function_calls_31)) == (1)));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p4_2_class_and_variable_share_name_Limit {
    pub value: i32,
}
pub static mut Limit_34: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 12 });
pub unsafe fn test_35() {
    let mut limit: p4_2_class_and_variable_share_name_Limit =
        p4_2_class_and_variable_share_name_Limit { value: 3 };
    assert!(((limit.value) == (3)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut Limit_34)) == (12)));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut Limit_34)) += limit.value;
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut Limit_34)) == (15)));
    assert!(((limit.value) == (3)));
}
pub static mut int_calls_36: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut unit_calls_37: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub type p4_3_1_typedef_parameter_redeclares_function_Unit = i32;
pub const p4_3_1_typedef_parameter_redeclares_function_Unit_single:
    p4_3_1_typedef_parameter_redeclares_function_Unit = 1;
pub unsafe fn describe_38(mut unit: p4_3_1_typedef_parameter_redeclares_function_Unit) -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut unit_calls_37)).prefix_inc();
    return ((unit as i32) + (20));
}
pub unsafe fn test_39() {
    assert!(((unsafe { describe_40(3,) }) == (13)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut int_calls_36)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut unit_calls_37)) == (0)));
    assert!(
        ((unsafe { describe_38(p4_3_1_typedef_parameter_redeclares_function_Unit_single,) })
            == (21))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut int_calls_36)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut unit_calls_37)) == (1)));
}
pub unsafe fn describe_40(mut amount: i32) -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut int_calls_36)).prefix_inc();
    return ((amount) + (10));
}
pub static mut value_calls_41: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut pointer_calls_42: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub unsafe fn describe_43(mut renamed: i32) -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut value_calls_41)).prefix_inc();
    return ((renamed) + (10));
}
pub unsafe fn describe_44(mut pointer: *mut i32) -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut pointer_calls_42)).prefix_inc();
    return ((*pointer) + (20));
}
pub unsafe fn test_45() {
    let mut amount: i32 = 3;
    assert!(((unsafe { describe_43(amount,) }) == (13)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut value_calls_41)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut pointer_calls_42)) == (0)));
    assert!(((unsafe { describe_44((&mut amount as *mut i32),) }) == (23)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut value_calls_41)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut pointer_calls_42)) == (1)));
}
pub static mut value_calls_46: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut pointer_calls_47: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub unsafe fn describe_48(mut value: i32) -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut value_calls_46)).prefix_inc();
    return ((value) + (10));
}
pub unsafe fn describe_49(mut pointer: *mut i32) -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut pointer_calls_47)).prefix_inc();
    return ((*pointer) + (20));
}
pub unsafe fn test_50() {
    let mut amount: i32 = 3;
    assert!(((unsafe { describe_48(amount,) }) == (13)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut value_calls_46)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut pointer_calls_47)) == (0)));
    assert!(((unsafe { describe_49((&mut amount as *mut i32),) }) == (23)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut value_calls_46)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut pointer_calls_47)) == (1)));
}
pub static mut type_parameter_calls_51: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut value_parameter_calls_52: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub unsafe fn describe_53(mut value: i32) -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut type_parameter_calls_51)).prefix_inc();
    return ((value) + (10));
}
pub unsafe fn describe_54() -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut value_parameter_calls_52)).prefix_inc();
    return ((3) + (20));
}
pub unsafe fn test_55() {
    assert!(((unsafe { describe_53(3,) }) == (13)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut type_parameter_calls_51)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut value_parameter_calls_52)) == (0)));
    assert!(((unsafe { describe_54() }) == (23)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut type_parameter_calls_51)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut value_parameter_calls_52)) == (1)));
}
pub static mut version_calls_56: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub unsafe fn version_58() -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut version_calls_56)).prefix_inc();
    return 2;
}
pub static mut revision_57: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 40 });
pub unsafe fn test_59() {
    assert!(((unsafe { version_58() }) == (2)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut version_calls_56)) == (1)));
    assert!(((unsafe { version_58() }) == (2)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut version_calls_56)) == (2)));
    assert!(
        ((&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut revision_57)) as *mut i32)
            == (&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut revision_57))
                as *mut i32))
    );
    (*std::cell::LazyCell::force_mut(&mut *&raw mut revision_57)) = 41;
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut revision_57)) == (41)));
}
pub unsafe fn next_id_60() -> i32 {
    return (*(*std::cell::LazyCell::force_mut(&mut *&raw mut last_61)).prefix_inc());
}
pub unsafe fn next_id_62() -> i32 {
    return (*(*std::cell::LazyCell::force_mut(&mut *&raw mut last_63)).prefix_inc());
}
pub unsafe fn test_64() {
    assert!(((unsafe { next_id_60() }) == (1)));
    assert!(((unsafe { next_id_60() }) == (2)));
    assert!(((unsafe { next_id_62() }) == (1)));
    assert!(((unsafe { next_id_60() }) == (3)));
    assert!(((unsafe { next_id_62() }) == (2)));
}
static mut last_61: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
static mut last_63: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_1() });
    (unsafe { test_3() });
    (unsafe { test_5() });
    (unsafe { test_7() });
    (unsafe { test_10() });
    (unsafe { test_12() });
    (unsafe { test_15() });
    (unsafe { test_18() });
    (unsafe { test_19() });
    (unsafe { test_22() });
    (unsafe { test_25() });
    (unsafe { test_30() });
    (unsafe { test_33() });
    (unsafe { test_35() });
    (unsafe { test_39() });
    (unsafe { test_45() });
    (unsafe { test_50() });
    (unsafe { test_55() });
    (unsafe { test_59() });
    (unsafe { test_64() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const level_0);
    std::cell::LazyCell::force(&*&raw const function_calls_2);
    std::cell::LazyCell::force(&*&raw const count_6);
    std::cell::LazyCell::force(&*&raw const anon_8);
    std::cell::LazyCell::force(&*&raw const explicit_calls_13);
    std::cell::LazyCell::force(&*&raw const rvalue_calls_14);
    std::cell::LazyCell::force(&*&raw const lvalue_calls_16);
    std::cell::LazyCell::force(&*&raw const rvalue_calls_17);
    std::cell::LazyCell::force(&*&raw const lvalue_calls_20);
    std::cell::LazyCell::force(&*&raw const rvalue_calls_21);
    std::cell::LazyCell::force(&*&raw const friend_calls_23);
    std::cell::LazyCell::force(&*&raw const library_calls_26);
    std::cell::LazyCell::force(&*&raw const local_calls_27);
    std::cell::LazyCell::force(&*&raw const function_calls_31);
    std::cell::LazyCell::force(&*&raw const Limit_34);
    std::cell::LazyCell::force(&*&raw const int_calls_36);
    std::cell::LazyCell::force(&*&raw const unit_calls_37);
    std::cell::LazyCell::force(&*&raw const value_calls_41);
    std::cell::LazyCell::force(&*&raw const pointer_calls_42);
    std::cell::LazyCell::force(&*&raw const value_calls_46);
    std::cell::LazyCell::force(&*&raw const pointer_calls_47);
    std::cell::LazyCell::force(&*&raw const type_parameter_calls_51);
    std::cell::LazyCell::force(&*&raw const value_parameter_calls_52);
    std::cell::LazyCell::force(&*&raw const version_calls_56);
    std::cell::LazyCell::force(&*&raw const revision_57);
}
