use syn::Lit;

pub fn lit_to_string(lit: &Lit) -> String {
    match lit {
        Lit::Str(str) => str.value(),
        Lit::ByteStr(b_str) => String::from_utf8(b_str.value()).unwrap(),
        Lit::Byte(b) => b.value().to_string(),
        Lit::Char(c) => c.value().to_string(),
        Lit::Int(i) => i.to_string(),
        Lit::Float(f) => f.to_string(),
        Lit::Bool(b) => b.value().to_string(),
        Lit::Verbatim(_) => todo!(),
    }
}
