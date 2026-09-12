extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn len_0(s: Ptr<Box<[u8]>>) -> i32 {
    let n: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((((s.to_strong().as_pointer() as Ptr<u8>)
        .offset((*n.borrow()) as isize)
        .read()) as i32)
        != (('\0' as u8) as i32))
    {
        (*n.borrow_mut()).prefix_inc();
    }
    return (*n.borrow());
}
pub fn len5_1(s: Ptr<u8>) -> i32 {
    let s: Value<Ptr<u8>> = Rc::new(RefCell::new(s));
    let n: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((((*s.borrow()).offset((*n.borrow()) as isize).read()) as i32)
        != (('\0' as u8) as i32))
    {
        (*n.borrow_mut()).prefix_inc();
    }
    return (*n.borrow());
}
pub fn sum_2(a: Ptr<Box<[i32]>>) -> i32 {
    return ((((a.to_strong().as_pointer() as Ptr<i32>)
        .offset((0) as isize)
        .read())
        + ((a.to_strong().as_pointer() as Ptr<i32>)
            .offset((1) as isize)
            .read()))
        + ((a.to_strong().as_pointer() as Ptr<i32>)
            .offset((2) as isize)
            .read()));
}
pub fn fill_3(a: Ptr<Box<[i32]>>, v: i32) {
    let v: Value<i32> = Rc::new(RefCell::new(v));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 3) {
        (a.to_strong().as_pointer() as Ptr<i32>)
            .offset((*i.borrow()) as isize)
            .write((*v.borrow()));
        (*i.borrow_mut()).prefix_inc();
    }
}
pub fn sum_twice_4(a: Ptr<Box<[i32]>>) -> i32 {
    return (({ sum_2(((a).clone() as Ptr<Box<[i32]>>)) })
        + ({ sum_2(((a).clone() as Ptr<Box<[i32]>>)) }));
}
pub fn fill_and_sum_5(a: Ptr<Box<[i32]>>, v: i32, out: Ptr<i32>) {
    let v: Value<i32> = Rc::new(RefCell::new(v));
    ({
        let _a: Ptr<Box<[i32]>> = ((a).clone() as Ptr<Box<[i32]>>);
        let _v: i32 = (*v.borrow());
        fill_3(_a, _v)
    });
    let __rhs = ({ sum_twice_4(((a).clone() as Ptr<Box<[i32]>>)) });
    out.write(__rhs);
}
pub fn pick_6(s: Ptr<Box<[u8]>>) -> Ptr<Box<[u8]>> {
    return ((s).clone() as Ptr<Box<[u8]>>);
}
#[derive(Default)]
pub struct Point {
    pub x: Value<i32>,
    pub y: Value<i32>,
}
impl Clone for Point {
    fn clone(&self) -> Self {
        let __this: Value<Point> = Rc::new(RefCell::new(Self {
            x: Rc::new(RefCell::new((*self.x.borrow()))),
            y: Rc::new(RefCell::new((*self.y.borrow()))),
        }));
        let this: Ptr<Point> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Point {
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
pub fn sum_points_7(p: Ptr<Box<[Point]>>) -> i32 {
    return {
        let _lhs = {
            let _lhs = {
                let _lhs = (*(*(p.to_strong().as_pointer() as Ptr<Point>)
                    .offset((0) as isize)
                    .upgrade()
                    .deref())
                .x
                .borrow());
                _lhs + (*(*(p.to_strong().as_pointer() as Ptr<Point>)
                    .offset((0) as isize)
                    .upgrade()
                    .deref())
                .y
                .borrow())
            };
            _lhs + (*(*(p.to_strong().as_pointer() as Ptr<Point>)
                .offset((1) as isize)
                .upgrade()
                .deref())
            .x
            .borrow())
        };
        _lhs + (*(*(p.to_strong().as_pointer() as Ptr<Point>)
            .offset((1) as isize)
            .upgrade()
            .deref())
        .y
        .borrow())
    };
}
pub fn shift_points_8(p: Ptr<Box<[Point]>>, d: i32) {
    let d: Value<i32> = Rc::new(RefCell::new(d));
    (*(*(p.to_strong().as_pointer() as Ptr<Point>)
        .offset((0) as isize)
        .upgrade()
        .deref())
    .x
    .borrow_mut()) += (*d.borrow());
    (*(*(p.to_strong().as_pointer() as Ptr<Point>)
        .offset((1) as isize)
        .upgrade()
        .deref())
    .y
    .borrow_mut()) += (*d.borrow());
}
pub fn total_len_9(names: Ptr<Box<[Ptr<u8>]>>) -> i32 {
    return (({
        len5_1(
            ((names.to_strong().as_pointer() as Ptr<Ptr<u8>>)
                .offset((0) as isize)
                .read())
            .clone(),
        )
    }) + ({
        len5_1(
            ((names.to_strong().as_pointer() as Ptr<Ptr<u8>>)
                .offset((1) as isize)
                .read())
            .clone(),
        )
    }));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ len_0(Ptr::from_string_literal_array(b"beta"),) }) == 4));
    let buf: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::from(*b"abcd\0")));
    assert!((({ len_0((buf.as_pointer() as Ptr<Box<[u8]>>),) }) == 4));
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
    assert!((({ sum_2((arr.as_pointer() as Ptr<Box<[i32]>>),) }) == 6));
    ({ fill_3((arr.as_pointer() as Ptr<Box<[i32]>>), 7) });
    assert!((({ sum_2((arr.as_pointer() as Ptr<Box<[i32]>>),) }) == 21));
    assert!((({ sum_twice_4((arr.as_pointer() as Ptr<Box<[i32]>>),) }) == 42));
    let out: Value<i32> = Rc::new(RefCell::new(0));
    ({ fill_and_sum_5((arr.as_pointer() as Ptr<Box<[i32]>>), 2, out.as_pointer()) });
    assert!(((*out.borrow()) == 12));
    assert!(((*arr.borrow())[(0) as usize] == 2));
    let lit: Ptr<Box<[u8]>> = Ptr::from_string_literal_array(b"beta");
    assert!((({ len_0(((lit).clone() as Ptr<Box<[u8]>>),) }) == 4));
    assert!(
        ((((({ pick_6(Ptr::from_string_literal_array(b"beta"),) })
            .to_strong()
            .as_pointer() as Ptr::<u8>)
            .offset((0) as isize)
            .read()) as i32)
            == (('b' as u8) as i32))
    );
    assert!(
        (({ len_0((({ pick_6((buf.as_pointer() as Ptr<Box<[u8]>>),) }) as Ptr<Box<[u8]>>),) })
            == 4)
    );
    let pts: Value<Box<[Point]>> = Rc::new(RefCell::new(Box::new([
        Point {
            x: Rc::new(RefCell::new(1)),
            y: Rc::new(RefCell::new(2)),
        },
        Point {
            x: Rc::new(RefCell::new(3)),
            y: Rc::new(RefCell::new(4)),
        },
    ])));
    assert!((({ sum_points_7((pts.as_pointer() as Ptr<Box<[Point]>>),) }) == 10));
    ({ shift_points_8((pts.as_pointer() as Ptr<Box<[Point]>>), 10) });
    assert!(((*(*pts.borrow())[(0) as usize].x.borrow()) == 11));
    assert!(((*(*pts.borrow())[(1) as usize].y.borrow()) == 14));
    assert!((({ sum_points_7((pts.as_pointer() as Ptr<Box<[Point]>>),) }) == 30));
    let names: Value<Box<[Ptr<u8>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::from_string_literal(b"ab"),
        Ptr::from_string_literal(b"cde"),
    ])));
    assert!((({ total_len_9((names.as_pointer() as Ptr<Box<[Ptr::<u8>]>>),) }) == 5));
    return 0;
}
