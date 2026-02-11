use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn tagged(_attr: TokenStream, item: TokenStream) -> TokenStream {
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
        tags: i32
    });

    let expanded = quote! {
        #input

        impl MyTrait for #struct_name {
            fn injected_field(&self) -> &i32 {
                &self.tags
            }
        }
    };

    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
