use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Error, Result};

pub(crate) fn derive(input: &syn::DeriveInput) -> Result<TokenStream> {
    let ty = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let mut match_item_quote = quote!();
    for attr in &input.attrs {
        if !attr.path.is_ident("profiles") {
            continue;
        }
        let meta = attr.parse_meta()?;
        if let syn::Meta::List(list) = meta {
            for item in list.nested.iter() {
                let item = item.into_token_stream();
                let profile = item.to_string();
                let profile = profile.trim();
                let profile = format!("profile{}{}.yaml", std::path::MAIN_SEPARATOR, profile);
                if !profile.is_empty() {
                    match_item_quote = quote! {
                        #match_item_quote
                        #profile => include_str!(#profile).to_string(),
                    };
                }
            }
        } else {
            return Err(Error::new_spanned(
                attr,
                "#[profiles(dev, prod,...)] style not match",
            ));
        }
    }
    let result = quote! {
        impl #impl_generics #ty #ty_generics #where_clause {
            pub fn from_env(env: &str) -> Result<Self> {
                let location = if env.is_empty() {
                    format!("{}/{}.yaml", RESOURCES_DIR_NAME, crate_name!())
                } else {
                    format!("{}/{}-{}.yaml", RESOURCES_DIR_NAME, crate_name!(), env)
                };
                let path = Path::new(&location);
                if !path.exists() {
                    ensure_dir(path.parent().unwrap_or(&Path::new("/")))?;
                    let default_config_content = match env {
                        #match_item_quote
                        _ => serde_json::to_string(&Self::default())?,
                    };
                    write_content(&default_config_content, &location)?;
                }
                let app_config = load_config::<Self>(&location, ConfigStyle::Yaml)?;
                Ok(app_config)
            }
        }
    };
    Ok(result)
}
