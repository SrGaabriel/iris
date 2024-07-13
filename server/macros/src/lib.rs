use proc_macro::TokenStream;
use syn::{Attribute, Expr, LitInt, parse_macro_input, Token};
use syn::parse::{Parse, Parser};
use quote::quote;
use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn packet(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut id: Option<LitInt> = None;
    let attributes = {
        let parser = syn::meta::parser(|meta| {
            if meta.path.is_ident("id") {
                id = Some(meta.value()?.parse()?);
                Ok(())
            } else {
                Err(syn::Error::new(meta.path.span(), "Unknown attribute"))
            }
        });
        parser.parse(args).unwrap()
    };

    let struct_item = parse_macro_input!(item as syn::ItemStruct);
    let ident = struct_item.ident.clone();
    let expanded = quote! {
        #struct_item

        impl Packet for #ident {
            fn get_id(&self) -> i32 {
                #id
            }

            fn encode_data(&self) -> Vec<u8> {
                let mut buffer = Vec::new();
                prost::Message::encode(self, &mut buffer).unwrap();
                buffer
            }

            fn decode_data(buffer: &[u8]) -> Result<Self, prost::DecodeError> {
                prost::Message::decode(buffer)
            }
        }

        unsafe impl Send for #ident {}
        unsafe impl Sync for #ident {}
    };

    TokenStream::from(expanded)
}