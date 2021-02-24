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
        MyContext {  value: "Hello, world!".to_string() }
    }

    fn teardown(self) {
        // Perform any teardown you wish.
    }
}

#[test_context(MyContext)]
#[test]
fn test_works(ctx: &mut MyContext) {
    assert_eq!(ctx.value, "Hello, world!");
}
```

Alternatively, you can use `async` functions in your test context by using the
`AsyncTestContext`.

```rust
use test_context::{test_context, AsyncTestContext};

struct MyAsyncContext {
    value: String
}

#[async_trait::async_trait]
impl AsyncTestContext for MyAsyncContext {
    async fn setup() -> MyAsyncContext {
        MyAsyncContext { value: "Hello, world!".to_string() }
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

License: MIT
