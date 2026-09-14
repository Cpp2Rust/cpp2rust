# Classes and Structs

A class becomes a struct with one field per data member, an `impl` block holding
its constructors and methods, and trait implementations after it. Given

```cpp
class Counter {
  int count_;

public:
  Counter(int start) : count_(start) {}
  ~Counter() { count_ = 0; }
  int get() const { return count_; }
  void set(int v) { count_ = v; }
};
```

the unsafe model produces

```rust
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Counter {
    count_: i32,
}
impl Counter {
    pub unsafe fn Counter(mut start: i32) -> Self {
        let mut this = Self { count_: start };
        this
    }
    pub unsafe fn get(&self) -> i32 {
        return self.count_;
    }
    pub unsafe fn set(&mut self, mut v: i32) {
        self.count_ = v;
    }
}
```

and the refcount model produces

```rust
#[derive(Default)]
pub struct Counter {
    count_: Value<i32>,
}
impl Counter {
    pub fn Counter(start: i32) -> Self {
        let start: Value<i32> = Rc::new(RefCell::new(start));
        let mut this = Self {
            count_: Rc::new(RefCell::new(*start.borrow())),
        };
        this
    }
    pub fn get(&self) -> i32 {
        return *self.count_.borrow();
    }
    pub fn set(&self, v: i32) {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        *self.count_.borrow_mut() = *v.borrow();
    }
}
impl Drop for Counter {
    fn drop(&mut self) {
        *self.count_.borrow_mut() = 0;
    }
}
impl Clone for Counter {
    fn clone(&self) -> Self {
        let mut this = Self {
            count_: Rc::new(RefCell::new(*self.count_.borrow())),
        };
        this
    }
}
impl ByteRepr for Counter { /* byte_size, to_bytes, from_bytes */ }
```

The unsafe model adds `#[repr(C)]` and derives what it can; the refcount model
writes most impls by hand. Which traits are emitted, and when they are derived
rather than written, is on the [Traits](./traits.md) page.

Fields keep their C++ access: `pub` for public members, nothing for private
ones. A class nested in another class is emitted as its own top-level struct,
named `Outer_Inner` (see [Naming](./naming.md)); Rust has no nested types, and
the outer struct refers to it by that name. A record that is only
forward-declared, or whose definition is never converted because it is only used
behind pointers, is emitted at the end of the file as an empty
`pub struct Name;` (see [The Translation Pipeline](../pipeline.md)).

A constructor becomes an associated function named after the class. It opens
with `let mut this = Self { ... }`, one field per member initializer, then runs
the C++ body and returns `this`. Only implicit or defaulted copy and move
constructors are supported: a user-defined one stops the translation, and the
implicit copy is what the `Clone` impl represents.

Methods take `&self` when `const` and `&mut self` otherwise in the unsafe model.
In the refcount model they always take `&self`, since mutation goes through the
fields' `RefCell`s. Inside a method, `this` is `self`.

A destructor with a body becomes `impl Drop` in the refcount model.

> [!WARNING]
>
> The unsafe model does not emit destructors at all; a user-defined destructor
> is silently dropped ([#310](https://github.com/Cpp2Rust/cpp2rust/issues/310)).

## Inheritance

An abstract class becomes a trait with one method per pure virtual function, and
a class deriving from it implements the trait with its overrides. Given

```cpp
class Animal {
public:
  virtual bool bark() const = 0;
};

class Dog : public Animal {
  bool bark() const override { return true; }
};
```

the unsafe model produces (attributes omitted)

```rust
pub unsafe trait Animal {
    unsafe fn bark(&self) -> bool;
}
pub struct Dog {}
unsafe impl Animal for Dog {
    unsafe fn bark(&self) -> bool {
        return true;
    }
}
```

and the refcount model produces (attributes and the `Clone` and `ByteRepr` impls
omitted)

```rust
pub trait Animal {
    fn bark(&self) -> bool;
}
pub struct Dog {}
impl Animal for Dog {
    fn bark(&self) -> bool {
        return true;
    }
}
```

Non-virtual methods of the derived class go into its own `impl Dog` block as
usual. Because the base is a trait, pointers to it are `*mut dyn Animal` in the
unsafe model and [`PtrDyn<dyn Animal>`](../../runtime/ptr-dyn.md) in the
refcount model, and a `Dog *` is upcast at the call site. Only the first base
class is considered, and only virtual methods go through the trait; bases with
data members or non-virtual methods, and multiple inheritance, are outside the
supported subset.

## Templates

Class templates are translated by full instantiation: each instantiation used by
the program becomes its own struct and `impl` block, named after the template
arguments (see [Naming](./naming.md)). `MyContainer<int>` and
`MyContainer<char>` become `MyContainer_int_` and `MyContainer_char_`, each with
a complete copy of the methods specialized for its element type. Nothing is
shared between instantiations, and Rust generics are not used.

## Flexible array members

A trailing array member of size 0, 1, or `[]` that C code over-indexes into
memory allocated past the struct is detected with clang's
`isFlexibleArrayMemberLike`. In the unsafe model an access to such a member is
not an array index, which Rust would bounds-check against the declared length,
but pointer arithmetic from the array's start: `s.bytes[i]` becomes
`*s.bytes.as_mut_ptr().add(i as usize)`, and `&s.bytes[i]` the same without the
leading `*`. The refcount model has no dedicated handling; the pattern works
when the array is a union member, because the union accessor returns a `Ptr`
over the whole allocation that can be offset freely.
