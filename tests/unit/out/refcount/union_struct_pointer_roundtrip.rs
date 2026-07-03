extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct pair {
    pub x: Value<i32>,
    pub y: Value<i32>,
}
impl ByteRepr for pair {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.x.borrow()).to_bytes(&mut buf[0..4]);
        (*self.y.borrow()).to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
            y: Rc::new(RefCell::new(<i32>::from_bytes(&buf[4..8]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let arr: Value<Box<[pair]>> = Rc::new(RefCell::new(
        (0..3).map(|_| <pair>::default()).collect::<Box<[pair]>>(),
    ));
    (*(*arr.borrow())[(0) as usize].x.borrow_mut()) = 10;
    (*(*arr.borrow())[(1) as usize].x.borrow_mut()) = 20;
    (*(*arr.borrow())[(2) as usize].x.borrow_mut()) = 30;
    pub struct anon_0 {
        __bytes: Value<Box<[u8]>>,
    }
    impl anon_0 {
        pub fn p(&self) -> Ptr<Ptr<pair>> {
            (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
        }
        pub fn bits(&self) -> Ptr<u64> {
            (self.__bytes.as_pointer() as Ptr<u8>).reinterpret_cast()
        }
    }
    impl Clone for anon_0 {
        fn clone(&self) -> Self {
            anon_0 {
                __bytes: Rc::new(RefCell::new(self.__bytes.borrow().clone())),
            }
        }
    }
    impl Default for anon_0 {
        fn default() -> Self {
            anon_0 {
                __bytes: Rc::new(RefCell::new(Box::from([0u8; 8]))),
            }
        }
    }
    impl ByteRepr for anon_0 {
        fn byte_size() -> usize {
            8
        }
        fn to_bytes(&self, buf: &mut [u8]) {
            buf.copy_from_slice(&self.__bytes.borrow());
        }
        fn from_bytes(buf: &[u8]) -> Self {
            anon_0 {
                __bytes: Rc::new(RefCell::new(Box::from(buf))),
            }
        }
    };
    let u: Value<anon_0> = <Value<anon_0>>::default();
    (*u.borrow_mut())
        .p()
        .write(((arr.as_pointer() as Ptr<pair>).offset(1)));
    let q: Value<Ptr<pair>> = Rc::new(RefCell::new(((*u.borrow()).p().read()).clone()));
    assert!(((((*(*(*q.borrow()).upgrade().deref()).x.borrow()) == 20) as i32) != 0));
    assert!(
        ((({
            let _lhs = (*q.borrow()).clone();
            _lhs == ((arr.as_pointer() as Ptr<pair>).offset(1))
        }) as i32)
            != 0)
    );
    let rhs_0 = ((((*u.borrow()).bits().read()) as u64).wrapping_add((8usize as u64))) as u64;
    (*u.borrow_mut()).bits().write(rhs_0);
    assert!(((((*(*((*u.borrow()).p().read()).upgrade().deref()).x.borrow()) == 30) as i32) != 0));
    assert!(
        ((({
            let _lhs = ((*u.borrow()).p().read()).clone();
            _lhs == ((arr.as_pointer() as Ptr<pair>).offset(2))
        }) as i32)
            != 0)
    );
    let rhs_0 = ((((*u.borrow()).bits().read()) as u64)
        .wrapping_sub(((2_usize).wrapping_mul((8usize as usize)) as u64))) as u64;
    (*u.borrow_mut()).bits().write(rhs_0);
    assert!(((((*(*((*u.borrow()).p().read()).upgrade().deref()).x.borrow()) == 10) as i32) != 0));
    assert!(
        ((({
            let _lhs = ((*u.borrow()).p().read()).clone();
            _lhs == ((arr.as_pointer() as Ptr<pair>).offset(0))
        }) as i32)
            != 0)
    );
    return 0;
}
