# Global Variables

Global variables are mapped to thread-local storage, because a `Value<T>` cannot
be a true Rust global. A global must be `Sync`, since every thread can reach it,
and both `Rc` and `RefCell` are single-threaded types: the reference counter and
the borrow checks are not atomic. Thread-local storage sidesteps the requirement
by giving each thread its own copy, which matches the original semantics because
`cpp2rust` does not currently support multi-threaded code.
