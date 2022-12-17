use syn::{Error, GenericArgument, Lit, PathArguments, Result, Token, Type, ReturnType};

pub mod apidoc;
pub mod enumer;
pub mod enumprop;
pub mod httpcode;
pub mod multipart;
pub mod validator;
pub mod config;
pub mod component;

pub fn type_parameter_of_option(ty: &Type) -> Option<&Type> {
    let path = match ty {
        Type::Path(ty) => &ty.path,
        _ => return None,
    };

    let last = path.segments.last().unwrap();
    if last.ident != "Option" {
        return None;
    }

    let bracketed = match &last.arguments {
        PathArguments::AngleBracketed(bracketed) => bracketed,
        _ => return None,
    };

    if bracketed.args.len() != 1 {
        return None;
    }

    match &bracketed.args[0] {
        GenericArgument::Type(arg) => Some(arg),
        _ => None,
    }
}

pub fn type_add_colon2(ret: &mut Type) {
    if let Type::Path(p) = ret {
        let path = &mut p.path;
        let segs = &mut path.segments;
        for seg in segs {
            if let PathArguments::AngleBracketed(ref mut ab) = &mut seg.arguments {
                if ab.colon2_token.is_none() {
                    ab.colon2_token = Some(<Token![::]>::default());
                }
            }
        }
    }
}

pub fn nv_attr_to_str_value(nv: &syn::MetaNameValue) -> Result<String> {
    let value = &nv.lit;
    if let Lit::Str(str) = value {
        return Ok(str.value());
    }
    Err(Error::new_spanned(nv, "value style error"))
}

pub fn nv_attr_to_bool_value(nv: &syn::MetaNameValue) -> Result<bool> {
    let value = &nv.lit;
    if let Lit::Bool(b) = value {
        return Ok(b.value());
    }
    Err(Error::new_spanned(nv, "value style error"))
}

pub fn ret_type_add_colon2(ret: &mut ReturnType) {
    if let ReturnType::Type(_r, t) = ret {
        type_add_colon2(t);
    }
}