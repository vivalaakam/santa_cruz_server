mod create;
mod delete;
mod get;
mod list;
pub mod messages;
mod update;

use std::collections::HashMap;

use prost_types::{DescriptorProto, ServiceDescriptorProto};
use quote::__private::TokenStream;

use crate::naive_snake_case::naive_snake_case;
use crate::proto_service::create::proto_service_create;
use crate::proto_service::delete::proto_service_delete;
use crate::proto_service::get::proto_service_get;
use crate::proto_service::list::proto_service_list;
use crate::proto_service::update::proto_service_update;
use crate::CodegenPackage;

pub fn proto_service(
    service: &ServiceDescriptorProto,
    messages: &HashMap<&str, DescriptorProto>,
    package: &CodegenPackage,
) -> TokenStream {
    let message = messages.get(&package.message).expect("oops");

    let snake = naive_snake_case(message.name());
    let service_name = quote::format_ident!("{}Service", message.name());
    let service_server = quote::format_ident!("{}_service_server", snake);

    let list_tokens = proto_service_list(service, messages, package);
    let get_tokens = proto_service_get(service, messages, package);
    let create_tokens = proto_service_create(service, messages, package);
    let update_tokens = proto_service_update(service, messages, package);
    let delete_tokens = proto_service_delete(service, messages, package);

    quote::quote! {
        #[tonic::async_trait]
        impl proto::santa_cruz::#service_server::#service_name for #service_name
        {
            #list_tokens

            #get_tokens

            #create_tokens

            #update_tokens

            #delete_tokens
        }
    }
}
