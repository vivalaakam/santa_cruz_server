use std::collections::{HashMap, HashSet};

use prost_types::{DescriptorProto, MethodDescriptorProto, ServiceDescriptorProto};
use quote::__private::Ident;

use crate::CodegenPackage;

pub fn proto_service_messages(
    service: &ServiceDescriptorProto,
    messages: &HashMap<&str, DescriptorProto>,
    package: &CodegenPackage,
) -> HashSet<Ident> {
    let mut result = HashSet::new();

    for method in &service.method {
        let MethodDescriptorProto {
            input_type,
            output_type,
            ..
        } = method.clone();

        result.insert(quote::format_ident!(
            "{}",
            input_type
                .unwrap()
                .split(".")
                .collect::<Vec<_>>()
                .last()
                .unwrap()
        ));

        result.insert(quote::format_ident!(
            "{}",
            output_type
                .unwrap()
                .split(".")
                .collect::<Vec<_>>()
                .last()
                .unwrap()
        ));
    }

    if let Some(message) = messages.get(package.message) {
        for field in &message.field {
            if let Some(type_name) = &field.type_name {
                result.insert(quote::format_ident!(
                    "{}",
                    type_name.split(".").collect::<Vec<_>>().last().unwrap()
                ));
            }
        }
    }

    result
}
