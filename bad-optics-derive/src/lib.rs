use std::collections::HashMap;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DataStruct, DeriveInput, Field, Ident, Lit, Meta, Type, Visibility};

// TODO add attributes to rename the lens/module/skip making lenses/idk

#[proc_macro_derive(Optics, attributes(mod_name))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let mod_name = Ident::new(&get_mod_name(&input), Span::call_site());
    let container_name = Ident::new(&get_container_name(&input), Span::call_site());

    let expanded = match input.data {
        syn::Data::Struct(s) => expand_struct(s, name, &mod_name, &container_name),
        syn::Data::Enum(_) => todo!("not yet implemented for prisms"),
        syn::Data::Union(_) => panic!("this macro does not work on unions"),
    };

    // Hand the output tokens back to the compiler
    proc_macro::TokenStream::from(expanded)
}

fn get_mod_name(input: &DeriveInput) -> String {
    for i in &input.attrs {
        if let Ok(Meta::NameValue(meta)) = i.parse_meta() {
            if let Some(ident) = meta.path.get_ident() {
                if ident == "mod_name" {
                    if let Lit::Str(a) = meta.lit {
                        return a.value();
                    }
                }
            }
        }
    }

    input.ident.to_string().to_lowercase()
}

fn get_container_name(input: &DeriveInput) -> String {
    for i in &input.attrs {
        if let Ok(Meta::NameValue(meta)) = i.parse_meta() {
            if let Some(ident) = meta.path.get_ident() {
                if ident == "container_name" {
                    if let Lit::Str(a) = meta.lit {
                        return a.value();
                    }
                }
            }
        }
    }

    format!("{}LensContainer", input.ident.to_string())
}

fn expand_struct(
    data: DataStruct,
    name: &Ident,
    mod_name: &Ident,
    container_name: &Ident,
) -> TokenStream {
    let fields = match &data.fields {
        syn::Fields::Named(n) => n.named.iter(),
        syn::Fields::Unnamed(_) => todo!(),
        syn::Fields::Unit => todo!(),
    }
    .filter(|f| matches!(f.vis, Visibility::Public(_)));

    let lens_funcs = fields
        .clone()
        .map(|field| {
            let fname = field.ident.as_ref().unwrap();
            let ty = &field.ty;
            quote! {
                pub fn #fname() ->
                    bad_optics::lenses::Lens<
                        bad_optics::lenses::lens_with_ref::LensWithRef<
                            bad_optics::lenses::Lens<
                                bad_optics::lenses::lens::FuncLens<#name, #ty>
                            >,
                            bad_optics::lenses::Lens<
                                bad_optics::lenses::to::ToRefInner<#name, #ty>
                            >,
                            #name
                        >
                    >
                {
                    bad_optics::field_lens_with_ref!(#name, #fname)
                }
            }
        })
        .collect::<TokenStream>();

    let group_impls = group_by_type(fields)
        .into_iter()
        .map(|(ty, fields)| {
            let lenses = fields
                .into_iter()
                .map(|field| {
                    let fname = field.ident.unwrap();
                    quote! {
                        bad_optics::field_lens_with_ref!(#name, #fname),
                    }
                })
                .collect::<TokenStream>();

            quote! {
                impl bad_optics::has_lens::HasLensOf<#ty> for #name {
                    fn get() ->
                          Vec<
                              bad_optics::lenses::Lens<
                                  bad_optics::lenses::lens_with_ref::LensWithRef<
                                    bad_optics::lenses::Lens<
                                        bad_optics::lenses::lens::FuncLens<#name, #ty>
                                        >,
                                      bad_optics::lenses::Lens<
                                        bad_optics::lenses::to::ToRefInner<#name, #ty>
                                        >,
                                    #name
                                  >
                                >
                            >
                    {
                        vec![
                            #lenses
                        ]
                    }
                }
            }
        })
        .collect::<TokenStream>();

    quote! {
        mod #mod_name {
            use super::*;

            pub struct #container_name;

            impl HasLens for #name {
                type Lenses = #container_name;
            }

            impl #container_name {
                #lens_funcs
            }

            #group_impls
        }
    }
}

fn group_by_type<'a>(fields: impl Iterator<Item = &'a Field>) -> Vec<(Type, Vec<Field>)> {
    let mut map = HashMap::<Type, Vec<Field>>::new();

    for field in fields {
        if let Some(f) = map.get_mut(&field.ty) {
            f.push(field.clone());
        } else {
            map.insert(field.ty.clone(), vec![field.clone()]);
        }
    }

    map.into_iter().collect()
}
