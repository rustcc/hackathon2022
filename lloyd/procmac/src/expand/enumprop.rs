use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Data, Error, Lit, Result};

pub(crate) fn derive(input: &syn::DeriveInput) -> Result<TokenStream> {
    let ty = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let data = match &input.data {
        Data::Enum(data) => data,
        _ => return Err(Error::new_spanned(input, "only enum are supported")),
    };
    let prop_ident = Ident::new(format!("{}Prop", ty).as_str(), Span::call_site());

    let mut member_token = quote!();
    let mut value_token = quote!();
    let mut exist_prop = false;
    let mut exist_default = false;
    for var in &data.variants {
        let attrs = &var.attrs;
        let ident = &var.ident;
        let mut members_value = quote!();
        let mut set_value = false;
        for attr in attrs {
            if !attr.path.is_ident("prop") {
                continue;
            }
            set_value = true;
            let attr_meta = attr.parse_meta()?;
            let prop_meta = match attr_meta {
                syn::Meta::List(p) => p,
                _ => return Err(Error::new_spanned(input, "#[prop] format error]")),
            };
            for field in prop_meta.nested {
                let field = match field {
                    syn::NestedMeta::Meta(m) => m,
                    syn::NestedMeta::Lit(_) => {
                        return Err(Error::new_spanned(input, "#[prop] format error]"))
                    }
                };
                let field = match field {
                    syn::Meta::NameValue(nv) => nv,
                    _ => return Err(Error::new_spanned(input, "#[prop] format error]")),
                };

                let member_item = field.path.to_token_stream();
                let member_value;
                match field.lit {
                    Lit::Float(v) => {
                        if !exist_prop {
                            member_token = quote! {
                                #member_token
                                #member_item: f64,
                            };
                        }

                        member_value = v.to_token_stream();
                    }
                    Lit::Str(v) => {
                        if !exist_prop {
                            member_token = quote! {
                                #member_token
                                #member_item: String,
                            };
                        }
                        member_value = quote!(#v.into());
                    }
                    Lit::Int(v) => {
                        if !exist_prop {
                            member_token = quote! {
                                #member_token
                                #member_item: i64,
                            };
                        }
                        member_value = v.to_token_stream();
                    }
                    Lit::Bool(v) => {
                        if !exist_prop {
                            member_token = quote! {
                                #member_token
                                #member_item: bool,
                            };
                        }
                        member_value = v.to_token_stream();
                    }
                    _ => return Err(Error::new_spanned(input, "#[prop] format error]")),
                }
                members_value = quote! {
                    #members_value
                    #member_item: #member_value,
                };
            }
            exist_prop = true;
        }
        if !set_value {
            exist_default = true;
        } else {
            value_token = quote! {
                #value_token
                #ty::#ident {..} => #prop_ident {
                    #members_value
                },
            };
        }
    }
    if exist_default {
        value_token = quote! {
            #value_token
            _ => #prop_ident {
                ..Default::default()
            },
        };
    }
    let result = quote! {

        #[derive(Default, Debug)]
        pub struct #prop_ident {
            #member_token
        }

        impl #impl_generics #ty #ty_generics #where_clause {
            pub fn prop(&self) -> #prop_ident {
                match self {
                    #value_token
                }
            }
        }
    };

    Ok(result)
}
