use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(Describe)]
pub fn derive_describe(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let fields = match &ast.data {
        syn::Data::Struct(DataStruct {
            struct_token: _,
            fields: Fields::Named(field),
            semi_token: _,
        }) => &field.named,
        _ => panic!("Only for structs"),
    };

    let fields = fields.iter().map(|field| field.ident.as_ref().unwrap());

    let generated = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!(#name))
                #(
                    .field(stringify!(#fields), &self.#fields)
                )*
                .finish()
            }
        }
        impl Describe for #name {
            fn describe(&self) -> String {
                format!("{}", &self)
            }
        }
    };
    generated.into()
}

// Example Test
//#[test]
//fn test_example() {
//    #[derive(Describe)]
//    struct Person {
//        name: String,
//        age: u32,
//    }
//
//    let person = Person {
//        name: "Alice".to_string(),
//        age: 30,
//    };
//
//    assert_eq!(person.describe(), "Person { name: \"Alice\", age: 30 }");
//}
