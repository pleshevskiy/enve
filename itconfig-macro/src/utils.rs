use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{Path, Type};

const OPTION_PATH_IDENTS: &[&str] = &["Option|", "std|option|Option|", "core|option|Option|"];
const VEC_PATH_IDENTS: &[&str] = &["Vec|", "std|vec|Vec|"];

#[derive(Debug)]
pub enum SupportedBox {
    Vec,
    Option,
}

pub fn vec_to_token_stream_2<T>(input: &[T]) -> Vec<TokenStream2>
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

fn is_option_path_ident(path_ident: &str) -> bool {
    OPTION_PATH_IDENTS.iter().any(|s| path_ident == *s)
}

fn is_vec_path_ident(path_ident: &str) -> bool {
    VEC_PATH_IDENTS.iter().any(|s| path_ident == *s)
}

pub fn maybe_supported_box(ty: &Type) -> Option<SupportedBox> {
    match ty {
        Type::Path(ty_path) if ty_path.qself.is_none() => {
            let path_ident = path_ident(&ty_path.path);
            if is_option_path_ident(&path_ident) {
                Some(SupportedBox::Option)
            } else if is_vec_path_ident(&path_ident) {
                Some(SupportedBox::Vec)
            } else {
                None
            }
        }
        _ => None,
    }
}
