use std::collections::HashMap;

use prost_types::{DescriptorProto, ServiceDescriptorProto};
use quote::__private::TokenStream;

use crate::naive_snake_case::naive_snake_case;
use crate::proto_request_name::proto_request_name;
use crate::proto_request_params::proto_request_params;
use crate::CodegenPackage;

pub fn proto_service_get(
    service: &ServiceDescriptorProto,
    messages: &HashMap<&str, DescriptorProto>,
    package: &CodegenPackage,
) -> TokenStream {
    if package.get.is_none() {
        return quote::quote! {};
    }

    let message = messages.get(&package.message).unwrap();

    let action = &service
        .method
        .clone()
        .into_iter()
        .find(|m| m.name() == package.get.unwrap());

    if let Some(action) = action {
        let proto_service_name = proto_request_name(action, messages);
        let proto_service_params = proto_request_params(action, messages);

        let return_by_id =
            quote::format_ident!("return_{}_by_id", naive_snake_case(message.name()));

        return quote::quote! {
            async fn #proto_service_name {
                let MeExtension { user_id } = request.extensions().get::<MeExtension>().unwrap();
                #proto_service_params

                self.#return_by_id(*id, *user_id).await
            }
        };
    }

    return quote::quote! {};
}
