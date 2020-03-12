use proc_macro2::TokenStream as TokenStream2;
use syn::{Type, Expr, Ident, Attribute};


pub struct RootNamespace {
    pub name: Option<Ident>,
    pub variables: Vec<Variable>,
    pub namespaces: Vec<Namespace>,
    pub meta: Vec<Attribute>,
}


pub struct Namespace {
    pub name: Ident,
    pub variables: Vec<Variable>,
    pub namespaces: Vec<Namespace>,
    pub env_prefix: Option<String>,
    pub meta: Vec<Attribute>,
}


pub struct Variable {
    pub is_static: bool,
    pub name: Ident,
    pub ty: Type,
    pub initial: Option<Expr>,
    pub concat_parts: Option<Vec<TokenStream2>>,
    pub env_name: Option<String>,
    pub meta: Vec<Attribute>,
}
