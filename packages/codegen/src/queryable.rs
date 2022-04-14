use prost_types::DescriptorProto;
use quote::__private::TokenStream;

use crate::CodegenPackage;

pub fn queryable(message: &DescriptorProto, package: &CodegenPackage) -> TokenStream {
    let message_name = quote::format_ident!("{}", message.name());
    let table_name = format!("{}", package.table);

    let queryable_fields = &message
        .field
        .clone()
        .into_iter()
        .map(|field| format!("{}", field.name()))
        .collect::<Vec<_>>();

    quote::quote! {
        impl Queryable for #message_name {
            fn fields() -> Vec<&'static str> {
               vec![ #(#queryable_fields ,)* ]
            }

            fn table() -> &'static str {
                #table_name
            }

            fn query() -> QueryBuilder {
                let mut query = QueryBuilder::new( #message_name::table() );
                query.fields( #message_name::fields() );

                query
            }
        }
    }
}
