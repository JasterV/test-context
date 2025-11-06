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
    /// The argument was passed as an owned value (mut `ContextType`). Only valid with `skip_teardown`.
    OwnedMut,
    /// The argument was passed as an immutable reference (`&ContextType`).
    Reference,
    /// The argument was passed as a mutable reference (`&mut ContextType`).
    MutableReference,
}

impl ContextArgMode {
    pub fn is_owned(&self) -> bool {
        match self {
            ContextArgMode::Owned => true,
            ContextArgMode::OwnedMut => true,
            ContextArgMode::Reference => false,
            ContextArgMode::MutableReference => false,
        }
    }
}

#[derive(Clone)]
pub enum TestArg {
    Any(FnArg),
    Context(ContextArg),
}

impl TestArg {
    pub fn parse_arg_with_expected_context(arg: FnArg, expected_context_type: &syn::Type) -> Self {
        let syn::FnArg::Typed(pat_type) = &arg else {
            return Self::Any(arg);
        };

        let syn::Pat::Ident(pat_ident) = &*pat_type.pat else {
            return Self::Any(arg);
        };

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

            return TestArg::Context(ContextArg {
                name: pat_ident.ident.clone(),
                mode,
            });
        }

        if !types_equal(arg_type, expected_context_type) {
            return TestArg::Any(arg);
        }

        // To determine mutability for an owned type, we check the identifier pattern.
        let mode = if pat_ident.mutability.is_some() {
            // This catches signatures like: `mut my_ctx: ContextType`
            ContextArgMode::OwnedMut
        } else {
            // This catches signatures like: `my_ctx: ContextType`
            ContextArgMode::Owned
        };

        TestArg::Context(ContextArg {
            name: pat_ident.ident.clone(),
            mode,
        })
    }
}

fn types_equal(a: &syn::Type, b: &syn::Type) -> bool {
    if let (syn::Type::Path(a_path), syn::Type::Path(b_path)) = (a, b) {
        return a_path.path.segments.last().unwrap().ident
            == b_path.path.segments.last().unwrap().ident;
    }
    quote!(#a).to_string() == quote!(#b).to_string()
}
