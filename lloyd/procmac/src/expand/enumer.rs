use proc_macro::TokenStream;
use syn::DeriveInput;
use syn::Variant;

use quote::quote;
use syn::Attribute;
use syn::{self};

use crate::tools::lit_to_string;

pub fn enumerable(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let enum_name = &ast.ident;
    let variants = collect_variants(&ast);

    let mut by_id_quote = quote! {};
    let mut from_str_quote = quote! {};
    let mut gather_quote = quote! {};
    let mut note_quote = quote! {};

    for item_variant in variants {
        let var_path = &item_variant.variant.ident;
        let str_path = var_path.to_string();
        let note = item_variant.note;
        let note = match note {
            Some(note) => note,
            None => String::from(""),
        };
        by_id_quote = quote! {
            #by_id_quote
            if value == (#enum_name::#var_path as i32) {
                return Some(#enum_name::#var_path);
            }
        };

        from_str_quote = quote! {
            #from_str_quote
            #str_path => Some(#enum_name::#var_path),
        };
        gather_quote = quote! {
            #gather_quote
            vec.push(EnumDesc {
                id: #enum_name::#var_path as i32,
                name: #str_path.into(),
                note: #note.into(),
            });
        };
        note_quote = quote! {
            #note_quote
            #enum_name::#var_path => #note.into(),
        }
    }
    by_id_quote = quote! {
        #by_id_quote
        return None;
    };
    let impl_quote = quote! {

        impl EnumFrom<Option<i32>> for #enum_name {
            fn enum_by(opt: Option<i32>) -> Option<Self> {
                match opt {
                    Some(value) => {
                        #by_id_quote
                    },
                    None => None,
                }
            }
        }

        impl EnumFrom<&i32> for #enum_name {
            fn enum_by(value: &i32) -> Option<Self> {
                let value = *value;
                #by_id_quote
            }
        }

        impl EnumFrom<i32> for #enum_name {
            fn enum_by(value: i32) -> Option<Self> {
                #by_id_quote
            }
        }

        impl EnumFrom<Option<&str>> for #enum_name {
            fn enum_by(opt: Option<&str>) -> Option<Self> {
                match opt {
                    Some(value) => {
                        match value {
                            #from_str_quote
                            _ => None,
                        }
                    },
                    None => None,
                }
            }
        }

        impl EnumFrom<&str> for #enum_name {
            fn enum_by(value: &str) -> Option<Self> {
                match value {
                    #from_str_quote
                    _ => None,
                }
            }
        }

        impl #enum_name {
            pub fn gather() -> Vec<EnumDesc> {
                let mut vec = vec![];
                #gather_quote
                vec
            }

            pub fn note(&self) -> String {
                match self {
                    #note_quote
                }
            }
        }
    };
    TokenStream::from(impl_quote)
}

pub struct Attr<'b> {
    pub origin_attr: &'b Attribute,
}

pub struct ParsedVariant<'a, 'b> {
    pub variant: &'a Variant,
    pub attrs: Vec<Attr<'b>>,
    pub note: Option<String>,
}

// 收集属性
pub fn collect_variants(ast: &syn::DeriveInput) -> Vec<ParsedVariant> {
    let mut ret_variants: Vec<ParsedVariant> = vec![];
    match &ast.data {
        syn::Data::Enum(de) => {
            let variants = &de.variants;
            if variants.is_empty() {
                return ret_variants;
            }
            for item in variants.iter() {
                let mut attrs: Vec<Attr> = Vec::new();
                let origin_attrs = &item.attrs;
                let mut note = None;
                for arigin_attr in origin_attrs {
                    if arigin_attr.path.is_ident("note") {
                        if note.is_some() {
                            panic!("dumplicate note");
                        }

                        let meta = arigin_attr.parse_meta().unwrap();
                        note = match meta {
                            syn::Meta::List(list) => {
                                if list.nested.len() != 1 {
                                    panic!("note fmt like note(\"男\")");
                                }
                                let desc_meta = &list.nested[0];
                                match desc_meta {
                                    syn::NestedMeta::Meta(_) => {
                                        panic!("note fmt like note(\"男\")")
                                    }
                                    syn::NestedMeta::Lit(lit) => Some(lit_to_string(lit)),
                                }
                            }
                            _ => panic!("note fmt like note(\"男\")"),
                        }
                    }
                    let attr = Attr {
                        origin_attr: arigin_attr,
                    };
                    attrs.push(attr);
                }
                let parsed_variant = ParsedVariant {
                    variant: item,
                    attrs,
                    note,
                };
                ret_variants.push(parsed_variant);
            }
        }
        _ => {
            panic!("not support type");
        }
    }
    ret_variants
}
