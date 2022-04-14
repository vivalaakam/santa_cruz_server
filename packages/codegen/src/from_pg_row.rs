use prost_types::DescriptorProto;
use quote::__private::TokenStream;

pub fn from_pg_row(message: &DescriptorProto) -> TokenStream {
    let message_name = quote::format_ident!("{}", message.name());
    let fields = &message
        .field
        .clone()
        .into_iter()
        .map(|field| {
            let name = quote::format_ident!("{}", field.name());
            let formatted = format!("{}", field.name());
            let data_type = match field.name() {
                "id" => quote::quote! { i32 },
                "created_at" | "updated_at" => quote::quote! { DateTime<Utc> },
                _ => match field.r#type.unwrap() {
                    9 => quote::quote! { String },
                    _ => quote::quote! { unknown },
                },
            };

            let into = match field.name() {
                "created_at" | "updated_at" => quote::quote! { .to_rfc3339() },
                _ => quote::quote! {},
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
