use proc_macro::TokenStream;
use quote::{__private::TokenStream as QuoteTokenStream, quote};
use syn::{self, Data, Ident, Variant};

#[proc_macro_derive(Message, attributes(message))]
pub fn derive_message(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    impl_message_macro(&ast)
}

fn bytes_messages(ast: &syn::DeriveInput) -> (Vec<&QuoteTokenStream>, Vec<&Ident>) {
    let variants: Vec<&Variant> = match &ast.data {
        Data::Enum(ref data_enum) => data_enum.variants.iter().collect(),
        _ => unimplemented!("#[derive(Message)] expects enum"),
    };

    variants.iter().map(|variant| {
        let message : &Ident= &variant.ident;
        let bytes = variant
            .attrs
            .iter()
            .find_map(|attr| match attr.path.is_ident("message") {
                true => Some(&attr.tokens),
                false => None,
            })
            .expect("#[derive(Message)] expects attribute macro #[message(...)] on each variant, found none");
        (bytes, message)
    }).unzip()
}

fn impl_message_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let (bytes, messages) = bytes_messages(ast);

    let expanded = quote! {
        impl Message for #name {
            fn to_bytes(&self) -> Bytes {
                Bytes::from(*self)
            }
        }

        impl TryFrom<Bytes> for #name {
            type Error = MessageError;

            fn try_from(value: Bytes) -> Result<Self, Self::Error> {
                #[allow(unused_parens)]
                match value {
                    #( #bytes => Ok(Self::#messages), )*
                    bytes => Err(MessageError::UnknownMessage(bytes)),
                }
            }
        }

        impl From<#name> for Bytes {
            fn from(value: #name) -> Self {
                #[allow(unused_parens)]
                match value {
                    #( #name::#messages => #bytes, )*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
