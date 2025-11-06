mod args;

use args::TestContextArgs;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;

#[derive(PartialEq, Eq, Debug)]
enum ContextArgMode {
    /// The argument was passed as an owned value (`ContextType`). Only valid with `skip_teardown`.
    Owned,
    /// The argument was passed as an immutable reference (`&ContextType`).
    Reference,
    /// The argument was passed as a mutable reference (`&mut ContextType`).
    MutableReference,
}

struct ContextArgInfo {
    /// The identifier name used for the context argument.
    pub name: syn::Ident,
    /// The mode in which the context was passed to the test function.
    pub mode: ContextArgMode,
}

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

    let (input, context_arg_info) =
        remove_context_arg(input, args.context_type.clone(), args.skip_teardown);

    let input = refactor_input_body(input, &args, context_arg_info);

    quote! { #input }.into()
}

fn remove_context_arg(
    mut input: syn::ItemFn,
    expected_context_type: syn::Type,
    skip_teardown: bool,
) -> (syn::ItemFn, ContextArgInfo) {
    // 1. Partition the function arguments into two groups:
    //    (Context arguments, Other arguments)
    let (context_args, new_args) = input
        .sig
        .inputs
        .into_iter()
        .partition::<Punctuated<_, _>, _>(|arg| {
            // Check if the argument is the context argument
            if let syn::FnArg::Typed(pat_type) = arg {
                if let syn::Pat::Ident(_) = &*pat_type.pat {
                    let arg_type = &*pat_type.ty;

                    // Check for mutable/immutable reference
                    if let syn::Type::Reference(type_ref) = arg_type {
                        return types_equal(&type_ref.elem, &expected_context_type);
                    }

                    // If skip_teardown is true, we also consider the fact
                    // that the context type could be fully owned and not just a reference
                    if skip_teardown && types_equal(arg_type, &expected_context_type) {
                        return true;
                    } else if types_equal(arg_type, &expected_context_type) {
                        panic!("If skip_teardown is false, we can't use an owned type")
                    } else {
                        return false;
                    }
                }
            }
            false
        });

    if context_args.len() != 1 {
        panic!("Exactly one Context argument needs to be provided to the test");
    }

    let context_arg = context_args.into_iter().next().unwrap();

    input.sig.inputs = new_args;

    // 2. Extract the identifier and mode from the single context argument found (if any).
    let context_arg_info = if let syn::FnArg::Typed(pat_type) = context_arg
        && let syn::Pat::Ident(pat_ident) = *pat_type.pat
    {
        let arg_type = &*pat_type.ty;

        let mode = if let syn::Type::Reference(type_ref) = arg_type {
            if type_ref.mutability.is_some() {
                ContextArgMode::MutableReference
            } else {
                ContextArgMode::Reference
            }
        } else {
            ContextArgMode::Owned
        };

        ContextArgInfo {
            name: pat_ident.ident,
            mode,
        }
    } else {
        panic!("Invalid context argument provided, it must be a reference or an owned type");
    };

    (input, context_arg_info)
}

fn refactor_input_body(
    mut input: syn::ItemFn,
    args: &TestContextArgs,
    context_arg_info: ContextArgInfo,
) -> syn::ItemFn {
    let context_type = &args.context_type;
    let result_name = format_ident!("wrapped_result");
    let body = &input.block;
    let is_async = input.sig.asyncness.is_some();

    // Determine the identifier and its mode. Default to "test_ctx" and MutableReference.
    let (context_arg_name, context_mode) = (context_arg_info.name, context_arg_info.mode);

    let context_binding = match context_mode {
        ContextArgMode::Owned => quote! { let #context_arg_name = __context; },
        ContextArgMode::Reference => quote! { let #context_arg_name = &__context; },
        ContextArgMode::MutableReference => quote! { let #context_arg_name = &mut __context; },
    };

    let body = match (is_async, args.skip_teardown) {
        // ASYNC and SKIP_TEARDOWN
        (true, true) => {
            quote! {
                use test_context::futures::FutureExt;
                let mut __context = <#context_type as test_context::AsyncTestContext>::setup().await;
                #context_binding
                let #result_name = std::panic::AssertUnwindSafe( async { #body } ).catch_unwind().await;
            }
        }
        // SYNC and SKIP_TEARDOWN
        (false, true) => {
            quote! {
                let mut __context = <#context_type as test_context::TestContext>::setup();
                let #result_name = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    #context_binding
                    #body
                }));
            }
        }
        // ASYNC and TEARDOWN (Teardown requires context ownership, so the test body must use &mut)
        (true, false) => {
            quote! {
                use test_context::futures::FutureExt;
                let mut __context = <#context_type as test_context::AsyncTestContext>::setup().await;
                // MUST bind as &mut regardless of user's original signature to allow teardown
                #context_binding
                let #result_name = std::panic::AssertUnwindSafe( async { #body } ).catch_unwind().await;
                <#context_type as test_context::AsyncTestContext>::teardown(__context).await;
            }
        }
        // SYNC and TEARDOWN (Teardown requires context ownership, so the test body must use &mut)
        (false, false) => {
            quote! {
                let mut __context = <#context_type as test_context::TestContext>::setup();
                let #result_name = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    #context_binding
                    #body
                }));
                <#context_type as test_context::TestContext>::teardown(__context);
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

// Note: The rest of the functions (test_context, refactor_input_body, types_equal) remain unchanged.
fn types_equal(a: &syn::Type, b: &syn::Type) -> bool {
    if let (syn::Type::Path(a_path), syn::Type::Path(b_path)) = (a, b) {
        return a_path.path.segments.last().unwrap().ident
            == b_path.path.segments.last().unwrap().ident;
    }
    quote!(#a).to_string() == quote!(#b).to_string()
}
