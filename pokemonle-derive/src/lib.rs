use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(StructName)]
pub fn struct_name_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    quote! {
        impl ::pokemonle_trait::StructName for #name {
            fn struct_name() -> &'static str {
                stringify!(#name)
            }
        }

    }
    .into()
}
