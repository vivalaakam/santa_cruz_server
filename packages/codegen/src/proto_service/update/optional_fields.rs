use std::collections::HashMap;

use prost_types::{DescriptorProto, FieldDescriptorProto, MethodDescriptorProto};
use quote::__private::TokenStream;

pub fn proto_service_update_optional_fields(
    action: &MethodDescriptorProto,
    messages: &HashMap<&str, DescriptorProto>,
) -> Vec<TokenStream> {
    let MethodDescriptorProto { input_type, .. } = action.clone();

    let input_type = input_type.unwrap();

    let mut result = vec![];

    let req = messages
        .get(input_type.split(".").collect::<Vec<_>>().last().unwrap())
        .expect("input message not found");

    for field in &req.field {
        let FieldDescriptorProto { name, .. } = field.clone();
        let field_key = name.unwrap();
        if field_key != "id" {
            let field_value = quote::format_ident!("{}", field_key);

            result.push(quote::quote! {
                if let Some(#field_value) = #field_value {
                    query_builder.field_with_argument(#field_key, #field_value);
                }
            })
        }
    }

    return result;
}
