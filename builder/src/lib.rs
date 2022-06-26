use proc_macro::TokenStream;
use syn::{
    parse_macro_input,
    DeriveInput,
};
use quote::{ quote, format_ident };


#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_name = input.ident;
    let command_type_name = format_ident!("{}Builder", type_name);
    let expanded = quote! {
        pub struct #command_type_name {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_div: Option<String>,
        }

        impl #type_name {
            pub fn builder() -> #command_type_name {
                #command_type_name {
                    executable: None,
                    args: None,
                    env: None,
                    current_div: None,
                }
            }
        }
    };

    TokenStream::from(expanded)
}
