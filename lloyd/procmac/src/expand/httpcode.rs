use crate::ast::{Enum, Input};
use crate::attr::{parse_token_expr, Attrs, Display, Transparent};
use crate::fmt::member_to_ident;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeSet as Set;
use syn::parse::ParseStream;
use syn::{Attribute, DeriveInput, Error, LitInt, LitStr, Result};

pub struct CodeAttrs<'a> {
    pub inherit: Option<Transparent<'a>>,
    pub input: Option<CodeValue<'a>>,
    pub from: Option<&'a Attribute>,
}

#[derive(Clone)]
pub struct CodeValue<'a> {
    pub status: LitInt,
    pub code: LitStr,
    pub display: Display<'a>,
}

impl<'a> Attrs<'a> for CodeAttrs<'a> {
    fn get(input: &'a [syn::Attribute]) -> Result<Self> {
        let mut code_attr = CodeAttrs {
            inherit: None,
            input: None,
            from: None,
        };
        for attr in input {
            if attr.path.is_ident("code") {
                parse_code_attribute(&mut code_attr, attr)?;
            }
            if attr.path.is_ident("from") {
                parse_from_attribute(&mut code_attr, attr)?;
            }
        }
        Ok(code_attr)
    }

    fn span(&self) -> Option<proc_macro2::Span> {
        if let Some(input) = &self.input {
            Some(input.display.fmt.span())
        } else if let Some(transparent) = &self.inherit {
            Some(transparent.span)
        } else {
            None
        }
    }
}

fn parse_from_attribute<'a>(code_attr: &mut CodeAttrs<'a>, attr: &'a Attribute) -> Result<()> {
    if !attr.tokens.is_empty() {
        return Ok(());
    }
    if code_attr.from.is_some() {
        return Err(Error::new_spanned(attr, "duplicate #[from] attribute"));
    }
    code_attr.from = Some(attr);
    Ok(())
}

fn parse_code_attribute<'a>(code_attr: &mut CodeAttrs<'a>, attr: &'a syn::Attribute) -> Result<()> {
    syn::custom_keyword!(transparent);
    attr.parse_args_with(|input: ParseStream| {
        if let Some(kw) = input.parse::<Option<transparent>>()? {
            if code_attr.inherit.is_some() {
                return Err(Error::new_spanned(
                    attr,
                    "duplicate #[code(transparent)] attribute",
                ));
            }
            code_attr.inherit = Some(Transparent {
                original: attr,
                span: kw.span,
            });
            return Ok(());
        }

        let code_meta = &attr.parse_meta()?;
        let status;
        let code;
        let tips;
        match code_meta {
            syn::Meta::Path(_) => {
                return Err(Error::new_spanned(
                    attr,
                    "#[code(...)] Path style not support",
                ))
            }
            syn::Meta::List(val) => {
                match &val.nested[0] {
                    syn::NestedMeta::Meta(_) => {
                        return Err(Error::new_spanned(
                            attr,
                            "#[code(status..)] Meta style not support",
                        ))
                    }
                    syn::NestedMeta::Lit(data_lit) => {
                        if let syn::Lit::Int(data) = data_lit {
                            status = data;
                        } else {
                            return Err(Error::new_spanned(
                                attr,
                                "#[code(status..)] value type not support",
                            ));
                        }
                    }
                }
                match &val.nested[1] {
                    syn::NestedMeta::Meta(_) => {
                        return Err(Error::new_spanned(
                            attr,
                            "#[code(.code.)] Meta style not support",
                        ))
                    }
                    syn::NestedMeta::Lit(data_lit) => {
                        if let syn::Lit::Str(data) = data_lit {
                            code = data;
                        } else {
                            return Err(Error::new_spanned(
                                attr,
                                "#[code(.code.)] value type not support",
                            ));
                        }
                    }
                }
                match &val.nested[2] {
                    syn::NestedMeta::Meta(_) => {
                        return Err(Error::new_spanned(
                            attr,
                            "#[code(..tips)] Meta style not support",
                        ))
                    }
                    syn::NestedMeta::Lit(data_lit) => {
                        if let syn::Lit::Str(data) = data_lit {
                            tips = data;
                        } else {
                            return Err(Error::new_spanned(
                                attr,
                                "#[code(..tips)] value type not support",
                            ));
                        }
                    }
                }
            }
            syn::Meta::NameValue(_) => {
                return Err(Error::new_spanned(
                    attr,
                    "#[code(...)] NameValue style not support",
                ))
            }
        }

        let display = Display {
            original: attr,
            fmt: tips.clone(),
            args: parse_token_expr(input, false)?,
            has_bonus_display: false,
            implied_bounds: Set::new(),
            named_args: Vec::new(),
        };
        if code_attr.input.is_some() {
            return Err(Error::new_spanned(
                attr,
                "only one #[code(...)] attribute is allowed",
            ));
        }
        let code_value = CodeValue {
            status: status.clone(),
            code: code.clone(),
            display,
        };
        code_attr.input = Some(code_value);
        Ok(())
    })
}

pub fn derive(node: &DeriveInput) -> Result<TokenStream> {
    let input = Input::from_syn(node)?;
    Ok(match input {
        Input::Struct(_) => {
            return Err(Error::new_spanned(node, "HttpCode not support struct type"))
        }
        Input::Enum(input) => impl_enum(input),
    })
}

fn impl_enum<'a>(input: Enum<'a, CodeAttrs<'a>>) -> TokenStream {
    let ty = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut status_snip = quote! {};
    let mut code_snip = quote! {};
    let mut tips_snip = quote! {};
    for variant in input.variants {
        let ident = variant.ident;
        let mut pat = quote! {};
        let mut from_member = quote! {};
        let mut members = vec![];
        if !variant.fields.is_empty() {
            let mut named = false;
            for field in &variant.fields {
                let member = field.member.clone();
                let (member_token, name) = member_to_ident(&member);
                named = name;
                if field.attrs.from.is_some() {
                    from_member = quote! {#member_token};
                }
                pat = quote! {
                    #pat #member_token,
                };
                members.push(member);
            }
            if named {
                pat = quote! {{#pat}}
            } else {
                pat = quote! {(#pat)}
            }
        } else {
            pat = quote! {{}};
        }
        let mut code_attr = variant.attrs;
        if code_attr.inherit.is_none() && code_attr.input.is_none() {
            code_attr.inherit = input.attrs.inherit;
            code_attr.input = input.attrs.input.clone();
        }
        let mut status_ret = quote! {};
        let mut code_ret = quote! {};
        let mut tips_ret = quote! {};
        if let Some(code_input) = code_attr.input {
            let status = code_input.status;
            status_ret = quote! {#status};

            let code = code_input.code;
            code_ret = quote! {#code.to_string()};

            let mut display = code_input.display;
            display.expand_shorthand(&members);

            let fmt = display.fmt;

            let named_args = display.named_args;
            tips_ret = quote! {format!(#fmt, #(#named_args),*)};
        } else if let Some(_code_inherit) = code_attr.inherit {
            status_ret = quote! {#from_member.status()};
            code_ret = quote! {#from_member.code()};
            tips_ret = quote! {#from_member.tips()};
        }
        status_snip = quote! {
            #status_snip
            #ty::#ident #pat => {#status_ret},
        };
        code_snip = quote! {
            #code_snip
            #ty::#ident #pat => {#code_ret},
        };
        tips_snip = quote! {
            #tips_snip
            #ty::#ident #pat => {#tips_ret},
        };
    }
    let result = quote! {
       impl #impl_generics HttpCode for #ty #ty_generics #where_clause {
            fn status(&self) -> u16 {
                match self {
                    #status_snip
                }
            }
            fn code(&self) -> String {
                match self {
                    #code_snip
                }
            }
            fn tips(&self) -> String {
                match self {
                    #tips_snip
                }
            }
       }
    };
    result
}
