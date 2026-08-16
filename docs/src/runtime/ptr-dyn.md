# Virtual Classes

A pointer to a virtual class cannot be a `Ptr<T>`. The class is translated as a
Rust trait, and trait objects are unsized, which Rust marks with `dyn`. The
runtime provides a dedicated `PtrDyn<dyn T>` type for these pointers, kept
separate so the generic `Ptr` pays no cost for dynamic dispatch.

A `PtrDyn` is created at the point where C++ converts a derived pointer to a
base pointer. `to_strong` upgrades the `Ptr<Derived>` into its `Value<Derived>`,
Rust's unsized coercion turns that into a `Value<dyn Base>`, and
`as_pointer_dyn` takes the weak reference back out:

```cpp
struct Base { virtual int f() const = 0; };
struct Derived : Base { int f() const override { return 1; } };

Derived d;
Base *b = &d;
int r = b->f();
```

```rust
let d: Value<Derived> = Rc::new(RefCell::new(<Derived>::default()));
let b: Value<PtrDyn<dyn Base>> = Rc::new(RefCell::new(
    ((d.as_pointer()).to_strong() as Value<dyn Base>).as_pointer_dyn(),
));
let r: Value<i32> = Rc::new(RefCell::new(({ (*(*b.borrow()).upgrade().deref()).f() })));
```

A virtual call goes through `upgrade`, which returns a `StrongPtrDyn<dyn T>`
holding the strong reference; its `deref` and `deref_mut` borrow the object and
the call dispatches through the trait's vtable.

> [!WARNING] `StrongPtrDyn` is set to be removed for the same reasons as
> [`StrongPtr`](./rc.md#strong-pointers): it holds a strong reference that can
> outlive the object's C++ lifetime, and even as a temporary it spans the whole
> virtual call, so a method that deletes its own object panics on `delete`.

`PtrDyn` is far smaller than `Ptr`: it is either null or a weak reference to a
single object. It has no arithmetic, no comparison, no array kinds, and no byte
view. Because `to_strong` is only defined for single-value pointers, a base
pointer into an array of polymorphic objects (a `Derived arr[N]` walked through
a `Base *`) cannot be formed.
