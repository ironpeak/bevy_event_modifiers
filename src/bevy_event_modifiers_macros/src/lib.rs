extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(EventModifier)]
pub fn derive_event_modifier(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    println!("{:?}", ast);

    let struct_name = &ast.ident;

    let data = match ast.data {
        Data::Struct(data) => data,
        _ => panic!("Only structs are supported"),
    };

    let output = quote! {
        impl<'a, 'b, 'c> EventModifier for #struct_name<'a, 'b, 'c> {
            fn register_type(app: &mut App) -> &mut App {
                app
            }
        }
    };

    TokenStream::from(output)
}
