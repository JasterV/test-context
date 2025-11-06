mod macro_args;
mod test_args;

use crate::test_args::{ContextArg, ContextArgMode, TestArg};
use macro_args::TestContextArgs;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::ItemFn;

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

    let (input, context_args) = remove_context_args(input, args.context_type.clone());

    if context_args.len() != 1 {
        panic!("Exactly one Context argument must be defined");
    }

    let context_arg = context_args.into_iter().next().unwrap();

    if !args.skip_teardown && context_arg.mode == ContextArgMode::Owned {
        panic!(
            "It is not possible to take ownership of the context if the teardown has to be ran."
        );
    }

    let input = refactor_input_body(input, &args, context_arg);

    quote! { #input }.into()
}

fn remove_context_args(
    mut input: syn::ItemFn,
    expected_context_type: syn::Type,
) -> (syn::ItemFn, Vec<ContextArg>) {
    let test_args: Vec<TestArg> = input
        .sig
        .inputs
        .into_iter()
        .map(|arg| TestArg::parse_arg_with_expected_context(arg, &expected_context_type))
        .collect();

    let context_args: Vec<ContextArg> = test_args
        .iter()
        .cloned()
        .filter_map(|arg| match arg {
            TestArg::Any(_) => None,
            TestArg::Context(context_arg_info) => Some(context_arg_info),
        })
        .collect();

    let new_args: syn::punctuated::Punctuated<_, _> = test_args
        .into_iter()
        .filter_map(|arg| match arg {
            TestArg::Any(fn_arg) => Some(fn_arg),
            TestArg::Context(_) => None,
        })
        .collect();

    input.sig.inputs = new_args;

    (input, context_args)
}

fn refactor_input_body(
    input: syn::ItemFn,
    args: &TestContextArgs,
    context_arg: ContextArg,
) -> syn::ItemFn {
    let context_type = &args.context_type;
    let result_name = format_ident!("wrapped_result");
    let body = &input.block;
    let is_async = input.sig.asyncness.is_some();
    let context_arg_name = context_arg.name;

    let context_binding = match context_arg.mode {
        ContextArgMode::Owned => quote! { let #context_arg_name = __context; },
        ContextArgMode::Reference => quote! { let #context_arg_name = &__context; },
        ContextArgMode::MutableReference => quote! { let #context_arg_name = &mut __context; },
    };

    let body = if args.skip_teardown && is_async {
        quote! {
            use test_context::futures::FutureExt;
            let mut __context = <#context_type as test_context::AsyncTestContext>::setup().await;
            #context_binding
            let #result_name = std::panic::AssertUnwindSafe( async { #body } ).catch_unwind().await;
        }
    } else if args.skip_teardown && !is_async {
        quote! {
            let mut __context = <#context_type as test_context::TestContext>::setup();
            let #result_name = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                #context_binding
                #body
            }));
        }
    } else if !args.skip_teardown && is_async {
        quote! {
            use test_context::futures::FutureExt;
            let mut __context = <#context_type as test_context::AsyncTestContext>::setup().await;
            #context_binding
            let #result_name = std::panic::AssertUnwindSafe( async { #body } ).catch_unwind().await;
            <#context_type as test_context::AsyncTestContext>::teardown(__context).await;
        }
    }
    // !args.skip_teardown && !is_async
    else {
        quote! {
            let mut __context = <#context_type as test_context::TestContext>::setup();
            let #result_name = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                #context_binding
                #body
            }));
            <#context_type as test_context::TestContext>::teardown(__context);
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

    ItemFn {
        block: Box::new(syn::parse2(body).unwrap()),
        ..input
    }
}
