use std::str::FromStr;

use crate::ast::{Enum, Input, Struct};
use crate::attr::Attrs;
use crate::fmt::member_to_ident;
use crate::tools::lit_to_string;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    Attribute, Data, DeriveInput, Error, Fields, ItemFn, Lit, LitStr, Meta, NestedMeta, Result,
    ReturnType,
};
use tools::case::RenameRule;

use super::validator::{get_validate_meta, ValidateItem};
use super::{nv_attr_to_bool_value, nv_attr_to_str_value, ret_type_add_colon2, type_add_colon2};

pub struct ApiFieldAttr<'a> {
    pub note: Option<LitStr>,
    pub attr: Option<&'a Attribute>,
    pub serde: Serde,
    pub validate: Vec<ValidateItem>,
    pub doc_ty: Option<TokenStream>,
}

pub struct Serde {
    // #[serde(rename_all(serialize = "lowercase"))]
    pub serialize_rule: Option<RenameRule>,

    // #[serde(rename_all(deserialize = "lowercase"))]
    pub deserialize_rule: Option<RenameRule>,

    // #[serde(skip_serializing)]
    pub skip_serializing: bool,

    // #[serde(skip_deserializing)]
    pub skip_deserializing: bool,

    // #[serde(rename = "name")]
    pub rename: Option<String>,

    // #[serde(rename(serialize = "ser_name"))]
    pub serialize: Option<String>,

    // #[serde(rename(deserialize = "de_name"))]
    pub deserialize: Option<String>,

    // #[serde(tag = "t")]
    pub tag: Option<String>,

    // #[serde(content = "c")]
    pub content: Option<String>,

    // #[serde(default)]
    pub default: bool,

    // #[serde(flatten)]
    pub flatten: bool,
}

impl<'a> Attrs<'a> for ApiFieldAttr<'a> {
    fn get(input: &'a [syn::Attribute]) -> Result<Self> {
        let mut expect_attr = ApiFieldAttr {
            note: None,
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
            validate: vec![],
            doc_ty: None,
        };
        for attr in input {
            if attr.path.is_ident("note") {
                if expect_attr.note.is_some() {
                    return Err(Error::new_spanned(attr, "duplicate #[note] attribute"));
                }
                parse_api_attribute(&mut expect_attr, attr)?;
            } else if attr.path.is_ident("serde") {
                parse_serde_attribute(&mut expect_attr.serde, attr)?;
            } else if attr.path.is_ident("validate") {
                let validate_item = parse_validate_attribute(attr)?;
                expect_attr.validate.push(validate_item);
            }
        }
        Ok(expect_attr)
    }

    fn span(&self) -> Option<proc_macro2::Span> {
        None
    }
}

pub fn parse_validate_attribute(attr: &Attribute) -> Result<ValidateItem> {
    let meta = attr.parse_meta()?;
    let validate_item = match meta {
        syn::Meta::Path(p) => return Err(Error::new_spanned(p, "validate not supported Path")),
        syn::Meta::List(inner) => get_validate_meta(&inner),
        syn::Meta::NameValue(n) => {
            return Err(Error::new_spanned(n, "validate not supported NameValue"))
        }
    }?;
    Ok(validate_item)
}

pub fn parse_serde_attribute(serde: &mut Serde, attr: &syn::Attribute) -> Result<()> {
    let meta = attr.parse_meta()?;

    if let Meta::List(list) = meta {
        for meta in list.nested {
            if let NestedMeta::Meta(meta) = &meta {
                match meta {
                    Meta::Path(path) => {
                        if path.is_ident("default") {
                            serde.default = true;
                            continue;
                        }
                        if path.is_ident("skip") {
                            serde.skip_deserializing = true;
                            serde.skip_serializing = true;
                            continue;
                        }
                        if path.is_ident("skip_deserializing") {
                            serde.skip_deserializing = true;
                            continue;
                        }
                        if path.is_ident("skip_serializing") {
                            serde.skip_serializing = true;
                            continue;
                        }
                        if path.is_ident("flatten") {
                            serde.flatten = true;
                            continue;
                        }
                    }
                    Meta::List(list) => {
                        if list.path.is_ident("rename") {
                            for i in 0..list.nested.len() {
                                let meta = &list.nested[i];
                                if let NestedMeta::Meta(Meta::NameValue(nv)) = meta {
                                    if nv.path.is_ident("serialize") {
                                        let serialize = nv_attr_to_str_value(nv)?;
                                        serde.serialize = Some(serialize);
                                    } else if nv.path.is_ident("deserialize") {
                                        let deserialize = nv_attr_to_str_value(nv)?;
                                        serde.deserialize = Some(deserialize);
                                    }
                                }
                            }
                            continue;
                        }
                        if list.path.is_ident("rename_all") {
                            for rule in &list.nested {
                                if let NestedMeta::Meta(Meta::NameValue(nv)) = rule {
                                    let rule = nv_attr_to_str_value(nv)?;
                                    let name_rule = RenameRule::from_str(&rule);
                                    match name_rule {
                                        Ok(r) => {
                                            if nv.path.is_ident("serialize") {
                                                serde.serialize_rule = Some(r);
                                            } else if nv.path.is_ident("deserialize") {
                                                serde.deserialize_rule = Some(r);
                                            }
                                        }
                                        Err(_) => {
                                            return Err(Error::new_spanned(
                                                attr,
                                                "rename rule not exist",
                                            ))
                                        }
                                    }
                                }
                            }
                            continue;
                        }
                    }
                    Meta::NameValue(nv) => {
                        if nv.path.is_ident("rename") {
                            let rename = nv_attr_to_str_value(nv)?;
                            serde.rename = Some(rename);
                            continue;
                        }
                        if nv.path.is_ident("rename_all") {
                            let name_rule = nv_attr_to_str_value(nv)?;
                            let name_rule = RenameRule::from_str(&name_rule);
                            match name_rule {
                                Ok(r) => {
                                    serde.serialize_rule = Some(r);
                                    serde.deserialize_rule = Some(r);
                                }
                                Err(_) => {
                                    return Err(Error::new_spanned(attr, "rename rule not exist"))
                                }
                            }
                            continue;
                        }
                        if nv.path.is_ident("tag") {
                            let tag = nv_attr_to_str_value(nv)?;
                            serde.tag = Some(tag);
                            continue;
                        }
                        if nv.path.is_ident("content") {
                            let content = nv_attr_to_str_value(nv)?;
                            serde.content = Some(content);
                            continue;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn parse_api_attribute<'a>(
    field_attr: &mut ApiFieldAttr<'a>,
    attr: &'a syn::Attribute,
) -> Result<()> {
    let meta = attr.parse_meta()?;
    if let Meta::List(list) = meta {
        let note = &list.nested[0];
        if list.nested.len() > 1 {
            let ty = &list.nested[1];
            let ty = ty.to_token_stream().to_string();
            let ty = ty.trim_matches('"');
            let ty: TokenStream = TokenStream::from_str(ty)?;
            field_attr.doc_ty = Some(ty);
        }
        if let NestedMeta::Lit(Lit::Str(val)) = note {
            field_attr.note = Some(val.clone());
            field_attr.attr = Some(attr);
            return Ok(());
        }
    }
    Err(Error::new_spanned(attr, "ApiModel only support lit style"))
}

pub fn derive(node: &DeriveInput) -> Result<TokenStream> {
    let input = Input::from_syn(node)?;
    Ok(match input {
        Input::Struct(input) => impl_struct(input),
        Input::Enum(input) => impl_enum(input),
    })
}

fn impl_enum<'a>(input: Enum<'a, ApiFieldAttr<'a>>) -> TokenStream {
    let enum_ty = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut vec_field = quote! {};
    let mut grow_model = quote! {};
    let model_note = input
        .attrs
        .note
        .map(|f| f.value().trim().to_string())
        .unwrap_or_default();
    let mut api_ty_quote = quote!(ApiFieldType::Enumer);

    if let Some(tag) = input.attrs.serde.tag {
        let content = input.attrs.serde.content.unwrap_or_default();
        api_ty_quote = quote!(ApiFieldType::TagEnumer {tag: #tag.into(), content: #content.into()});
    }
    for variant in input.variants {
        let origin_name = variant.ident.to_string();
        let (serialize, deserialize) = serde_field_name(
            origin_name.as_str(),
            &input.attrs.serde.serialize_rule,
            &input.attrs.serde.deserialize_rule,
            &variant.attrs.serde,
        );
        let note = variant.attrs.note;
        let note = note
            .map(|f| f.value().trim().to_string())
            .unwrap_or_default();
        let fields_size = variant.fields.len();
        let validate_quote = validate_token_stream(&variant.attrs.validate);
        let mut exist_inner = false;
        let member_quote;
        let mut inner_model_ty = quote! {ApiFieldType::Object};
        let skip_serialize = variant.attrs.serde.skip_serializing;
        let skip_deserialize = variant.attrs.serde.skip_deserializing;
        let mut serialize_quote = quote!(None);
        let mut deserialize_quote = quote!(None);
        match &variant.original.fields {
            syn::Fields::Named(_) => {
                if fields_size == 0 {
                    if !skip_serialize {
                        serialize_quote = quote! {
                            Some(ApiField {
                                name: #serialize.into(),
                                ty: ApiFieldType::Object,
                                inner: None,
                                option: false,
                            })
                        };
                    }
                    if !skip_deserialize {
                        deserialize_quote = quote! {
                            Some(ApiField {
                                name: #deserialize.into(),
                                ty: ApiFieldType::Object,
                                inner: None,
                                option: false,
                            })
                        };
                    }
                    member_quote = quote! {
                        ApiMember {
                            validate: #validate_quote,
                            note: #note.into(),
                            default: false,
                            serialize: #serialize_quote,
                            deserialize: #deserialize_quote,
                        }
                    };
                } else {
                    if !skip_serialize {
                        serialize_quote = quote! {
                            Some(ApiField {
                                name: #serialize.into(),
                                ty: ApiFieldType::Object,
                                inner: Some(format!("{}::{}", model_id, #origin_name)),
                                option: false,
                            })
                        };
                    }
                    if !skip_deserialize {
                        deserialize_quote = quote! {
                            Some(ApiField {
                                name: #deserialize.into(),
                                ty: ApiFieldType::Object,
                                inner: Some(format!("{}::{}", model_id, #origin_name)),
                                option: false,
                            })
                        };
                    }
                    member_quote = quote! {
                        ApiMember {
                            validate: #validate_quote,
                            note: #note.into(),
                            default: false,
                            serialize: #serialize_quote,
                            deserialize: #deserialize_quote,
                        }
                    };
                    exist_inner = true;
                }
            }
            syn::Fields::Unnamed(_) => {
                if fields_size == 0 {
                    if !skip_serialize {
                        serialize_quote = quote! {
                            Some(ApiField {
                                name: #serialize.into(),
                                ty: ApiFieldType::EmptyArray,
                                inner: None,
                                option: false,
                            })
                        };
                    }
                    if !skip_deserialize {
                        deserialize_quote = quote! {
                            Some(ApiField {
                                name: #deserialize.into(),
                                ty: ApiFieldType::EmptyArray,
                                inner: None,
                                option: false,
                            })
                        };
                    }
                    member_quote = quote! {
                        ApiMember {
                            validate: #validate_quote,
                            note: #note.into(),
                            default: false,
                            serialize: #serialize_quote,
                            deserialize: #deserialize_quote,
                        }
                    };
                } else if fields_size == 1 {
                    let mut ty = variant.fields[0].ty.clone();
                    type_add_colon2(&mut ty);
                    let inner_ty = match variant.attrs.doc_ty {
                        Some(inner) => inner,
                        None => ty.to_token_stream(),
                    };
                    let is_default = variant.fields[0].attrs.serde.default;
                    if !skip_serialize {
                        serialize_quote = quote! {
                            Some(ApiField {
                                name: #serialize.into(),
                                ty: #inner_ty::api_ty(),
                                inner: #inner_ty::api_model_id(),
                                option: #inner_ty::api_is_option(),
                            })
                        };
                    }
                    if !skip_deserialize {
                        deserialize_quote = quote! {
                            Some(ApiField {
                                name: #deserialize.into(),
                                ty: #inner_ty::api_ty(),
                                inner: #inner_ty::api_model_id(),
                                option: #inner_ty::api_is_option(),
                            })
                        };
                    }
                    member_quote = quote! {
                        ApiMember {
                            validate: #validate_quote,
                            note: #note.into(),
                            default: #is_default,
                            serialize: #serialize_quote,
                            deserialize: #deserialize_quote,
                        }
                    };
                    grow_model = quote! {
                        #grow_model
                        #inner_ty::api_grow_models(map);
                    };
                } else {
                    if !skip_serialize {
                        serialize_quote = quote! {
                            Some(ApiField {
                                name: #serialize.into(),
                                ty: ApiFieldType::IsomerismArray,
                                inner: Some(format!("{}::{}", model_id, #origin_name)),
                                option: false,
                            })
                        };
                    }
                    if !skip_deserialize {
                        deserialize_quote = quote! {
                            Some(ApiField {
                                name: #deserialize.into(),
                                ty: ApiFieldType::IsomerismArray,
                                inner: Some(format!("{}::{}", model_id, #origin_name)),
                                option: false,
                            })
                        };
                    }
                    member_quote = quote! {
                        ApiMember {
                            validate: #validate_quote,
                            note: #note.into(),
                            default: false,
                            serialize: #serialize_quote,
                            deserialize: #deserialize_quote,
                        }
                    };
                    inner_model_ty = quote! {ApiFieldType::IsomerismArray};
                    exist_inner = true;
                }
            }
            syn::Fields::Unit => {
                if !skip_serialize {
                    serialize_quote = quote! {
                        Some(ApiField {
                            name: "-".into(),
                            ty: ApiFieldType::ConstString(#serialize.into()),
                            inner: None,
                            option: false,
                        })
                    };
                }
                if !skip_deserialize {
                    deserialize_quote = quote! {
                        Some(ApiField {
                            name: "-".into(),
                            ty: ApiFieldType::ConstString(#deserialize.into()),
                            inner: None,
                            option: false,
                        })
                    };
                }
                member_quote = quote! {
                    ApiMember {
                        validate: #validate_quote,
                        note: #note.into(),
                        default: false,
                        serialize: #serialize_quote,
                        deserialize: #deserialize_quote,
                    }
                };
            }
        }
        vec_field = quote! {
            #vec_field
            members.push(#member_quote);
        };
        if exist_inner {
            let mut inner_members = quote!(let mut inner_members = vec![];);
            for field in variant.fields {
                let (member_token, _) = member_to_ident(&field.member);
                let inner_origin_name = member_token.to_string();
                let (inner_serialize, inner_deserialize) = serde_field_name(
                    inner_origin_name.as_str(),
                    &variant.attrs.serde.serialize_rule,
                    &variant.attrs.serde.deserialize_rule,
                    &field.attrs.serde,
                );
                let note = field.attrs.note;
                let note = note
                    .map(|f| f.value().trim().to_string())
                    .unwrap_or_default();
                let validate_quote = validate_token_stream(&field.attrs.validate);

                let mut ty = field.ty.clone();
                type_add_colon2(&mut ty);
                let inner_ty = match field.attrs.doc_ty {
                    Some(inner) => inner,
                    None => ty.to_token_stream(),
                };
                let is_default = field.attrs.serde.default;
                let skip_serialize = field.attrs.serde.skip_serializing;
                let skip_deserialize = field.attrs.serde.skip_deserializing;
                let mut serialize_quote = quote!(None);
                let mut deserialize_quote = quote!(None);
                if !skip_serialize {
                    serialize_quote = quote! {
                        Some(ApiField {
                            name: #inner_serialize.into(),
                            ty: #inner_ty::api_ty(),
                            inner: #inner_ty::api_model_id(),
                            option: #inner_ty::api_is_option(),
                        })
                    };
                }
                if !skip_deserialize {
                    deserialize_quote = quote! {
                        Some(ApiField {
                            name: #inner_deserialize.into(),
                            ty: #inner_ty::api_ty(),
                            inner: #inner_ty::api_model_id(),
                            option: #inner_ty::api_is_option(),
                        })
                    };
                }
                inner_members = quote! {
                    #inner_members
                    inner_members.push(ApiMember {
                        validate: #validate_quote,
                        note: #note.into(),
                        default: #is_default,
                        serialize: #serialize_quote,
                        deserialize: #deserialize_quote,
                    });
                };
                grow_model = quote! {
                    #grow_model
                    #inner_ty::api_grow_models(map);
                };
            }
            grow_model = quote! {
                #grow_model
                #inner_members
                let inner_model_id = format!("{}::{}", model_id, #origin_name);
                let inner_model = ApiModel {
                    members: inner_members,
                    note: "".into(),
                    ty: #inner_model_ty,
                    model_id: Some(inner_model_id.to_string()),
                };
                map.insert(inner_model_id, inner_model);
            };
        }
    }

    let result = quote! {
        impl #impl_generics ApiModelTrait for #enum_ty #ty_generics #where_clause {
            fn api_grow_models(map: &mut std::collections::HashMap<String, ApiModel>) {
                let model_id = Self::api_model_id();
                if let Some(model_id) = model_id {
                    if map.contains_key(&model_id) {
                        return;
                    }
                    let mut members = vec![];
                    #vec_field
                    let self_model = ApiModel {
                        members,
                        note: Self::api_note(),
                        ty: Self::api_ty(),
                        model_id: Self::api_model_id(),
                    };
                    map.insert(model_id.clone(), self_model);
                    #grow_model
                }
            }

            fn api_ty() -> ApiFieldType {
                #api_ty_quote
            }

            fn api_model_id() -> Option<String> {
                Some(std::any::type_name::<Self>().to_string())
            }

            fn api_note() -> String {
                #model_note.into()
            }
        }
    };
    result
}

pub fn serde_field_name(
    origin_name: &str,
    serialize_rule: &Option<RenameRule>,
    deserialize_rule: &Option<RenameRule>,
    serde: &Serde,
) -> (String, String) {
    let mut serialize = origin_name.to_string();
    let mut deserialize = origin_name.to_string();
    if let Some(r) = serialize_rule {
        serialize = r.apply_to_field(origin_name);
    }
    if let Some(r) = deserialize_rule {
        deserialize = r.apply_to_field(origin_name);
    }
    if let Some(n) = &serde.rename {
        serialize = n.clone();
        deserialize = serialize.clone();
    }
    if let Some(s) = &serde.serialize {
        serialize = s.clone();
    }
    if let Some(d) = &serde.deserialize {
        deserialize = d.clone();
    }
    (serialize, deserialize)
}

fn impl_struct<'a>(input: Struct<'a, ApiFieldAttr<'a>>) -> TokenStream {
    let ty = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut vec_field = quote! {};
    let mut grow_model = quote! {};
    let mut flatten_stream = quote!();
    let mut api_ty_quote = quote!();
    let mut api_model_id_quote = quote! { Some(std::any::type_name::<Self>().to_string()) };
    let mut is_new_type = false;
    let mut api_is_option = quote!(false);
    if let Data::Struct(data) = &input.original.data {
        if let Fields::Unnamed(fields) = &data.fields {
            let fields_len = fields.unnamed.len();
            if fields_len == 0 {
                api_ty_quote = quote! { ApiFieldType::EmptyArray };
            } else if fields_len == 1 {
                let mut ty = fields.unnamed[0].ty.clone();
                type_add_colon2(&mut ty);
                api_ty_quote = quote! { #ty::api_ty() };
                api_model_id_quote = quote! { #ty::api_model_id() };
                api_is_option = quote! { #ty::api_is_option() };
                is_new_type = true;
            } else {
                api_ty_quote = quote! { ApiFieldType::IsomerismArray };
            }
        }
    }
    if api_ty_quote.is_empty() {
        let tag = input.attrs.serde.tag;
        if let Some(tag) = tag {
            let name = match input.attrs.serde.rename {
                Some(name) => name,
                None => ty.to_string(),
            };
            vec_field = quote! {
                #vec_field
                members.push(ApiMember {
                    validate: vec![],
                    note: "tag".into(),
                    default: false,
                    serialize: Some(ApiField {
                        name: #tag.into(),
                        ty: ApiFieldType::ConstString(#name.into()),
                        inner: None,
                        option: false,
                    }),
                    deserialize: None,
                });
            };
            api_ty_quote = quote! { ApiFieldType::TagStuct {tag: #tag.into(), name: #name.into()} };
        } else {
            api_ty_quote = quote! { ApiFieldType::Object };
        }
    }

    for field in input.fields {
        let field_ty = field.ty;
        let field_ty = &mut field_ty.clone();
        type_add_colon2(field_ty);

        let member = member_to_ident(&field.member);
        let origin_name = member.0.to_string();
        let (serialize, deserialize) = serde_field_name(
            origin_name.as_str(),
            &input.attrs.serde.serialize_rule,
            &input.attrs.serde.deserialize_rule,
            &field.attrs.serde,
        );

        let note = field.attrs.note;
        let note = note
            .map(|f| f.value().trim().to_string())
            .unwrap_or_default();
        let inner_ty = match field.attrs.doc_ty {
            Some(inner) => inner,
            None => field_ty.to_token_stream(),
        };

        let validate_quote = validate_token_stream(&field.attrs.validate);

        let flatten = field.attrs.serde.flatten;
        let is_default = field.attrs.serde.default;
        let mut serialize_quote = quote! {None};
        if !field.attrs.serde.skip_serializing {
            serialize_quote = quote! {
                Some(ApiField {
                    name: #serialize.into(),
                    ty: #inner_ty::api_ty(),
                    inner: #inner_ty::api_model_id(),
                    option: #inner_ty::api_is_option(),
                })
            };
        }
        let mut deserialize_quote = quote! {None};
        if !field.attrs.serde.skip_deserializing {
            deserialize_quote = quote! {
                Some(ApiField {
                    name: #deserialize.into(),
                    ty: #inner_ty::api_ty(),
                    inner: #inner_ty::api_model_id(),
                    option: #inner_ty::api_is_option(),
                })
            };
        }

        if !is_new_type && !flatten {
            vec_field = quote! {
                #vec_field
                members.push(ApiMember {
                    validate: #validate_quote,
                    note: #note.into(),
                    default: #is_default,
                    serialize: #serialize_quote,
                    deserialize: #deserialize_quote,
                });
            };
        }
        if flatten {
            let mut item_stream = quote!(let item = item.clone(););
            if field.attrs.serde.skip_deserializing
                || field.attrs.serde.skip_serializing
                || is_default
            {
                item_stream = quote!(let mut item = item.clone(););
                if field.attrs.serde.skip_deserializing {
                    item_stream = quote! {
                        #item_stream
                        item.deserialize = None;
                    };
                }
                if field.attrs.serde.skip_serializing {
                    item_stream = quote! {
                        #item_stream
                        item.serialize = None;
                    };
                }
                if is_default {
                    item_stream = quote! {
                        #item_stream
                        item.default = #is_default;
                    };
                }
            }
            flatten_stream = quote! {
                #flatten_stream
                let flatten_id = #inner_ty::api_model_id().unwrap_or_default();
                let flatten = map.remove(&flatten_id);
                if let Some(flatten) = flatten {
                    for item in flatten.members {
                        #item_stream
                        self_fields.push(item);
                    }
                }
            };
        }
        grow_model = quote! {
            #grow_model
            #field_ty::api_grow_models(map);
        }
    }
    if !flatten_stream.is_empty() {
        flatten_stream = quote! {
            let self_model = map.remove(&model_id);
            let mut self_model = match self_model {
                Some(self_model) => self_model,
                None => return,
            };
            let mut self_fields = self_model.members;
            #flatten_stream
            self_model.members = self_fields;
            map.insert(model_id, self_model);
        };
    }
    let mut self_fields_quote = quote!();
    if !is_new_type {
        self_fields_quote = quote! {
            let mut members = vec![];
            #vec_field
            let self_model = ApiModel {
                members,
                note: Self::api_note(),
                ty: Self::api_ty(),
                model_id: Self::api_model_id(),
            };
            map.insert(model_id.to_string(), self_model);
        };
    }
    let result = quote! {
        impl #impl_generics ApiModelTrait for #ty #ty_generics #where_clause {
            fn api_grow_models(map: &mut std::collections::HashMap<String, ApiModel>) {
                let model_id = Self::api_model_id();
                if let Some(model_id) = model_id {
                    if map.contains_key(&model_id) {
                        return;
                    }
                    #self_fields_quote
                    #grow_model
                    #flatten_stream
                }
            }

            fn api_ty() -> ApiFieldType {
                #api_ty_quote
            }

            fn api_model_id() -> Option<String> {
                #api_model_id_quote
            }

            fn api_is_option() -> bool {
                #api_is_option
            }
        }
    };
    result
}

fn validate_token_stream(validates: &Vec<ValidateItem>) -> TokenStream {
    let mut validate_quote = quote!();
    for validate_item in validates {
        match &validate_item.validate_type {
            super::validator::ValidateType::Null => continue,
            super::validator::ValidateType::Length(min, max) => {
                let min = lit_to_string(min);
                let max = lit_to_string(max);
                validate_quote = quote! {
                    #validate_quote  ValidateType::Length(#min.into(), #max.into()),
                };
            }
            super::validator::ValidateType::Range(min, max) => {
                let min = lit_to_string(min);
                let max = lit_to_string(max);
                validate_quote = quote! {
                    #validate_quote ValidateType::Range(#min.into(), #max.into()),
                };
            }
            super::validator::ValidateType::Enumer(enumer) => {
                let path = enumer.segments.to_token_stream();
                validate_quote = quote! {
                    #validate_quote ValidateType::Enumer(#path::gather()),
                };
            }
            super::validator::ValidateType::Func(path) => {
                let path = path.to_string();
                validate_quote = quote! {
                    #validate_quote ValidateType::Func(#path.into()),
                };
            }
            super::validator::ValidateType::Phone => {
                validate_quote = quote! {
                    #validate_quote ValidateType::Phone,
                };
            }
            super::validator::ValidateType::Email => {
                validate_quote = quote! {
                    #validate_quote ValidateType::Email,
                };
            }
            super::validator::ValidateType::Inner => {
                validate_quote = quote! {
                    #validate_quote ValidateType::Inner,
                };
            }
            super::validator::ValidateType::Regex(lit) => {
                let message = lit_to_string(lit);
                validate_quote = quote! {
                    #validate_quote ValidateType::Regex(#message.into()),
                };
            }
            super::validator::ValidateType::Inspector(path) => {
                validate_quote = quote! {
                    #validate_quote ValidateType::Inspector(#path::desc().into()),
                };
            }
        }
    }
    validate_quote = quote! {
        vec![#validate_quote]
    };
    validate_quote
}

pub fn attribute(attr: Vec<NestedMeta>, input: &mut ItemFn, method: String) -> Result<TokenStream> {
    let mut url: String = String::default();
    let mut tag_str: String = String::default();
    let mut sign_str: String = String::default();
    let mut deprecated: bool = false;
    let mut name: String = String::default();
    let mut note: String = String::default();
    let mut opt: String = String::default();
    let mut auth: bool = true;
    let mut power: bool = true;
    let mut mod_name = quote!(parent_mod_name(module_path!()));
    for node in attr.iter() {
        if let NestedMeta::Meta(Meta::NameValue(nv)) = node {
            if nv.path.is_ident("url") {
                url = nv_attr_to_str_value(nv)?;
            } else if nv.path.is_ident("tags") {
                tag_str = nv_attr_to_str_value(nv)?;
            } else if nv.path.is_ident("signs") {
                sign_str = nv_attr_to_str_value(nv)?;
            } else if nv.path.is_ident("deprecated") {
                deprecated = nv_attr_to_bool_value(nv)?;
            } else if nv.path.is_ident("name") {
                name = nv_attr_to_str_value(nv)?;
            } else if nv.path.is_ident("note") {
                note = nv_attr_to_str_value(nv)?;
            } else if nv.path.is_ident("opt") {
                opt = nv_attr_to_str_value(nv)?;
            } else if nv.path.is_ident("auth") {
                auth = nv_attr_to_bool_value(nv)?;
            } else if nv.path.is_ident("power") {
                power = nv_attr_to_bool_value(nv)?;
            } else if nv.path.is_ident("mod_name") {
                let val = nv_attr_to_str_value(nv)?;
                mod_name = quote! { #val.into() };
            } else {
                return Err(Error::new_spanned(node, "no attr item"));
            }
            continue;
        }
        return Err(Error::new_spanned(node, "value style error"));
    }

    let mut param_quote = quote! {
        let mut path_in = vec![];
        let mut query_in = vec![];
        let mut header_in = vec![];
        let mut body_in = None;
        let mut body_out = None;
    };
    for p in &mut input.sig.inputs {
        if let syn::FnArg::Typed(r) = p {
            let req_ty = &mut r.ty;
            type_add_colon2(req_ty);
            param_quote = quote! {
                #param_quote
                let param_ty = #req_ty::api_param_type();
                match param_ty {
                    ApiParamTypeEnum::Path => {
                        let map = #req_ty::api_models().into_iter();
                        for (_, v) in map {
                            path_in.push(v.clone());
                        }
                    },
                    ApiParamTypeEnum::Query => {
                        let map = #req_ty::api_models().into_iter();
                        for (_, v) in map {
                            query_in.push(v.clone());
                        }
                    },
                    ApiParamTypeEnum::Header => {
                        let map = #req_ty::api_models().into_iter();
                        for (_, v) in map {
                            header_in.push(v.clone());
                        }
                    },
                    ApiParamTypeEnum::Body(inner) => {
                        body_in = Some(ApiBodyParam {
                            content_type: inner,
                            model_id: #req_ty::api_type_id().unwrap_or_default(),
                            models: #req_ty::api_models(),
                        });
                    },
                    ApiParamTypeEnum::None => {},
                }
            }
        }
    }

    let resp_ty = &mut input.sig.output.clone();
    ret_type_add_colon2(resp_ty);
    if let ReturnType::Type(_, resp_ty) = resp_ty {
        param_quote = quote! {
            #param_quote
            let param_ty = #resp_ty::api_param_type();
            if let ApiParamTypeEnum::Body(inner) = param_ty {
                body_out = Some(ApiBodyParam {
                    content_type: inner,
                    model_id: #resp_ty::api_type_id().unwrap_or_default(),
                    models: #resp_ty::api_models(),
                });
            }
        };
    }

    let fn_ident = &mut input.sig.ident.clone();
    let fn_name = &mut fn_ident.to_string();

    let api_fn_ident = Ident::new(format!("{}_api", fn_name).as_str(), Span::call_site());
    let route_fn_ident = Ident::new(format!("{}_route", fn_name).as_str(), Span::call_site());
    let req_method_ident = Ident::new(method.as_str(), Span::call_site());

    let route = quote!(#req_method_ident(#fn_ident));
    let mut url_token = quote!(#url.into());
    if url.is_empty() {
        url_token = quote!(default_path(module_path!(), crate_name!()));
    }
    let api = quote!(
        ApiOperation {
        method: #method.into(),
        url: #url_token,
        tags: #tag_str.split(",").map(|item| item.to_string()).collect(),
        signs: #sign_str.split(",").map(|item| item.to_string()).collect(),
        deprecated: #deprecated,
        name: #name.into(),
        note: #note.into(),
        crate_name: crate_name!().to_string(),
        mod_name: #mod_name,
        mod_path: module_path!().to_string(),
        path_in,
        query_in,
        header_in,
        body_in,
        body_out,
        opt: #opt.into(),
        auth: #auth.into(),
        power: #power.into(),
    });
    let impl_api = quote! (
        pub fn #api_fn_ident() -> ApiOperation {
            #param_quote
            #api
        }
    );
    let impl_axum_router = quote! (
        pub fn #route_fn_ident() -> AxumApiRoute<AppState> {
            AxumApiRoute {
                route: axum::routing::#route,
                api: #api_fn_ident(),
            }
        }
    );
    let result = quote!(
        #input
        #impl_api
        #impl_axum_router
    );
    Ok(result)
}
