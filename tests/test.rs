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
fn test_sync_teardown(ctx: &mut Wrapper) {
    ctx.n = 2;
}

#[test_context(Context)]
#[test]
#[should_panic(expected = "Number changed")]
fn test_panicking_teardown(ctx: &mut Context) {
    ctx.n = 2;
    panic!("First panic");
}

struct AsyncContext {
    n: u32,
}

#[async_trait::async_trait]
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
