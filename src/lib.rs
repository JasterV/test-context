//! A library for providing custom setup/teardown for Rust tests without needing a test harness.
//!
//! ```
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
//! ```
//! use test_context::{test_context, AsyncTestContext};
//!
//! struct MyAsyncContext {
//!     value: String
//! }
//!
//! #[async_trait::async_trait]
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
//! fn test_works(ctx: &mut MyAsyncContext) {
//!     assert_eq!(ctx.value, "Hello, World!");
//! }
//! ```
//!
//! The `AsyncTestContext` works well with async test wrappers like
//! [`actix_rt::test`](https://docs.rs/actix-rt/1.1.1/actix_rt/attr.test.html) or
//! [`tokio::test`](https://docs.rs/tokio/1.0.2/tokio/attr.test.html).
//!
//! ```
//! # use test_context::{test_context, AsyncTestContext};
//! # struct MyAsyncContext {
//! #     value: String
//! # }
//! # #[async_trait::async_trait]
//! # impl AsyncTestContext for MyAsyncContext {
//! #     async fn setup() -> MyAsyncContext {
//! #         MyAsyncContext { value: "Hello, world!".to_string() }
//! #     }
//! #     async fn teardown(self) {
//! #         // Perform any teardown you wish.
//! #     }
//! # }
//! #[test_context(MyAsyncContext)]
//! #[tokio::test]
//! async fn test_async_works(ctx: &mut MyAsyncContext) {
//!     assert_eq!(ctx.value, "Hello, World!");
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
