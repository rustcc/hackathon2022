use crate::ast::{Input, Struct};
use crate::attr::Attrs;
use crate::fmt::member_to_ident;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, Error, Meta, NestedMeta, Result};

use super::apidoc::{parse_serde_attribute, serde_field_name, Serde};
use super::{type_add_colon2, type_parameter_of_option};

pub struct MultipartFieldAttr<'a> {
    pub attr: Option<&'a Attribute>,
    pub serde: Serde,
    pub method: Option<ConvertMethod>,
}

pub enum ConvertMethod {
    Text,
    Deserialize,
    Parse,
    Custom,
}

impl<'a> Attrs<'a> for MultipartFieldAttr<'a> {
    fn get(input: &'a [syn::Attribute]) -> Result<Self> {
        let mut expect_attr = MultipartFieldAttr {
            attr: None,
            serde: Serde {
                rename: None,
                serialize: None,
                deserialize: None,
                default: false,
                deserialize_rule: None,
                flatten: false,
                serialize_rule: None,
                skip_deserializing: false,
                skip_serializing: false,
                tag: None,
                content: None,
            },
            method: None,
        };
        for attr in input {
            if attr.path.is_ident("serde") {
                parse_serde_attribute(&mut expect_attr.serde, attr)?;
            } else if attr.path.is_ident("method") {
                expect_attr.method = parse_method_attribute(attr)?;
            }
        }
        Ok(expect_attr)
    }

    fn span(&self) -> Option<proc_macro2::Span> {
        None
    }
}

fn parse_method_attribute(attr: &Attribute) -> Result<Option<ConvertMethod>> {
    let meta = attr.parse_meta()?;
    if let Meta::List(list) = meta {
        let method = &list.nested[0];
        if let NestedMeta::Meta(inner) = method {
            match inner {
                Meta::Path(inner) => {
                    if inner.is_ident("text") {
                        return Ok(Some(ConvertMethod::Text));
                    }
                    if inner.is_ident("custom") {
                        return Ok(Some(ConvertMethod::Custom));
                    }
                    if inner.is_ident("deserialize") {
                        return Ok(Some(ConvertMethod::Deserialize));
                    }
                    if inner.is_ident("parse") {
                        return Ok(Some(ConvertMethod::Parse));
                    }
                    return Err(Error::new_spanned(attr, "method not support"));
                }
                _ => {
                    return Err(Error::new_spanned(
                        attr,
                        "FromMultipart only support path style",
                    ))
                }
            }
        }
    }
    Err(Error::new_spanned(
        attr,
        "FromMultipart only support path style",
    ))
}

pub fn derive(node: &DeriveInput) -> Result<TokenStream> {
    let input = Input::from_syn(node)?;
    Ok(match input {
        Input::Struct(input) => impl_struct(input),
        Input::Enum(_) => return Err(Error::new_spanned(node, "Multipart not support enum type")),
    })
}

fn impl_struct<'a>(input: Struct<'a, MultipartFieldAttr<'a>>) -> TokenStream {
    let ty = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let mut define_token = quote! {};
    let mut value_token = quote! {};
    let mut new_self_token = quote! {};
    for field in input.fields {
        let field_ty = field.ty;
        let field_ty = &mut field_ty.clone();
        type_add_colon2(field_ty);

        let ty_option = type_parameter_of_option(field_ty);
        let option = ty_option.is_some();
        let member = member_to_ident(&field.member);
        let field_ident = member.0;
        let origin_name = field_ident.to_string();
        let (_, deserialize) = serde_field_name(
            origin_name.as_str(),
            &input.attrs.serde.serialize_rule,
            &input.attrs.serde.deserialize_rule,
            &field.attrs.serde,
        );
        define_token = quote! {
            #define_token
            let mut #field_ident = None;
        };

        let set_token;
        match field.attrs.method {
            Some(method) => match method {
                ConvertMethod::Text => {
                    set_token = quote! {
                        #field_ident = Some(field.text().await?);
                    };
                }
                ConvertMethod::Deserialize => {
                    set_token = quote! {
                        let text = field.text().await?;
                        #field_ident = Some(serde_json::from_str::<#field_ty>(text.as_str())?);
                    };
                }
                ConvertMethod::Parse => {
                    set_token = quote! {
                        let text = field.text().await?;
                        #field_ident = Some(text.parse().map_err(|e| Error::BadRequest(Some(Box::new(e))))?);
                    };
                }
                ConvertMethod::Custom => {
                    set_token = quote! {
                        #field_ident = Some(#field_ty::from_field(field).await?);
                    };
                }
            },
            None => {
                set_token = quote! {
                    #field_ident = Some(#field_ty::from_field(field).await?);
                };
            }
        }
        value_token = quote! {
            #value_token
            #deserialize => {
                #set_token
            }
        };

        if option {
            new_self_token = quote! {
                #new_self_token
                #field_ident: match #field_ident {
                    Some(inner) => inner,
                    None => None,
                },
            }
        } else {
            new_self_token = quote! {
                #new_self_token
                #field_ident: match #field_ident {
                    Some(inner) => inner,
                    None => return Err(Error::MissField(#deserialize.into())),
                },
            }
        }
    }
    new_self_token = quote! {
        Self {
            #new_self_token
        }
    };
    let result = quote! {
        #[async_trait]
        impl #impl_generics FromMultipart for #ty #ty_generics #where_clause {
            async fn from_multipart(
                mut multipart: Multipart
            ) -> Result<Self, Error> {
                #define_token
                loop {
                    let field = multipart.next_field().await?;
                    if let Some(field) = field {
                        if let Some(__name) = field.name() {
                            match __name {
                                #value_token
                                _ => {
                                    continue;
                                }
                            }
                        }
                    } else {
                        break;
                    }
                }
                Ok(#new_self_token)
            }
        }
    };

    result
}
