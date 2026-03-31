use proc_macro::TokenStream;
mod macro_impl;

#[proc_macro_attribute]
/// Generate the nodes and subtypes for the provided tree-sitter grammar.
/// The resulting types inherit the visibility and ALL attributes of the parent type.
pub fn generate_nodes(attr: TokenStream, item: TokenStream) -> TokenStream {
    let crate_name = syn::parse_macro_input!(attr as syn::Ident);
    macro_impl::macro_impl(crate_name, item.into()).into()
}
