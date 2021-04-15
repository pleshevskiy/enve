use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{Path, Type};

pub fn vec_to_token_stream_2<T>(input: &Vec<T>) -> Vec<TokenStream2>
where
    T: ToTokens,
{
    input.iter().map(|ns| ns.into_token_stream()).collect()
}

fn path_ident(path: &Path) -> String {
    path.segments
        .iter()
        .into_iter()
        .fold(String::with_capacity(250), |mut acc, v| {
            acc.push_str(&v.ident.to_string());
            acc.push('|');
            acc
        })
}

fn is_option_path_ident(path_ident: String) -> bool {
    vec!["Option|", "std|option|Option|", "core|option|Option|"]
        .into_iter()
        .find(|s| &path_ident == *s)
        .is_some()
}

pub fn is_option_type(ty: &Type) -> bool {
    match ty {
        Type::Path(ty_path) => {
            ty_path.qself.is_none() && is_option_path_ident(path_ident(&ty_path.path))
        }
        _ => false,
    }
}
