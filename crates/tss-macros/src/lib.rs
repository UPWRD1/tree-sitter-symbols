use proc_macro::TokenStream;
mod macro_impl;

#[proc_macro_attribute]
pub fn generate_nodes(attr: TokenStream, item: TokenStream) -> TokenStream {
    let crate_name = syn::parse_macro_input!(attr as syn::Ident).to_string();
    macro_impl::generate_nodes(crate_name, item.into()).into()
}
