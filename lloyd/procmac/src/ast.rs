use crate::attr::Attrs;
use crate::generics::ParamsInScope;
use proc_macro2::Span;
use syn::{
    Data, DataEnum, DataStruct, DeriveInput, Error, Fields, Generics, Ident, Index, Member, Result,
    Type,
};

pub enum Input<'a, T: Attrs<'a>> {
    Struct(Struct<'a, T>),
    Enum(Enum<'a, T>),
}

pub struct Struct<'a, T: Attrs<'a>> {
    pub original: &'a DeriveInput,
    pub attrs: T,
    pub ident: Ident,
    pub generics: &'a Generics,
    pub fields: Vec<Field<'a, T>>,
}

pub struct Enum<'a, T: Attrs<'a>> {
    pub original: &'a DeriveInput,
    pub attrs: T,
    pub ident: Ident,
    pub generics: &'a Generics,
    pub variants: Vec<Variant<'a, T>>,
}

pub struct Variant<'a, T: Attrs<'a>> {
    pub original: &'a syn::Variant,
    pub attrs: T,
    pub ident: Ident,
    pub fields: Vec<Field<'a, T>>,
}

pub struct Field<'a, T: Attrs<'a>> {
    pub original: &'a syn::Field,
    pub attrs: T,
    pub member: Member,
    pub ty: &'a Type,
    pub contains_generic: bool,
}

impl<'a, T: Attrs<'a>> Input<'a, T> {
    pub fn from_syn(node: &'a DeriveInput) -> Result<Self> {
        match &node.data {
            Data::Struct(data) => Struct::from_syn(node, data).map(Input::Struct),
            Data::Enum(data) => Enum::from_syn(node, data).map(Input::Enum),
            Data::Union(_) => Err(Error::new_spanned(
                node,
                "union as errors are not supported",
            )),
        }
    }
}

impl<'a, T: Attrs<'a>> Struct<'a, T> {
    fn from_syn(node: &'a DeriveInput, data: &'a DataStruct) -> Result<Self> {
        let attrs = T::get(&node.attrs)?;
        let scope = ParamsInScope::new(&node.generics);
        let span = attrs.span().unwrap_or_else(Span::call_site);
        let fields = Field::multiple_from_syn(&data.fields, &scope, span)?;
        Ok(Struct {
            original: node,
            attrs,
            ident: node.ident.clone(),
            generics: &node.generics,
            fields,
        })
    }
}

impl<'a, T: Attrs<'a>> Enum<'a, T> {
    fn from_syn(node: &'a DeriveInput, data: &'a DataEnum) -> Result<Self> {
        let attrs = T::get(&node.attrs)?;
        let scope = ParamsInScope::new(&node.generics);
        let span = attrs.span().unwrap_or_else(Span::call_site);
        let variants = data
            .variants
            .iter()
            .map(|node| {
                let variant = Variant::<T>::from_syn(node, &scope, span)?;
                Ok(variant)
            })
            .collect::<Result<_>>()?;
        Ok(Enum {
            original: node,
            attrs,
            ident: node.ident.clone(),
            generics: &node.generics,
            variants,
        })
    }
}

impl<'a, T: Attrs<'a>> Variant<'a, T> {
    fn from_syn(node: &'a syn::Variant, scope: &ParamsInScope<'a>, span: Span) -> Result<Self> {
        let attrs = T::get(&node.attrs)?;
        let span = attrs.span().unwrap_or(span);
        Ok(Variant {
            original: node,
            attrs,
            ident: node.ident.clone(),
            fields: Field::multiple_from_syn(&node.fields, scope, span)?,
        })
    }
}

impl<'a, T: Attrs<'a>> Field<'a, T> {
    fn multiple_from_syn(
        fields: &'a Fields,
        scope: &ParamsInScope<'a>,
        span: Span,
    ) -> Result<Vec<Self>> {
        fields
            .iter()
            .enumerate()
            .map(|(i, field)| Field::from_syn(i, field, scope, span))
            .collect()
    }

    fn from_syn(
        i: usize,
        node: &'a syn::Field,
        scope: &ParamsInScope<'a>,
        span: Span,
    ) -> Result<Self> {
        Ok(Field {
            original: node,
            attrs: T::get(&node.attrs)?,
            member: node.ident.clone().map(Member::Named).unwrap_or_else(|| {
                Member::Unnamed(Index {
                    index: i as u32,
                    span,
                })
            }),
            ty: &node.ty,
            contains_generic: scope.intersects(&node.ty),
        })
    }
}