mod args;

use args::TestContextArgs;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

/// Macro to use on tests to add the setup/teardown functionality of your context.
///
/// Ordering of this attribute is important, and typically `test_context` should come
/// before other test attributes. For example, the following is valid:
///
/// ```ignore
/// #[test_context(MyContext)]
/// #[test]
/// fn my_test() {
/// }
/// ```
///
/// The following is NOT valid...
///
/// ```ignore
/// #[test]
/// #[test_context(MyContext)]
/// fn my_test() {
/// }
/// ```
#[proc_macro_attribute]
pub fn test_context(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(attr as TestContextArgs);
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let (input, context_arg_name) = remove_context_arg(input, args.context_type.clone());
    let input = refactor_input_body(input, &args, context_arg_name);

    quote! { #input }.into()
}

fn refactor_input_body(
    mut input: syn::ItemFn,
    args: &TestContextArgs,
    context_arg_name: Option<Ident>,
) -> syn::ItemFn {
    let context_type = &args.context_type;
    let context_arg_name = context_arg_name.unwrap_or_else(|| format_ident!("test_ctx"));
    let result_name = format_ident!("wrapped_result");
    let body = &input.block;
    let is_async = input.sig.asyncness.is_some();

    let body = match (is_async, args.skip_teardown) {
        (true, true) => {
            quote! {
            use test_context::futures::FutureExt;
                let #context_arg_name = <#context_type as test_context::AsyncTestContext>::setup().await;
                let #result_name = std::panic::AssertUnwindSafe( async { #body } ).catch_unwind().await;
            }
        }
        (true, false) => {
            quote! {
            use test_context::futures::FutureExt;
                let mut #context_arg_name = <#context_type as test_context::AsyncTestContext>::setup().await;
                let #result_name = std::panic::AssertUnwindSafe( async { #body } ).catch_unwind().await;
                <#context_type as test_context::AsyncTestContext>::teardown(#context_arg_name).await;
            }
        }
        (false, true) => {
            quote! {
                let mut #context_arg_name= <#context_type as test_context::TestContext>::setup();
                let #result_name = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let #context_arg_name = &mut #context_arg_name;
                    #body
                }));
            }
        }
        (false, false) => {
            quote! {
                let mut #context_arg_name = <#context_type as test_context::TestContext>::setup();
                let #result_name = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    #body
                }));
                <#context_type as test_context::TestContext>::teardown(#context_arg_name);
            }
        }
    };

    let body = quote! {
        {
            #body
            match #result_name {
                Ok(value) => value,
                Err(err) => {
                    std::panic::resume_unwind(err);
                }
            }
        }
    };

    input.block = Box::new(syn::parse2(body).unwrap());

    input
}

fn remove_context_arg(
    mut input: syn::ItemFn,
    expected_context_type: syn::Type,
) -> (syn::ItemFn, Option<syn::Ident>) {
    let mut context_arg_name = None;
    let mut new_args = syn::punctuated::Punctuated::new();

    for arg in &input.sig.inputs {
        // Extract function arg:
        if let syn::FnArg::Typed(pat_type) = arg {
            // Extract arg identifier:
            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                // Check that context arg is only ref or mutable ref:
                if let syn::Type::Reference(type_ref) = &*pat_type.ty {
                    // Check that context has expected type:
                    if types_equal(&type_ref.elem, &expected_context_type) {
                        context_arg_name = Some(pat_ident.ident.clone());
                        continue;
                    }
                }
            }
        }

        new_args.push(arg.clone());
    }

    input.sig.inputs = new_args;

    (input, context_arg_name)
}

fn types_equal(a: &syn::Type, b: &syn::Type) -> bool {
    if let (syn::Type::Path(a_path), syn::Type::Path(b_path)) = (a, b) {
        return a_path.path.segments.last().unwrap().ident
            == b_path.path.segments.last().unwrap().ident;
    }
    quote!(#a).to_string() == quote!(#b).to_string()
}
