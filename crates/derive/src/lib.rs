use syn::parse::Parser;

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

// Report

#[proc_macro_attribute]
pub fn add_report_fields(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream  {
    let mut ast = syn::parse_macro_input!(input as syn::DeriveInput);
    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {           
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    fields.named.push(syn::Field::parse_named.parse2(quote::quote! { pub job_id: crate::job::ID}).unwrap());
                    fields.named.push(syn::Field::parse_named.parse2(quote::quote! { pub cuk: crate::kinds::ComputingUnit }).unwrap());
                    fields.named.push(syn::Field::parse_named.parse2(quote::quote! { pub input: Input }).unwrap());
                    fields.named.push(syn::Field::parse_named.parse2(quote::quote! { pub output: Output }).unwrap());
                }   
                _ => {
                    ()
                }
            }              
            
            return quote::quote! {
                #ast
            }.into();
        }
        _ => panic!("only for struct")
    }
}

#[proc_macro_derive(ImplReport)]
pub fn derive_report_generator(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let syn::DeriveInput { ident, .. } = syn::parse_macro_input!(input);
    let output = quote::quote! {
        impl #ident {
            pub fn new(t: (crate::job::ID, crate::kinds::ComputingUnit, &SerializedFormat, &Vec<SerializedFormat>)) -> Self {
                let (job_id, cuk, input_ser, output_sers) = t;
                let input = Input::ser_from(input_ser);
                let mut output = Output::blank();
                for output_ser in output_sers.iter() {
                    output.append(&mut Output::ser_from(output_ser));
                }
                Self { job_id, cuk, input, output }
            }
        }
    };
    output.into()
}

#[proc_macro_derive(InputFileRequirements)]
pub fn derive_input_file_requirements(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let syn::DeriveInput { ident, .. } = syn::parse_macro_input!(input);
    let output = quote::quote! {
        impl crate::traits::TraitFileRequirements for #ident {}
    };
    output.into()
}


