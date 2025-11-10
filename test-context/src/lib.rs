#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

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
