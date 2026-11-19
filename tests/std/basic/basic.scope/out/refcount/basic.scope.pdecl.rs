extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() {
    let mut level: i32 = 5;
    {
        let mut saved: i32 = level;
        let mut level: i32 = (saved + 1);
        let mut next: i32 = (level + 1);
        assert!((saved == 5));
        assert!((level == 6));
        assert!((next == 7));
    }
    assert!((level == 5));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct p1_own_address_in_initializer_SelfLinked {
    #[offset(0)]
    #[byte_size(8)]
    pub next: Ptr<p1_own_address_in_initializer_SelfLinked>,
    #[offset(8)]
    pub value: i32,
}
pub fn test_1() {
    let node: Value<p1_own_address_in_initializer_SelfLinked> =
        Rc::new(RefCell::new(p1_own_address_in_initializer_SelfLinked {
            next: Ptr::<p1_own_address_in_initializer_SelfLinked>::null(),
            value: 3,
        }));
    let mut outer: Ptr<p1_own_address_in_initializer_SelfLinked> = (node.as_pointer());
    {
        let node: Value<p1_own_address_in_initializer_SelfLinked> =
            <Value<p1_own_address_in_initializer_SelfLinked>>::default();
        *node.borrow_mut() = p1_own_address_in_initializer_SelfLinked {
            next: (node.as_pointer()),
            value: 7,
        };
        assert!(({ ({ { (*node.borrow()).next.clone() } } == { (node.as_pointer()) }) }));
        assert!(({ ({ { (*node.borrow()).next.clone() } } != { (outer).clone() }) }));
        assert!(({ ({ (*node.borrow()).next.clone() }.with(|__s| __s.value) == 7) }));
    }
    assert!(({ ({ (*node.borrow()).next.clone() }).is_null() }));
    assert!(({ ({ (*node.borrow()).value } == 3) }));
}
pub fn test_2() {
    let length: Value<i32> = Rc::new(RefCell::new(2));
    {
        let mut length: [i32; 2] = [7, 8];
        assert!(
            ((::std::mem::size_of::<[i32; 2]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize))
                == 2_usize)
        );
        assert!((length[(0) as usize] == 7));
        assert!((length[(1) as usize] == 8));
    }
    assert!(({ ((*length.borrow()) == 2) }));
}
pub fn test_3() {
    let mut limit: i32 = 12;
    {
        assert!(((anon_4_limit as i32) == 13));
        assert!(((anon_4_above as i32) == 14));
    }
    assert!((limit == 12));
}
pub type anon_4 = u32;
pub const anon_4_limit: anon_4 = 13;
pub const anon_4_above: anon_4 = 14;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct p9_func_declared_before_function_body_RecordsConstructorName {
    #[offset(0)]
    #[byte_size(8)]
    pub name: Ptr<i8>,
}
impl p9_func_declared_before_function_body_RecordsConstructorName {
    pub fn new() -> Self {
        let __this: Value<p9_func_declared_before_function_body_RecordsConstructorName> =
            Rc::new(RefCell::new(Self {
                name: Ptr::<i8>::from_string_literal(b"RecordsConstructorName"),
            }));
        let this: Ptr<p9_func_declared_before_function_body_RecordsConstructorName> =
            __this.as_pointer();
        assert!(
            (this.with(|__s| __s.name.clone())
                == Ptr::<i8>::from_string_literal(b"RecordsConstructorName"))
        );
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for p9_func_declared_before_function_body_RecordsConstructorName {
    fn default() -> Self {
        { p9_func_declared_before_function_body_RecordsConstructorName::new() }
    }
}
pub fn plain_function_5() -> Ptr<i8> {
    return Ptr::<i8>::from_string_literal(b"plain_function");
}
pub fn test_6() {
    assert!(
        ({
            let mut __it1 = ({ plain_function_5() }).to_c_string_iterator();
            let mut __it2 =
                Ptr::<i8>::from_string_literal(b"plain_function").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0)
    );
    assert!((({ plain_function_5() }) == ({ plain_function_5() })));
    let mut object: p9_func_declared_before_function_body_RecordsConstructorName =
        p9_func_declared_before_function_body_RecordsConstructorName::new();
    assert!(
        ({
            let mut __it1 = object.name.to_c_string_iterator();
            let mut __it2 =
                Ptr::<i8>::from_string_literal(b"RecordsConstructorName").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0)
    );
    let mut enclosing: Ptr<i8> = Ptr::<i8>::from_string_literal(b"test");
    let lambda: Value<FnPtr<fn() -> Ptr<i8>>> = Rc::new(RefCell::new(
        FnPtr::<fn() -> Ptr<i8>>::new(|| -> Ptr<i8> {
            {
                return Ptr::<i8>::from_string_literal(b"operator()");
            }
        }),
    ));
    assert!(({ ({ ({ (*lambda.borrow()).call() }).clone() } != { (enclosing).clone() }) }));
    assert!(
        ({
            let mut __it1 = enclosing.to_c_string_iterator();
            let mut __it2 = Ptr::<i8>::from_string_literal(b"test").to_c_string_iterator();
            loop {
                let __c1 = __it1.next();
                let __c2 = __it2.next();
                if __c1 != __c2 {
                    break (__c1.unwrap_or(0) as u8 as i32) - (__c2.unwrap_or(0) as u8 as i32);
                }
                if __c1.is_none() {
                    break 0;
                }
            }
        } == 0)
    );
}
pub fn test_7() {
    let values: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([2, 5, 9])));
    let mut steps: i32 = 0;
    let mut sum: i32 = 0;
    'loop_: for values in IterRange::new(values.decay(), values.decay().offset((3_i64) as isize)) {
        let mut values: i32 = (values.read());
        values *= 2;
        steps.prefix_inc();
        sum += values;
    }
    assert!((steps == 3));
    assert!((sum == 32));
    assert!(({ ((*values.borrow())[(0) as usize] == 2) }));
    assert!(({ ((*values.borrow())[(1) as usize] == 5) }));
    assert!(({ ((*values.borrow())[(2) as usize] == 9) }));
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_0() });
    ({ test_1() });
    ({ test_2() });
    ({ test_3() });
    ({ test_6() });
    ({ test_7() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
