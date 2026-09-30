extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct registry {
    #[offset(0)]
    #[byte_size(8)]
    pub slot: AnyPtr,
    #[offset(8)]
    pub level: i64,
}
pub type field = u32;
pub const field_FIELD_SLOT: field = 0;
pub const field_FIELD_LEVEL: field = 1;
pub fn registry_update_0(r: Ptr<registry>, field: field, __args: &[VaArg]) -> i32 {
    let r: Value<Ptr<registry>> = Rc::new(RefCell::new(r));
    let field: Value<field> = Rc::new(RefCell::new(field));
    let result: Value<i32> = Rc::new(RefCell::new(0));
    let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
    (*ap.borrow_mut()) = VaList::new(__args);
    'switch: {
        let __match_cond = ((*field.borrow()) as u32);
        match __match_cond {
            __v if __v == ((field_FIELD_SLOT as i32) as u32) => {
                field!((*r.borrow()), slot).write((*ap.borrow_mut()).arg::<AnyPtr>());
                break 'switch;
            }
            __v if __v == ((field_FIELD_LEVEL as i32) as u32) => {
                field!((*r.borrow()), level).write((*ap.borrow_mut()).arg::<i64>());
                break 'switch;
            }
            _ => {
                (*result.borrow_mut()) = 1;
                break 'switch;
            }
        }
    };
    return (*result.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let r: Value<registry> = Rc::new(RefCell::new(registry {
        slot: AnyPtr::default(),
        level: 0_i64,
    }));
    let payload: Value<i32> = Rc::new(RefCell::new(7));
    assert!(
        (((({
            registry_update_0(
                (r.as_pointer()),
                field_FIELD_SLOT,
                &[(payload.as_pointer()).into()],
            )
        }) == 0) as i32)
            != 0)
    );
    assert!(
        (((({ registry_update_0((r.as_pointer()), field_FIELD_LEVEL, &[(5_i64).into(),]) }) == 0)
            as i32)
            != 0)
    );
    assert!(
        ((({
            let _lhs = { (*r.borrow()).slot.clone() };
            _lhs == (payload.as_pointer()).to_any()
        }) as i32)
            != 0)
    );
    assert!(
        (((({ (*r.borrow()).slot.clone() }
            .reinterpret_cast::<i32>()
            .read())
            == 7) as i32)
            != 0)
    );
    assert!(((({ (*r.borrow()).level } == 5_i64) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
