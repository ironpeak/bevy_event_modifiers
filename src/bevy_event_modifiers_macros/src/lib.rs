extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, AngleBracketedGenericArguments, Data,
    DeriveInput, Field, GenericArgument, Ident, PathArguments, Type,
};

#[proc_macro_derive(EventModifier)]
pub fn derive_event_modifier(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let (user_impl_generics, _, _) = ast.generics.split_for_impl();

    let struct_name = &ast.ident;

    let data = match ast.data {
        Data::Struct(data) => data,
        _ => panic!("Only structs are supported"),
    };

    let input_field = data
        .fields
        .iter()
        .find(|field| {
            field
                .ident
                .as_ref()
                .map(|ident| ident == "input")
                .unwrap_or(false)
        })
        .expect("No input field found");

    let priority_field = data
        .fields
        .iter()
        .find(|field| {
            field
                .ident
                .as_ref()
                .map(|ident| ident == "priority")
                .unwrap_or(false)
        })
        .expect("No priority field found");

    let context_fields: Vec<&Field> = data
        .fields
        .iter()
        .filter(|field| {
            field
                .ident
                .as_ref()
                .map(|ident| ident != "input" && ident != "priority" && ident != "output")
                .unwrap_or(false)
        })
        .collect();

    let output_field = data
        .fields
        .iter()
        .find(|field| {
            field
                .ident
                .as_ref()
                .map(|ident| ident == "output")
                .unwrap_or(false)
        })
        .expect("No output field found");

    let input_ty = &input_field.ty;

    let priority_ty = &priority_field.ty;

    let struct_context_name = Ident::new(&format!("{}Context", struct_name), Span::call_site());
    let struct_context_fields = context_fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        let field_ty = &field.ty;
        quote! {
            pub #field_ident: #field_ty
        }
    });

    let system_context_params = context_fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        let field_ty = match &field.ty {
            Type::Path(reference) => match reference.path.segments[0].ident.to_string().as_str() {
                "Query" => {
                    let args = match &reference.path.segments[0].arguments {
                        PathArguments::AngleBracketed(args) => {
                            let mut param_args: Punctuated<GenericArgument, Comma> =
                                Punctuated::new();
                            for arg in &args.args {
                                if matches!(arg, GenericArgument::Lifetime(_)) {
                                    continue;
                                }
                                param_args.push(arg.clone());
                            }
                            param_args
                        }
                        _ => panic!("Only angle bracketed arguments are supported"),
                    };
                    let mut field_ty = field.ty.clone();
                    match &mut field_ty {
                        Type::Path(reference) => {
                            reference.path.segments[0].arguments =
                                PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                    colon2_token: None,
                                    lt_token: Default::default(),
                                    args: args,
                                    gt_token: Default::default(),
                                });
                        }
                        _ => panic!("Only paths are supported"),
                    }
                    field_ty
                }
                _ => field.ty.clone(),
            },
            _ => panic!("Only references are supported"),
        };
        quote! {
            #field_ident: #field_ty
        }
    });

    let struct_modifier_name = Ident::new(&format!("{}Modifier", struct_name), Span::call_site());

    let output_ty = &output_field.ty;

    let output = quote! {
        pub struct #struct_context_name #user_impl_generics {
            #(#struct_context_fields),*
        }

        #[derive(bevy_ecs::prelude::Component)]
        pub struct #struct_modifier_name {
            pub priority: #priority_ty,
            pub modify: fn(&mut #struct_context_name, &mut #output_ty),
        }

        impl Ord for #struct_modifier_name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.priority.cmp(&other.priority)
            }
        }

        impl PartialOrd for #struct_modifier_name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.priority.partial_cmp(&other.priority)
            }
        }

        impl Eq for #struct_modifier_name {

        }

        impl PartialEq for #struct_modifier_name {
            fn eq(&self, other: &Self) -> bool {
                self.priority == other.priority
            }
        }

        impl #user_impl_generics #struct_name #user_impl_generics {
            fn system(
                mut events_in: bevy_ecs::prelude::EventReader<#input_ty>,
                #(#system_context_params),*,
                modifiers: bevy_ecs::prelude::Query<&#struct_modifier_name>,
                mut events_out: bevy_ecs::prelude::EventWriter<#output_ty>,
            ) {
                let modifiers = modifiers
                    .iter()
                    .sort::<&#struct_modifier_name>()
                    .collect::<Vec<_>>();
                for event in events_in.read() {
                    for modifier in &modifiers {
                    }
                }
            }
        }

        impl #user_impl_generics bevy_event_modifiers::prelude::EventModifier for #struct_name #user_impl_generics {
            fn register_type(app: &mut bevy_app::prelude::App) -> &mut bevy_app::prelude::App {
                app.add_event::<#input_ty>();
                app.add_event::<#output_ty>();
                app.add_systems(bevy_app::prelude::Update, #struct_name ::system);
                app
            }
        }
    };

    TokenStream::from(output)
}
