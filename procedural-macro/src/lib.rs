use quote::{format_ident, quote};
use syn::spanned::Spanned;

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input: syn::DeriveInput = syn::parse_macro_input!(input);

    let name = &derive_input.ident;
    let builder_name = format_ident!("{}Builder", name);
    let fields = get_fields(&derive_input);

    if let Some(error_messages) = parse_errors(&fields) {
        return error_messages.into();
    }

    let struct_body: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .map(|x| {
            let name = x.name;
            let ty = x.ty;
            if x.repeated
                .as_ref()
                .expect("Unexpected repeated_name error")
                .is_none()
            {
                quote! {
                    #name: std::option::Option<#ty>
                }
            } else {
                quote! {
                    #name: #ty
                }
            }
        })
        .collect();

    let initializers: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .map(|x| {
            let name = x.name;
            let initial_value = if x
                .repeated
                .as_ref()
                .expect("Unexpected repeated_name error")
                .is_none()
            {
                quote! {None}
            } else {
                quote! {vec![]}
            };
            quote! {
                #name: #initial_value
            }
        })
        .collect();

    let setters = gen_setters(&fields);

    let build_fn = gen_build_fn(name, &fields);

    let gen = quote! {
        struct #builder_name {
            #(#struct_body),*
        }

        impl #name {
            fn builder() -> #builder_name {
                #builder_name {
                    #(#initializers),*
                }
            }
        }

        impl #builder_name {
            #setters

            #build_fn
        }
    };

    gen.into()
}

struct FieldRepr<'a> {
    name: &'a syn::Ident,
    ty: &'a syn::Type,
    optional: bool,
    repeated: Result<Option<syn::Ident>, syn::Error>,
}

fn get_fields(derive_input: &syn::DeriveInput) -> Vec<FieldRepr> {
    let fields = {
        if let syn::Data::Struct(data_struct) = &derive_input.data {
            if let syn::Fields::Named(fields) = &data_struct.fields {
                &fields.named
            } else {
                panic!("Only structs with named fields are supported");
            }
        } else {
            panic!("Only non-empty structs are supported");
        }
    };

    fields
        .iter()
        .map(|x| FieldRepr {
            name: x.ident.as_ref().expect("Expected identifier"),
            ty: match optional(&x.ty) {
                true => extract_optional(&x.ty).expect("Expected Option Type"),
                false => &x.ty,
            },
            optional: optional(&x.ty),
            repeated: parse_repeated(&x.attrs),
        })
        .collect()
}

fn parse_errors(fields: &[FieldRepr]) -> Option<proc_macro2::TokenStream> {
    let error_messages: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .filter(|field| field.repeated.is_err())
        .map(|field| {
            field
                .repeated
                .as_ref()
                .expect_err("Expected repeated_name error")
                .to_compile_error()
        })
        .collect();
    if error_messages.is_empty() {
        None
    } else {
        Some(quote! {
            #(#error_messages)*
        })
    }
}

fn parse_repeated(attrs: &[syn::Attribute]) -> Result<Option<syn::Ident>, syn::Error> {
    for attr in attrs.iter() {
        if let Ok(syn::Meta::List(meta_list)) = attr.parse_meta() {
            if meta_list.path.is_ident("builder") {
                for nested_meta in meta_list.nested.iter() {
                    if let syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) = nested_meta {
                        if name_value.path.is_ident("each") {
                            if let syn::Lit::Str(lit_str) = &name_value.lit {
                                return Ok(Some(format_ident!("{}", lit_str.value())));
                            }
                        } else {
                            return Err(syn::Error::new(
                                attr.tokens.span(),
                                "expected `builder(each = \"...\")`",
                            ));
                        }
                    }
                }
            }
        }
    }

    Ok(None)
}

fn optional(full_ty: &syn::Type) -> bool {
    match full_ty {
        syn::Type::Path(t) => match t.path.segments.first() {
            Some(t) => t.ident == "Option",
            _ => false,
        },
        _ => false,
    }
}

fn extract_optional(t: &syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Path(t) = t {
        if let Some(t) = t.path.segments.first() {
            if let syn::PathArguments::AngleBracketed(t) = &t.arguments {
                if let Some(syn::GenericArgument::Type(t)) = t.args.first() {
                    return Some(t);
                }
            }
        }
    }

    None
}

fn gen_setters(fields: &[FieldRepr]) -> proc_macro2::TokenStream {
    let setter_functions = fields
        .iter()
        .map(|field| {
            let name = field.name;
            if let Some(repeated_name) = field
                .repeated
                .as_ref()
                .expect("Unexpected repeated_name error")
                .as_ref()
            {
                let ty = extract_optional(field.ty).expect("Expected vector type");
                quote! {
                    fn #repeated_name(&mut self, x: #ty) -> &mut Self{
                        self.#name.push(x);
                        self
                    }
                }
            } else {
                let ty = field.ty;
                quote! {
                    fn #name(&mut self, x: #ty) -> &mut Self{
                        self.#name = std::option::Option::Some(x);
                        self
                    }
                }
            }
        })
        .collect::<Vec<proc_macro2::TokenStream>>();

    quote! {
        #(#setter_functions)*
    }
}

fn gen_build_fn(struct_name: &syn::Ident, fields: &[FieldRepr]) -> proc_macro2::TokenStream {
    let field_assignments: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .map(|field| {
            let name = field.name;
            let error = format!("Field '{}' is not initialized.", name);
            if !field.optional
                && field
                    .repeated
                    .as_ref()
                    .expect("Unexpected repeated_name error")
                    .is_none()
            {
                quote! {
                    #name: self.#name.take().ok_or(#error)?
                }
            } else if field
                .repeated
                .as_ref()
                .expect("Unexpected repeated_name error")
                .is_none()
            {
                quote! {
                    #name: self.#name.take()
                }
            } else {
                quote! {
                    #name: self.#name.clone()
                }
            }
        })
        .collect();

    quote! {
        fn build(&mut self) -> std::result::Result<#struct_name, std::boxed::Box<dyn std::error::Error>> {
            std::result::Result::Ok(#struct_name {
                #(#field_assignments),*
            })
        }
    }
}
