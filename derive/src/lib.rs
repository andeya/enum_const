use proc_macro::TokenStream;

use quote::quote;
use syn::parse_macro_input;
use syn::{Data, DataEnum, DeriveInput};

/// example:
/// ```
/// #[derive(EnumConst)]
/// enum Foo {
///     X = 1,
///     Y = 2,
///     Z,
/// }
/// // expanded impl code:
/// #[automatically_derived]
/// #[allow(unused_qualifications)]
/// impl ::enum_const::EnumConst for Foo {
///     fn get_const_isize(&self) -> Option<isize> {
///         match self {
///             Self::X => Some(1),
///             Self::Y => Some(2),
///             _ => None,
///         }
///     }
///     fn from_const_isize(i: isize) -> Option<Self> {
///         match i {
///             1 => Some(Self::X),
///             2 => Some(Self::Y),
///             _ => None,
///         }
///     }
/// }
/// ```
/// command to check expanded code: `cargo +nightly rustc -- -Zunstable-options
/// --pretty=expanded`
#[proc_macro_derive(EnumConst)]
pub fn enum_const(item: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(item as DeriveInput);
    // let vis = input.vis;
    let name = &input.ident;
    let expanded = match input.data {
        Data::Struct(_) => {
            panic!("EnumConst Unsupported Struct")
        }
        Data::Union(_) => {
            panic!("EnumConst Unsupported Union")
        }
        Data::Enum(DataEnum { ref variants, .. }) => {
            let mut get_const_isize_items = quote! {};
            let mut from_const_isize_items = quote! {};
            for x in variants {
                if let Some((_, expr)) = &x.discriminant {
                    let ident = &x.ident;
                    get_const_isize_items = quote! {
                        #get_const_isize_items
                        Self::#ident => Some(#expr),
                    };
                    from_const_isize_items = quote! {
                        #from_const_isize_items
                        #expr => Some(Self::#ident),
                    };
                }
            }
            quote! {
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::enum_const::EnumConst for #name {
                    fn get_const_isize(&self) -> Option<isize> {
                        match self {
                            #get_const_isize_items
                            _ => None,
                        }
                    }

                    fn from_const_isize(i: isize) -> Option<Self> {
                        match i {
                            #from_const_isize_items
                            _ => None,
                        }
                    }
                }
            }
        }
    };

    // #[cfg(debug_assertions)]
    // println!("derive EnumConst expanded:\n{}", expanded);

    expanded.into()
}
