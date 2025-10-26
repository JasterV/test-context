mod args;

use args::TestContextArgs;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Block, Ident};

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
    let is_async = input.sig.asyncness.is_some();

    let (new_input, context_arg_name) =
        extract_and_remove_context_arg(input.clone(), args.context_type.clone());

    let wrapper_body = if is_async {
        async_wrapper_body(args, &context_arg_name, &input.block)
    } else {
        sync_wrapper_body(args, &context_arg_name, &input.block)
    };

    let mut result_input = new_input;
    result_input.block = Box::new(syn::parse2(wrapper_body).unwrap());

    quote! { #result_input }.into()
}

fn async_wrapper_body(
    args: TestContextArgs,
    context_arg_name: &Option<syn::Ident>,
    body: &Box<Block>,
) -> proc_macro2::TokenStream {
    let context_type = args.context_type;
    let result_name = format_ident!("wrapped_result");

    let binding = format_ident!("test_ctx");
    let context_name = context_arg_name.as_ref().unwrap_or(&binding);

    let body = if args.skip_teardown {
        quote! {
            let #context_name = <#context_type as test_context::AsyncTestContext>::setup().await;
            let #result_name = std::panic::AssertUnwindSafe( async { #body } ).catch_unwind().await;
        }
    } else {
        quote! {
            let mut #context_name = <#context_type as test_context::AsyncTestContext>::setup().await;
            let #result_name = std::panic::AssertUnwindSafe( async { #body } ).catch_unwind().await;
            <#context_type as test_context::AsyncTestContext>::teardown(#context_name).await;
        }
    };

    let handle_wrapped_result = handle_result(result_name);

    quote! {
        {
            use test_context::futures::FutureExt;
            #body
            #handle_wrapped_result
        }
    }
}

fn sync_wrapper_body(
    args: TestContextArgs,
    context_arg_name: &Option<syn::Ident>,
    body: &Box<Block>,
) -> proc_macro2::TokenStream {
    let context_type = args.context_type;
    let result_name = format_ident!("wrapped_result");

    let binding = format_ident!("test_ctx");
    let context_name = context_arg_name.as_ref().unwrap_or(&binding);

    let body = if args.skip_teardown {
        quote! {
            let mut #context_name = <#context_type as test_context::TestContext>::setup();
            let #result_name = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let #context_name = &mut #context_name;
                #body
            }));
        }
    } else {
        quote! {
            let mut #context_name = <#context_type as test_context::TestContext>::setup();
            let #result_name = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                #body
            }));
            <#context_type as test_context::TestContext>::teardown(#context_name);
        }
    };

    let handle_wrapped_result = handle_result(result_name);

    quote! {
        {
            #body
            #handle_wrapped_result
        }
    }
}

fn handle_result(result_name: Ident) -> proc_macro2::TokenStream {
    quote! {
        match #result_name {
            Ok(value) => value,
            Err(err) => {
                std::panic::resume_unwind(err);
            }
        }
    }
}

fn extract_and_remove_context_arg(
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
