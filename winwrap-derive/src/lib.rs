// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, FnArg, ItemFn};

macro_rules! derive_handle {
    ($name:ident) => (
    #[proc_macro_derive($name)]
    #[allow(non_snake_case)]
    pub fn $name(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
        let ast = parse_macro_input!(ast as DeriveInput);
        let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();
        let struct_name = &ast.ident;
        let k = match ast.data {
            Data::Struct(_) => {
                quote! {
                    impl #impl_generics crate::handle::$name for #struct_name #type_generics #where_clause {}
                }
            }
            _ => panic!(concat!(stringify!($name)," can only be derived from struct")),
        };
        k.into()
    }
    );
}

derive_handle! {WritableHandle}
derive_handle! {ReadableHandle}
derive_handle! {CancelableIoHandle}
derive_handle! {WaitableHandle}
derive_handle! {DuplicatableHandle}

/// Generates `xxx` function from `xxx_a` function.
#[proc_macro_attribute]
pub fn ansi_fn(_attr: proc_macro::TokenStream, ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_fn(ast, true)
}

/// Generates `xxx` function from `xxx_w` function.
#[proc_macro_attribute]
pub fn unicode_fn(_attr: proc_macro::TokenStream, ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_fn(ast, false)
}

fn generate_fn(ast: proc_macro::TokenStream, is_ansi: bool) -> proc_macro::TokenStream {
    let orig_fn = parse_macro_input!(ast as ItemFn);
    let orig_name = orig_fn.sig.ident.to_string();
    assert!(orig_name.ends_with(if is_ansi { "_a" } else { "_w" }));
    let ident = syn::Ident::new(&orig_name, proc_macro2::Span::call_site()); // create_file_w
    // new function name.
    // e.g. create_file_w -> create_file
    let new_ident = syn::Ident::new(&orig_name[..orig_name.len() - 2], proc_macro2::Span::call_site());

    let (impl_generics, _, where_clause) = orig_fn.sig.generics.split_for_impl();
    let vis = orig_fn.vis.clone(); // Visibility
    let unsafety = orig_fn.sig.unsafety.clone(); // Unsafety

    // generates argument tokens.
    let mut idents = proc_macro2::TokenStream::new();
    for x in orig_fn.sig.inputs.iter() {
        match x {
            FnArg::Typed(pt) => {
                let x = pt.pat.as_ref();
                idents.extend(quote! {#x,});
            }
            _ => panic!("Unknown arg type"),
        }
    }
    let rty = orig_fn.sig.output.clone(); // ReturnType

    let m = orig_fn.sig.inputs.clone(); // FnArg declarations.
    let body = quote! { #ident(#idents) };

    let new_fn = if is_ansi {
        quote! {
        #[cfg(feature = "ansi")]
        #vis #unsafety fn #new_ident #impl_generics(#m) #rty #where_clause{
            #body
        }
    }
    } else {
        quote! {
        #[cfg(not(feature = "ansi"))]
        #vis #unsafety fn #new_ident #impl_generics(#m) #rty #where_clause{
            #body
        }
    }
    };
    let item = quote! { #orig_fn #new_fn };
    item.into()
}
