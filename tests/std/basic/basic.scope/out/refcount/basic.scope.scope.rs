extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static level_0: Value<i32> = Rc::new(RefCell::new(11));
);
pub fn test_1() {
    let mut level: i32 = 22;
    assert!((level == 22));
    {
        assert!((level_0.with(|rc| *rc.borrow()) == 11));
        level_0.with(|rc| *rc.borrow_mut() = 33);
    }
    assert!((level == 22));
    assert!((level_0.with(|rc| *rc.borrow()) == 33));
}
thread_local!(
    pub static function_calls_2: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn test_3() {
    let mut measure: i32 = 5;
    {
        assert!((({ measure_4() }) == 7));
        assert!((function_calls_2.with(|rc| *rc.borrow()) == 1));
    }
    assert!((measure == 5));
}
pub fn measure_4() -> i32 {
    (*function_calls_2.with(Value::clone).borrow_mut()).prefix_inc();
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum {}
pub fn test_5() {
    assert!(((p2_11_unscoped_enumerator_in_both_scopes_Direction_north as i32) == 4));
    assert!(((p2_11_unscoped_enumerator_in_both_scopes_Direction_north as i32) == 4));
    assert!(((p2_11_unscoped_enumerator_in_both_scopes_Direction_south as i32) == 9));
    assert!(((p2_11_unscoped_enumerator_in_both_scopes_Direction_south as i32) == 9));
    assert!(((p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum_Size_small as i32) == 3));
    assert!(((p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum_Size_small as i32) == 3));
    assert!(((p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum_Size_large as i32) == 8));
    assert!(((p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum_Size_large as i32) == 8));
    let object: Value<p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum> = Rc::new(
        RefCell::new(<p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum>::default()),
    );
    assert!(
        (({
            p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnumImpl::scale(
                &object.as_pointer(),
                2,
            )
        }) == 16)
    );
    assert!(((p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum_Size_small as i32) == 3));
}
thread_local!(
    pub static count_6: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct p2_12_class_name_in_own_scope_HasStaticCounter {}
impl p2_12_class_name_in_own_scope_HasStaticCounter {
    pub fn bump() {
        (*count_6.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub fn test_7() {
    ({ p2_12_class_name_in_own_scope_HasStaticCounter::bump() });
    assert!((count_6.with(|rc| *rc.borrow()) == 1));
    ({ p2_12_class_name_in_own_scope_HasStaticCounter::bump() });
    assert!((count_6.with(|rc| *rc.borrow()) == 2));
    count_6.with(|rc| *rc.borrow_mut() = 6);
    assert!((count_6.with(|rc| *rc.borrow()) == 6));
}
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(8)]
pub struct anon_8 {
    #[offset(0)]
    #[byte_size(8)]
    __bytes: Box<[u8]>,
}
impl anon_8 {
    pub fn namespace_count(this: Ptr<Self>) -> Ptr<i32> {
        this.reinterpret_cast()
    }
    pub fn namespace_ratio(this: Ptr<Self>) -> Ptr<f64> {
        this.reinterpret_cast()
    }
}
impl Default for anon_8 {
    fn default() -> Self {
        anon_8 {
            __bytes: Box::from([0u8; 8]),
        }
    }
}
thread_local!(
    pub static anon_8: Value<anon_8> = Rc::new(RefCell::new(<anon_8>::default()));
);
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(8)]
pub struct anon_9 {
    #[offset(0)]
    #[byte_size(8)]
    __bytes: Box<[u8]>,
}
impl anon_9 {
    pub fn member_count(this: Ptr<Self>) -> Ptr<i32> {
        this.reinterpret_cast()
    }
    pub fn member_ratio(this: Ptr<Self>) -> Ptr<f64> {
        this.reinterpret_cast()
    }
}
impl Default for anon_9 {
    fn default() -> Self {
        anon_9 {
            __bytes: Box::from([0u8; 8]),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct p2_13_anonymous_union_members_in_parent_scope_HasAnonymousUnion {
    #[offset(0)]
    pub tag: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub anon_9: anon_9,
}
pub fn test_10() {
    anon_8::namespace_count(anon_8.with(|v| v.as_pointer())).write(4);
    assert!(((anon_8::namespace_count(anon_8.with(|v| v.as_pointer())).read()) == 4));
    anon_8::namespace_ratio(anon_8.with(|v| v.as_pointer())).write(0.25_f64);
    assert!(((anon_8::namespace_ratio(anon_8.with(|v| v.as_pointer())).read()) == 0.25_f64));
    let object: Value<p2_13_anonymous_union_members_in_parent_scope_HasAnonymousUnion> = Rc::new(
        RefCell::new(<p2_13_anonymous_union_members_in_parent_scope_HasAnonymousUnion>::default()),
    );
    (*object.borrow_mut()).tag = 1;
    anon_9::member_count(field_ptr!(object.as_pointer(), anon_9)).write(6);
    assert!(((anon_9::member_count(field_ptr!(object.as_pointer(), anon_9)).read()) == 6));
    assert!(
        (({
            p2_13_anonymous_union_members_in_parent_scope_HasAnonymousUnionImpl::read_count(
                &object.as_pointer(),
            )
        }) == 6)
    );
    anon_9::member_ratio(field_ptr!(object.as_pointer(), anon_9)).write(0.5_f64);
    assert!(((anon_9::member_ratio(field_ptr!(object.as_pointer(), anon_9)).read()) == 0.5_f64));
    assert!(({ ({ (*object.borrow()).tag } == 1) }));
    let anon_11: Value<anon_11> = Rc::new(RefCell::new(<anon_11>::default()));
    anon_11::block_count(anon_11.as_pointer()).write(9);
    assert!(((anon_11::block_count(anon_11.as_pointer()).read()) == 9));
    anon_11::block_ratio(anon_11.as_pointer()).write(0.75_f64);
    assert!(((anon_11::block_ratio(anon_11.as_pointer()).read()) == 0.75_f64));
}
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(8)]
pub struct anon_11 {
    #[offset(0)]
    #[byte_size(8)]
    __bytes: Box<[u8]>,
}
impl anon_11 {
    pub fn block_count(this: Ptr<Self>) -> Ptr<i32> {
        this.reinterpret_cast()
    }
    pub fn block_ratio(this: Ptr<Self>) -> Ptr<f64> {
        this.reinterpret_cast()
    }
}
impl Default for anon_11 {
    fn default() -> Self {
        anon_11 {
            __bytes: Box::from([0u8; 8]),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct p3_2_const_qualified_member_is_distinct_HasConstOverload {
    #[offset(0)]
    pub plain_calls: i32,
    #[offset(4)]
    pub const_calls: i32,
}
impl Default for p3_2_const_qualified_member_is_distinct_HasConstOverload {
    fn default() -> Self {
        {
            p3_2_const_qualified_member_is_distinct_HasConstOverload {
                plain_calls: 0,
                const_calls: 0,
            }
        }
    }
}
pub fn test_12() {
    let object: Value<p3_2_const_qualified_member_is_distinct_HasConstOverload> = Rc::new(
        RefCell::new(<p3_2_const_qualified_member_is_distinct_HasConstOverload>::default()),
    );
    let view: Ptr<p3_2_const_qualified_member_is_distinct_HasConstOverload> = object.as_pointer();
    assert!(
        (({
            p3_2_const_qualified_member_is_distinct_HasConstOverloadImpl::get_1(
                &object.as_pointer(),
            )
        }) == 1)
    );
    assert!(({ ({ (*object.borrow()).plain_calls } == 1) }));
    assert!(({ ({ (*object.borrow()).const_calls } == 0) }));
    assert!(
        (({ p3_2_const_qualified_member_is_distinct_HasConstOverloadImpl::get_2(&view,) }) == 2)
    );
    assert!(({ ({ (*object.borrow()).plain_calls } == 1) }));
    assert!(({ ({ (*object.borrow()).const_calls } == 1) }));
}
thread_local!(
    pub static explicit_calls_13: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static rvalue_calls_14: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload {
    #[offset(0)]
    pub base: i32,
}
impl Default for p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload {
    fn default() -> Self {
        {
            p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload {
                base: 10,
            }
        }
    }
}
pub fn test_15() {
    let object: Value<
        p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload,
    > = Rc::new(RefCell::new(
        <p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload>::default(),
    ));
    assert!(
        (({
            <Ptr<p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload> as p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverloadImpl>::pick_1 ( object .as_pointer()  , 1  , )
        }) == 11)
    );
    assert!((explicit_calls_13.with(|rc| *rc.borrow()) == 1));
    assert!((rvalue_calls_14.with(|rc| *rc.borrow()) == 0));
    assert!(
        (({
            p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverloadImpl :: pick_2 ( &Rc::new(RefCell::new(<p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload  >::default() )).as_pointer() , 2  , )
        }) == 112)
    );
    assert!((explicit_calls_13.with(|rc| *rc.borrow()) == 1));
    assert!((rvalue_calls_14.with(|rc| *rc.borrow()) == 1));
}
thread_local!(
    pub static lvalue_calls_16: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static rvalue_calls_17: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct p3_2_ref_qualified_members_are_distinct_HasRefQualifiedOverloads {}
pub fn test_18() {
    let object: Value<p3_2_ref_qualified_members_are_distinct_HasRefQualifiedOverloads> = Rc::new(
        RefCell::new(<p3_2_ref_qualified_members_are_distinct_HasRefQualifiedOverloads>::default()),
    );
    assert!(
        (({
            p3_2_ref_qualified_members_are_distinct_HasRefQualifiedOverloadsImpl::get_1(
                &object.as_pointer(),
            )
        }) == 3)
    );
    assert!((lvalue_calls_16.with(|rc| *rc.borrow()) == 1));
    assert!((rvalue_calls_17.with(|rc| *rc.borrow()) == 0));
    assert!(
        (({
            p3_2_ref_qualified_members_are_distinct_HasRefQualifiedOverloadsImpl::get_2(
                &Rc::new(RefCell::new(
                    <p3_2_ref_qualified_members_are_distinct_HasRefQualifiedOverloads>::default(),
                ))
                .as_pointer(),
            )
        }) == 4)
    );
    assert!((lvalue_calls_16.with(|rc| *rc.borrow()) == 1));
    assert!((rvalue_calls_17.with(|rc| *rc.borrow()) == 1));
    assert!(
        (({
            p3_2_ref_qualified_members_are_distinct_HasRefQualifiedOverloadsImpl::get_2(
                &object.as_pointer(),
            )
        }) == 4)
    );
    assert!((lvalue_calls_16.with(|rc| *rc.borrow()) == 1));
    assert!((rvalue_calls_17.with(|rc| *rc.borrow()) == 2));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverload {
    #[offset(0)]
    pub plain_calls: i32,
    #[offset(4)]
    pub const_calls: i32,
}
impl Default for p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverload {
    fn default() -> Self {
        {
            p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverload {
                plain_calls: 0,
                const_calls: 0,
            }
        }
    }
}
pub fn test_19() {
    let object: Value<p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverload> =
        Rc::new(RefCell::new(
            <p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverload>::default(
            ),
        ));
    let view: Ptr<p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverload> =
        object.as_pointer();
    assert!(
        (({
            p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverloadImpl :: get_1_int ( &object .as_pointer()  , 10  , )
        }) == 11)
    );
    assert!(({ ({ (*object.borrow()).plain_calls } == 1) }));
    assert!(({ ({ (*object.borrow()).const_calls } == 0) }));
    assert!(
        (({
            p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverloadImpl :: get_2_int ( &view  , 10  , )
        }) == 12)
    );
    assert!(({ ({ (*object.borrow()).plain_calls } == 1) }));
    assert!(({ ({ (*object.borrow()).const_calls } == 1) }));
}
thread_local!(
    pub static lvalue_calls_20: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static rvalue_calls_21: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct p3_4_ref_qualified_member_templates_are_distinct_HasRefQualifiedTemplateOverloads {}
pub fn test_22() {
    let object : Value<p3_4_ref_qualified_member_templates_are_distinct_HasRefQualifiedTemplateOverloads > = Rc::new(RefCell::new(<p3_4_ref_qualified_member_templates_are_distinct_HasRefQualifiedTemplateOverloads  >::default() )) ;
    assert!(
        (({
            p3_4_ref_qualified_member_templates_are_distinct_HasRefQualifiedTemplateOverloadsImpl :: get_1_int ( &object .as_pointer()  , 10  , )
        }) == 13)
    );
    assert!((lvalue_calls_20.with(|rc| *rc.borrow()) == 1));
    assert!((rvalue_calls_21.with(|rc| *rc.borrow()) == 0));
    assert!(
        (({
            p3_4_ref_qualified_member_templates_are_distinct_HasRefQualifiedTemplateOverloadsImpl :: get_2_int ( &Rc::new(RefCell::new(<p3_4_ref_qualified_member_templates_are_distinct_HasRefQualifiedTemplateOverloads  >::default() )).as_pointer() , 10  , )
        }) == 14)
    );
    assert!((lvalue_calls_20.with(|rc| *rc.borrow()) == 1));
    assert!((rvalue_calls_21.with(|rc| *rc.borrow()) == 1));
    assert!(
        (({
            p3_4_ref_qualified_member_templates_are_distinct_HasRefQualifiedTemplateOverloadsImpl :: get_2_int ( &object .as_pointer()  , 10  , )
        }) == 14)
    );
    assert!((lvalue_calls_20.with(|rc| *rc.borrow()) == 1));
    assert!((rvalue_calls_21.with(|rc| *rc.borrow()) == 2));
}
thread_local!(
    pub static friend_calls_23: Value<i32> = Rc::new(RefCell::new(0_i32));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct p4_friend_definition_defines_earlier_friend_declaration_DeclaresFriend {}
pub fn shared_friend_24() -> i32 {
    (*friend_calls_23.with(Value::clone).borrow_mut()).prefix_inc();
    return 7;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct p4_friend_definition_defines_earlier_friend_declaration_DefinesFriend {}
pub fn test_25() {
    assert!((({ shared_friend_24() }) == 7));
    assert!((friend_calls_23.with(|rc| *rc.borrow()) == 1));
}
thread_local!(
    pub static library_calls_26: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static local_calls_27: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn describe_28(mut amount: i32) -> i32 {
    (*library_calls_26.with(Value::clone).borrow_mut()).prefix_inc();
    return (amount + 10);
}
pub fn describe_29(mut amount: f64) -> i32 {
    (*local_calls_27.with(Value::clone).borrow_mut()).prefix_inc();
    return ((amount as i32) + 20);
}
pub fn test_30() {
    assert!((({ describe_28(1,) }) == 11));
    assert!((library_calls_26.with(|rc| *rc.borrow()) == 1));
    assert!((local_calls_27.with(|rc| *rc.borrow()) == 0));
    assert!((({ describe_29(2_f64,) }) == 22));
    assert!((library_calls_26.with(|rc| *rc.borrow()) == 1));
    assert!((local_calls_27.with(|rc| *rc.borrow()) == 1));
}
thread_local!(
    pub static function_calls_31: Value<i32> = Rc::new(RefCell::new(0_i32));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct p4_2_class_and_function_share_name_Reading {
    #[offset(0)]
    pub value: i32,
}
pub fn Reading_32(mut raw_: i32) -> i32 {
    (*function_calls_31.with(Value::clone).borrow_mut()).prefix_inc();
    return (raw_ + 1);
}
pub fn test_33() {
    let mut reading: p4_2_class_and_function_share_name_Reading =
        p4_2_class_and_function_share_name_Reading { value: 4 };
    assert!((reading.value == 4));
    assert!((function_calls_31.with(|rc| *rc.borrow()) == 0));
    assert!((({ Reading_32(reading.value,) }) == 5));
    assert!((function_calls_31.with(|rc| *rc.borrow()) == 1));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct p4_2_class_and_variable_share_name_Limit {
    #[offset(0)]
    pub value: i32,
}
thread_local!(
    pub static Limit_34: Value<i32> = Rc::new(RefCell::new(12));
);
pub fn test_35() {
    let mut limit: p4_2_class_and_variable_share_name_Limit =
        p4_2_class_and_variable_share_name_Limit { value: 3 };
    assert!((limit.value == 3));
    assert!((Limit_34.with(|rc| *rc.borrow()) == 12));
    Limit_34.with(|rc| *rc.borrow_mut() += limit.value);
    assert!((Limit_34.with(|rc| *rc.borrow()) == 15));
    assert!((limit.value == 3));
}
thread_local!(
    pub static int_calls_36: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static unit_calls_37: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub type p4_3_1_typedef_parameter_redeclares_function_Unit = i32;
pub const p4_3_1_typedef_parameter_redeclares_function_Unit_single:
    p4_3_1_typedef_parameter_redeclares_function_Unit = 1;
pub fn describe_38(mut unit: p4_3_1_typedef_parameter_redeclares_function_Unit) -> i32 {
    (*unit_calls_37.with(Value::clone).borrow_mut()).prefix_inc();
    return ((unit as i32) + 20);
}
pub fn test_39() {
    assert!((({ describe_40(3,) }) == 13));
    assert!((int_calls_36.with(|rc| *rc.borrow()) == 1));
    assert!((unit_calls_37.with(|rc| *rc.borrow()) == 0));
    assert!((({ describe_38(p4_3_1_typedef_parameter_redeclares_function_Unit_single,) }) == 21));
    assert!((int_calls_36.with(|rc| *rc.borrow()) == 1));
    assert!((unit_calls_37.with(|rc| *rc.borrow()) == 1));
}
pub fn describe_40(mut amount: i32) -> i32 {
    (*int_calls_36.with(Value::clone).borrow_mut()).prefix_inc();
    return (amount + 10);
}
thread_local!(
    pub static value_calls_41: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static pointer_calls_42: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn describe_43(mut renamed: i32) -> i32 {
    (*value_calls_41.with(Value::clone).borrow_mut()).prefix_inc();
    return (renamed + 10);
}
pub fn describe_44(mut pointer: Ptr<i32>) -> i32 {
    (*pointer_calls_42.with(Value::clone).borrow_mut()).prefix_inc();
    return ((pointer.read()) + 20);
}
pub fn test_45() {
    let amount: Value<i32> = Rc::new(RefCell::new(3));
    assert!(({ (({ describe_43((*amount.borrow()),) }) == 13) }));
    assert!((value_calls_41.with(|rc| *rc.borrow()) == 1));
    assert!((pointer_calls_42.with(|rc| *rc.borrow()) == 0));
    assert!((({ describe_44((amount.as_pointer()),) }) == 23));
    assert!((value_calls_41.with(|rc| *rc.borrow()) == 1));
    assert!((pointer_calls_42.with(|rc| *rc.borrow()) == 1));
}
thread_local!(
    pub static value_calls_46: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static pointer_calls_47: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn describe_48(mut value: i32) -> i32 {
    (*value_calls_46.with(Value::clone).borrow_mut()).prefix_inc();
    return (value + 10);
}
pub fn describe_49(mut pointer: Ptr<i32>) -> i32 {
    (*pointer_calls_47.with(Value::clone).borrow_mut()).prefix_inc();
    return ((pointer.read()) + 20);
}
pub fn test_50() {
    let amount: Value<i32> = Rc::new(RefCell::new(3));
    assert!(({ (({ describe_48((*amount.borrow()),) }) == 13) }));
    assert!((value_calls_46.with(|rc| *rc.borrow()) == 1));
    assert!((pointer_calls_47.with(|rc| *rc.borrow()) == 0));
    assert!((({ describe_49((amount.as_pointer()),) }) == 23));
    assert!((value_calls_46.with(|rc| *rc.borrow()) == 1));
    assert!((pointer_calls_47.with(|rc| *rc.borrow()) == 1));
}
thread_local!(
    pub static type_parameter_calls_51: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static value_parameter_calls_52: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn describe_53(mut value: i32) -> i32 {
    (*type_parameter_calls_51.with(Value::clone).borrow_mut()).prefix_inc();
    return (value + 10);
}
pub fn describe_54() -> i32 {
    (*value_parameter_calls_52.with(Value::clone).borrow_mut()).prefix_inc();
    return (3 + 20);
}
pub fn test_55() {
    assert!((({ describe_53(3,) }) == 13));
    assert!((type_parameter_calls_51.with(|rc| *rc.borrow()) == 1));
    assert!((value_parameter_calls_52.with(|rc| *rc.borrow()) == 0));
    assert!((({ describe_54() }) == 23));
    assert!((type_parameter_calls_51.with(|rc| *rc.borrow()) == 1));
    assert!((value_parameter_calls_52.with(|rc| *rc.borrow()) == 1));
}
thread_local!(
    pub static version_calls_56: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn version_58() -> i32 {
    (*version_calls_56.with(Value::clone).borrow_mut()).prefix_inc();
    return 2;
}
thread_local!(
    pub static revision_57: Value<i32> = Rc::new(RefCell::new(40));
);
pub fn test_59() {
    assert!((({ version_58() }) == 2));
    assert!((version_calls_56.with(|rc| *rc.borrow()) == 1));
    assert!((({ version_58() }) == 2));
    assert!((version_calls_56.with(|rc| *rc.borrow()) == 2));
    assert!(((revision_57.with(|v| v.as_pointer())) == (revision_57.with(|v| v.as_pointer()))));
    revision_57.with(|rc| *rc.borrow_mut() = 41);
    assert!((revision_57.with(|rc| *rc.borrow()) == 41));
}
pub fn next_id_60() -> i32 {
    return (*last_61.with(Value::clone).borrow_mut()).prefix_inc();
}
pub fn next_id_62() -> i32 {
    return (*last_63.with(Value::clone).borrow_mut()).prefix_inc();
}
pub fn test_64() {
    assert!((({ next_id_60() }) == 1));
    assert!((({ next_id_60() }) == 2));
    assert!((({ next_id_62() }) == 1));
    assert!((({ next_id_60() }) == 3));
    assert!((({ next_id_62() }) == 2));
}
thread_local!(
    static last_61: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    static last_63: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_1() });
    ({ test_3() });
    ({ test_5() });
    ({ test_7() });
    ({ test_10() });
    ({ test_12() });
    ({ test_15() });
    ({ test_18() });
    ({ test_19() });
    ({ test_22() });
    ({ test_25() });
    ({ test_30() });
    ({ test_33() });
    ({ test_35() });
    ({ test_39() });
    ({ test_45() });
    ({ test_50() });
    ({ test_55() });
    ({ test_59() });
    ({ test_64() });
    return 0;
}
pub trait p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnumImpl {
    fn scale(&self, count: i32) -> i32;
}
impl p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnumImpl
    for Ptr<p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum>
{
    fn scale(&self, mut count: i32) -> i32 {
        return (count
            * (p2_11_unscoped_enumerator_in_both_scopes_HasUnscopedEnum_Size_large as i32));
    }
}
pub trait p2_13_anonymous_union_members_in_parent_scope_HasAnonymousUnionImpl {
    fn read_count(&self) -> i32;
}
impl p2_13_anonymous_union_members_in_parent_scope_HasAnonymousUnionImpl
    for Ptr<p2_13_anonymous_union_members_in_parent_scope_HasAnonymousUnion>
{
    fn read_count(&self) -> i32 {
        return (anon_9::member_count(field_ptr!((*self), anon_9)).read());
    }
}
pub trait p3_2_const_qualified_member_is_distinct_HasConstOverloadImpl {
    fn get_1(&self) -> i32;
    fn get_2(&self) -> i32;
}
impl p3_2_const_qualified_member_is_distinct_HasConstOverloadImpl
    for Ptr<p3_2_const_qualified_member_is_distinct_HasConstOverload>
{
    fn get_1(&self) -> i32 {
        field!((*self), plain_calls).with_mut(|__v| __v.prefix_inc());
        return 1;
    }
    fn get_2(&self) -> i32 {
        field!((*self), const_calls).with_mut(|__v| __v.prefix_inc());
        return 2;
    }
}
pub trait p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverloadImpl {
    fn pick_1(
        self_: Ptr<p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload>,
        amount: i32,
    ) -> i32;
    fn pick_2(&self, amount: i32) -> i32;
}
impl p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverloadImpl
    for Ptr<p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload>
{
    fn pick_1(
        self_: Ptr<p3_2_explicit_object_and_rvalue_members_are_distinct_HasExplicitObjectOverload>,
        mut amount: i32,
    ) -> i32 {
        (*explicit_calls_13.with(Value::clone).borrow_mut()).prefix_inc();
        return ({ self_.with(|__s| __s.base) } + { amount });
    }
    fn pick_2(&self, mut amount: i32) -> i32 {
        (*rvalue_calls_14.with(Value::clone).borrow_mut()).prefix_inc();
        return (((*self).with(|__s| __s.base) + amount) + 100);
    }
}
pub trait p3_2_ref_qualified_members_are_distinct_HasRefQualifiedOverloadsImpl {
    fn get_1(&self) -> i32;
    fn get_2(&self) -> i32;
}
impl p3_2_ref_qualified_members_are_distinct_HasRefQualifiedOverloadsImpl
    for Ptr<p3_2_ref_qualified_members_are_distinct_HasRefQualifiedOverloads>
{
    fn get_1(&self) -> i32 {
        (*lvalue_calls_16.with(Value::clone).borrow_mut()).prefix_inc();
        return 3;
    }
    fn get_2(&self) -> i32 {
        (*rvalue_calls_17.with(Value::clone).borrow_mut()).prefix_inc();
        return 4;
    }
}
pub trait p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverloadImpl {
    fn get_1_int(&self, value: i32) -> i32;
    fn get_2_int(&self, value: i32) -> i32;
}
impl p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverloadImpl
    for Ptr<p3_4_const_qualified_member_templates_are_distinct_HasConstTemplateOverload>
{
    fn get_1_int(&self, mut value: i32) -> i32 {
        field!((*self), plain_calls).with_mut(|__v| __v.prefix_inc());
        return (value + 1);
    }
    fn get_2_int(&self, mut value: i32) -> i32 {
        field!((*self), const_calls).with_mut(|__v| __v.prefix_inc());
        return (value + 2);
    }
}
pub trait p3_4_ref_qualified_member_templates_are_distinct_HasRefQualifiedTemplateOverloadsImpl {
    fn get_1_int(&self, value: i32) -> i32;
    fn get_2_int(&self, value: i32) -> i32;
}
impl p3_4_ref_qualified_member_templates_are_distinct_HasRefQualifiedTemplateOverloadsImpl
    for Ptr<p3_4_ref_qualified_member_templates_are_distinct_HasRefQualifiedTemplateOverloads>
{
    fn get_1_int(&self, mut value: i32) -> i32 {
        (*lvalue_calls_20.with(Value::clone).borrow_mut()).prefix_inc();
        return (value + 3);
    }
    fn get_2_int(&self, mut value: i32) -> i32 {
        (*rvalue_calls_21.with(Value::clone).borrow_mut()).prefix_inc();
        return (value + 4);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = level_0.with(|_| ());
    let _ = function_calls_2.with(|_| ());
    let _ = count_6.with(|_| ());
    let _ = anon_8.with(|_| ());
    let _ = explicit_calls_13.with(|_| ());
    let _ = rvalue_calls_14.with(|_| ());
    let _ = lvalue_calls_16.with(|_| ());
    let _ = rvalue_calls_17.with(|_| ());
    let _ = lvalue_calls_20.with(|_| ());
    let _ = rvalue_calls_21.with(|_| ());
    let _ = friend_calls_23.with(|_| ());
    let _ = library_calls_26.with(|_| ());
    let _ = local_calls_27.with(|_| ());
    let _ = function_calls_31.with(|_| ());
    let _ = Limit_34.with(|_| ());
    let _ = int_calls_36.with(|_| ());
    let _ = unit_calls_37.with(|_| ());
    let _ = value_calls_41.with(|_| ());
    let _ = pointer_calls_42.with(|_| ());
    let _ = value_calls_46.with(|_| ());
    let _ = pointer_calls_47.with(|_| ());
    let _ = type_parameter_calls_51.with(|_| ());
    let _ = value_parameter_calls_52.with(|_| ());
    let _ = version_calls_56.with(|_| ());
    let _ = revision_57.with(|_| ());
}
