use std::collections::HashMap;

use prost_types::{DescriptorProto, MethodDescriptorProto, ServiceDescriptorProto};
use quote::__private::TokenStream;

use crate::proto_request_name::proto_request_name;
use crate::proto_request_params::proto_request_params;
use crate::CodegenPackage;

pub fn proto_service_list(
    service: &ServiceDescriptorProto,
    messages: &HashMap<&str, DescriptorProto>,
    package: &CodegenPackage,
) -> TokenStream {
    if package.list.is_none() {
        return quote::quote! {};
    }

    let message = messages.get(&package.message).unwrap();

    let action = &service
        .method
        .clone()
        .into_iter()
        .find(|m| m.name() == package.list.unwrap());

    if let Some(action) = action {
        let MethodDescriptorProto { output_type, .. } = action.clone();

        let output_type = output_type.unwrap();

        let res = messages
            .get(output_type.split(".").collect::<Vec<_>>().last().unwrap())
            .expect("output not found");

        let res_name = quote::format_ident!("{}", res.name());
        let res_field = &res.field.first().unwrap();

        let message_name = quote::format_ident!("{}", message.name());
        let res_field_name = quote::format_ident!("{}", res_field.name());

        let proto_service_name = proto_request_name(action, messages);
        let proto_service_params = proto_request_params(action, messages);

        return quote::quote! {
            async fn #proto_service_name {
                let MeExtension { user_id } = request.extensions().get::<MeExtension>().unwrap();
                #proto_service_params

                let mut query_builder = #message_name::query();
                query_builder.where_raw("((permissions ->> CAST(${index} as text))::integer > 0 OR (permissions ->> '0')::integer > 0)", user_id);

                let sql = query_builder.select_query();

                let #res_field_name = sqlx::query_with(sql.0.as_str(), sql.1)
                    .fetch_all(&self.pool)
                    .await
                    .expect("error")
                    .into_iter()
                    .map(|row| row.into())
                    .collect();

                Ok(Response::new(#res_name { #res_field_name }))
            }
        };
    }

    return quote::quote! {};
}
