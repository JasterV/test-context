use quote::quote;
use syn::FnArg;

#[derive(Clone)]
pub struct ContextArg {
    /// The identifier name used for the context argument.
    pub name: syn::Ident,
    /// The mode in which the context was passed to the test function.
    pub mode: ContextArgMode,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum ContextArgMode {
    /// The argument was passed as an owned value (`ContextType`). Only valid with `skip_teardown`.
    Owned,
    /// The argument was passed as an immutable reference (`&ContextType`).
    Reference,
    /// The argument was passed as a mutable reference (`&mut ContextType`).
    MutableReference,
}

#[derive(Clone)]
pub enum TestArg {
    Any(FnArg),
    Context(ContextArg),
}

impl TestArg {
    pub fn parse_arg_with_expected_context(arg: FnArg, expected_context_type: &syn::Type) -> Self {
        // Check if the argument is the context argument
        if let syn::FnArg::Typed(pat_type) = &arg
            && let syn::Pat::Ident(pat_ident) = &*pat_type.pat
        {
            let arg_type = &*pat_type.ty;
            // Check for mutable/immutable reference
            if let syn::Type::Reference(type_ref) = arg_type
                && types_equal(&type_ref.elem, expected_context_type)
            {
                let mode = if type_ref.mutability.is_some() {
                    ContextArgMode::MutableReference
                } else {
                    ContextArgMode::Reference
                };

                TestArg::Context(ContextArg {
                    name: pat_ident.ident.clone(),
                    mode,
                })
            } else if types_equal(arg_type, expected_context_type) {
                TestArg::Context(ContextArg {
                    name: pat_ident.ident.clone(),
                    mode: ContextArgMode::Owned,
                })
            } else {
                TestArg::Any(arg)
            }
        } else {
            TestArg::Any(arg)
        }
    }
}

fn types_equal(a: &syn::Type, b: &syn::Type) -> bool {
    if let (syn::Type::Path(a_path), syn::Type::Path(b_path)) = (a, b) {
        return a_path.path.segments.last().unwrap().ident
            == b_path.path.segments.last().unwrap().ident;
    }
    quote!(#a).to_string() == quote!(#b).to_string()
}
