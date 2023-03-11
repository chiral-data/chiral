
#[proc_macro_derive(Serialization)]
pub fn derive_serial(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let syn::DeriveInput { ident, .. } = syn::parse_macro_input!(input);
    let output = quote::quote! {
        impl Serialization for #ident {
            fn ser_to(&self) -> SerializedFormat { serde_json::to_string(self).unwrap() }
            fn ser_from(content: &SerializedFormat) -> Self { serde_json::from_str(content).unwrap() }
        }
    };
    output.into()
}
