use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Params)]
pub fn params_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = ast.ident;

    let fields = match ast.data {
        Data::Struct(data) => data
            .fields
            .into_iter()
            .filter_map(|f| Some((f.ident, f.ty))),
        _ => panic!("unsupported"),
    }
    .into_iter();

    let struct_fields = fields.clone().map(|(field, ty)| {
        quote! {
            #field: Option<#ty>,
        }
    });

    let default_fields = fields.clone().map(|(field, _)| {
        quote! {
            #field: None,
        }
    });

    let setters = fields.map(|(field, ty)| {
        quote! {
           pub fn #field(&mut self, #field: #ty) -> &mut Self {
               self.#field = Some(#field);
               self
           }
        }
    });

    let output = quote! {
        impl #ident {}
    };

    TokenStream::new()
}
