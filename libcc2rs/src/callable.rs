// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

macro_rules! callable {
    ($name:ident; $($a:ident: $A:ident),*) => {
        pub trait $name<$($A,)* R> {
            fn call(&self, $($a: $A),*) -> R;
        }

        impl<F, $($A,)* R> $name<$($A,)* R> for F
        where
            F: Fn($($A),*) -> R,
        {
            #[inline]
            fn call(&self, $($a: $A),*) -> R {
                self($($a),*)
            }
        }

        impl<F, $($A,)* R> $name<$($A,)* R> for Option<F>
        where
            F: Fn($($A),*) -> R,
        {
            #[inline]
            fn call(&self, $($a: $A),*) -> R {
                self.as_ref().unwrap()($($a),*)
            }
        }

        impl<$($A,)* R> $name<$($A,)* R> for Option<unsafe fn($($A),*) -> R> {
            #[inline]
            fn call(&self, $($a: $A),*) -> R {
                unsafe { self.unwrap()($($a),*) }
            }
        }

        impl<F, $($A,)* R> $name<$($A,)* R> for crate::fn_ptr::FnPtr<F>
        where
            F: Fn($($A),*) -> R + 'static,
        {
            #[inline]
            fn call(&self, $($a: $A),*) -> R {
                (**self)($($a),*)
            }
        }
    };
}

callable!(Callable0;);
callable!(Callable1; a1: A1);
callable!(Callable2; a1: A1, a2: A2);
callable!(Callable3; a1: A1, a2: A2, a3: A3);
