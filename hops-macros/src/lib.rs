use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, Type, parse_macro_input};

#[proc_macro_attribute]
pub fn tagged(attr: TokenStream, item: TokenStream) -> TokenStream {
    let tag_type = parse_macro_input!(attr as Type);
    let mut input = parse_macro_input!(item as ItemStruct);
    let struct_name = &input.ident;
    let fields = match &mut input.fields {
        syn::Fields::Named(fields) => fields,
        _ => {
            return syn::Error::new_spanned(
                input,
                "This macro only works on structs with named fields",
            )
            .to_compile_error()
            .into();
        }
    };
    fields.named.push(syn::parse_quote! {
        pub tags: Vec<#tag_type>
    });
    let expanded = quote! {
        #input
        impl Tagged<#tag_type> for #struct_name {
            fn tag(&self) -> &Vec<#tag_type> {
                &self.tags
            }
            fn tag_mut(&mut self) -> &mut Vec<#tag_type> {
                &mut self.tags
            }
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn named(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemStruct);
    let struct_name = &input.ident;
    let fields = match &mut input.fields {
        syn::Fields::Named(fields) => fields,
        _ => {
            return syn::Error::new_spanned(
                input,
                "This macro only works on structs with named fields",
            )
            .to_compile_error()
            .into();
        }
    };
    fields.named.push(syn::parse_quote! {
        pub name_core: NameCore
    });
    let expanded = quote! {
        #input
        impl Named for #struct_name {
            fn name_core(&self) -> &NameCore {
                &self.name_core
            }
            fn name_core_mut(&mut self) -> &mut NameCore {
                &mut self.name_core
            }
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn scored(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemStruct);
    let struct_name = &input.ident;
    let fields = match &mut input.fields {
        syn::Fields::Named(fields) => fields,
        _ => {
            return syn::Error::new_spanned(
                input,
                "This macro only works on structs with named fields",
            )
            .to_compile_error()
            .into();
        }
    };
    fields.named.push(syn::parse_quote! {
        pub score: u32
    });
    let expanded = quote! {
        #input
        impl Score for #struct_name {
            fn score(&self) -> u32 {
                self.score
            }
            fn set_score(&mut self, new_score: u32) {
                self.score = new_score;
            }
        }
    };
    TokenStream::from(expanded)
}
