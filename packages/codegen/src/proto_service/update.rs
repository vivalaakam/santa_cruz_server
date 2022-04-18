use std::collections::HashMap;

use prost_types::{DescriptorProto, ServiceDescriptorProto};
use quote::__private::TokenStream;

use crate::naive_snake_case::naive_snake_case;
use crate::proto_request_name::proto_request_name;
use crate::proto_request_params::proto_request_params;
use crate::proto_service::update::optional_fields::proto_service_update_optional_fields;
use crate::CodegenPackage;

mod optional_fields;

pub fn proto_service_update(
    service: &ServiceDescriptorProto,
    messages: &HashMap<&str, DescriptorProto>,
    package: &CodegenPackage,
) -> TokenStream {
    if package.update.is_none() {
        return quote::quote! {};
    }

    let message = messages.get(&package.message).unwrap();

    let action = &service
        .method
        .clone()
        .into_iter()
        .find(|m| m.name() == package.update.unwrap());

    if let Some(action) = action {
        let proto_service_name = proto_request_name(action, messages);
        let proto_service_params = proto_request_params(action, messages);
        let message_name = quote::format_ident!("{}", message.name());

        let service_name = quote::format_ident!("{}Service", message.name());

        let return_by_id =
            quote::format_ident!("return_{}_by_id", naive_snake_case(message.name()));

        let get_by_id = quote::format_ident!("get_{}_by_id", naive_snake_case(message.name()));

        let optional_fields = proto_service_update_optional_fields(action, messages);

        return quote::quote! {
            async fn #proto_service_name {
                let MeExtension { user_id } = request.extensions().get::<MeExtension>().unwrap();

                #proto_service_params

                let original = #service_name::#get_by_id(&self.pool, *id, *user_id).await;

                if original.is_none() {
                    return Err(Status::not_found(format!(
                        "object #{} not found",
                        id.to_string()
                    )));
                }

                let mut query_builder = #message_name::query();

                #( #optional_fields )*

                if !query_builder.has_fields() {
                    return Ok(Response::new(original.unwrap()));
                }

                query_builder.field_with_argument("updated_at", Utc::now());

                query_builder.where_eq("id", id);

                let sql = query_builder.update_query();

                sqlx::query_with(sql.0.as_str(), sql.1)
                    .execute(&self.pool)
                    .await
                    .expect("update_workout_repeat error");

                self.#return_by_id(*id, *user_id).await
            }
        };
    }

    return quote::quote! {};
}
