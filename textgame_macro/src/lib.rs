use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(EnumVariantCount)]
pub fn derive_enum_variant_count(input: TokenStream) -> TokenStream {

    let derive_input = parse_macro_input!(input as DeriveInput);

    let (ident, length) = match derive_input.data {
        syn::Data::Enum(item) => (derive_input.ident, item.variants.len()),
        _ => panic!("EnumVariantCount: expected enum, but found non-enum item"),
    };

    let new_item = quote! {
        impl #ident {
            pub fn variant_count() -> usize {
                #length
            }
        }
    };
    new_item.into()
}
