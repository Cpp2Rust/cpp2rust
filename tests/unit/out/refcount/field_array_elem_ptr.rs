extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Point {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(48)]
pub struct Shape {
    #[offset(0)]
    pub id: i32,
    #[offset(4)]
    #[byte_size(16)]
    pub coords: Value<Box<[i32]>>,
    #[offset(20)]
    #[byte_size(24)]
    pub points: Value<Box<[Point]>>,
    #[offset(44)]
    pub tail: i32,
}
impl Clone for Shape {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            coords: Rc::new(RefCell::new((*self.coords.borrow()).clone())),
            points: Rc::new(RefCell::new((*self.points.borrow()).clone())),
            tail: self.tail.clone(),
        }
    }
}
impl Default for Shape {
    fn default() -> Self {
        Shape {
            id: 0_i32,
            coords: Rc::new(RefCell::new((0..4).map(|_| 0_i32).collect::<Box<[i32]>>())),
            points: Rc::new(RefCell::new(
                (0..3).map(|_| <Point>::default()).collect::<Box<[Point]>>(),
            )),
            tail: 0_i32,
        }
    }
}
pub fn sum_0(p: Ptr<i32>, n: i32) -> i32 {
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(p));
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let s: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*n.borrow())) {
        (*s.borrow_mut()) += { ((*p.borrow()).offset((*i.borrow()) as isize).read()) };
        (*i.borrow_mut()).prefix_inc();
    }
    return (*s.borrow());
}
pub fn set_y_1(p: Ptr<Point>, y: i32) {
    let p: Value<Ptr<Point>> = Rc::new(RefCell::new(p));
    let y: Value<i32> = Rc::new(RefCell::new(y));
    field!((*p.borrow()), y).write((*y.borrow()));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<Shape> = Rc::new(RefCell::new(Shape {
        id: 1,
        coords: Rc::new(RefCell::new(Box::new([10, 20, 30, 40]))),
        points: Rc::new(RefCell::new(Box::new([
            Point { x: 1, y: 2 },
            Point { x: 3, y: 4 },
            Point { x: 5, y: 6 },
        ]))),
        tail: 99,
    }));
    let c: Value<Ptr<i32>> = Rc::new(RefCell::new(
        ((array_field_ptr!(s.as_pointer(), coords) as Ptr<i32>).offset((1) as isize)),
    ));
    assert!((((*c.borrow()).read()) == 20));
    (*c.borrow()).write(21);
    assert!(
        (((array_field_ptr!(s.as_pointer(), coords) as Ptr::<i32>)
            .offset((1) as isize)
            .read())
            == 21)
    );
    (*c.borrow_mut()) += 2;
    assert!((((*c.borrow()).read()) == 40));
    assert!(
        (((*c.borrow()).clone()
            - ((array_field_ptr!(s.as_pointer(), coords) as Ptr::<i32>).offset((0) as isize)))
            as i64
            == 3_i64)
    );
    assert!((((*c.borrow()).offset((-1_i32) as isize).read()) == 30));
    assert!((({ sum_0((array_field_ptr!(s.as_pointer(), coords) as Ptr::<i32>), 4,) }) == 101));
    assert!(
        (({
            sum_0(
                ((array_field_ptr!(s.as_pointer(), coords) as Ptr<i32>).offset((2) as isize)),
                2,
            )
        }) == 70)
    );
    let p: Value<Ptr<Point>> = Rc::new(RefCell::new(
        ((array_field_ptr!(s.as_pointer(), points) as Ptr<Point>).offset((1) as isize)),
    ));
    assert!(((*p.borrow()).with(|__s| __s.x) == 3));
    ({ set_y_1((*p.borrow()).offset((1) as isize), 60) });
    assert!(
        ({
            (*(array_field_ptr!(s.as_pointer(), points) as Ptr<Point>)
                .offset((2) as isize)
                .upgrade()
                .deref())
            .y
        } == 60)
    );
    let py: Value<Ptr<i32>> = Rc::new(RefCell::new(
        (field_ptr!(
            (array_field_ptr!(s.as_pointer(), points) as Ptr<Point>).offset((0) as isize),
            y
        )),
    ));
    (*py.borrow()).write(7);
    assert!(
        ({
            (*(array_field_ptr!(s.as_pointer(), points) as Ptr<Point>)
                .offset((0) as isize)
                .upgrade()
                .deref())
            .y
        } == 7)
    );
    let px: Value<Ptr<i32>> = Rc::new(RefCell::new(
        (field_ptr!(((*p.borrow()).offset((1) as isize)), x)),
    ));
    {
        let _ptr = (*px.borrow()).clone();
        _ptr.write(_ptr.read() + 50)
    };
    assert!(
        ({
            (*(array_field_ptr!(s.as_pointer(), points) as Ptr<Point>)
                .offset((2) as isize)
                .upgrade()
                .deref())
            .x
        } == 55)
    );
    let sp: Value<Ptr<Shape>> = Rc::new(RefCell::new((s.as_pointer())));
    let d: Value<Ptr<i32>> = Rc::new(RefCell::new(
        (array_field_ptr!((*sp.borrow()), coords) as Ptr<i32>).offset((3) as isize),
    ));
    (*d.borrow()).write(41);
    assert!(
        (((array_field_ptr!(s.as_pointer(), coords) as Ptr::<i32>)
            .offset((3) as isize)
            .read())
            == 41)
    );
    field!(
        (array_field_ptr!((*sp.borrow()), points) as Ptr<Point>).offset((1) as isize),
        y
    )
    .write({
        ({
            ((array_field_ptr!((*sp.borrow()), coords) as Ptr<i32>)
                .offset((0) as isize)
                .read())
        } + {
            {
                (*(array_field_ptr!((*sp.borrow()), points) as Ptr<Point>)
                    .offset((0) as isize)
                    .upgrade()
                    .deref())
                .x
            }
        })
    });
    assert!(
        ({
            (*(array_field_ptr!(s.as_pointer(), points) as Ptr<Point>)
                .offset((1) as isize)
                .upgrade()
                .deref())
            .y
        } == 11)
    );
    let q: Value<Ptr<Point>> = Rc::new(RefCell::new(
        (array_field_ptr!((*sp.borrow()), points) as Ptr<Point>),
    ));
    field!((*q.borrow()).offset((2) as isize), x).write(8);
    assert!(
        ({
            (*(array_field_ptr!((*sp.borrow()), points) as Ptr<Point>)
                .offset((2) as isize)
                .upgrade()
                .deref())
            .x
        } == 8)
    );
    let t: Value<Shape> = Rc::new(RefCell::new((*s.borrow()).clone()));
    (array_field_ptr!(t.as_pointer(), coords) as Ptr<i32>)
        .offset((0) as isize)
        .write(0);
    field!(
        (array_field_ptr!(t.as_pointer(), points) as Ptr<Point>).offset((0) as isize),
        x
    )
    .write(0);
    assert!(
        (((array_field_ptr!(s.as_pointer(), coords) as Ptr::<i32>)
            .offset((0) as isize)
            .read())
            == 10)
            && ({
                (*(array_field_ptr!(s.as_pointer(), points) as Ptr<Point>)
                    .offset((0) as isize)
                    .upgrade()
                    .deref())
                .x
            } == 1)
    );
    assert!(
        ((((array_field_ptr!(t.as_pointer(), coords) as Ptr::<i32>)
            .offset((1) as isize)
            .read())
            == 21)
            && ({
                (*(array_field_ptr!(t.as_pointer(), points) as Ptr<Point>)
                    .offset((2) as isize)
                    .upgrade()
                    .deref())
                .y
            } == 60))
            && ({ (*t.borrow()).tail } == 99)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
