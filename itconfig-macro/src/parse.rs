use crate::ast::*;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseBuffer, ParseStream, Result};
use syn::token::{Brace, Colon, Comma, FatArrow, Lt};
use syn::{
    braced, parenthesized, parse_str, Attribute, Error, Expr, Lit, Meta, MetaList, MetaNameValue,
    NestedMeta, Token, Type,
};

fn fill_env_prefix(prefix: String) -> Box<dyn Fn(Namespace) -> Namespace> {
    Box::new(move |mut ns| {
        let env_prefix = match &ns.env_prefix {
            None => {
                let env_prefix = format!("{}{}_", prefix, ns.name.clone().to_string());
                ns.env_prefix = Some(env_prefix.clone());
                env_prefix
            }
            Some(env_prefix) => env_prefix.clone(),
        };

        if !ns.namespaces.is_empty() {
            ns.namespaces = ns
                .namespaces
                .into_iter()
                .map(fill_env_prefix(ns.env_prefix.clone().unwrap()))
                .collect()
        }

        if !ns.variables.is_empty() {
            ns.variables = ns
                .variables
                .into_iter()
                .map(|mut var| {
                    if var.env_name.is_none() {
                        var.env_name = Some(
                            format!("{}{}", env_prefix.clone(), &var.name.to_string())
                                .to_uppercase(),
                        );
                    }
                    var
                })
                .collect()
        }

        ns
    })
}

fn parse_namespace_content(
    input: &ParseBuffer,
    variables: &mut Vec<Variable>,
    namespaces: &mut Vec<Namespace>,
) -> Result<()> {
    let attributes: Vec<Attribute> = input.call(Attribute::parse_outer)?;
    if input.peek2(Brace) {
        let mut namespace: Namespace = input.parse()?;

        for attr in attributes {
            if attr.path.is_ident("env_prefix") {
                namespace.env_prefix = parse_attribute(attr, "env_prefix", &namespace.env_prefix)?;
            } else {
                namespace.meta.push(attr);
            }
        }

        namespaces.push(namespace);
    } else {
        let mut variable: Variable = input.parse()?;

        for attr in attributes {
            if attr.path.is_ident("env_name") {
                variable.env_name = parse_attribute(attr, "env_name", &variable.env_name)?;
            } else {
                variable.meta.push(attr);
            }
        }

        variables.push(variable);
    }

    Ok(())
}

fn parse_attribute(
    attr: Attribute,
    name: &'static str,
    var: &Option<String>,
) -> Result<Option<String>> {
    if var.is_some() {
        let message = format!("You cannot use {} meta twice", &name);
        return Err(Error::new_spanned(attr, message));
    }

    match attr.parse_meta()? {
        Meta::NameValue(MetaNameValue {
            lit: Lit::Str(lit_str),
            ..
        }) => Ok(Some(lit_str.value())),
        _ => {
            let message = format!("expected #[{} = \"...\"]", &name);
            Err(Error::new_spanned(attr, message))
        }
    }
}

impl Parse for RootNamespace {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut name: Option<Ident> = None;
        let mut with_module = true;
        let mut meta: Vec<Attribute> = vec![];

        let attributes: Vec<Attribute> = input.call(Attribute::parse_inner)?;
        for attr in attributes {
            if attr.path.is_ident("config") {
                match attr.parse_meta()? {
                    Meta::List(MetaList { nested, .. }) => {
                        let message =
                            format!("expected #[config(name = \"...\")] or #[config(unwrap)]");
                        match nested.first().unwrap() {
                            NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                                path,
                                lit: Lit::Str(lit_str),
                                ..
                            })) => {
                                if path.is_ident("name") {
                                    name = Some(Ident::new(&lit_str.value(), Span::call_site()));
                                } else {
                                    Err(Error::new_spanned(attr, message))?;
                                }
                            }
                            NestedMeta::Meta(Meta::Path(path)) => {
                                if path.is_ident("unwrap") {
                                    name = None;
                                    with_module = false;
                                } else {
                                    Err(Error::new_spanned(attr, message))?;
                                }
                            }
                            _ => {
                                Err(Error::new_spanned(attr, message))?;
                            }
                        }
                    }
                    _ => {
                        let message = format!("expected #[config(...)]");
                        Err(Error::new_spanned(attr, message))?;
                    }
                }
            } else {
                meta.push(attr);
            }
        }

        if with_module && name.is_none() {
            name = Some(Ident::new("config", Span::call_site()));
        }

        let mut variables: Vec<Variable> = vec![];
        let mut namespaces: Vec<Namespace> = vec![];
        while !input.is_empty() {
            parse_namespace_content(&input, &mut variables, &mut namespaces)?;
        }

        let prefix = String::new();
        let namespaces = namespaces
            .into_iter()
            .map(fill_env_prefix(prefix.clone()))
            .collect();

        Ok(RootNamespace {
            name,
            variables,
            namespaces,
            meta,
        })
    }
}

impl Parse for Namespace {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let mut variables: Vec<Variable> = vec![];
        let mut namespaces: Vec<Namespace> = vec![];

        let content;
        braced!(content in input);
        while !content.is_empty() {
            parse_namespace_content(&content, &mut variables, &mut namespaces)?;
        }

        input.parse::<Comma>().ok();

        Ok(Namespace {
            name,
            variables,
            namespaces,
            env_prefix: None,
            meta: vec![],
        })
    }
}

impl Parse for Variable {
    fn parse(input: ParseStream) -> Result<Self> {
        let is_static = input.parse::<Token![static]>().ok().is_some();
        let name: Ident = input.parse()?;

        let is_concat = input.peek(Lt);
        let mut concat_parts = None;
        let mut initial = None;

        let ty: Type = if is_concat {
            parse_str("String")?
        } else if input.peek(Colon) {
            input.parse::<Colon>()?;
            input.parse()?
        } else {
            parse_str("&'static str")?
        };

        if is_concat {
            input.parse::<Lt>()?;

            let content;
            parenthesized!(content in input);

            let mut tmp_vec: Vec<TokenStream2> = vec![];
            while !content.is_empty() {
                if content.peek(Ident::peek_any) {
                    let concat_var: Variable = content.parse()?;
                    let name = &concat_var.name;
                    let env_name = &concat_var.env_name.clone().unwrap_or(name.to_string());

                    let get_variable = if concat_var.initial.is_some() {
                        let initial = concat_var.initial.as_ref().unwrap();
                        quote!(::itconfig::get_env_or_set_default(#env_name, #initial))
                    } else {
                        quote!(::itconfig::get_env_or_panic(#env_name))
                    };

                    tmp_vec.push(get_variable);
                } else {
                    let part: Lit = content.parse()?;
                    tmp_vec.push(quote!(#part.to_string()));
                }
                content.parse::<Comma>().ok();
            }
            concat_parts = Some(tmp_vec);
        } else {
            initial = input
                .parse::<FatArrow>()
                .ok()
                .and_then(|_| input.parse::<Expr>().ok());
        };

        input.parse::<Comma>().ok();

        Ok(Variable {
            is_static,
            name,
            ty,
            initial,
            concat_parts,
            env_name: None,
            meta: vec![],
        })
    }
}
