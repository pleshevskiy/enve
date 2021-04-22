use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{GenericArgument, Path, PathArguments, Type};

const OPTION_PATH_IDENTS: &[&str] = &["Option|", "std|option|Option|", "core|option|Option|"];
const VEC_PATH_IDENTS: &[&str] = &["Vec|", "std|vec|Vec|"];

#[derive(Debug, Clone)]
pub enum SupportedBox {
    Vec(Option<String>),
    Option,
    OptionVec(Option<String>),
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
            let ty_path_ident = path_ident(&ty_path.path);
            if is_option_path_ident(&ty_path_ident) {
                match &ty_path.path.segments.iter().last().unwrap().arguments {
                    PathArguments::AngleBracketed(params) => match params.args.first() {
                        Some(GenericArgument::Type(Type::Path(inner_ty_path))) => {
                            let ty_path_ident = path_ident(&inner_ty_path.path);
                            if is_vec_path_ident(&ty_path_ident) {
                                Some(SupportedBox::OptionVec(None))
                            } else {
                                Some(SupportedBox::Option)
                            }
                        }
                        _ => Some(SupportedBox::Option),
                    },
                    _ => Some(SupportedBox::Option),
                }
            } else if is_vec_path_ident(&ty_path_ident) {
                Some(SupportedBox::Vec(None))
            } else {
                None
            }
        }
        _ => None,
    }
}
