extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct S {
    pub as_: Value<i32>,
    pub async_: Value<i32>,
    pub await_: Value<i32>,
    pub crate_: Value<i32>,
    pub dyn_: Value<i32>,
    pub fn_: Value<i32>,
    pub impl_: Value<i32>,
    pub in_: Value<i32>,
    pub let_: Value<i32>,
    pub loop_: Value<i32>,
    pub match_: Value<i32>,
    pub mod_: Value<i32>,
    pub move_: Value<i32>,
    pub mut_: Value<i32>,
    pub pub_: Value<i32>,
    pub ref_: Value<i32>,
    pub self_: Value<i32>,
    pub Self_: Value<i32>,
    pub super_: Value<i32>,
    pub trait_: Value<i32>,
    pub type_: Value<i32>,
    pub unsafe_: Value<i32>,
    pub use_: Value<i32>,
    pub where_: Value<i32>,
    pub abstract_: Value<i32>,
    pub become_: Value<i32>,
    pub box_: Value<i32>,
    pub final_: Value<i32>,
    pub gen_: Value<i32>,
    pub macro_: Value<i32>,
    pub override_: Value<i32>,
    pub priv_: Value<i32>,
    pub unsized_: Value<i32>,
    pub yield_: Value<i32>,
    pub macro_rules_: Value<i32>,
    pub raw_: Value<i32>,
    pub safe_: Value<i32>,
    pub vec_: Value<i32>,
}
impl Clone for S {
    fn clone(&self) -> Self {
        let mut this = Self {
            as_: Rc::new(RefCell::new((*self.as_.borrow()))),
            async_: Rc::new(RefCell::new((*self.async_.borrow()))),
            await_: Rc::new(RefCell::new((*self.await_.borrow()))),
            crate_: Rc::new(RefCell::new((*self.crate_.borrow()))),
            dyn_: Rc::new(RefCell::new((*self.dyn_.borrow()))),
            fn_: Rc::new(RefCell::new((*self.fn_.borrow()))),
            impl_: Rc::new(RefCell::new((*self.impl_.borrow()))),
            in_: Rc::new(RefCell::new((*self.in_.borrow()))),
            let_: Rc::new(RefCell::new((*self.let_.borrow()))),
            loop_: Rc::new(RefCell::new((*self.loop_.borrow()))),
            match_: Rc::new(RefCell::new((*self.match_.borrow()))),
            mod_: Rc::new(RefCell::new((*self.mod_.borrow()))),
            move_: Rc::new(RefCell::new((*self.move_.borrow()))),
            mut_: Rc::new(RefCell::new((*self.mut_.borrow()))),
            pub_: Rc::new(RefCell::new((*self.pub_.borrow()))),
            ref_: Rc::new(RefCell::new((*self.ref_.borrow()))),
            self_: Rc::new(RefCell::new((*self.self_.borrow()))),
            Self_: Rc::new(RefCell::new((*self.Self_.borrow()))),
            super_: Rc::new(RefCell::new((*self.super_.borrow()))),
            trait_: Rc::new(RefCell::new((*self.trait_.borrow()))),
            type_: Rc::new(RefCell::new((*self.type_.borrow()))),
            unsafe_: Rc::new(RefCell::new((*self.unsafe_.borrow()))),
            use_: Rc::new(RefCell::new((*self.use_.borrow()))),
            where_: Rc::new(RefCell::new((*self.where_.borrow()))),
            abstract_: Rc::new(RefCell::new((*self.abstract_.borrow()))),
            become_: Rc::new(RefCell::new((*self.become_.borrow()))),
            box_: Rc::new(RefCell::new((*self.box_.borrow()))),
            final_: Rc::new(RefCell::new((*self.final_.borrow()))),
            gen_: Rc::new(RefCell::new((*self.gen_.borrow()))),
            macro_: Rc::new(RefCell::new((*self.macro_.borrow()))),
            override_: Rc::new(RefCell::new((*self.override_.borrow()))),
            priv_: Rc::new(RefCell::new((*self.priv_.borrow()))),
            unsized_: Rc::new(RefCell::new((*self.unsized_.borrow()))),
            yield_: Rc::new(RefCell::new((*self.yield_.borrow()))),
            macro_rules_: Rc::new(RefCell::new((*self.macro_rules_.borrow()))),
            raw_: Rc::new(RefCell::new((*self.raw_.borrow()))),
            safe_: Rc::new(RefCell::new((*self.safe_.borrow()))),
            vec_: Rc::new(RefCell::new((*self.vec_.borrow()))),
        };
        this
    }
}
impl ByteRepr for S {}
pub fn foo_0(
    as_: i32,
    async_: i32,
    await_: i32,
    crate_: i32,
    dyn_: i32,
    fn_: i32,
    impl_: i32,
    in_: i32,
    let_: i32,
    loop_: i32,
    match_: i32,
    mod_: i32,
    move_: i32,
    mut_: i32,
    pub_: i32,
    ref_: i32,
    self_: i32,
    Self_: i32,
    super_: i32,
    trait_: i32,
    type_: i32,
    unsafe_: i32,
    use_: i32,
    where_: i32,
    abstract_: i32,
    become_: i32,
    box_: i32,
    final_: i32,
    gen_: i32,
    macro_: i32,
    override_: i32,
    priv_: i32,
    unsized_: i32,
    yield_: i32,
    macro_rules_: i32,
    raw_: i32,
    safe_: i32,
    vec_: i32,
    dummy: i32,
) -> i32 {
    let as_: Value<i32> = Rc::new(RefCell::new(as_));
    let async_: Value<i32> = Rc::new(RefCell::new(async_));
    let await_: Value<i32> = Rc::new(RefCell::new(await_));
    let crate_: Value<i32> = Rc::new(RefCell::new(crate_));
    let dyn_: Value<i32> = Rc::new(RefCell::new(dyn_));
    let fn_: Value<i32> = Rc::new(RefCell::new(fn_));
    let impl_: Value<i32> = Rc::new(RefCell::new(impl_));
    let in_: Value<i32> = Rc::new(RefCell::new(in_));
    let let_: Value<i32> = Rc::new(RefCell::new(let_));
    let loop_: Value<i32> = Rc::new(RefCell::new(loop_));
    let match_: Value<i32> = Rc::new(RefCell::new(match_));
    let mod_: Value<i32> = Rc::new(RefCell::new(mod_));
    let move_: Value<i32> = Rc::new(RefCell::new(move_));
    let mut_: Value<i32> = Rc::new(RefCell::new(mut_));
    let pub_: Value<i32> = Rc::new(RefCell::new(pub_));
    let ref_: Value<i32> = Rc::new(RefCell::new(ref_));
    let self_: Value<i32> = Rc::new(RefCell::new(self_));
    let Self_: Value<i32> = Rc::new(RefCell::new(Self_));
    let super_: Value<i32> = Rc::new(RefCell::new(super_));
    let trait_: Value<i32> = Rc::new(RefCell::new(trait_));
    let type_: Value<i32> = Rc::new(RefCell::new(type_));
    let unsafe_: Value<i32> = Rc::new(RefCell::new(unsafe_));
    let use_: Value<i32> = Rc::new(RefCell::new(use_));
    let where_: Value<i32> = Rc::new(RefCell::new(where_));
    let abstract_: Value<i32> = Rc::new(RefCell::new(abstract_));
    let become_: Value<i32> = Rc::new(RefCell::new(become_));
    let box_: Value<i32> = Rc::new(RefCell::new(box_));
    let final_: Value<i32> = Rc::new(RefCell::new(final_));
    let gen_: Value<i32> = Rc::new(RefCell::new(gen_));
    let macro_: Value<i32> = Rc::new(RefCell::new(macro_));
    let override_: Value<i32> = Rc::new(RefCell::new(override_));
    let priv_: Value<i32> = Rc::new(RefCell::new(priv_));
    let unsized_: Value<i32> = Rc::new(RefCell::new(unsized_));
    let yield_: Value<i32> = Rc::new(RefCell::new(yield_));
    let macro_rules_: Value<i32> = Rc::new(RefCell::new(macro_rules_));
    let raw_: Value<i32> = Rc::new(RefCell::new(raw_));
    let safe_: Value<i32> = Rc::new(RefCell::new(safe_));
    let vec_: Value<i32> = Rc::new(RefCell::new(vec_));
    let dummy: Value<i32> = Rc::new(RefCell::new(dummy));
    return 0;
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S {
        as_: Rc::new(RefCell::new(0)),
        async_: Rc::new(RefCell::new(0)),
        await_: Rc::new(RefCell::new(0)),
        crate_: Rc::new(RefCell::new(0)),
        dyn_: Rc::new(RefCell::new(0)),
        fn_: Rc::new(RefCell::new(0)),
        impl_: Rc::new(RefCell::new(0)),
        in_: Rc::new(RefCell::new(0)),
        let_: Rc::new(RefCell::new(0)),
        loop_: Rc::new(RefCell::new(0)),
        match_: Rc::new(RefCell::new(0)),
        mod_: Rc::new(RefCell::new(0)),
        move_: Rc::new(RefCell::new(0)),
        mut_: Rc::new(RefCell::new(0)),
        pub_: Rc::new(RefCell::new(0)),
        ref_: Rc::new(RefCell::new(0)),
        self_: Rc::new(RefCell::new(0)),
        Self_: Rc::new(RefCell::new(0)),
        super_: Rc::new(RefCell::new(0)),
        trait_: Rc::new(RefCell::new(0)),
        type_: Rc::new(RefCell::new(0)),
        unsafe_: Rc::new(RefCell::new(0)),
        use_: Rc::new(RefCell::new(0)),
        where_: Rc::new(RefCell::new(0)),
        abstract_: Rc::new(RefCell::new(0)),
        become_: Rc::new(RefCell::new(0)),
        box_: Rc::new(RefCell::new(0)),
        final_: Rc::new(RefCell::new(0)),
        gen_: Rc::new(RefCell::new(0)),
        macro_: Rc::new(RefCell::new(0)),
        override_: Rc::new(RefCell::new(0)),
        priv_: Rc::new(RefCell::new(0)),
        unsized_: Rc::new(RefCell::new(0)),
        yield_: Rc::new(RefCell::new(0)),
        macro_rules_: Rc::new(RefCell::new(0)),
        raw_: Rc::new(RefCell::new(0)),
        safe_: Rc::new(RefCell::new(0)),
        vec_: Rc::new(RefCell::new(0)),
    }));
    let as_: Value<i32> = Rc::new(RefCell::new(0));
    let async_: Value<i32> = Rc::new(RefCell::new(0));
    let await_: Value<i32> = Rc::new(RefCell::new(0));
    let crate_: Value<i32> = Rc::new(RefCell::new(0));
    let dyn_: Value<i32> = Rc::new(RefCell::new(0));
    let fn_: Value<i32> = Rc::new(RefCell::new(0));
    let impl_: Value<i32> = Rc::new(RefCell::new(0));
    let in_: Value<i32> = Rc::new(RefCell::new(0));
    let let_: Value<i32> = Rc::new(RefCell::new(0));
    let loop_: Value<i32> = Rc::new(RefCell::new(0));
    let match_: Value<i32> = Rc::new(RefCell::new(0));
    let mod_: Value<i32> = Rc::new(RefCell::new(0));
    let move_: Value<i32> = Rc::new(RefCell::new(0));
    let mut_: Value<i32> = Rc::new(RefCell::new(0));
    let pub_: Value<i32> = Rc::new(RefCell::new(0));
    let ref_: Value<i32> = Rc::new(RefCell::new(0));
    let self_: Value<i32> = Rc::new(RefCell::new(0));
    let Self_: Value<i32> = Rc::new(RefCell::new(0));
    let super_: Value<i32> = Rc::new(RefCell::new(0));
    let trait_: Value<i32> = Rc::new(RefCell::new(0));
    let type_: Value<i32> = Rc::new(RefCell::new(0));
    let unsafe_: Value<i32> = Rc::new(RefCell::new(0));
    let use_: Value<i32> = Rc::new(RefCell::new(0));
    let where_: Value<i32> = Rc::new(RefCell::new(0));
    let abstract_: Value<i32> = Rc::new(RefCell::new(0));
    let become_: Value<i32> = Rc::new(RefCell::new(0));
    let box_: Value<i32> = Rc::new(RefCell::new(0));
    let final_: Value<i32> = Rc::new(RefCell::new(0));
    let gen_: Value<i32> = Rc::new(RefCell::new(0));
    let macro_: Value<i32> = Rc::new(RefCell::new(0));
    let override_: Value<i32> = Rc::new(RefCell::new(0));
    let priv_: Value<i32> = Rc::new(RefCell::new(0));
    let unsized_: Value<i32> = Rc::new(RefCell::new(0));
    let yield_: Value<i32> = Rc::new(RefCell::new(0));
    let macro_rules_: Value<i32> = Rc::new(RefCell::new(0));
    let raw_: Value<i32> = Rc::new(RefCell::new(0));
    let safe_: Value<i32> = Rc::new(RefCell::new(0));
    let vec_: Value<i32> = Rc::new(RefCell::new(0));
    return ({
        let _as: i32 = 0;
        let _async: i32 = 0;
        let _await: i32 = 0;
        let _crate: i32 = 0;
        let _dyn: i32 = 0;
        let _fn: i32 = 0;
        let _impl: i32 = 0;
        let _in: i32 = 0;
        let _let: i32 = 0;
        let _loop: i32 = 0;
        let _match: i32 = 0;
        let _mod: i32 = 0;
        let _move: i32 = 0;
        let _mut: i32 = 0;
        let _pub: i32 = 0;
        let _ref: i32 = 0;
        let _self: i32 = 0;
        let _Self: i32 = 0;
        let _super: i32 = 0;
        let _trait: i32 = 0;
        let _type: i32 = 0;
        let _unsafe: i32 = 0;
        let _use: i32 = 0;
        let _where: i32 = 0;
        let _abstract: i32 = 0;
        let _become: i32 = 0;
        let _box: i32 = 0;
        let _final: i32 = 0;
        let _gen: i32 = 0;
        let _macro: i32 = 0;
        let _override: i32 = 0;
        let _priv: i32 = 0;
        let _unsized: i32 = 0;
        let _yield: i32 = 0;
        let _macro_rules: i32 = 0;
        let _raw: i32 = 0;
        let _safe: i32 = 0;
        let _vec: i32 = 0;
        let _dummy: i32 = 0;
        foo_0(
            _as,
            _async,
            _await,
            _crate,
            _dyn,
            _fn,
            _impl,
            _in,
            _let,
            _loop,
            _match,
            _mod,
            _move,
            _mut,
            _pub,
            _ref,
            _self,
            _Self,
            _super,
            _trait,
            _type,
            _unsafe,
            _use,
            _where,
            _abstract,
            _become,
            _box,
            _final,
            _gen,
            _macro,
            _override,
            _priv,
            _unsized,
            _yield,
            _macro_rules,
            _raw,
            _safe,
            _vec,
            _dummy,
        )
    });
}
