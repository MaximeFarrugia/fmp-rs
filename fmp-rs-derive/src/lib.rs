use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(FmpEndpoint, attributes(fmp))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let attrs = &input.attrs.iter().map(|x| x.parse_args());
    let (impl_generics, ty_generics, where_clause) = &input.generics.split_for_impl();
    eprintln!("{input:#?}");
    eprintln!("{attrs:#?}");
    let tokens = quote! {
        impl #impl_generics fmp_rs::api::endpoint::Endpoint for #name #ty_generics #where_clause {
            fn endpoint(&self) -> Cow<'static, str> {
                return std::borrow::Cow::Borrowed("string");
            }
        }
    };
    return tokens.into();
}
