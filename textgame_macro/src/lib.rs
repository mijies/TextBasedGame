use proc_macro::TokenStream;
use quote::quote;
// use syn;

#[proc_macro_derive(EnumVariantCount)]
pub fn derive_enum_variant_count(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    gen_enum_variant_count(input).into()
}

fn gen_enum_variant_count(input: syn::DeriveInput) -> proc_macro2::TokenStream {
    let ident = input.ident;
    let length = match input.data {
        syn::Data::Enum(ref item) => item.variants.len(),
        _ => panic!("EnumVariantCount: expected enum, but found non-enum item"),
    };

    quote! {
        impl #ident {
            pub fn variant_count() -> usize {
                #length
            }
        }
    }
}

#[proc_macro_attribute]
pub fn decorate(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_ar = syn::parse_macro_input!(attr as syn::AttributeArgs);
    let item_fn = syn::parse_macro_input!(item as syn::ItemFn);
    gen_decorate(attr_ar, item_fn).into()
}

fn gen_decorate(attr_ar: syn::AttributeArgs, item_fn: syn::ItemFn) -> proc_macro2::TokenStream {
    let visib = &item_fn.vis;
    let block = &item_fn.block;
    let ident = &item_fn.sig.ident;
    let punct = &item_fn.sig.inputs;
    let retty = &item_fn.sig.output;

    let args = parse_decorate_args(punct);
    let path = parse_decorate_attr(attr_ar);

    quote! {
        #visib fn #ident (#punct) #retty {
            fn wrapped (#punct) #retty #block;
            #path(wrapped, #( #args, )*)
        }
    }
}

fn parse_decorate_args(punct: &syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>) -> Vec<&std::boxed::Box<syn::Pat>> {
    punct.into_iter()
        .map(|arg| match arg {
            syn::FnArg::Typed(p) => &p.pat,
            _ => panic!("gen_decorate: not supported on types with self"), // FnArg::Reciever(r)
        })
        .collect::<Vec<_>>()
}

fn parse_decorate_attr(attr_ar: syn::AttributeArgs) -> syn::Path {
    if attr_ar.len() != 1 {
        panic!("decorate: only single attribute path supported")
    }
    match attr_ar.first() {
        Some(syn::NestedMeta::Meta(syn::Meta::Path(ref path))) => path.clone(),
        _ => panic!("decorate: only meta path item supported"),
    }
}
