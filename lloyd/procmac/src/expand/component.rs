use crate::ast::{Input, Struct};
use crate::attr::Attrs;
use crate::expand::type_parameter_of_option;
use crate::fmt::member_to_ident;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, Error, Result};

use super::type_add_colon2;

pub struct ComponentFieldAttr<'a> {
    pub extension: bool,
    pub attr: Option<&'a Attribute>,
}

impl<'a> Attrs<'a> for ComponentFieldAttr<'a> {
    fn get(input: &'a [syn::Attribute]) -> Result<Self> {
        let mut expect_attr = ComponentFieldAttr {
            attr: None,
            extension: false,
        };
        for attr in input {
            if attr.path.is_ident("extension") {
                expect_attr.attr = Some(attr);
                expect_attr.extension = true;
            }
        }
        Ok(expect_attr)
    }

    fn span(&self) -> Option<proc_macro2::Span> {
        None
    }
}

pub fn derive(node: &DeriveInput) -> Result<TokenStream> {
    let input = Input::from_syn(node)?;
    Ok(match input {
        Input::Struct(input) => impl_struct(input),
        Input::Enum(_) => {
            return Err(Error::new_spanned(
                node,
                "component only struct are supported",
            ))
        }
    })
}

fn impl_struct<'a>(input: Struct<'a, ComponentFieldAttr<'a>>) -> TokenStream {
    let ty = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut item_statement = quote! {};
    let mut member_value = quote! {};
    let extension = input.attrs.extension;
    for field in input.fields {
        let field_ty = field.ty;
        let field_ty = &mut field_ty.clone();
        type_add_colon2(field_ty);
        let ty_option = type_parameter_of_option(field_ty);
        let option = ty_option.is_some();
        let member = member_to_ident(&field.member);
        let member_ident = member.0;
        if field.attrs.extension || extension {
            if option {
                item_statement = quote! {
                    #item_statement
                    let #member_ident = extensions.get::<#ty_option>();
                };
            } else {
                item_statement = quote! {
                    #item_statement
                    let #member_ident = extensions.get::<#field_ty>()
                        .ok_or_else(|| {
                            Error::MissExtension(format!(
                                "Extension of type `{}` was not found. Perhaps you forgot to add it? See `axum::Extension`.",
                                std::any::type_name::<#field_ty>()
                            ))
                        })
                        .map(|x| x.clone())?;
                };
            }
        } else {
            item_statement = quote! {
                #item_statement
                let #member_ident = #field_ty::injection(extensions)?;
            };
        }
        member_value = quote! {
            #member_value
            #member_ident
        }
    }
    let result = quote! {
        impl #impl_generics ComponentTrait for #ty #ty_generics #where_clause {

            fn injection(extensions: &Extensions) -> Result<Self, Error> {
                #item_statement
                Ok(Self {
                    #member_value
                })
            }
        }
    };
    result
}
