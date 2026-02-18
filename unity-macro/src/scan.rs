use proc_macro::TokenStream;
use proc_macro2::TokenStream as Quote;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, token::Comma, FnArg, ForeignItemFn, ItemFn};

#[derive(deluxe::ParseMetaItem)]
struct ScanInfo(String, String, String, #[deluxe(default)] usize);

impl ScanInfo {
    pub fn get_scan_fn(self, arg_count: usize) -> Quote {
        let ScanInfo(namespace, class, method, forced_arg_count) = self;
        
        let arg_count = if forced_arg_count == usize::default() {
            arg_count
        } else {
            forced_arg_count
        };

        let ctx = super::utils::context();

        quote!(
            pub const NAMESPACE: &str = #namespace;
            pub const CLASS_NAME: &str = #class;
            pub const METHOD_NAME: &str = #method;
            pub const ARG_COUNT: usize = #arg_count;

            static INFO: #ctx::LazyLock<&'static mut #ctx::MethodInfo> = #ctx::LazyLock::new(||
                #ctx::Il2CppClass::from_name(NAMESPACE, CLASS_NAME)
                    .expect(&format!("Failed to find class {}.{}", NAMESPACE, CLASS_NAME))
                    .get_method_from_name(METHOD_NAME, ARG_COUNT)
                    .expect(&format!("Failed to find method {}.{}({}) arg count {}", NAMESPACE, CLASS_NAME, METHOD_NAME, ARG_COUNT))
            );

            pub fn as_base() -> #ctx::MethodInfo {
                #ctx::MethodInfo::new_from(INFO.clone())
            }

            pub fn get_ref<'a>() -> &'a #ctx::MethodInfo {
                &INFO
            }

            pub fn get_offset() -> usize {
                static OFFSETS: #ctx::LazyLock<usize> = #ctx::LazyLock::new(|| {
                    let method = &INFO;
                    let text = #ctx::scan::get_text();
                    unsafe { method.method_ptr.offset_from(text.as_ptr()) as usize}
                });
                *OFFSETS
            }
        )
    }
}

fn get_fn_arg_count(inputs: &Punctuated<FnArg, Comma>) -> usize {
    let mut count = 0;
    for input in inputs {
        let FnArg::Typed(pat_type) = input else { continue };
        let syn::Pat::Ident(pat_ident) = &*pat_type.pat else { continue };
        let name = pat_ident.ident.to_string();
        if &name[0..1] != "_"
        && name != "method_info"
        && name != "this"
        {
            count += 1;
        }
    }
    count
}

pub fn hook(attr: TokenStream, item: TokenStream) -> TokenStream {
    // parse
    let hook_function = parse_macro_input!(item as ItemFn);
    let scan_info = match deluxe::parse::<ScanInfo>(attr) {
        Ok(info) => info,
        Err(err) => return err.to_compile_error().into(),
    };

    // prepare tokens
    let scan_fn_name = &hook_function.sig.ident;
    let scan_fn_token = scan_info.get_scan_fn(get_fn_arg_count(&hook_function.sig.inputs));

    quote!(
        #[skyline::hook(offset = #scan_fn_name::get_offset())]
        #hook_function
        pub mod #scan_fn_name {
            #scan_fn_token
        }
    )
    .into()
}

pub fn from_offset(attr: TokenStream, item: TokenStream) -> TokenStream {
    // parse
    let function = parse_macro_input!(item as ForeignItemFn);
    let scan_info = match deluxe::parse::<ScanInfo>(attr) {
        Ok(info) => info,
        Err(err) => return err.to_compile_error().into(),
    };

    // prepare tokens
    let scan_fn_name = &function.sig.ident;
    let scan_module = scan_info.get_scan_fn(get_fn_arg_count(&function.sig.inputs));

    quote!(
        #[skyline::from_offset(#scan_fn_name::get_offset() as usize)]
        #function
        #[doc(hidden)]
        pub mod #scan_fn_name {
            #scan_module
        }
    )
    .into()
}
