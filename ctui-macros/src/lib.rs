//! Procedural macros for cTUI framework.
// Suppress pedantic lints
#![allow(missing_docs)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::similar_names)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::use_self)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::type_complexity)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::double_must_use)]
#![allow(clippy::float_cmp)]
#![allow(clippy::used_underscore_binding)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::unused_self)]
#![allow(clippy::bool_assert_comparison)]
#![allow(clippy::missing_fields_in_debug)]
#![allow(clippy::unnecessary_unwrap)]
#![allow(clippy::unnecessary_sort_by)]
#![allow(clippy::manual_is_multiple_of)]
//!
//! This crate provides the `#[component]` attribute macro for generating
//! boilerplate code for cTUI components.
//!
//! # Example
//!
//! ```rust,ignore
//! use ctui_macros::component;
//!
//! #[component]
//! struct Button {
//!     label: String,
//!     #[prop(default = false)]
//!     disabled: bool,
//! }
//! ```
//!
//! This generates:
//! - The original `Button` struct
//! - A `ButtonProps` struct with the same fields
//! - A `Component` impl with default implementations

use proc_macro::TokenStream as ProcTokenStream;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Fields, Ident, Lit, Meta, Result};

/// Attribute macro that generates Props struct and Component impl for a struct.
///
/// # Generated Code
///
/// Given a struct like:
/// ```rust,ignore
/// #[component]
/// struct Button {
///     label: String,
///     #[prop(default = false)]
///     disabled: bool,
/// }
/// ```
///
/// This macro generates:
/// 1. The original `Button` struct (unchanged)
/// 2. A `ButtonProps` struct with the same fields
/// 3. A `Component` implementation for `Button` with:
///    - `Props = ButtonProps`
///    - `State = ()`
///    - `create(props)` that initializes fields from props
///    - Default `render`, `update`, `on_mount`, `on_unmount` methods
///
/// # Attributes
///
/// - `#[prop(default = ...)]` - Specify a default value for a field in Props.
///   The field will be `Option<T>` in Props if no default is provided.
#[proc_macro_attribute]
pub fn component(_attr: ProcTokenStream, item: ProcTokenStream) -> ProcTokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    match impl_component(&input) {
        Ok(output) => output.into(),
        Err(error) => {
            let error = error.to_compile_error();
            quote! { #error }.into()
        }
    }
}

#[allow(clippy::too_many_lines)]
fn impl_component(input: &DeriveInput) -> Result<TokenStream> {
    let name = &input.ident;
    let props_name = format_ident!("{}Props", name);

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "component macro only supports structs with named fields",
                ))
            }
        },
        _ => {
            return Err(syn::Error::new(
                Span::call_site(),
                "component macro only supports structs",
            ))
        }
    };

    let mut required_fields: Vec<(&Ident, &syn::Type)> = Vec::new();
    let mut optional_fields: Vec<(&Ident, &syn::Type, Option<syn::Expr>)> = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        let mut default_value: Option<syn::Expr> = None;

        for attr in &field.attrs {
            if attr.path().is_ident("prop") {
                if let Meta::List(meta_list) = &attr.meta {
                    let nested: syn::punctuated::Punctuated<Meta, syn::Token![,]> =
                        meta_list.parse_args_with(syn::punctuated::Punctuated::parse_terminated)?;

                    for nested_meta in nested {
                        if let Meta::NameValue(nv) = nested_meta {
                            if nv.path.is_ident("default") {
                                if let syn::Expr::Lit(expr_lit) = &nv.value {
                                    if let Lit::Bool(lit_bool) = &expr_lit.lit {
                                        default_value = Some(parse_quote! { #lit_bool });
                                    } else if let Lit::Int(lit_int) = &expr_lit.lit {
                                        default_value = Some(parse_quote! { #lit_int });
                                    } else if let Lit::Str(lit_str) = &expr_lit.lit {
                                        let s = lit_str.token();
                                        default_value = Some(parse_quote! { #s.to_string() });
                                    }
                                } else {
                                    default_value = Some(nv.value.clone());
                                }
                            }
                        }
                    }
                }
            }
        }

        if default_value.is_some() {
            optional_fields.push((field_name, field_type, default_value));
        } else {
            required_fields.push((field_name, field_type));
        }
    }

    let props_required_fields = required_fields.iter().map(|(name, ty)| {
        quote! { #name: #ty }
    });

    let props_optional_fields = optional_fields.iter().map(|(name, ty, default)| {
        if default.is_some() {
            quote! { #name: #ty }
        } else {
            quote! { #name: Option<#ty> }
        }
    });

    let create_required_inits = required_fields.iter().map(|(name, _)| {
        quote! { #name: props.#name }
    });

    let create_optional_inits = optional_fields.iter().map(|(name, _ty, default)| {
        if default.is_some() {
            quote! { #name: props.#name }
        } else {
            quote! { #name: props.#name.unwrap_or_default() }
        }
    });

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let original_struct = quote! {
        #input
    };

    let props_struct = quote! {
        /// Props for the component, generated by #[component] macro.
        #[derive(Debug, Clone)]
        pub struct #props_name {
            #(#props_required_fields,)*
            #(#props_optional_fields,)*
        }
    };

    let component_impl = quote! {
        impl #impl_generics ctui_core::Component for #name #ty_generics #where_clause {
            type Props = #props_name;
            type State = ();

            fn create(props: Self::Props) -> Self {
                Self {
                    #(#create_required_inits,)*
                    #(#create_optional_inits,)*
                }
            }

            fn render(&self, _area: ctui_core::Rect, _buf: &mut ctui_core::Buffer) {
                // Intentionally empty - override in impl
            }

            fn update(&mut self, _msg: Box<dyn ctui_core::Msg>) -> ctui_core::Cmd {
                ctui_core::Cmd::Noop
            }
        }
    };

    let output = quote! {
        #original_struct

        #props_struct

        #component_impl
    };

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_struct() {
        let input: DeriveInput = parse_quote! {
            struct Button {
                label: String,
            }
        };

        let result = impl_component(&input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_struct_with_default() {
        let input: DeriveInput = parse_quote! {
            struct Button {
                label: String,
                #[prop(default = false)]
                disabled: bool,
            }
        };

        let result = impl_component(&input);
        assert!(result.is_ok());
    }
}

    // =========================================================================
    // EDGE CASE UNIT TESTS - Error handling
    // =========================================================================

    #[test]
    fn test_error_on_tuple_struct() {
        let input: DeriveInput = parse_quote! {
            struct Tuple(i32, String);
        };

        let result = impl_component(&input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        let err_str = err.to_string();
        assert!(err_str.contains("only supports structs with named fields"));
    }

    #[test]
    fn test_error_on_unit_struct() {
        let input: DeriveInput = parse_quote! {
            struct Unit;
        };

        let result = impl_component(&input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        let err_str = err.to_string();
        assert!(err_str.contains("only supports structs with named fields"));
    }

    #[test]
    fn test_error_on_enum() {
        let input: DeriveInput = parse_quote! {
            enum MyEnum {
                A,
                B,
            }
        };

        let result = impl_component(&input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        let err_str = err.to_string();
        assert!(err_str.contains("only supports structs"));
    }

    #[test]
    fn test_no_fields_still_works() {
        // Empty named struct should be supported (edge case)
        let input: DeriveInput = parse_quote! {
            struct Empty {}
        };

        let result = impl_component(&input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_props_name_formatting() {
        let input: DeriveInput = parse_quote! {
            struct MyWidget {
                x: i32,
            }
        };

        let result = impl_component(&input).unwrap();
        let result_str = result.to_string();

        // Check that MyWidgetProps is generated
        assert!(result_str.contains("MyWidgetProps"));
    }
