// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

pub struct IgnoreRule;

pub trait TupleElement<const N: usize> {
    type Type;

    fn element(&self) -> &Self::Type;

    fn element_mut(&mut self) -> &mut Self::Type;
}

pub trait TupleGet {
    fn get<const N: usize>(&self) -> &<Self as TupleElement<N>>::Type
    where
        Self: TupleElement<N>,
    {
        TupleElement::<N>::element(self)
    }

    fn get_mut<const N: usize>(&mut self) -> &mut <Self as TupleElement<N>>::Type
    where
        Self: TupleElement<N>,
    {
        TupleElement::<N>::element_mut(self)
    }
}

macro_rules! impl_tuple_get {
    (@element ($($t:ident),+) $n:tt $element:ident) => {
        impl<$($t),+> TupleElement<$n> for ($($t,)+) {
            type Type = $element;

            fn element(&self) -> &$element {
                &self.$n
            }

            fn element_mut(&mut self) -> &mut $element {
                &mut self.$n
            }
        }
    };
    ($tuple:tt; $($n:tt $element:ident),+) => {
        impl<$($element),+> TupleGet for ($($element,)+) {}
        $(impl_tuple_get!(@element $tuple $n $element);)+
    };
}

impl_tuple_get!((T0, T1); 0 T0, 1 T1);
