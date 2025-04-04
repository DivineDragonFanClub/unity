use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::Attribute;
use syn::{parse_macro_input, Type, TypePath, ItemStruct, Generics, GenericParam, punctuated::Punctuated, Token};
use syn::parse::Result;

#[derive(deluxe::ParseMetaItem)]
struct ClassData(String, String);

#[derive(Default, Debug)]
struct ClassAttributes {
    static_type: Option<Type>,
    interfaces: Vec<TypePath>
}

pub fn class(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemStruct);

    let ClassData(namespace, class) = match deluxe::parse(attrs) {
        Ok(info) => info,
        Err(err) => return err.to_compile_error().into(),
    };

    let ClassAttributes { static_type, interfaces } = process_attrs(&mut input.attrs).unwrap();
    
    let vis = &input.vis;
    let name = &input.ident;
    let fields_name = Ident::new(&format!("{}Fields", name), Span::call_site());

    let generics = strip_generics_attrs(input.generics);
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    // let generic_type = &generics.type_params();
    // dbg!(&type_generics);

    let static_method = static_type.and_then(|static_ty| {
        Some(quote! {
            impl #impl_generics #name #type_generics #where_clause {
                pub fn get_static_fields(&self) -> &#static_ty {
                    unsafe { std::mem::transmute(self.klass.static_fields) }
                }
            }
        })
    });

    let fields = input.fields;


    let ctx = super::utils::context();
    
    quote! {
        /// New Il2CppObject structure using the name from the struct item
        #[repr(C)]
        #vis struct #name #impl_generics #where_clause {
            pub klass: &'static mut #ctx::Il2CppClass,
            monitor: *const u8,
            pub fields: #fields_name #type_generics,
        }

        /// Original structure with Fields appended to the name
        #[repr(C)]
        #vis struct #fields_name #impl_generics #where_clause #fields

        // Optional Static impl here
        // TODO: Make sure the type provided implements Il2CppClassData or something
        #static_method

        impl #impl_generics #ctx::Il2CppObjectMethods for #name #type_generics #where_clause {
            fn get_class(&self) -> &#ctx::Il2CppClass {
                &self.klass
            }

            fn get_class_mut(&mut self) -> &mut #ctx::Il2CppClass {
                &mut self.klass
            }
        }

        // AsRef/AsMut to the Fields variant

        // impl #impl_generics AsRef<#fields_name #type_generics> for #name #type_generics #where_clause {
        //     fn as_ref(&self) -> &#fields_name #type_generics {
        //         &self.fields
        //     }
        // }

        // impl #impl_generics AsMut<#fields_name #type_generics> for #name #type_generics #where_clause {
        //     fn as_mut(&mut self) -> &mut #fields_name #type_generics {
        //         &mut self.fields
        //     }
        // }

        // Deref/DerefMut to the Fields variant

        impl #impl_generics std::ops::Deref for #name #type_generics #where_clause {
            type Target = #fields_name #type_generics;

            fn deref(&self) -> &Self::Target {
                &self.fields
            }
        }

        impl #impl_generics std::ops::DerefMut for #name #type_generics #where_clause {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.fields
            }
        }

        // implement Il2CppClassData trait

        #[doc(hidden)]
        impl #impl_generics #ctx::Il2CppClassData for #name #type_generics #where_clause {
            const NAMESPACE: &'static str = #namespace;
            const CLASS: &'static str = #class;

            fn class() -> &'static #ctx::Il2CppClass {
                static CLASS_TYPE: #ctx::LazyLock<&'static mut #ctx::Il2CppClass> = #ctx::LazyLock::new(|| {
                    #ctx::Il2CppClass::from_name(#namespace, #class)
                        .expect(&format!("Failed to find class {}.{}", #namespace, #class))
                });

                &CLASS_TYPE
            }

            fn class_mut() -> &'static mut #ctx::Il2CppClass {
                Self::class().clone()
            }
        }

        #(
            impl #impl_generics #interfaces for #name #type_generics #where_clause { }
        )*
    }.into()  
}

// Remove the attributes on Generics
// TODO: Collect and return them
fn strip_generics_attrs(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.attrs.clear();
        }
    }
    generics
}

// TODO: Use the Deluxe parsers instead of doing it by hand.
// TODO: Remove the attributes after parsing them
fn process_attrs(attrs: &mut [Attribute]) -> Result<ClassAttributes> {
    let mut attributes = ClassAttributes::default();

    for attribute in attrs {
        if let Some(ident) = attribute.meta.path().get_ident() {
            match ident.to_string().as_str() {
                "static_fields" => {
                    attributes.static_type = Some(attribute.parse_args()?);
                },
                "interfaces" => {
                    for attr in attribute.parse_args_with(Punctuated::<TypePath, Token![,]>::parse_terminated).unwrap() {
                        attributes.interfaces.push(attr);
                    }
                },
                _ => (),
            }
        }
    }
    
    Ok(attributes)
}