use prost_types::DescriptorProto;
use prost_types::FieldDescriptorProto;
use quote::__private::TokenStream;

pub fn from_pg_row(message: &DescriptorProto) -> TokenStream {
    let message_name = quote::format_ident!("{}", message.name());
    let fields = &message
        .field
        .clone()
        .into_iter()
        .map(|field| {
            let FieldDescriptorProto {
                type_name, name, ..
            } = field;

            let original_name = name.unwrap_or_default();

            let formatted = format!("{}", original_name);
            let name = quote::format_ident!("{}", original_name);
            let data_type = match original_name.as_str() {
                "id" => quote::quote! { i32 },
                "created_at" | "updated_at" => quote::quote! { DateTime<Utc> },
                _ => match field.r#type.unwrap() {
                    5 => quote::quote! { i32 },
                    9 => quote::quote! { String },
                    14 => {
                        let enum_name = quote::format_ident!(
                            "{}",
                            type_name
                                .unwrap()
                                .split(".")
                                .collect::<Vec<_>>()
                                .last()
                                .unwrap()
                        );
                        quote::quote! {
                            #enum_name
                        }
                    }
                    _ => quote::quote! {
                        unknown
                    },
                },
            };

            let into = match original_name.as_str() {
                "created_at" | "updated_at" => quote::quote! { .to_rfc3339() },
                _ => match field.r#type.unwrap() {
                    14 => quote::quote! { .into() },
                    _ => quote::quote! {},
                },
            };

            quote::quote! { #name: row.get::<#data_type, _>(#formatted)#into, }
        })
        .collect::<Vec<_>>();

    quote::quote! {
        impl From<PgRow> for #message_name {
            fn from(row: PgRow) -> Self {
                #message_name {
                    #(#fields)*
                }
            }
        }
    }
}
