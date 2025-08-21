//! A library for providing custom setup/teardown for Rust tests without needing a test harness.
//!
//! ```no_run
//! use test_context::{test_context, TestContext};
//!
//! struct MyContext {
//!     value: String
//! }
//!
//! impl TestContext for MyContext {
//!     fn setup() -> MyContext {
//!         MyContext {  value: "Hello, world!".to_string() }
//!     }
//!
//!     fn teardown(self) {
//!         // Perform any teardown you wish.
//!     }
//! }
//!
//! #[test_context(MyContext)]
//! #[test]
//! fn test_works(ctx: &mut MyContext) {
//!     assert_eq!(ctx.value, "Hello, world!");
//! }
//! ```
//!
//! Alternatively, you can use `async` functions in your test context by using the
//! `AsyncTestContext`.
//!
//! ```no_run
//! use test_context::{test_context, AsyncTestContext};
//!
//! struct MyAsyncContext {
//!     value: String
//! }
//!
//! impl AsyncTestContext for MyAsyncContext {
//!     async fn setup() -> MyAsyncContext {
//!         MyAsyncContext { value: "Hello, world!".to_string() }
//!     }
//!
//!     async fn teardown(self) {
//!         // Perform any teardown you wish.
//!     }
//! }
//!
//! #[test_context(MyAsyncContext)]
//! #[test]
//! fn test_works(ctx: &mut MyAsyncContext) {
//!     assert_eq!(ctx.value, "Hello, World!");
//! }
//! ```
//!
//! The `AsyncTestContext` works well with async test wrappers like
//! [`actix_rt::test`](https://docs.rs/actix-rt/1.1.1/actix_rt/attr.test.html) or
//! [`tokio::test`](https://docs.rs/tokio/1.0.2/tokio/attr.test.html).
//!
//! ```no_run
//!  use test_context::{test_context, AsyncTestContext};
//!
//!  struct MyAsyncContext {
//!      value: String
//!  }
//!
//!  impl AsyncTestContext for MyAsyncContext {
//!      async fn setup() -> MyAsyncContext {
//!          MyAsyncContext { value: "Hello, world!".to_string() }
//!      }
//!      async fn teardown(self) {
//!          // Perform any teardown you wish.
//!      }
//!  }
//!
//! #[test_context(MyAsyncContext)]
//! #[tokio::test]
//! async fn test_async_works(ctx: &mut MyAsyncContext) {
//!     assert_eq!(ctx.value, "Hello, World!");
//! }
//! ```
//!
//! # Attribute order
//!
//! Attribute order matters. Always place `#[test_context(...)]` before other test attributes
//! like `#[tokio::test]` or `#[test]`.
//!
//! Why: Rust expands attributes in source order. `#[test_context]` wraps your function and
//! re-attaches the remaining attributes to the wrapper; it must run first so the test attributes
//! apply to the wrapper that performs setup/teardown.
//!
//! Valid:
//! ```ignore
//! #[test_context(MyAsyncContext)]
//! #[tokio::test]
//! async fn my_test(ctx: &mut MyAsyncContext) {}
//! ```
//!
//! Invalid:
//! ```ignore
//! #[tokio::test]
//! #[test_context(MyAsyncContext)]
//! async fn my_test(ctx: &mut MyAsyncContext) {}
//! ```
//!
//! # Skipping the teardown execution
//!
//! If what you need is to take full __ownership__ of the context and don't care about the
//! teardown execution for a specific test, you can use the `skip_teardown` keyword on the macro
//! like this:
//!
//! ```no_run
//!  use test_context::{test_context, TestContext};
//!
//!  struct MyContext {}
//!
//!  impl TestContext for MyContext {
//!      fn setup() -> MyContext {
//!          MyContext {}
//!      }
//!  }
//!
//! #[test_context(MyContext, skip_teardown)]
//! #[test]
//! fn test_without_teardown(ctx: MyContext) {
//!   // Perform any operations that require full ownership of your context
//! }
//! ```

// Reimported to allow for use in the macro.
pub use futures;

pub use test_context_macros::test_context;

/// The trait to implement to get setup/teardown functionality for tests.
pub trait TestContext
where
    Self: Sized,
{
    /// Create the context. This is run once before each test that uses the context.
    fn setup() -> Self;

    /// Perform any additional cleanup of the context besides that already provided by
    /// normal "drop" semantics.
    fn teardown(self) {}
}

/// The trait to implement to get setup/teardown functionality for async tests.
pub trait AsyncTestContext
where
    Self: Sized,
{
    /// Create the context. This is run once before each test that uses the context.
    fn setup() -> impl std::future::Future<Output = Self> + Send;

    /// Perform any additional cleanup of the context besides that already provided by
    /// normal "drop" semantics.
    fn teardown(self) -> impl std::future::Future<Output = ()> + Send {
        async {}
    }
}

// Automatically impl TestContext for anything Send that impls AsyncTestContext.
//
// A future improvement may be to use feature flags to enable using a specific runtime
// to run the future synchronously. This is the easiest way to implement it, though, and
// introduces no new dependencies.
impl<T> TestContext for T
where
    T: AsyncTestContext + Send,
{
    fn setup() -> Self {
        futures::executor::block_on(<T as AsyncTestContext>::setup())
    }

    fn teardown(self) {
        futures::executor::block_on(<T as AsyncTestContext>::teardown(self))
    }
}
