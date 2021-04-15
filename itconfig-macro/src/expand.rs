use crate::ast::*;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::Path;
use syn::Type;

fn vec_to_token_stream_2<T>(input: &Vec<T>) -> Vec<TokenStream2>
where
    T: ToTokens,
{
    input.iter().map(|ns| ns.into_token_stream()).collect()
}

impl ToTokens for RootNamespace {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.name;
        let variables = vec_to_token_stream_2(&self.variables);
        let namespaces = vec_to_token_stream_2(&self.namespaces);

        let init_variables = self
            .variables
            .iter()
            .map(|var| {
                let name = &var.name;
                let var_meta = vec_to_token_stream_2(&var.meta);

                quote!(
                    #(#var_meta)*
                    #name();
                )
            })
            .collect::<Vec<TokenStream2>>();
        let init_namespaces = self
            .namespaces
            .iter()
            .map(|ns| {
                let name = &ns.name;
                let ns_meta = vec_to_token_stream_2(&ns.meta);

                quote!(
                    #(#ns_meta)*
                    #name::init();
                )
            })
            .collect::<Vec<TokenStream2>>();

        let inner_meta: Vec<TokenStream2> = if name.is_none() {
            vec![]
        } else if self.meta.is_empty() {
            vec![quote!(#![allow(non_snake_case)])]
        } else {
            vec_to_token_stream_2(&self.meta)
        };

        let inner_rules = quote! {
            #(#inner_meta)*

            #(#namespaces)*

            #(#variables)*

            pub fn init() {
                #(#init_variables)*
                #(#init_namespaces)*
            }
        };

        tokens.append_all(match self.name.as_ref() {
            None => inner_rules,
            Some(name) => quote! {
                pub mod #name {
                    #inner_rules
                }
            },
        });
    }
}

impl ToTokens for Namespace {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.name;
        let variables = vec_to_token_stream_2(&self.variables);
        let namespaces = vec_to_token_stream_2(&self.namespaces);
        let meta = vec_to_token_stream_2(&self.meta);

        let init_variables = self
            .variables
            .iter()
            .map(|var| {
                let name = &var.name;
                let var_meta = vec_to_token_stream_2(&var.meta);

                quote!(
                    #(#var_meta)*
                    #name();
                )
            })
            .collect::<Vec<TokenStream2>>();
        let init_namespaces = self
            .namespaces
            .iter()
            .map(|ns| {
                let name = &ns.name;
                let ns_meta = vec_to_token_stream_2(&ns.meta);

                quote!(
                    #(#ns_meta)*
                    #name::init();
                )
            })
            .collect::<Vec<TokenStream2>>();

        tokens.append_all(quote!(
            #(#meta)*
            pub mod #name {
                #(#namespaces)*

                #(#variables)*

                pub fn init() {
                    #(#init_variables)*
                    #(#init_namespaces)*
                }
            }
        ))
    }
}

impl ToTokens for Variable {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let ty = &self.ty;
        let name = &self.name;
        let env_name = &self
            .env_name
            .clone()
            .unwrap_or(name.to_string().to_uppercase());
        let meta = vec_to_token_stream_2(&self.meta);

        let get_variable: TokenStream2 = if self.concat_parts.is_some() {
            let concat_parts = self.concat_parts.as_ref().unwrap();
            quote! {{
                let value_parts: Vec<String> = vec!(#(#concat_parts),*);
                let value = value_parts.join("");
                ::std::env::set_var(#env_name, value.as_str());
                value
            }}
        } else if self.initial.is_some() {
            let initial = self.initial.as_ref().unwrap();
            quote!(::itconfig::get_env_or_set_default(#env_name, #initial))
        } else if is_option_type(&self.ty) {
            quote!(::itconfig::maybe_get_env(#env_name))
        } else {
            quote!(::itconfig::get_env_or_panic(#env_name))
        };

        if self.is_static {
            tokens.append_all(quote!(
                #(#meta)*
                pub fn #name() -> #ty {
                    ::lazy_static::lazy_static! {
                        static ref #name: #ty = #get_variable;
                    }

                    (*#name).clone()
                }
            ));
        } else {
            tokens.append_all(quote!(
                #(#meta)*
                pub fn #name() -> #ty {
                    #get_variable
                }
            ));
        }
    }
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

fn is_option_type(ty: &Type) -> bool {
    match ty {
        Type::Path(ty_path) => {
            ty_path.qself.is_none() && is_option_path_ident(path_ident(&ty_path.path))
        }
        _ => false,
    }
}
