use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, LitStr};

#[derive(Default, FromDeriveInput)]
#[darling(default, attributes(pokemonle))]
struct StructNameOpts {
    tags: Vec<LitStr>,
}

#[proc_macro_derive(StructName, attributes(pokemonle))]
pub fn struct_name_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let opts = StructNameOpts::from_derive_input(&input).unwrap();
    let tags_str = &opts.tags.iter().map(LitStr::value).collect::<Vec<_>>();

    quote! {
        impl ::pokemonle_trait::StructName for #name {
            fn struct_name() -> &'static str {
                stringify!(#name)
            }

            fn tags() -> &'static [&'static str] {
                &[#(#tags_str),*]
            }
        }

    }
    .into()
}
