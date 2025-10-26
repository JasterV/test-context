[![crates.io](https://img.shields.io/crates/v/test-context?label=latest)](https://crates.io/crates/test-context)
[![Documentation](https://docs.rs/test-context/badge.svg)](https://docs.rs/test-context)
![License](https://img.shields.io/crates/l/test-context.svg)
[![Github](https://github.com/markhildreth/test-context/workflows/Rust/badge.svg?branch=main)](https://github.com/markhildreth/test-context/actions)

# test-context

A library for providing custom setup/teardown for Rust tests without needing a test harness.

```rust
use test_context::{test_context, TestContext};

struct MyContext {
    value: String
}

impl TestContext for MyContext {
    fn setup() -> MyContext {
        MyContext {  value: "Hello, World!".to_string() }
    }

    fn teardown(self) {
        // Perform any teardown you wish.
    }
}

#[test_context(MyContext)]
#[test]
fn test_works(ctx: &mut MyContext) {
    assert_eq!(ctx.value, "Hello, World!");
}

struct MyGenericContext<T> {
    value: T
}

impl TestContext for MyGenericContext<u32> {
    fn setup() -> MyGenericContext<u32> {
        MyGenericContext { value: 1 }
    }
}

#[test_context(MyGenericContext<u32>)]
#[test]
fn test_generic_type(ctx: &mut MyGenericContext<u32>) {
    assert_eq!(ctx.value, 1);
}
```

Alternatively, you can use `async` functions in your test context by using the
`AsyncTestContext`.

```rust
use test_context::{test_context, AsyncTestContext};

struct MyAsyncContext {
    value: String
}

impl AsyncTestContext for MyAsyncContext {
    async fn setup() -> MyAsyncContext {
        MyAsyncContext { value: "Hello, World!".to_string() }
    }

    async fn teardown(self) {
        // Perform any teardown you wish.
    }
}

#[test_context(MyAsyncContext)]
fn test_works(ctx: &mut MyAsyncContext) {
    assert_eq!(ctx.value, "Hello, World!");
}
```

The `AsyncTestContext` works well with async test wrappers like
[`actix_rt::test`](https://docs.rs/actix-rt/1.1.1/actix_rt/attr.test.html) or
[`tokio::test`](https://docs.rs/tokio/1.0.2/tokio/attr.test.html).

```rust
#[test_context(MyAsyncContext)]
#[tokio::test]
async fn test_works(ctx: &mut MyAsyncContext) {
    assert_eq!(ctx.value, "Hello, World!");
}
```

### Attribute order

Place `#[test_context(...)]` before other test attributes like `#[tokio::test]` or `#[test]`.

Why: Attributes expand in source order. `#[test_context]` generates a wrapper and reattaches
the remaining attributes to it. It must run first so the test attribute applies to the wrapper
that runs setup/teardown.

Valid:

```rust
#[test_context(MyAsyncContext)]
#[tokio::test]
async fn my_test(ctx: &mut MyAsyncContext) {}
```

Invalid:

```rust
#[tokio::test]
#[test_context(MyAsyncContext)]
async fn my_test(ctx: &mut MyAsyncContext) {}
```

## Using AsyncTestContext in sync tests that require Tokio

By default, when you use an `AsyncTestContext` in a synchronous test (no `#[tokio::test]`),
this crate runs `setup`/`teardown` using the `futures` executor. If your context calls
Tokio-only APIs (e.g., `tokio::time::sleep`, timers, or Tokio sockets) during setup/teardown,
enable the optional `tokio-runtime` feature so those steps run inside a Tokio runtime:

```toml
[dependencies]
test-context = { version = "0.5", features = ["tokio-runtime"] }
```

With this feature, the crate tries to reuse an existing runtime; if none is present, it creates
an ephemeral current-thread Tokio runtime around `setup` and `teardown` for sync tests. Async
tests annotated with `#[tokio::test]` continue to work as usual without the feature.

## Skipping the teardown execution

Also, if you don't care about the
teardown execution for a specific test, you can use the `skip_teardown` keyword on the macro
like this:

```rust
 use test_context::{test_context, TestContext};

 struct MyContext {}

 impl TestContext for MyContext {
     fn setup() -> MyContext {
         MyContext {}
     }
 }

#[test_context(MyContext, skip_teardown)]
#[test]
fn test_without_teardown(ctx: &mut MyContext) {
  // Perform any operations that require full ownership of your context
}
```

## ⚠️ Ensure that the context type specified in the macro matches the test function argument type exactly

The error occurs when a context type with an absolute path is mixed with an it's alias.

For example:

```
mod database {
    use test_context::TestContext;

    pub struct Connection;

    impl TestContext for :Connection {
    	fn setup() -> Self {Connection}
    	fn teardown(self) {...}
	}
}
```

✅The following code will work:
```
use database::Connection as DbConn;

#[test_context(DbConn)]
#[test]
fn test1(ctx: &mut DbConn) {
	//some test logic
}

// or

use database::Connection

#[test_context(database::Connection)]
#[test]
fn test1(ctx: &mut database::Connection) {
	//some test logic
}
```

❌The following code will not work:
```
use database::Connection as DbConn;

#[test_context(database::Connection)]
#[test]
fn test1(ctx: &mut DbConn) {
	//some test logic
}

// or

use database::Connection as DbConn;

#[test_context(DbConn)]
#[test]
fn test1(ctx: &mut database::Connection) {
	//some test logic
}
```

Type mismatches will cause context parsing to fail during either static analysis or compilation.


License: MIT
