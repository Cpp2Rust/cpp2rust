extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0(mut x: i32) -> i32 {
    return x;
}
pub unsafe fn ptr_1(mut x: *mut i32) -> *mut i32 {
    return x;
}
pub unsafe fn bar_2(x: *mut i32) -> *mut i32 {
    return x;
}
#[derive(Copy, Clone, Default)]
pub struct X1 {
    pub v: i32,
}
#[derive(Copy, Clone, Default)]
pub struct X2 {
    pub v: *mut X1,
}
impl X2 {
    pub unsafe fn get(&mut self) -> *mut X1 {
        return self.v;
    }
}
#[derive(Copy, Clone, Default)]
pub struct X3 {
    pub v: *mut X2,
}
impl X3 {
    pub unsafe fn get(&mut self) -> *mut X2 {
        return self.v;
    }
}
#[derive(Copy, Clone, Default)]
pub struct X4 {
    pub v: X3,
}
impl X4 {
    pub unsafe fn get(&mut self) -> *mut X3 {
        return &mut self.v as *mut X3;
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x1: i32 = 0;
    let mut x2: i32 = (unsafe {
        let _x: i32 = x1;
        foo_0(_x)
    });
    let mut x3: i32 = (((unsafe {
        let _x: i32 = x2;
        foo_0(_x)
    }) + (unsafe {
        let _x: i32 = x1;
        foo_0(_x)
    })) + (1));
    x2 += 1;
    x2 += (unsafe {
        let _x: i32 = x1;
        foo_0(_x)
    });
    x3 += (((unsafe {
        let _x: i32 = x2;
        foo_0(_x)
    }) + (unsafe {
        let _x: i32 = x3;
        foo_0(_x)
    })) + (1));
    let mut p1: *mut i32 = (&mut x1 as *mut i32);
    let mut p2: *mut i32 = (unsafe {
        let _x: *mut i32 = p1;
        ptr_1(_x)
    });
    p1 = p2;
    p2 = (unsafe {
        let _x: *mut i32 = p1;
        ptr_1(_x)
    });
    let r1: *mut i32 = &mut x1 as *mut i32;
    let r2: *mut i32 = (unsafe {
        let _x: *mut i32 = &mut x1 as *mut i32;
        bar_2(_x)
    });
    let r3: *mut i32 = (unsafe {
        let _x: *mut i32 = r1;
        bar_2(_x)
    });
    (*r2) += x1;
    (*r3) += (*r1);
    let mut x4: i32 = (((unsafe {
        let _x: i32 = x3;
        foo_0(_x)
    }) + (*(unsafe {
        let _x: *mut i32 = (&mut x3 as *mut i32);
        ptr_1(_x)
    }))) + (*(unsafe {
        let _x: *mut i32 = &mut x2 as *mut i32;
        bar_2(_x)
    })));
    let mut a: X1 = X1 { v: 0 };
    let mut b: X2 = X2 {
        v: &mut a as *mut X1,
    };
    let mut c: X3 = X3 {
        v: (&mut b as *mut X2),
    };
    let mut d: X4 = X4 { v: c.clone() };
    (*(*d.v.v).v).v = 0;
    (*(unsafe { (*(unsafe { (*(unsafe { d.get() })).get() })).get() })).v = 0;
    d.v.v = (&mut b as *mut X2);
    let r4: *const i32 =
        &(*(unsafe { (*(unsafe { (*(unsafe { d.get() })).get() })).get() })).v as *const i32;
    let r5: *mut X1 = (unsafe { (*(unsafe { (*(unsafe { d.get() })).get() })).get() });
    let mut p: *mut X2 = (unsafe { (*(unsafe { d.get() })).get() });
    let r6: *mut X3 = (unsafe { d.get() });
    let r7: *mut X3 = &mut d.v as *mut X3;
    let r8: *mut i32 = &mut (*(unsafe { (*(unsafe { d.v.get() })).get() })).v as *mut i32;
    let mut x5: i32 = (*(unsafe { (*(unsafe { (*(unsafe { d.get() })).get() })).get() })).v;
    (*(unsafe {
        let _x: *mut i32 = &mut x1 as *mut i32;
        bar_2(_x)
    })) += 10;
    (*(unsafe {
        let _x: *mut i32 = &mut x1 as *mut i32;
        bar_2(_x)
    }))
    .postfix_inc();
    let mut bar_out: i32 = (*(unsafe {
        let _x: *mut i32 =
            &mut (*(unsafe { (*(unsafe { (*(unsafe { d.get() })).get() })).get() })).v as *mut i32;
        bar_2(_x)
    }));
    let mut bar_inc: i32 = (*(unsafe {
        let _x: *mut i32 = &mut x1 as *mut i32;
        bar_2(_x)
    }))
    .prefix_inc();
    bar_inc = (*(unsafe {
        let _x: *mut i32 = &mut x1 as *mut i32;
        bar_2(_x)
    }))
    .postfix_inc();
    bar_inc = (((*(unsafe {
        let _x: *mut i32 = &mut x1 as *mut i32;
        bar_2(_x)
    })) + (unsafe {
        let _x: i32 = x4;
        foo_0(_x)
    })) + (1));
    (*(unsafe {
        let _x: *mut i32 =
            &mut (*(unsafe { (*(unsafe { (*(unsafe { d.get() })).get() })).get() })).v as *mut i32;
        bar_2(_x)
    })) += 10;
    (*(unsafe {
        let _x: *mut i32 =
            &mut (*(unsafe { (*(unsafe { (*(unsafe { d.get() })).get() })).get() })).v as *mut i32;
        bar_2(_x)
    }))
    .postfix_inc();
    let mut bar_inc2: i32 = (*(unsafe {
        let _x: *mut i32 =
            &mut (*(unsafe { (*(unsafe { (*(unsafe { d.get() })).get() })).get() })).v as *mut i32;
        bar_2(_x)
    }))
    .prefix_inc();
    bar_inc2 = (*(unsafe {
        let _x: *mut i32 =
            &mut (*(unsafe { (*(unsafe { (*(unsafe { d.get() })).get() })).get() })).v as *mut i32;
        bar_2(_x)
    }))
    .postfix_inc();
    (*(unsafe {
        let _x: *mut i32 = (&mut x1 as *mut i32);
        ptr_1(_x)
    }))
    .prefix_inc();
    (*(unsafe {
        let _x: *mut i32 = (&mut x1 as *mut i32);
        ptr_1(_x)
    })) += 1;
    (*(unsafe {
        let _x: *mut i32 = (&mut (*(unsafe { (*(unsafe { (*(unsafe { d.get() })).get() })).get() }))
            .v as *mut i32);
        ptr_1(_x)
    }))
    .prefix_inc();
    (*(unsafe {
        let _x: *mut i32 = (&mut (*(unsafe { (*(unsafe { (*(unsafe { d.get() })).get() })).get() }))
            .v as *mut i32);
        ptr_1(_x)
    })) += 1;
    (*(&mut (*(unsafe {
        let _x: *mut i32 = (&mut (*(unsafe { (*(unsafe { (*(unsafe { d.get() })).get() })).get() }))
            .v as *mut i32);
        ptr_1(_x)
    })) as *mut i32)) += 1;
    let mut ptr1: i32 = (*(unsafe {
        let _x: *mut i32 = (&mut (*(unsafe { (*(unsafe { (*(unsafe { d.get() })).get() })).get() }))
            .v as *mut i32);
        ptr_1(_x)
    }))
    .postfix_inc();
    let ptr2: *mut i32 = &mut (*(unsafe {
        let _x: *mut i32 = (&mut (*(unsafe { (*(unsafe { (*(unsafe { d.get() })).get() })).get() }))
            .v as *mut i32);
        ptr_1(_x)
    })) as *mut i32;
    let mut ptr3: *mut i32 = (&mut (*(unsafe {
        let _x: *mut i32 = (&mut (*(unsafe { (*(unsafe { (*(unsafe { d.get() })).get() })).get() }))
            .v as *mut i32);
        ptr_1(_x)
    })) as *mut i32);
    let mut vptr: i32 = (*(unsafe {
        let _x: *mut i32 = (&mut (*(unsafe { (*(unsafe { (*(unsafe { d.get() })).get() })).get() }))
            .v as *mut i32);
        ptr_1(_x)
    }));
    let mut pref: *mut i32 = (unsafe {
        let _x: *mut i32 =
            &mut (*(unsafe { (*(unsafe { (*(unsafe { d.get() })).get() })).get() })).v as *mut i32;
        bar_2(_x)
    });
    (*(unsafe {
        let _x: *mut i32 =
            &mut (*(unsafe { (*(unsafe { (*(unsafe { d.get() })).get() })).get() })).v as *mut i32;
        bar_2(_x)
    }))
    .postfix_inc();
    return (((*(unsafe {
        let _x: *mut i32 = (&mut (*(unsafe { (*(unsafe { (*(unsafe { d.get() })).get() })).get() }))
            .v as *mut i32);
        ptr_1(_x)
    })) + (*(unsafe {
        let _x: *mut i32 =
            &mut (*(unsafe { (*(unsafe { (*(unsafe { d.get() })).get() })).get() })).v as *mut i32;
        bar_2(_x)
    }))) + (unsafe {
        let _x: i32 = (*(unsafe { (*(unsafe { (*(unsafe { d.get() })).get() })).get() })).v;
        foo_0(_x)
    }));
}
