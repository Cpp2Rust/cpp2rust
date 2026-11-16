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
pub struct SelfLinked {
    #[offset(0)]
    #[byte_size(8)]
    pub next: Ptr<SelfLinked>,
    #[offset(8)]
    pub value: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let node: Value<SelfLinked> = Rc::new(RefCell::new(SelfLinked {
        next: Ptr::<SelfLinked>::null(),
        value: 3,
    }));
    let mut outer: Ptr<SelfLinked> = (node.as_pointer());
    {
        let node: Value<SelfLinked> = <Value<SelfLinked>>::default();
        *node.borrow_mut() = SelfLinked {
            next: (node.as_pointer()),
            value: 7,
        };
        assert!(({ ({ { (*node.borrow()).next.clone() } } == { (node.as_pointer()) }) }));
        assert!(({ ({ { (*node.borrow()).next.clone() } } != { (outer).clone() }) }));
        assert!(({ ({ (*node.borrow()).next.clone() }.with(|__s| __s.value) == 7) }));
    }
    assert!(({ ({ (*node.borrow()).next.clone() }).is_null() }));
    assert!(({ ({ (*node.borrow()).value } == 3) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
