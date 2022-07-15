use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Request)]
pub fn request_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_request_macro(&ast)
}

fn impl_request_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Request for #name {
            fn to_bytes(&self) -> [u8; 64] {
                self.to_bytes_impl()
            }

            fn get_id(&self) -> u32 {
                self.device_id
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(Response)]
pub fn response_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_response_macro(&ast)
}

fn impl_response_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Response for #name {
            fn from_bytes(bytes: &[u8; 64]) -> anyhow::Result<Self> {
                Self::from_bytes_impl(bytes)
            }
        }
    };
    gen.into()
}
