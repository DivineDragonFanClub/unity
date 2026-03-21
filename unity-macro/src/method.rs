use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, parse_quote, BareFnArg, FnArg, LitInt, LitStr, Token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;

enum ClassMethodSearchType {
    Index(usize),
    Name(String),
}
struct ClassMethodAttrs(
    ClassMethodSearchType,
    bool,   // GenericMethod: Method<T1, T2, ..>,
    bool,   // vtable method
    Option<Ident>,  //  class for the method
);

impl Parse for ClassMethodAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut d = Self(ClassMethodSearchType::Index(0), false, false, None);
        let lookahead = input.lookahead1();
        if lookahead.peek(LitInt) {
            let int = input.parse::<LitInt>()?;
            let value = int.base10_parse::<usize>()?;
            d.0 = ClassMethodSearchType::Index(value);
        }
        else if lookahead.peek(LitStr) {
            let s = input.parse::<LitStr>()?;
            d.0 = ClassMethodSearchType::Name(s.value());
        }
        else { return Err(lookahead.error()); }
        loop{
            if input.is_empty() { break; }
            if input.peek(Token![,]) { input.parse::<Token![,]>()?; }
            else{
                if let Some(ident) = input.parse::<Ident>().ok() {
                    if ident.to_string() == "generic" { d.1 = true; }
                    else if ident.to_string() == "vtable" { d.2 = true; }
                    else if d.3.is_none() { d.3 = Some(ident); }
                }
                else { break; }
            }
        }
        Ok(d)
    }
}

/// Macro to generate functions from Il2CppClassData using:
/// Method Index / Method Name
///
/// Optional of Il2CppClassData Class for methods from Instantiated Generic Class / Parent Class
/// `generic`: uses function parameters to get the instantiated generic method
/// `vtable`: looks for the method in the class's vtable instead of the method table.
///
pub fn class_method(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut fn_sig = parse_macro_input!(input as syn::ForeignItemFn);
    let ClassMethodAttrs(index, is_generic, vtable, method_class) = parse_macro_input!(attr as ClassMethodAttrs);
    let mut inner_fn_type: syn::TypeBareFn = parse_quote!( fn() );
    inner_fn_type.output = fn_sig.sig.output.clone();
    let (inputs, arg_count) = into_bare_args(&fn_sig.sig.inputs);
    inner_fn_type.inputs = inputs;

    let visibility = fn_sig.vis;
    let generics = fn_sig.sig.generics.params.clone();

    let generic_method = if is_generic { quote!(let method = unity::get_generic_method!(method<#generics>);) } else { quote!() };
    let sig = fn_sig.sig;
    let mut args = get_arg_pats(&sig.inputs);
    let ctx = crate::utils::context();
    inner_fn_type.inputs.push(parse_quote!(&#ctx::MethodInfo));
    args.push(parse_quote!(method));

    let method_class =
        if let Some(class) = method_class { quote!( let class = Self::class().find_class_in_hierarchy(#class::class()).unwrap_or(#class::class()); ) }
        else { quote!( let class = Self::class(); ) };

    let get_method =
        match index {
            ClassMethodSearchType::Index(index) => {
                if vtable { quote!( let method = class.get_vtable()[#index].method_info;) }
                else { quote!( let method = class.get_methods().get(#index).unwrap();) }
            }
            ClassMethodSearchType::Name(name) => {
                if vtable { quote! (let method = class.get_virtual_method(#name).unwrap().method_info;) }
                else { quote!( let method = class.get_method_from_name(#name, #arg_count).unwrap(); ) }
            }
        };
    quote!(
        #visibility #sig {
            unsafe {
                #method_class
                #get_method
                #generic_method
                let fn_call = core::mem::transmute::<_,#inner_fn_type>(method.method_ptr);
                fn_call(#args)
            }
        }
    ).into()
}
fn into_bare_args(args: &Punctuated<FnArg, Comma>) -> (Punctuated<BareFnArg, Comma>, usize) {
    let mut count = 0;
    let args = args.iter()
        .map(|arg|{
            match arg {
                FnArg::Typed(pat_type) => {
                    count += 1;
                    BareFnArg {
                        attrs: pat_type.attrs.clone(),
                        name: None,
                        ty: (*pat_type.ty).clone()
                    }
                }
                FnArg::Receiver(rev) => {
                    BareFnArg {
                        attrs: rev.attrs.clone(),
                        name: None,
                        ty: (*rev.ty).clone()
                    }
                }
            }
        })
        .collect();
    (args, count)
}
fn get_arg_pats(args: &Punctuated<FnArg, Comma>) -> Punctuated<syn::Pat, Comma> {
    args.iter()
        .map(|arg|{
            match arg {
                FnArg::Typed(pat_type) => { (*pat_type.pat).clone() }
                FnArg::Receiver(_) => { parse_quote!(self) }
            }
        })
        .collect()
}