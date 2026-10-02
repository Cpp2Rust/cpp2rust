extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(DeepClone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(32)]
pub struct UserDefined {
    #[offset(0)]
    #[byte_size(4)]
    pub a: Value<Vec<i32>>,
    #[offset(8)]
    #[byte_size(24)]
    pub v: Value<Vec<i32>>,
}
impl Default for UserDefined {
    fn default() -> Self {
        UserDefined {
            a: Rc::new(RefCell::new(
                std::array::from_fn::<_, 1, _>(|_| Default::default()).to_vec(),
            )),
            v: Rc::new(RefCell::new(Default::default())),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct FieldIsLibcType {
    #[offset(0)]
    #[byte_size(16)]
    pub addr: libcc2rs::Sockaddr,
}
impl Default for FieldIsLibcType {
    fn default() -> Self {
        FieldIsLibcType {
            addr: Default::default(),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p: Value<libcc2rs::Pollfd> = Rc::new(RefCell::new(Default::default()));
    (*p.borrow_mut()).fd = -1_i32;
    (*p.borrow_mut()).events = 0_i16;
    (*p.borrow_mut()).revents = 2_i16;
    assert!(({ (*p.borrow()).fd } == -1_i32));
    assert!((({ (*p.borrow()).events } as i32) == 0));
    assert!((({ (*p.borrow()).revents } as i32) == 2));
    let ia: Value<libcc2rs::InAddr> = Rc::new(RefCell::new(Default::default()));
    (*ia.borrow_mut()).s_addr = 1_u32;
    assert!(({ (*ia.borrow()).s_addr } == 1_u32));
    let t: Value<libcc2rs::Tm> = Rc::new(RefCell::new(Default::default()));
    (*t.borrow_mut()).tm_year = 124;
    (*t.borrow_mut()).tm_mon = 5;
    (*t.borrow_mut()).tm_mday = 15;
    assert!(({ (*t.borrow()).tm_year } == 124));
    assert!(({ (*t.borrow()).tm_mon } == 5));
    assert!(({ (*t.borrow()).tm_mday } == 15));
    let st: Value<libcc2rs::Stat> = Rc::new(RefCell::new(Default::default()));
    (*st.borrow_mut()).st_size = 1024_i64;
    assert!(({ (*st.borrow()).st_size } == 1024_i64));
    let ud: Value<UserDefined> = Rc::new(RefCell::new(<UserDefined>::default()));
    assert!(((elem!(({ (*ud.borrow()).a.as_pointer() } as Ptr<i32>), 0_usize).read()) == 0));
    assert!(((*{ (*ud.borrow()).v.clone() }.borrow()).len() == 0_usize));
    let filt: Value<FieldIsLibcType> = Rc::new(RefCell::new(<FieldIsLibcType>::default()));
    assert!((({ (*filt.borrow()).addr.sa_family } as i32) == 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
