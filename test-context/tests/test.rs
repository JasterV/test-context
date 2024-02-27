use test_context::{test_context, AsyncTestContext, TestContext};

struct Context {
    n: u32,
}

impl TestContext for Context {
    fn setup() -> Self {
        Self { n: 1 }
    }

    fn teardown(self) {
        if self.n != 1 {
            panic!("Number changed");
        }
    }
}

#[test_context(Context)]
#[test]
fn test_sync_setup(ctx: &mut Context) {
    assert_eq!(ctx.n, 1);
}

#[test_context(Context)]
#[test]
#[should_panic(expected = "Number changed")]
fn test_sync_teardown(ctx: &mut Context) {
    ctx.n = 2;
}

#[test_context(Context)]
#[test]
#[should_panic(expected = "Number changed")]
fn test_panicking_teardown(ctx: &mut Context) {
    ctx.n = 2;
    panic!("First panic");
}

#[test_context(Context)]
fn return_value_func(ctx: &mut Context) -> u32 {
    ctx.n
}

#[test]
fn includes_return_value() {
    assert_eq!(return_value_func(), 1);
}

struct AsyncContext {
    n: u32,
}

impl AsyncTestContext for AsyncContext {
    async fn setup() -> Self {
        Self { n: 1 }
    }

    async fn teardown(self) {
        if self.n != 1 {
            panic!("Number changed");
        }
    }
}

#[test_context(AsyncContext)]
#[tokio::test]
async fn test_async_setup(ctx: &mut AsyncContext) {
    assert_eq!(ctx.n, 1);
}

#[test_context(AsyncContext)]
#[tokio::test]
#[should_panic(expected = "Number changed")]
async fn test_async_teardown(ctx: &mut AsyncContext) {
    ctx.n = 2;
}

#[test_context(AsyncContext)]
#[tokio::test]
#[should_panic(expected = "Number changed")]
async fn test_async_panicking_teardown(ctx: &mut AsyncContext) {
    ctx.n = 2;
    panic!("First panic");
}

#[test_context(AsyncContext)]
async fn async_return_value_func(ctx: &mut AsyncContext) -> u32 {
    ctx.n
}

#[tokio::test]
async fn async_includes_return_value() {
    assert_eq!(async_return_value_func().await, 1);
}

#[test_context(AsyncContext)]
#[test]
fn async_auto_impls_sync(ctx: &mut AsyncContext) {
    assert_eq!(ctx.n, 1);
}

#[test_context(Context)]
#[test]
fn use_different_name(test_data: &mut Context) {
    assert_eq!(test_data.n, 1);
}

#[test_context(AsyncContext)]
#[tokio::test]
async fn use_different_name_async(test_data: &mut AsyncContext) {
    assert_eq!(test_data.n, 1);
}

struct TeardownPanicContext {}

impl AsyncTestContext for TeardownPanicContext {
    async fn setup() -> Self {
        Self {}
    }

    async fn teardown(self) {
        panic!("boom!");
    }
}

#[test_context(TeardownPanicContext, skip_teardown)]
#[tokio::test]
async fn test_async_skip_teardown(mut _ctx: TeardownPanicContext) {}

#[test_context(TeardownPanicContext, skip_teardown)]
#[test]
fn test_sync_skip_teardown(mut _ctx: TeardownPanicContext) {}
