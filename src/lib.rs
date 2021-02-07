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
//! Works with other test wrappers like [`actix_rt::test`](https://docs.rs/actix-rt/1.1.1/actix_rt/attr.test.html) or
//! [`tokio::test`](https://docs.rs/tokio/1.0.2/tokio/attr.test.html) that turn your test function into an async
//! function.
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
//!         // Perform any teradown you wish.
//!     }
//! }
//!
//! #[test_context(MyAsyncContext)]
//! #[tokio::test]
//! async fn test_works(ctx: &mut MyAsyncContext) {
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
#[async_trait::async_trait]
pub trait AsyncTestContext
where
    Self: Sized,
{
    /// Create the context. This is run once before each test that uses the context.
    async fn setup() -> Self;

    /// Perform any additional cleanup of the context besides that already provided by
    /// normal "drop" semantics.
    async fn teardown(self) {}
}
