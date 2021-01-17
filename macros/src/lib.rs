use proc_macro::TokenStream;
use quote::{format_ident, quote};

/// Macro to use on tests to add the setup/teardown functionality of your context.
///
/// Ordering of this attribute is important, and typically `test_context` should come
/// before other test attributes. For example, the following is valid:
///
/// ```ignore
/// #[test_context(MyContext)]
/// #[test]
/// #[ignore]
/// fn my_test() {
/// }
/// ```
///
/// The following is NOT valid...
///
/// ```ignore
/// #[test]
/// #[ignore]
/// #[test_context(MyContext)]
/// fn my_test() {
/// }
/// ```
#[proc_macro_attribute]
pub fn test_context(attr: TokenStream, item: TokenStream) -> TokenStream {
    let context_type = syn::parse_macro_input!(attr as syn::Ident);
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let ret = &input.sig.output;
    let name = &input.sig.ident;
    let inner_body = &input.block;
    let attrs = &input.attrs;
    let is_async = input.sig.asyncness.is_some();

    let wrapped_name = format_ident!("__test_context_wrapped_{}", name);

    let outer_body = if is_async {
        quote! {
            {
                use futures::FutureExt;
                let mut ctx = <#context_type as test_context::AsyncTestContext>::setup().await;
                let wrapped_ctx = &mut ctx;
                let result = async move {
                    std::panic::AssertUnwindSafe(
                        #wrapped_name(wrapped_ctx)
                    ).catch_unwind().await
                }.await;
                <#context_type as test_context::AsyncTestContext>::teardown(ctx).await;
                if let Err(err) = result {
                    std::panic::resume_unwind(err);
                }
            }
        }
    } else {
        quote! {
            {
                let mut ctx = <#context_type as test_context::TestContext>::setup();
                let mut wrapper = std::panic::AssertUnwindSafe(&mut ctx);
                let result = std::panic::catch_unwind(move || {
                    #wrapped_name(*wrapper);
                });
                <#context_type as test_context::TestContext>::teardown(ctx);
                if let Err(err) = result {
                    std::panic::resume_unwind(err);
                }
            }
        }
    };

    let async_tag = if is_async {
        quote! { async }
    } else {
        quote! {}
    };

    let result = quote! {
        #(#attrs)*
        #async_tag fn #name() #ret #outer_body

        #async_tag fn #wrapped_name(ctx: &mut #context_type) #ret #inner_body
    };
    result.into()
}
