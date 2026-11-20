extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() {
    let m: Value<Map<i32, (Value<i32>, Value<Vec<i8>>)>> = Rc::new(RefCell::new(Map::new()));
    let key: Value<i32> = Rc::new(RefCell::new(1));
    assert!(
        ({
            (((*({ (*m.borrow_mut()).subscript(key.as_pointer(), || vec![0]) })
                .upgrade()
                .deref())
            .len()
                - 1)
                == 0_usize)
        })
    );
    assert!(
        (((({ (*m.borrow_mut()).subscript(key.as_pointer(), || vec![0]) })
            .decay()
            .read()) as i32)
            == (('\0' as i8) as i32))
    );
    assert!(({ ((*m.borrow()).len() == 1_usize) }));
}
pub fn test_1() {
    let m: Value<Map<i32, (Value<i32>, Value<Vec<i8>>)>> = Rc::new(RefCell::new(Map::new()));
    assert!(
        (((*match (Rc::new(RefCell::new(1)).as_pointer(),) {
            (__tmp_0,) => {
                ({ (*m.borrow_mut()).subscript_move(__tmp_0, |__k| cpp_move!(__k), || vec![0]) })
            }
        }
        .upgrade()
        .deref())
        .len()
            - 1)
            == 0_usize)
    );
    assert!(
        (((match (Rc::new(RefCell::new(1)).as_pointer(),) {
            (__tmp_1,) => {
                ({ (*m.borrow_mut()).subscript_move(__tmp_1, |__k| cpp_move!(__k), || vec![0]) })
            }
        }
        .decay()
        .read()) as i32)
            == (('\0' as i8) as i32))
    );
    assert!(({ ((*m.borrow()).len() == 1_usize) }));
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    ({ test_0() });
    ({ test_1() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
