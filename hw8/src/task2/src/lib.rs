use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};

#[proc_macro]
pub fn even_len_name_func_invoke(input: TokenStream) -> TokenStream {
    let even_func_name_idents = input
        .into_iter()
        .map(|name| name.to_string().replace('\"', ""))
        .filter(|x| x.to_string().len() % 2 == 0)
        .map(|name| Ident::new(&name, Span::call_site()))
        .collect::<Vec<_>>();
    TokenStream::from(quote::quote! {
    (  #(#even_func_name_idents() , )* )
    })
}

#[proc_macro]
pub fn gen_dummy_function(item: TokenStream) -> TokenStream {
    let length = item.to_string().parse().unwrap();
    let func_name = "f".to_string() + &*"o".to_string().repeat(length);
    let func_src = format!("fn {func_name}() -> u32 {{ {} }}", length + 1);
    func_src.parse().unwrap()
}
