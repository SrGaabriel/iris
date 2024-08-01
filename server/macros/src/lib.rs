use proc_macro::TokenStream;
use syn::{LitInt, parse_macro_input};
use syn::parse::Parser;
use quote::quote;
use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn packet(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut id: Option<LitInt> = None;
    {
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
        #[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        #struct_item
        
        impl crate::server::messages::PacketStaticId for #ident {
            fn get_id() -> i32 {
                #id
            }
        }

        impl crate::server::messages::Packet for #ident {
            fn get_id(&self) -> i32 {
                #id
            }

            fn encode_data(&self) -> Vec<u8> {
                let mut buffer = Vec::new();
                serde_json::to_writer(&mut buffer, self).expect("Failed to encode packet data");
                buffer
            }

            fn decode_data(buffer: &[u8]) -> Result<Self, serde_json::Error> {
                serde_json::from_slice(buffer)
            }
        }

        unsafe impl Send for #ident {}
        unsafe impl Sync for #ident {}
    };

    TokenStream::from(expanded)
}