use proc_macro::TokenStream;
use quote::format_ident;

#[proc_macro]
pub fn generate_params(items: TokenStream) -> TokenStream {
    let items = items.to_string();
    let items: Vec<&str> = items.split(" ").collect();
    let name = format_ident!("{}", items[0]);
    let curr_param = format_ident!("{}", items[1]);
    // let par_type = format_ident!("{}", items[2].to_string());
    let param_type = format_ident!("{}", items[2]);


    let tokens = quote::quote! {
        let #name = fn_val.get_nth_param(#curr_param).unwrap().#param_type();
        println!("{}", #param_type)
    };

    tokens.into()
}
