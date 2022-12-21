use proc_macro2::{Delimiter, Group, Span, TokenStream, TokenTree};
use quote::{format_ident, quote, ToTokens};
use std::collections::BTreeSet as Set;
use std::iter::FromIterator;
use syn::parse::ParseStream;
use syn::{
    braced, bracketed, parenthesized, token, Attribute, Ident, Index, LitInt, LitStr, Result, Token,
};

pub trait Attrs<'a>: Sized {
    fn get(input: &'a [Attribute]) -> Result<Self>;

    fn span(&self) -> Option<Span>;
}

#[derive(Clone)]
pub struct Display<'a> {
    pub original: &'a Attribute,
    pub fmt: LitStr,
    pub args: TokenStream,
    pub has_bonus_display: bool,
    pub implied_bounds: Set<(usize, Trait)>,
    pub named_args: Vec<Ident>,
}

#[derive(Copy, Clone)]
pub struct Transparent<'a> {
    pub original: &'a Attribute,
    pub span: Span,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum Trait {
    Debug,
    Display,
    Octal,
    LowerHex,
    UpperHex,
    Pointer,
    Binary,
    LowerExp,
    UpperExp,
}

pub fn parse_token_expr(input: ParseStream, mut begin_expr: bool) -> Result<TokenStream> {
    let mut tokens = Vec::new();
    while !input.is_empty() {
        if begin_expr && input.peek(Token![.]) {
            if input.peek2(Ident) {
                input.parse::<Token![.]>()?;
                begin_expr = false;
                continue;
            }
            if input.peek2(LitInt) {
                input.parse::<Token![.]>()?;
                let int: Index = input.parse()?;
                let ident = format_ident!("_{}", int.index, span = int.span);
                tokens.push(TokenTree::Ident(ident));
                begin_expr = false;
                continue;
            }
        }

        begin_expr = input.peek(Token![break])
            || input.peek(Token![continue])
            || input.peek(Token![if])
            || input.peek(Token![in])
            || input.peek(Token![match])
            || input.peek(Token![mut])
            || input.peek(Token![return])
            || input.peek(Token![while])
            || input.peek(Token![+])
            || input.peek(Token![&])
            || input.peek(Token![!])
            || input.peek(Token![^])
            || input.peek(Token![,])
            || input.peek(Token![/])
            || input.peek(Token![=])
            || input.peek(Token![>])
            || input.peek(Token![<])
            || input.peek(Token![|])
            || input.peek(Token![%])
            || input.peek(Token![;])
            || input.peek(Token![*])
            || input.peek(Token![-]);

        let token: TokenTree = if input.peek(token::Paren) {
            let content;
            let delimiter = parenthesized!(content in input);
            let nested = parse_token_expr(&content, true)?;
            let mut group = Group::new(Delimiter::Parenthesis, nested);
            group.set_span(delimiter.span);
            TokenTree::Group(group)
        } else if input.peek(token::Brace) {
            let content;
            let delimiter = braced!(content in input);
            let nested = parse_token_expr(&content, true)?;
            let mut group = Group::new(Delimiter::Brace, nested);
            group.set_span(delimiter.span);
            TokenTree::Group(group)
        } else if input.peek(token::Bracket) {
            let content;
            let delimiter = bracketed!(content in input);
            let nested = parse_token_expr(&content, true)?;
            let mut group = Group::new(Delimiter::Bracket, nested);
            group.set_span(delimiter.span);
            TokenTree::Group(group)
        } else {
            input.parse()?
        };
        tokens.push(token);
    }
    Ok(TokenStream::from_iter(tokens))
}

impl ToTokens for Display<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let fmt = &self.fmt;
        let args = &self.args;
        tokens.extend(quote! {
            write!(__formatter, #fmt #args)
        });
    }
}

impl ToTokens for Trait {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let trait_name = format_ident!("{}", format!("{:?}", self));
        tokens.extend(quote!(std::fmt::#trait_name));
    }
}
