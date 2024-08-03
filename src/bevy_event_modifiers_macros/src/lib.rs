extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Attribute, Data, DeriveInput, Ident, Meta, Token, Type,
};

#[derive(Debug)]
struct EventModifierArg {
    pub(crate) name: Ident,
    pub(crate) value: Type,
}

impl Parse for EventModifierArg {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let name = input.parse()?;
        input.parse::<Token![=]>()?;
        let value = input.parse()?;
        Ok(Self { name, value })
    }
}

struct EventModifierAttributes {
    component: Type,
    input: Type,
    metadata: Type,
    output: Type,
    priority: Type,
}

fn parse_attributes(attrs: &[Attribute]) -> EventModifierAttributes {
    for attr in attrs {
        match &attr.meta {
            Meta::List(list) => {
                for segment in &list.path.segments {
                    if segment.ident != "modifier" {
                        continue;
                    }
                }
            }
            _ => continue,
        }
        let mut component = None;
        let mut input = None;
        let mut metadata = None;
        let mut output = None;
        let mut priority = None;
        for arg in attr
            .parse_args_with(
                Punctuated::<EventModifierArg, syn::Token![,]>::parse_separated_nonempty,
            )
            .expect("Failed to parse arguments")
        {
            match arg.name.to_string().as_str() {
                "component" => component = Some(arg.value),
                "input" => input = Some(arg.value),
                "metadata" => metadata = Some(arg.value),
                "output" => output = Some(arg.value),
                "priority" => priority = Some(arg.value),
                _ => panic!("Unknown argument `{}`", arg.name),
            }
        }
        if let (Some(component), Some(input), Some(metadata), Some(output), Some(priority)) =
            (component, input, metadata, output, priority)
        {
            return EventModifierAttributes {
                component,
                input,
                metadata,
                output,
                priority,
            };
        }
    }
    panic!("Missing required attribute `modifier` with arguments `input`, `metadata`, `modifier`, `output`, `priority`")
}

#[proc_macro_derive(EventModifierContext, attributes(modifier))]
pub fn derive_event_modifier_context(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let attributes = parse_attributes(&ast.attrs);

    let (user_impl_generics, _, _) = ast.generics.split_for_impl();

    let struct_name = &ast.ident;

    let data = match ast.data {
        Data::Struct(data) => data,
        _ => panic!("Only structs are supported"),
    };

    let component_ty = attributes.component;
    let input_ty = attributes.input;
    let priority_ty = attributes.priority;
    let metadata_ty = attributes.metadata;
    let output_ty = attributes.output;

    let system_param_names = data.fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        quote! {
            #field_ident,
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
            #field_ident: #field_ty,
        }
    });

    let output = quote! {
        #[derive(Component)]
        pub struct #component_ty {
            pub priority: #priority_ty,
            pub modify: fn(&mut #struct_name, &mut #metadata_ty, &mut #output_ty),
        }

        impl Ord for #component_ty {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.priority.cmp(&other.priority)
            }
        }

        impl PartialOrd for #component_ty {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.priority.partial_cmp(&other.priority)
            }
        }

        impl Eq for #component_ty {

        }

        impl PartialEq for #component_ty {
            fn eq(&self, other: &Self) -> bool {
                self.priority == other.priority
            }
        }

        impl #user_impl_generics #struct_name #user_impl_generics {
            pub fn system(
                mut p_events_in: EventReader<#input_ty>,
                #(#system_params)*
                p_modifiers: Query<&#component_ty>,
                mut p_events_out: EventWriter<#output_ty>,
            ) {
                let mut context = #struct_name {
                    #(#system_param_names)*
                };
                let modifiers = p_modifiers
                    .iter()
                    .sort::<&#component_ty>()
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
            fn register_type(app: &mut App) -> &mut App {
                app.add_event::<#input_ty>();
                app.add_event::<#output_ty>();
                app.add_systems(Update, #struct_name ::system.run_if(on_event::<#input_ty>()));
                app
            }
        }
    };

    TokenStream::from(output)
}
