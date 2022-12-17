use crate::attr::{Display, Trait};
use proc_macro2::TokenTree;
use quote::{format_ident, quote_spanned};
use std::collections::{BTreeSet as Set, HashMap as Map};
use syn::ext::IdentExt;
use syn::parse::{ParseStream, Parser};
use syn::{Ident, Index, LitStr, Member, Result, Token};

impl Display<'_> {
    pub fn expand_shorthand(&mut self, fields: &[Member]) {
        let raw_args = self.args.clone();
        let mut named_args = explicit_named_args.parse2(raw_args).unwrap();
        let mut member_index = Map::new();
        for (i, member) in fields.iter().enumerate() {
            member_index.insert(member, i);
        }

        let span = self.fmt.span();
        let fmt = self.fmt.value();
        let mut read = fmt.as_str();
        let mut out = String::new();
        let mut args = self.args.clone();
        let mut has_bonus_display = false;
        let mut implied_bounds = Set::new();

        let mut has_trailing_comma = false;
        if let Some(TokenTree::Punct(punct)) = args.clone().into_iter().last() {
            if punct.as_char() == ',' {
                has_trailing_comma = true;
            }
        }

        while let Some(brace) = read.find('{') {
            out += &read[..brace + 1];
            read = &read[brace + 1..];
            if read.starts_with('{') {
                out.push('{');
                read = &read[1..];
                continue;
            }
            let next = match read.chars().next() {
                Some(next) => next,
                None => return,
            };
            let member = match next {
                '0'..='9' => {
                    let int = take_int(&mut read);
                    let member = match int.parse::<u32>() {
                        Ok(index) => Member::Unnamed(Index { index, span }),
                        Err(_) => return,
                    };
                    if !member_index.contains_key(&member) {
                        out += &int;
                        continue;
                    }
                    member
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let ident = take_ident(&mut read);
                    match ident {
                        Ok(mut ident) => {
                            ident.set_span(span);
                            Member::Named(ident)
                        }
                        Err(_) => return,
                    }
                }
                _ => continue,
            };
            if let Some(&field) = member_index.get(&member) {
                let end_spec = match read.find('}') {
                    Some(end_spec) => end_spec,
                    None => return,
                };
                let bound = match read[..end_spec].chars().next_back() {
                    Some('?') => Trait::Debug,
                    Some('o') => Trait::Octal,
                    Some('x') => Trait::LowerHex,
                    Some('X') => Trait::UpperHex,
                    Some('p') => Trait::Pointer,
                    Some('b') => Trait::Binary,
                    Some('e') => Trait::LowerExp,
                    Some('E') => Trait::UpperExp,
                    Some(_) | None => Trait::Display,
                };
                implied_bounds.insert((field, bound));
            }
            let local = member_to_ident(&member).0;
            let mut formatvar = local.clone();
            if formatvar.to_string().starts_with("r#") {
                formatvar = format_ident!("r_{}", formatvar);
            }
            if formatvar.to_string().starts_with('_') {
                formatvar = format_ident!("{}", formatvar);
            }
            named_args.push(formatvar.clone());

            if !has_trailing_comma {
                args.extend(quote_spanned!(span=> ,));
            }
            args.extend(quote_spanned!(span=> #formatvar = #local));
            if read.starts_with('}') && member_index.contains_key(&member) {
                has_bonus_display = true;
                args.extend(quote_spanned!(span=> .as_display()));
            }
            has_trailing_comma = false;
        }

        out += read;
        self.fmt = LitStr::new(&out, self.fmt.span());
        self.args = args;
        self.has_bonus_display = has_bonus_display;
        self.implied_bounds = implied_bounds;
        self.named_args = named_args;
    }
}

pub fn member_to_ident(member: &Member) -> (Ident, bool) {
    match member {
        Member::Unnamed(index) => (format_ident!("_{}", index), false),
        Member::Named(ident) => (ident.clone(), true),
    }
}

pub fn explicit_named_args(input: ParseStream) -> Result<Vec<Ident>> {
    let mut named_args = vec![];

    while !input.is_empty() {
        if input.peek(Token![,]) && input.peek2(Ident::peek_any) && input.peek3(Token![=]) {
            input.parse::<Token![,]>()?;
            let ident = input.call(Ident::parse_any)?;
            input.parse::<Token![=]>()?;
            named_args.push(ident);
        } else {
            input.parse::<TokenTree>()?;
        }
    }

    Ok(named_args)
}

pub fn take_int(read: &mut &str) -> String {
    let mut int = String::new();
    for (i, ch) in read.char_indices() {
        match ch {
            '0'..='9' => int.push(ch),
            _ => {
                *read = &read[i..];
                break;
            }
        }
    }
    int
}

pub fn take_ident(read: &mut &str) -> Result<Ident> {
    let mut ident = String::new();
    let raw = read.starts_with("r#");
    if raw {
        ident.push_str("r#");
        *read = &read[2..];
    }
    for (i, ch) in read.char_indices() {
        match ch {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => ident.push(ch),
            _ => {
                *read = &read[i..];
                break;
            }
        }
    }
    Ident::parse_any.parse_str(&ident)
}
