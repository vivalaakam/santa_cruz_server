use prost_types::{DescriptorProto, MethodDescriptorProto};
use quote::__private::TokenStream;
use std::collections::HashMap;

pub fn proto_request_params(
    action: &MethodDescriptorProto,
    messages: &HashMap<&str, DescriptorProto>,
) -> TokenStream {
    let MethodDescriptorProto { input_type, .. } = action.clone();

    let input_type = input_type.unwrap();

    let req = messages
        .get(input_type.split(".").collect::<Vec<_>>().last().unwrap())
        .expect("input message not found");

    let req_name = quote::format_ident!("{}", req.name());

    let req_fields = &req
        .field
        .to_vec()
        .into_iter()
        .map(|f| quote::format_ident!("{}", f.name.unwrap()))
        .collect::<Vec<_>>();

    quote::quote! {
        let #req_name { #(#req_fields ,)* } = request.get_ref();
    }
}
