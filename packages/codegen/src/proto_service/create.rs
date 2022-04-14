use std::collections::HashMap;

use prost_types::{DescriptorProto, ServiceDescriptorProto};
use quote::__private::TokenStream;

use crate::naive_snake_case::naive_snake_case;
use crate::proto_request_name::proto_request_name;
use crate::proto_request_params::proto_request_params;
use crate::proto_service::create::restricted_fields::proto_service_create_restricted_fields;
use crate::CodegenPackage;

mod restricted_fields;

pub fn proto_service_create(
    service: &ServiceDescriptorProto,
    messages: &HashMap<&str, DescriptorProto>,
    package: &CodegenPackage,
) -> TokenStream {
    if package.create.is_none() {
        return quote::quote! {};
    }

    let message = messages.get(&package.message).unwrap();

    let action = &service
        .method
        .clone()
        .into_iter()
        .find(|m| m.name() == package.create.unwrap());

    if let Some(action) = action {
        let proto_service_name = proto_request_name(action, messages);
        let proto_service_params = proto_request_params(action, messages);
        let message_name = quote::format_ident!("{}", message.name());

        let return_by_id =
            quote::format_ident!("return_{}_by_id", naive_snake_case(message.name()));

        let proto_service_create_restricted_fields =
            proto_service_create_restricted_fields(action, messages);

        return quote::quote! {
            async fn #proto_service_name {
                let MeExtension { user_id } = request.extensions().get::<MeExtension>().unwrap();

                #proto_service_params

                let mut query_builder = #message_name::query();

                let mut permissions = HashMap::new();
                permissions.insert(user_id, 2);
                query_builder.field_with_argument("permissions", Json(permissions));

                #( #proto_service_create_restricted_fields )*

                let sql = query_builder.insert_query();

                let rec = sqlx::query_with(sql.0.as_str(), sql.1)
                    .fetch_one(&self.pool)
                    .await
                    .expect("create error");

                self.#return_by_id(rec.get::<i32, _>("id"), *user_id).await
            }
        };
    }

    return quote::quote! {};
}
