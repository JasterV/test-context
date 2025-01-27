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

## Skipping the teardown execution

If what you need is to take full **ownership** of the context and don't care about the
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
fn test_without_teardown(ctx: MyContext) {
  // Perform any operations that require full ownership of your context
}
```

License: MIT
