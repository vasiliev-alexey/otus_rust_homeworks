use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use std::vec;
use syn::parse_quote;
use syn::Stmt::Expr;

#[proc_macro]
pub fn macro2(input: TokenStream) -> TokenStream {
    println!("R-------------------------------------: {}", input);

    let input = input.into_iter().collect::<Vec<_>>();
    //let name = input[0].to_string().replace("\"", "");

    // let ident = Ident::new(&*name, Span::call_site());
    // for x in input {
    //     let name = x.to_string().replace("\"", "");
    //     println!("name:{}", name);
    //     let ident = Ident::new(&*name, Span::call_site());
    //
    // }
    // let mut field_constructors =
    //     syn::punctuated::Punctuated::<syn::Ident, syn::Token![,]>::default();

    let mut field_constructors = vec![];
    println!("R: {}", 1);
    for arg in input.iter().filter(|x| x.to_string().len() % 2 == 0) {
        let name = arg.to_string().replace("\"", "");
        println!("R1: {}", arg);
        let ident = Ident::new(&*(name), Span::call_site());

        field_constructors.push(ident)
        //  let mut res: i32;

        // #ident();
        // println!("R2: {}", res);

        //  #ident()
    }
    println!("R: {}", 2);
    //let r = parsed.value();
    //  let ident = Ident::new(&*r, Span::call_site());
    //  field_constructors.push(1);
    // field_constructors.push(2);
    // let _st: proc_macro2::TokenStream = parse_quote! {
    //    //    println!("{} started");
    //      //  ( #ident() , #ident() , #ident())
    //
    //    // (                #ident()    ,#ident()           )
    // (  1,2,3)
    //  // (#ident(),2,3)
    //
    //     //   println!("{} finished");
    //     };
    // _st.into()
    let str11 = "test()";
    println!("R: {}", 33);
    //     // (#field_constructors(),)
    //   let nums: Vec<_> = field_constructors.iter().collect();
    let ret = quote::quote! {
    (  #(#field_constructors() , )* )

    };
    TokenStream::from(ret)
}
