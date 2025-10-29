use std::marker::PhantomData;

use rstest::rstest;
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

struct ContextGeneric<T> {
    n: u32,
    _marker: PhantomData<T>,
}

struct ContextGenericType1;
impl TestContext for ContextGeneric<ContextGenericType1> {
    fn setup() -> Self {
        Self {
            n: 1,
            _marker: PhantomData,
        }
    }
}

#[test_context(ContextGeneric<ContextGenericType1>)]
#[test]
fn test_generic_type(ctx: &mut ContextGeneric<ContextGenericType1>) {
    assert_eq!(ctx.n, 1);
}

struct ContextGenericType2;
impl TestContext for ContextGeneric<ContextGenericType2> {
    fn setup() -> Self {
        Self {
            n: 2,
            _marker: PhantomData,
        }
    }
}

#[test_context(ContextGeneric<ContextGenericType2>)]
#[test]
fn test_generic_type_other(ctx: &mut ContextGeneric<ContextGenericType2>) {
    assert_eq!(ctx.n, 2);
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
async fn test_async_skip_teardown(_ctx: &mut TeardownPanicContext) {}

#[test_context(TeardownPanicContext, skip_teardown)]
#[test]
fn test_sync_skip_teardown(_ctx: &mut TeardownPanicContext) {}

struct GenericContext<T> {
    contents: T,
}

impl TestContext for GenericContext<u32> {
    fn setup() -> Self {
        Self { contents: 1 }
    }
}

impl TestContext for GenericContext<String> {
    fn setup() -> Self {
        Self {
            contents: "hello world".to_string(),
        }
    }
}

impl AsyncTestContext for GenericContext<u64> {
    async fn setup() -> Self {
        Self { contents: 1 }
    }
}

#[test_context(GenericContext<u32>)]
#[test]
fn test_generic_with_u32(ctx: &mut GenericContext<u32>) {
    assert_eq!(ctx.contents, 1);
}

#[test_context(GenericContext<String>)]
#[test]
fn test_generic_with_string(ctx: &mut GenericContext<String>) {
    assert_eq!(ctx.contents, "hello world");
}

#[test_context(GenericContext<u64>)]
#[tokio::test]
async fn test_async_generic(test_ctx: &mut GenericContext<u64>) {
    assert_eq!(test_ctx.contents, 1);
}

struct MyAsyncContext {
    what_the_of_life: u32,
}

impl AsyncTestContext for MyAsyncContext {
    async fn setup() -> Self {
        println!("I guess...");
        MyAsyncContext {
            what_the_of_life: 42,
        }
    }

    async fn teardown(self) {
        println!("Answer is {}", self.what_the_of_life);
    }
}

#[test_context(MyAsyncContext)]
#[rstest]
#[case("Hello, World!")]
#[tokio::test]
async fn test_async_generic_with_sync(#[case] value: String, test_ctx: &mut MyAsyncContext) {
    println!("Something happens sync... {}", value);
    assert_eq!(test_ctx.what_the_of_life, 42);
}

struct MyContext {
    what_the_of_life: u32,
}

impl TestContext for MyContext {
    fn setup() -> Self {
        println!("I guess...");
        MyContext {
            what_the_of_life: 42,
        }
    }

    fn teardown(self) {
        println!("Answer is {}", self.what_the_of_life);
    }
}

#[test_context(MyContext)]
#[rstest]
#[case("Hello, World!")]
#[test]
fn test_async_generic_with_async(test_ctx: &mut MyContext, #[case] value: String) {
    println!("Something happens async... {}", value);
    assert_eq!(test_ctx.what_the_of_life, 42);
}
