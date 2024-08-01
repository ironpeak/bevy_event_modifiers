extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Ident, Type};

#[proc_macro_derive(EventModifierContext)]
pub fn derive_event_modifier_context(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let (user_impl_generics, _, _) = ast.generics.split_for_impl();

    let struct_name = &ast.ident;

    let data = match ast.data {
        Data::Struct(data) => data,
        _ => panic!("Only structs are supported"),
    };

    let struct_name_str = struct_name.to_string();

    let base_name = struct_name_str
        .strip_suffix("ModifierContext")
        .expect("Struct name must end with 'ModifierContext'");

    let input_ty = Ident::new(&format!("{}In", base_name), Span::call_site());
    let modifier_ty = Ident::new(&format!("{}Modifier", base_name), Span::call_site());
    let priority_ty = Ident::new(&format!("{}ModifierPriority", base_name), Span::call_site());
    let metadata_ty = Ident::new(&format!("{}ModifierMetadata", base_name), Span::call_site());
    let output_ty = Ident::new(&format!("{}Out", base_name), Span::call_site());

    let system_param_names = data.fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        quote! {
            #field_ident
        }
    });

    let system_params = data.fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().expect("Field must have an identifier");

        // TODO: would prefer to do this logic using AST
        let field_ty_str = format!("{}", field.ty.to_token_stream())
            .replace("\n", "")
            .replace("\r\n", "")
            .replace("& ", "&")
            .replace(" & ", "&")
            .replace("< ", "<")
            .replace(" <", "<")
            .replace("> ", ">")
            .replace(" >", ">")
            .replace(", ", ",")
            .replace(" ,", ",")
            .replace("'s ", "")
            .replace("'w ", "")
            .replace("'s,", "")
            .replace("'w,", "");

        let field_ty = syn::parse_str::<Type>(&field_ty_str).expect("Failed to parse type");

        quote! {
            #field_ident: #field_ty
        }
    });

    let output = quote! {
        #[derive(bevy_ecs::prelude::Component)]
        pub struct #modifier_ty {
            pub priority: #priority_ty,
            pub modify: fn(&mut #struct_name, &mut #metadata_ty, &mut #output_ty),
        }

        impl Ord for #modifier_ty {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.priority.cmp(&other.priority)
            }
        }

        impl PartialOrd for #modifier_ty {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.priority.partial_cmp(&other.priority)
            }
        }

        impl Eq for #modifier_ty {

        }

        impl PartialEq for #modifier_ty {
            fn eq(&self, other: &Self) -> bool {
                self.priority == other.priority
            }
        }

        impl #user_impl_generics #struct_name #user_impl_generics {
            pub fn system(
                mut p_events_in: bevy_ecs::prelude::EventReader<#input_ty>,
                #(#system_params),*,
                p_modifiers: bevy_ecs::prelude::Query<&#modifier_ty>,
                mut p_events_out: bevy_ecs::prelude::EventWriter<#output_ty>,
            ) {
                let mut context = #struct_name {
                    #(#system_param_names),*,
                };
                let modifiers = p_modifiers
                    .iter()
                    .sort::<&#modifier_ty>()
                    .collect::<Vec<_>>();
                for event in p_events_in.read() {
                    let mut metadata = #metadata_ty ::init(&mut context, event);
                    let mut event_out = #output_ty ::init(&mut context, event);
                    for modifier in &modifiers {
                        (modifier.modify)(&mut context, &mut metadata, &mut event_out);
                    }
                    p_events_out.send(event_out);
                }
            }
        }

        impl #user_impl_generics bevy_event_modifiers::prelude::EventModifierContext for #struct_name #user_impl_generics {
            fn register_type(app: &mut bevy_app::prelude::App) -> &mut bevy_app::prelude::App {
                app.add_event::<#input_ty>();
                app.add_event::<#output_ty>();
                app.add_systems(bevy_app::prelude::Update, #struct_name ::system.run_if(on_event::<#input_ty>()));
                app
            }
        }
    };

    TokenStream::from(output)
}
