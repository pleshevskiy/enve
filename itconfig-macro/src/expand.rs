use crate::ast::*;
use crate::utils::{is_option_type, vec_to_token_stream_2};
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens, TokenStreamExt};

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
