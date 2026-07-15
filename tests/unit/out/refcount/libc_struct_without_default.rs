extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive()]
pub struct UserDefined {
    pub a: Value<Vec<i32>>,
    pub v: Value<Vec<i32>>,
}
impl Clone for UserDefined {
    fn clone(&self) -> Self {
        let mut this = Self {
            a: Rc::new(RefCell::new((*self.a.borrow()).clone())),
            v: Rc::new(RefCell::new((*self.v.borrow()).clone())),
        };
        this
    }
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
impl ByteRepr for UserDefined {}
#[derive()]
pub struct FieldIsLibcType {
    pub addr: Value<libcc2rs::Sockaddr>,
}
impl Clone for FieldIsLibcType {
    fn clone(&self) -> Self {
        let mut this = Self {
            addr: Rc::new(RefCell::new((*self.addr.borrow()).clone())),
        };
        this
    }
}
impl Default for FieldIsLibcType {
    fn default() -> Self {
        FieldIsLibcType {
            addr: Rc::new(RefCell::new(Default::default())),
        }
    }
}
impl ByteRepr for FieldIsLibcType {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p: Value<libcc2rs::Pollfd> = Rc::new(RefCell::new(Default::default()));
    (*(*p.borrow()).fd.borrow_mut()) = -1_i32;
    (*(*p.borrow()).events.borrow_mut()) = 0_i16;
    (*(*p.borrow()).revents.borrow_mut()) = 2_i16;
    assert!(((*(*p.borrow()).fd.borrow()) == -1_i32));
    assert!((((*(*p.borrow()).events.borrow()) as i32) == 0));
    assert!((((*(*p.borrow()).revents.borrow()) as i32) == 2));
    let ia: Value<libcc2rs::InAddr> = Rc::new(RefCell::new(Default::default()));
    (*(*ia.borrow()).s_addr.borrow_mut()) = 1_u32;
    assert!(((*(*ia.borrow()).s_addr.borrow()) == 1_u32));
    let t: Value<libcc2rs::Tm> = Rc::new(RefCell::new(Default::default()));
    (*(*t.borrow()).tm_year.borrow_mut()) = 124;
    (*(*t.borrow()).tm_mon.borrow_mut()) = 5;
    (*(*t.borrow()).tm_mday.borrow_mut()) = 15;
    assert!(((*(*t.borrow()).tm_year.borrow()) == 124));
    assert!(((*(*t.borrow()).tm_mon.borrow()) == 5));
    assert!(((*(*t.borrow()).tm_mday.borrow()) == 15));
    let st: Value<libcc2rs::Stat> = Rc::new(RefCell::new(Default::default()));
    (*(*st.borrow()).st_size.borrow_mut()) = 1024_i64;
    assert!(((*(*st.borrow()).st_size.borrow()) == 1024_i64));
    let ud: Value<UserDefined> = Rc::new(RefCell::new(<UserDefined>::default()));
    assert!(
        ((((*ud.borrow()).a.as_pointer() as Ptr<i32>)
            .offset(0_usize)
            .read())
            == 0)
    );
    assert!(((*(*ud.borrow()).v.borrow()).len() == 0_usize));
    let filt: Value<FieldIsLibcType> = Rc::new(RefCell::new(<FieldIsLibcType>::default()));
    assert!((((*(*(*filt.borrow()).addr.borrow()).sa_family.borrow()) as i32) == 0));
    return 0;
}
