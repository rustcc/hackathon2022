use std::str::FromStr;

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    punctuated::Punctuated, token::Comma, DataStruct, Error, Index, Lit, Member, MetaList,
    MetaNameValue, NestedMeta, Path, Result,
};

use crate::{fmt::member_to_ident, tools::lit_to_string};

use super::type_parameter_of_option;

pub struct ValidateItem {
    pub validate_type: ValidateType,
    pub message: Option<Lit>,
}

pub enum ValidateType {
    Null,
    Length(Lit, Lit),
    Range(Lit, Lit),
    Enumer(Path),
    Func(TokenStream),
    Inspector(Path),
    Phone,
    Email,
    Inner,
    Regex(Lit),
}

pub fn parse_prop_by_path(validate_item: &mut ValidateItem, path: &Path) -> Result<()> {
    if path.is_ident("phone") {
        validate_item.validate_type = ValidateType::Phone;
        return Ok(());
    }
    if path.is_ident("email") {
        validate_item.validate_type = ValidateType::Email;
        return Ok(());
    }
    if path.is_ident("inner") {
        validate_item.validate_type = ValidateType::Inner;
        return Ok(());
    }
    Ok(())
}
pub fn parse_prop_by_list(validate_item: &mut ValidateItem, list: &MetaList) -> Result<()> {
    if list.path.is_ident("length") {
        return parse_length(&list.nested, validate_item);
    }
    if list.path.is_ident("range") {
        return parse_range(&list.nested, validate_item);
    }
    if list.path.is_ident("enumer") {
        return parse_enumer(&list.nested, validate_item);
    }
    if list.path.is_ident("func") {
        return parse_func(&list.nested, validate_item);
    }
    if list.path.is_ident("inspector") {
        return parse_monitor(&list.nested, validate_item);
    }
    if list.path.is_ident("regex") {
        return parse_regex(&list.nested, validate_item);
    }
    Ok(())
}

fn parse_regex(
    nested: &Punctuated<NestedMeta, Comma>,
    validate_item: &mut ValidateItem,
) -> Result<()> {
    if nested.len() != 1 {
        return Err(Error::new_spanned(nested, "regex fmt is regex(\"**\")"));
    }
    let val = &nested[0];
    match val {
        NestedMeta::Lit(lit) => {
            validate_item.validate_type = ValidateType::Regex(lit.clone());
        }
        _ => return Err(Error::new_spanned(nested, "regex fmt is regex(\"**\")")),
    }
    Ok(())
}

fn parse_func(
    nested: &Punctuated<NestedMeta, Comma>,
    validate_item: &mut ValidateItem,
) -> Result<()> {
    if nested.len() != 1 {
        return Err(Error::new_spanned(nested, "func fmt is func(fn_name)"));
    }
    let val = &nested[0];
    let val = val.to_token_stream().to_string();
    let val = val.trim_matches('"');
    let val: TokenStream = TokenStream::from_str(val)?;
    validate_item.validate_type = ValidateType::Func(val);
    Ok(())
}

fn parse_monitor(
    nested: &Punctuated<NestedMeta, Comma>,
    validate_item: &mut ValidateItem,
) -> Result<()> {
    if nested.len() != 1 {
        return Err(Error::new_spanned(
            nested,
            "inspector fmt is inspector(ty_name)",
        ));
    }
    let val = &nested[0];
    match val {
        NestedMeta::Meta(inner) => match inner {
            syn::Meta::Path(p) => {
                validate_item.validate_type = ValidateType::Inspector(p.clone());
            }
            _ => {
                return Err(Error::new_spanned(
                    nested,
                    "inspector fmt is inspector(ty_name)",
                ))
            }
        },
        NestedMeta::Lit(_) => {
            return Err(Error::new_spanned(
                nested,
                "inspector fmt is inspector(ty_name)",
            ))
        }
    }
    Ok(())
}

fn parse_enumer(
    nested: &Punctuated<NestedMeta, Comma>,
    validate_item: &mut ValidateItem,
) -> Result<()> {
    if nested.len() != 1 {
        return Err(Error::new_spanned(nested, "enumer fmt is enumer(EnumType)"));
    }
    let val = &nested[0];
    match val {
        NestedMeta::Meta(inner) => match inner {
            syn::Meta::Path(p) => {
                validate_item.validate_type = ValidateType::Enumer(p.clone());
            }
            _ => return Err(Error::new_spanned(nested, "enumer fmt is enumer(EnumType)")),
        },
        NestedMeta::Lit(_) => {
            return Err(Error::new_spanned(nested, "enumer fmt is enumer(EnumType)"))
        }
    }
    Ok(())
}

fn parse_length(
    nested: &Punctuated<NestedMeta, Comma>,
    validate_item: &mut ValidateItem,
) -> Result<()> {
    if nested.len() != 2 {
        return Err(Error::new_spanned(nested, "length fmt is lenght(2,3)"));
    }
    let left = &nested[0];
    let left = match left {
        NestedMeta::Lit(inner) => inner.clone(),
        NestedMeta::Meta(_) => return Err(Error::new_spanned(nested, "length fmt is lenght(2,3)")),
    };
    let right = &nested[1];
    let right = match right {
        NestedMeta::Lit(inner) => inner.clone(),
        NestedMeta::Meta(_) => return Err(Error::new_spanned(nested, "length fmt is lenght(2,3)")),
    };
    validate_item.validate_type = ValidateType::Length(left, right);
    Ok(())
}

fn parse_range(
    nested: &Punctuated<NestedMeta, Comma>,
    validate_item: &mut ValidateItem,
) -> Result<()> {
    if nested.len() != 2 {
        return Err(Error::new_spanned(nested, "range fmt is range(2,3)"));
    }
    let left = &nested[0];
    let left = match left {
        NestedMeta::Lit(inner) => inner.clone(),
        NestedMeta::Meta(_) => return Err(Error::new_spanned(nested, "range fmt is range(2,3)")),
    };
    let right = &nested[1];
    let right = match right {
        NestedMeta::Lit(inner) => inner.clone(),
        NestedMeta::Meta(_) => return Err(Error::new_spanned(nested, "range fmt is range(2,3)")),
    };
    validate_item.validate_type = ValidateType::Range(left, right);
    Ok(())
}

pub fn parse_prop_by_nv(validate_item: &mut ValidateItem, nv: &MetaNameValue) -> Result<()> {
    if nv.path.is_ident("message") {
        validate_item.message = Some(nv.lit.clone());
        return Ok(());
    }
    Ok(())
}

pub fn get_validate_meta(input: &MetaList) -> Result<ValidateItem> {
    let mut validate_item = ValidateItem {
        validate_type: ValidateType::Null,
        message: None,
    };
    for item in input.nested.iter() {
        match item {
            syn::NestedMeta::Meta(inner) => match inner {
                syn::Meta::Path(p) => parse_prop_by_path(&mut validate_item, p)?,
                syn::Meta::List(l) => parse_prop_by_list(&mut validate_item, l)?,
                syn::Meta::NameValue(n) => parse_prop_by_nv(&mut validate_item, n)?,
            },
            syn::NestedMeta::Lit(_) => {
                return Err(Error::new_spanned(item, "validate input error"))
            }
        }
    }
    Ok(validate_item)
}

pub(crate) fn impl_struct(input: &syn::DeriveInput, data: &DataStruct) -> Result<TokenStream> {
    let ty = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut judge = quote!();
    for field in &data.fields {
        let ty = &field.ty;
        let ty_option = type_parameter_of_option(ty);
        let option = ty_option.is_some();

        let attrs = &field.attrs;
        for attr in attrs {
            if !attr.path.is_ident("validate") {
                continue;
            }
            let validate_item = attr_to_validate_item(attr, input)?;

            let field_ident = &field.ident;
            let field_name = match field_ident {
                Some(ident) => ident.to_string(),
                None => String::from(""),
            };
            let mut message = None;
            if let Some(input_message) = &validate_item.message {
                message = Some(lit_to_string(input_message));
            }
            let mut judge_snip = match judge_snap_token(&validate_item, message, field_name) {
                Some(value) => value,
                None => continue,
            };

            if option {
                judge_snip = quote! {
                    let v = &self.#field_ident;
                    if let Some(v) = v {
                        #judge_snip
                    }
                };
            } else {
                judge_snip = quote! {
                    let v = &self.#field_ident;
                    #judge_snip
                };
            }
            judge = quote! {
                #judge
                #judge_snip
            };
        }
    }
    let impl_validate = quote! {

        impl #impl_generics Validator for #ty #ty_generics #where_clause {
            fn validate(&self) -> Result<(), ValidateError> {
                #judge
                Ok(())
            }
        }

    };

    Ok(impl_validate)
}

fn attr_to_validate_item(attr: &syn::Attribute, input: &syn::DeriveInput) -> Result<ValidateItem> {
    let meta = attr.parse_meta()?;
    let validate_item = match &meta {
        syn::Meta::Path(_) => return Err(Error::new_spanned(input, "validate not supported Path")),
        syn::Meta::List(inner) => get_validate_meta(inner),
        syn::Meta::NameValue(_) => {
            return Err(Error::new_spanned(
                input,
                "validate not supported NameValue",
            ))
        }
    }?;
    Ok(validate_item)
}

fn judge_snap_token(
    validate_item: &ValidateItem,
    message: Option<String>,
    field_name: String,
) -> Option<TokenStream> {
    let judge_snip = match &validate_item.validate_type {
        ValidateType::Null => return None,
        ValidateType::Length(min, max) => {
            let message = match message {
                Some(message) => message,
                None => format!(
                    "#validate(length({}, {})): invalid value for {}",
                    lit_to_string(min),
                    lit_to_string(max),
                    field_name,
                ),
            };

            quote! {
                if v.len() < #min || v.len() > #max {
                    return Err(ValidateError::Invalid(#message.into()))
                }
            }
        }
        ValidateType::Range(min, max) => {
            let message = match message {
                Some(message) => message,
                None => format!(
                    "#validate(range({}, {})): invalid value for {}",
                    lit_to_string(min),
                    lit_to_string(max),
                    field_name,
                ),
            };

            quote! {
                if *v < #min || *v > #max {
                    return Err(ValidateError::Invalid(#message.into()))
                }
            }
        }
        ValidateType::Enumer(enumer) => {
            let message = match message {
                Some(message) => message,
                None => format!(
                    "#validate(enumer({})): invalid value for {}",
                    enumer.to_token_stream(),
                    field_name,
                ),
            };

            let path = enumer.segments.to_token_stream();
            quote! {
                if #path::enum_by(v).is_none() {
                    return Err(ValidateError::Invalid(#message.into()))
                }
            }
        }
        ValidateType::Func(func) => {
            let message = match message {
                Some(message) => message,
                None => format!(
                    "#validate(func({})): invalid value for {}",
                    func.to_token_stream(),
                    field_name,
                ),
            };

            quote! {
                if !#func (v) {
                    return Err(ValidateError::Invalid(#message.into()))
                }
            }
        }
        ValidateType::Phone => {
            let message = match message {
                Some(message) => message,
                None => format!("#validate(phone): invalid value for {}", field_name,),
            };

            quote! {
                if !is_mobile_phone (v) {
                    return Err(ValidateError::Invalid(#message.into()))
                }
            }
        }
        ValidateType::Email => {
            let message = match message {
                Some(message) => message,
                None => format!("#validate(email): invalid value for {}", field_name,),
            };

            quote! {
                if !is_email (v) {
                    return Err(ValidateError::Invalid(#message.into()))
                }
            }
        }
        ValidateType::Inner => {
            quote! {
                v.validate()?;
            }
        }
        ValidateType::Regex(regex) => {
            let message = match message {
                Some(message) => message,
                None => format!(
                    "#validate(regex({})): invalid value for {}",
                    lit_to_string(regex),
                    field_name,
                ),
            };

            quote! {
                if !regex (#regex, v) {
                    return Err(ValidateError::Invalid(#message.into()))
                }
            }
        }
        ValidateType::Inspector(verify) => {
            let message = match message {
                Some(message) => message,
                None => format!(
                    "#validate(inspector({})): invalid value for {}",
                    verify.to_token_stream(),
                    field_name,
                ),
            };

            let path = verify.segments.to_token_stream();
            quote! {
                if !#path::validate (v) {
                    return Err(ValidateError::Invalid(#message.into()))
                }
            }
        }
    };
    Some(judge_snip)
}

pub(crate) fn derive(input: &syn::DeriveInput) -> Result<TokenStream> {
    match &input.data {
        syn::Data::Struct(inner) => impl_struct(input, inner),
        syn::Data::Enum(inner) => impl_enum(input, inner),
        _ => Err(Error::new_spanned(
            input,
            "union as errors are not supported",
        )),
    }
}

fn impl_enum(input: &syn::DeriveInput, data: &syn::DataEnum) -> Result<TokenStream> {
    let ty = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let mut judge = quote!();

    for variant in &data.variants {
        if !variant.fields.is_empty() {
            let ident = &variant.ident;
            let mut members = vec![];
            let mut pat = quote! {};
            let mut named = false;
            let mut field_snip = quote! {};
            for (i, field) in (&variant.fields).into_iter().enumerate() {
                let ty = &field.ty;
                let ty_option = type_parameter_of_option(ty);
                let option = ty_option.is_some();

                let span = Span::call_site();
                let member = field.ident.clone().map(Member::Named).unwrap_or_else(|| {
                    Member::Unnamed(Index {
                        index: i as u32,
                        span,
                    })
                });
                let (member_token, name) = member_to_ident(&member);
                named = name;
                pat = quote! {
                    #pat #member_token,
                };
                members.push(member);

                let attrs = &field.attrs;
                for attr in attrs {
                    if !attr.path.is_ident("validate") {
                        continue;
                    }
                    let field_name = member_token.to_string();
                    let validate_item = attr_to_validate_item(attr, input)?;
                    let mut message = None;
                    if let Some(input_message) = &validate_item.message {
                        message = Some(lit_to_string(input_message));
                    }
                    let mut judge_snip = match judge_snap_token(&validate_item, message, field_name)
                    {
                        Some(value) => value,
                        None => continue,
                    };

                    if option {
                        judge_snip = quote! {
                            let v = #member_token;
                            if let Some(v) = v {
                                #judge_snip
                            }
                        };
                    } else {
                        judge_snip = quote! {
                            let v = #member_token;
                            #judge_snip
                        };
                    }
                    field_snip = quote! {
                        #field_snip
                        #judge_snip
                    };
                }
            }
            if named {
                pat = quote! {{#pat}}
            } else {
                pat = quote! {(#pat)}
            }
            judge = quote! {
                #judge
                #ty::#ident #pat => {#field_snip}
            };
        }
    }
    judge = quote! {
        #judge
        _ => return Ok(())
    };
    let impl_validate = quote! {

        impl #impl_generics Validator for #ty #ty_generics #where_clause {
            fn validate(&self) -> Result<(), ValidateError> {
                match self {
                    #judge
                };
                Ok(())
            }
        }

    };

    Ok(impl_validate)
}
