# Unions

> TODO: explain how the refcount model translates a C union as a `__bytes`
> buffer with one accessor per member, each returning a `reinterpret_cast` view
> over that buffer.
