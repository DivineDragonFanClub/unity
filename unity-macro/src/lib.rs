use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_quote, FieldsNamed};

mod utils;

fn inject_record_fields(fields: &mut FieldsNamed) {
    let mut new_fields: FieldsNamed = parse_quote! {
        {
            klass: *const u8,
            monitor: *const u8,
        }
    };

    new_fields.named.extend(std::mem::take(&mut fields.named).into_iter());
    fields.named = new_fields.named;
}

#[proc_macro_attribute]
pub fn object(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = syn::parse_macro_input!(item as syn::ItemStruct);

    match input.fields {
        syn::Fields::Named(mut named) => {
            inject_record_fields(&mut named);
            input.fields = syn::Fields::Named(named);
        },
        _ => return quote!(compile_error!("The structure is does not contain named arguments")).into(),
    }

    quote!(#input).into()
}

mod scan;

#[proc_macro_attribute]
pub fn hook(attr: TokenStream, item: TokenStream) -> TokenStream {
    scan::hook(attr, item)
}

#[proc_macro_attribute]
pub fn from_offset(attr: TokenStream, item: TokenStream) -> TokenStream {
    scan::from_offset(attr, item)
}

mod il2cpp;

#[proc_macro_attribute]
pub fn class(attr: TokenStream, item: TokenStream) -> TokenStream {
    il2cpp::class(attr, item)
}

mod method;

#[proc_macro_attribute]
pub fn class_method(attr: TokenStream, item: TokenStream) -> TokenStream {
    method::class_method(attr, item)
}
