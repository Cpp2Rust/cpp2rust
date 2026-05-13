extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct node {
    pub data: Value<i32>,
    pub next: Value<Ptr<node>>,
}
impl ByteRepr for node {}
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum opt {
    #[default]
    OPT_STRING_OUT = 0,
    OPT_FILE = 1,
    OPT_NODE = 2,
    OPT_NODE_OUT = 3,
}
impl From<i32> for opt {
    fn from(n: i32) -> opt {
        match n {
            0 => opt::OPT_STRING_OUT,
            1 => opt::OPT_FILE,
            2 => opt::OPT_NODE,
            3 => opt::OPT_NODE_OUT,
            _ => panic!("invalid opt value: {}", n),
        }
    }
}
pub fn dispatch_0(option: i32, __args: &[VaArg]) -> i32 {
    let option: Value<i32> = Rc::new(RefCell::new(option));
    let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
    (*ap.borrow_mut()) = VaList::new(__args);
    let result: Value<i32> = Rc::new(RefCell::new(0));
    'switch: {
        let __match_cond = (*option.borrow());
        match __match_cond {
            v if v == (opt::OPT_STRING_OUT as i32) => {
                let out: Value<Ptr<Ptr<u8>>> = Rc::new(RefCell::new(
                    ((*ap.borrow_mut()).arg::<Ptr<Ptr<u8>>>()).clone(),
                ));
                (*out.borrow()).write(Ptr::from_string_literal("hello"));
                (*result.borrow_mut()) = 1;
                break 'switch;
            }
            v if v == (opt::OPT_FILE as i32) => {
                let f: Value<Ptr<::std::fs::File>> = Rc::new(RefCell::new(
                    ((*ap.borrow_mut()).arg::<Ptr<::std::fs::File>>()).clone(),
                ));
                (*result.borrow_mut()) = ((!((*f.borrow()).is_null())) as i32).clone();
                break 'switch;
            }
            v if v == (opt::OPT_NODE as i32) => {
                let n: Value<Ptr<node>> = Rc::new(RefCell::new(
                    ((*ap.borrow_mut()).arg::<Ptr<node>>()).clone(),
                ));
                (*result.borrow_mut()) = (*(*(*n.borrow()).upgrade().deref()).data.borrow());
                break 'switch;
            }
            v if v == (opt::OPT_NODE_OUT as i32) => {
                let out: Value<Ptr<Ptr<node>>> = Rc::new(RefCell::new(
                    ((*ap.borrow_mut()).arg::<Ptr<Ptr<node>>>()).clone(),
                ));
                (*out.borrow()).write(Ptr::<node>::null());
                (*result.borrow_mut()) = 2;
                break 'switch;
            }
            _ => {}
        }
    };
    return (*result.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::<u8>::null()));
    assert!(
        (((({
            let _option: i32 = (opt::OPT_STRING_OUT as i32);
            dispatch_0(_option, &[(s.as_pointer()).into()])
        }) == 1) as i32)
            != 0)
    );
    assert!((((!((*s.borrow()).is_null())) as i32) != 0));
    assert!(
        (((({
            let _option: i32 = (opt::OPT_FILE as i32);
            dispatch_0(_option, &[libcc2rs::cout().into()])
        }) == 1) as i32)
            != 0)
    );
    assert!(
        (((({
            let _option: i32 = (opt::OPT_FILE as i32);
            dispatch_0(
                _option,
                &[(AnyPtr::default())
                    .cast::<::std::fs::File>()
                    .expect("ub:wrong type")
                    .into()],
            )
        }) == 0) as i32)
            != 0)
    );
    let head: Value<node> = Rc::new(RefCell::new(node {
        data: Rc::new(RefCell::new(42)),
        next: Rc::new(RefCell::new(Ptr::<node>::null())),
    }));
    assert!(
        (((({
            let _option: i32 = (opt::OPT_NODE as i32);
            dispatch_0(_option, &[(head.as_pointer()).into()])
        }) == 42) as i32)
            != 0)
    );
    let outp: Value<Ptr<node>> = Rc::new(RefCell::new((head.as_pointer())));
    assert!(
        (((({
            let _option: i32 = (opt::OPT_NODE_OUT as i32);
            dispatch_0(_option, &[(outp.as_pointer()).into()])
        }) == 2) as i32)
            != 0)
    );
    assert!(((((*outp.borrow()).is_null()) as i32) != 0));
    return 0;
}
