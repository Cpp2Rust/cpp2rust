extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct Pair {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub arr: Box<[i32]>,
}
impl Default for Pair {
    fn default() -> Self {
        {
            Pair {
                x: 0_i32,
                y: 0_i32,
                arr: (0..2).map(|_| 0_i32).collect::<Box<[i32]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Nested {
    #[offset(0)]
    #[byte_size(16)]
    pub p: Pair,
}
thread_local!(
    pub static calls_0: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn make_pair_1() -> Pair {
    (*calls_0.with(Value::clone).borrow_mut()).prefix_inc();
    return Pair {
        x: 1,
        y: 2,
        arr: Box::new([3, 4]),
    };
}
pub fn make_nested_2() -> Nested {
    return Nested {
        p: ({ make_pair_1() }),
    };
}
pub fn note_3(mut v: i32) {
    calls_0.with(|rc| *rc.borrow_mut() += v);
}
pub fn extended_temporaries_4(mut c: bool, mut v: i32) {
    let __tmp_0: Value<Pair>;
    let field: Ptr<i32> = field_ptr!(
        {
            __tmp_0 = Rc::new(RefCell::new(({ make_pair_1() })));
            __tmp_0.as_pointer()
        },
        y
    );
    let __tmp_1: Value<Nested>;
    let nested: Ptr<i32> = field_ptr!(
        field_ptr!(
            {
                __tmp_1 = Rc::new(RefCell::new(({ make_nested_2() })));
                __tmp_1.as_pointer()
            },
            p
        ),
        x
    );
    let __tmp_2: Value<Pair>;
    let element: Ptr<i32> = array_field_ptr!(
        {
            __tmp_2 = Rc::new(RefCell::new(({ make_pair_1() })));
            __tmp_2.as_pointer()
        },
        arr
    )
    .offset((1) as isize);
    let __tmp_3: Value<i32> = Rc::new(RefCell::new(if c { { ({ make_pair_1() }).x } } else { v }));
    let chosen: Ptr<i32> = __tmp_3.as_pointer();
    let __tmp_4: Value<Pair>;
    let comma: Ptr<i32> = {
        ({ note_3(10) });
        field_ptr!(
            {
                __tmp_4 = Rc::new(RefCell::new(({ make_pair_1() })));
                __tmp_4.as_pointer()
            },
            x
        )
    };
    let __tmp_5: Value<Pair>;
    let rvalue: Ptr<i32> = field_ptr!(
        {
            __tmp_5 = Rc::new(RefCell::new(({ make_pair_1() })));
            __tmp_5.as_pointer()
        },
        y
    );
    assert!(((field.read()) == 2));
    assert!(((nested.read()) == 1));
    assert!(((element.read()) == 4));
    assert!(((chosen.read()) == 1));
    assert!(((comma.read()) == 1));
    assert!(((rvalue.read()) == 2));
    assert!(((once_5.with(Ptr::clone).read()) == 3));
    assert!((calls_0.with(|rc| *rc.borrow()) == 18));
}
thread_local!(
    static __tmp_6: Value<Pair> = Rc::new(RefCell::new(({ make_pair_1() })));
    static once_5: Ptr<i32> =
        array_field_ptr!(__tmp_6.with(Value::as_pointer), arr).offset((0) as isize);
);
thread_local!(
    static __tmp_7: Value<Pair> = Rc::new(RefCell::new(({ make_pair_1() })));
    pub static global_field_6: Ptr<i32> = field_ptr!(__tmp_7.with(Value::as_pointer), x);
);
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(1));
    let r: Ptr<i32> = x.as_pointer();
    r.write(5);
    assert!(({ ((*x.borrow()) == 5) }));
    ({ extended_temporaries_4(true, 7) });
    assert!(((global_field_6.with(Ptr::clone).read()) == 1));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = calls_0.with(|_| ());
    let _ = global_field_6.with(|_| ());
}
